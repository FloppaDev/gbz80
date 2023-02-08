
use crate::{
    parse::{ lex, source::Input },
    error::init::{SplitErr, SplitErrType},
};

#[cfg(debug_assertions)]
use crate::program::fmt::title;

#[derive(Copy, Clone)]
pub struct LineIndex {
    value: usize,
}

pub struct Word<'a> {
    pub line_index: LineIndex,
    pub value: &'a str,
}

impl<'a> Word<'a> {

    fn read(&self, split: &Split<'a>) -> (&'a str, &'a str, usize, &'a str) {
        let line = split.lines[self.line_index.value]; 
        let line_number = split.line_numbers[self.line_index.value]; 

        (self.value, line, line_number, split.file) 
    }

}

/// Splits source file into words and stores original lines along with their numbers.
pub struct Split<'a> {
    file: &'a str,
    /// String slice of a whole line.
    lines: Vec<&'a str>,
    /// Maps line indices to line numbers from the source file.
    line_numbers: Vec<usize>,
    /// Word slices along with their line index.
    words: Vec<Word<'a>>,
}

impl<'a> Split<'a> {

    pub fn line_number(&self, index: LineIndex) -> usize {
        self.line_numbers[index.value]
    }

    pub fn words(&self) -> Vec<(&'a str, &'a str, usize, &'a str)> {
        self.words.iter().map(|w| w.read(self)).collect::<Vec<_>>()
    }

    /// Split source file into lines and words.
    pub fn new(
        input: &'a Input,
        symbols: &[&'a str],
    ) -> Result<Split<'a>, Vec<SplitErr<'a>>> {
        let mut errors = vec![];

        let mut str_literal = false; 
        let mut word_start = 0;
        let mut has_word = false;
        let mut dir_line = false;

        let mut splitter = Splitter {
            lines: vec![],
            line_numbers: vec![],
            words: vec![],
            current_line: usize::MAX,
            push_line: false,
            line_count: 0,
            directive: vec![],
            process: true,
        };

        for (l_i, line) in input.lines().enumerate() {
            for (c_i, ch) in line.chars().enumerate() {
                // String literal.
                 if ch == '"' {
                    // Push current word, or the string literal that just ended.
                    if has_word {
                        line.get(word_start..c_i).map_or_else(|| {
                            errors.push(SplitErr::new(
                                SplitErrType::InvalidWord, line, l_i + 1));
                        }, |word| {
                            splitter.push(word, l_i);
                            has_word = false;
                        });
                    }

                    str_literal = !str_literal;

                    if str_literal {
                        word_start = c_i; 
                    }

                    else {
                        line.get(word_start..c_i).map_or_else(|| {
                            errors.push(SplitErr::new(
                                SplitErrType::InvalidWord, line, l_i + 1));
                        }, |word| {
                            splitter.push(word, l_i);
                        });
                    }

                    continue;
                }

                if str_literal {
                    continue;
                }

                if ch == ';' {
                    break;
                }

                // Has a directive started?
                if ch == '#' {
                    // There can only be whitespace before.
                    if let Some(preceding) = line.get(..c_i) {
                        if preceding.trim().is_empty() {
                            dir_line = true;
                        }
                    }

                    else {
                        errors.push(SplitErr::new(
                            SplitErrType::MisplacedDirective, line, l_i + 1));
                        break;
                    }
                }

                if splitter.process || dir_line {
                    // Those are always treated as words, regardless of spaces.
                    if lex::is_char_word(ch) {
                        // Push current word.
                        if has_word { 
                            line.get(word_start..c_i).map_or_else(|| {
                                errors.push(SplitErr::new(
                                    SplitErrType::InvalidWord, line, l_i + 1));
                            }, |word| {
                                splitter.push(word, l_i);
                                has_word = false;
                            });
                        }

                        // Push character.
                        line.get(c_i..=c_i).map_or_else(|| {
                            errors.push(SplitErr::new(
                                SplitErrType::InvalidWord, line, l_i + 1));
                        }, |word| {
                            splitter.push(word, l_i);
                        });
                    }

                    else if ch.is_whitespace() {
                        if has_word { 
                            line.get(word_start..c_i).map_or_else(|| {
                                errors.push(SplitErr::new(
                                    SplitErrType::InvalidWord, line, l_i + 1));
                            }, |word| {
                                splitter.push(word, l_i);
                                has_word = false;
                            });
                        }
                    }

                    else if !has_word {
                        word_start = c_i;
                        has_word = true;
                    }
                }    
            } 

            // End of the line, push the current word.
            if has_word { 
                line.get(word_start..).map_or_else(|| {
                    errors.push(SplitErr::new(
                        SplitErrType::InvalidWord, line, l_i + 1));
                }, |word| {
                    splitter.push(word, l_i);
                    has_word = false;
                });
            }

            // Push the line if any word was pushed.
            if splitter.push_line {
                splitter.line_numbers.push(l_i + 1);
                splitter.lines.push(line.trim());

                splitter.push_line = false;
            }

            // Is there a directive to follow?
            if !splitter.directive.is_empty() {
                if splitter.directive[0] == "#endif" {
                    splitter.process = true;
                }

                else if splitter.directive[0] == "#else" {
                    splitter.process = !splitter.process;
                }

                else if splitter.directive.len() == 2 && splitter.directive[0] == "#if" {
                    splitter.process = symbols.contains(&splitter.directive[1]);
                }

                else {
                    errors.push(SplitErr::new(
                        SplitErrType::InvalidDirective, line, l_i + 1));
                }

                splitter.directive.clear();
            }

            // Line ended, reset values.
            word_start = 0;
            dir_line = false;
        }

        let Splitter{ lines, line_numbers, words, .. } = splitter;
        Ok(Split { file: &input.path, lines, line_numbers, words })
    }

    #[cfg(debug_assertions)]
    pub fn debug(&self) {
        title("Split words");
        
        if self.line_numbers.is_empty() {
            return;
        }

        let mut line_number = self.line_numbers[0];
        print!("L{line_number}\t│");

        for word in &self.words {
            let ln = self.line_number(word.line_index);

            if line_number != ln {
                print!("\nL{ln}\t│");
                line_number = ln;
            }

            print!("{}│", word.value);
        }

        println!("\n");
    }

}

struct Splitter<'a> {
    words: Vec<Word<'a>>,
    lines : Vec<&'a str>,
    line_numbers: Vec<usize>,
    current_line: usize,
    push_line: bool,
    line_count: usize,
    directive: Vec<&'a str>,
    process: bool,
}

impl<'a> Splitter<'a> {

    fn push(&mut self, value: &'a str, line_index: usize) {
        // Directive will be processed once the line has ended.
        if !self.directive.is_empty() || matches!(value, "#if"|"#else"|"#endif") {
            self.directive.push(value);
            return
        }

        // Request a new line if necessary.
        if self.current_line != line_index {
            self.push_line = true;
            self.current_line = line_index;
            self.line_count += 1;
        }

        let word = Word { 
            line_index: LineIndex { value: self.line_count - 1 },
            value 
        };

        self.words.push(word);
    }

}


