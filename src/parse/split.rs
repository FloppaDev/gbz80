
use crate::{
    parse::{
        text::charset,
        lex,
    },
    program::error::{SplitErr, SplitErrType},
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

/// Splits source file into words and stores original lines along with their numbers.
pub struct Split<'a> {
    /// String slice of a whole line.
    lines: Vec<&'a str>,

    /// Maps line indices to line numbers from the source file.
    line_numbers: Vec<usize>,

    /// Word slices along with their line index.
    words: Vec<Word<'a>>,
}

impl<'a> Split<'a> {

    pub fn line(&self, index: LineIndex) -> &'a str {
        self.lines[index.value]
    }

    pub fn line_number(&self, index: LineIndex) -> usize {
        self.line_numbers[index.value]
    }

    pub fn words(&self) -> &[Word<'a>] {
        &self.words
    }

    /// Split source file into lines and words.
    pub fn new(
        input: &'a str, 
        symbols: &[&'a str],
    ) -> Result<Split<'a>, Vec<SplitErr<'a>>> {
        let mut errors = vec![];

        let mut str_literal = false; 
        let mut word_start = 0;
        let mut has_word = false;
        let mut dir_line = false;

        let mut splitter = Splitter {
            input,
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
                        if let Some(word) = line.get(word_start..c_i) {
                            splitter.push(word, line, l_i);
                            has_word = false;
                        }

                        else {
                            errors.push(SplitErr::new(
                                SplitErrType::InvalidWord, line, l_i + 1));
                        }
                    }

                    str_literal = !str_literal;

                    if str_literal {
                        word_start = c_i; 
                    }

                    else if let Some(word) = line.get(word_start..c_i) {
                        splitter.push(word, line, l_i);
                    }

                    else {
                        errors.push(SplitErr::new(
                            SplitErrType::InvalidWord, line, l_i + 1));
                    }

                    continue
                }

                if str_literal {
                    continue
                }

                if ch == ';' {
                    break
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
                            if let Some(word) = line.get(word_start..c_i) {
                                splitter.push(word, line, l_i);
                                has_word = false;
                            }

                            else {
                                errors.push(SplitErr::new(
                                    SplitErrType::InvalidWord, line, l_i + 1));
                            }
                        }

                        // Push character.
                        if let Some(word) = line.get(c_i..=c_i) {
                            splitter.push(word, line, l_i);
                        }

                        else {
                            errors.push(SplitErr::new(
                                SplitErrType::InvalidWord, line, l_i + 1));
                        }
                    }

                    else if charset::is_space(ch) {
                        if has_word { 
                            if let Some(word) = line.get(word_start..c_i) {
                                splitter.push(word, line, l_i);
                                has_word = false;
                            }

                            else {
                                errors.push(SplitErr::new(
                                    SplitErrType::InvalidWord, line, l_i + 1));
                            }
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
                if let Some(word) = line.get(word_start..) {
                    splitter.push(word, line, l_i);
                    has_word = false;
                }

                else {
                    errors.push(SplitErr::new(
                        SplitErrType::InvalidWord, line, l_i + 1));
                }
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
        Ok(Split { lines, line_numbers, words })
    }

    #[cfg(debug_assertions)]
    pub fn debug(&self) {
        title("Split words");
        
        if self.line_numbers.is_empty() {
            return
        }

        let mut line_number = self.line_numbers[0];
        print!("L{}\t│", line_number);

        for word in &self.words {
            let ln = self.line_number(word.line_index);

            if line_number != ln {
                print!("\nL{}\t│", ln);
                line_number = ln;
            }

            print!("{}│", word.value);
        }

        println!("\n");
    }

}

struct Splitter<'a> {
    input: &'a str,
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

    fn push(&mut self, value: &'a str, line: &'a str, line_index: usize) {
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


