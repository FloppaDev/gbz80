use std::ops::Range;

pub struct Line {
    pub number: usize,
    pub words: Vec<Range<usize>>,
}

/// Split input into lines and words.
pub struct Split<'a> {
    pub input: &'a str,
    pub lines: Vec<Line>,
}

struct Splitter<'a> {
    /// Source file
    pub input: &'a str,
    /// Conditional compilation
    pub symbols: &'a [String],
    
    /// Holds source line number and range for every words
    pub lines: Vec<Line>,

    /// Current index in lines
    pub cur_line: usize,
    /// Line of the word in the source file
    pub source_line: usize,

    pub comment: bool,
    pub str_literal: bool,

    /// Has a word started
    pub has_word: bool,
    pub has_symbol: bool,
    pub dir_if: bool,
    pub dir_else: bool,

    /// Next word start index
    pub start: usize,
}

impl<'a> Splitter<'a> {
    pub fn new(input: &'a str, symbols: &'a [String]) -> Self {
        Self { input, symbols, lines: vec![], cur_line: 0, source_line: 1 }
    }

    pub fn run(mut self) -> Split<'a> {
        for (i, c) in input.chars().enumerate() {
            if is_new_line(&c) {
                self.add_word();
                self.cur_line += 1;
                self.comment = false;
                self.source_line += 1;
            }else {
                if self.comment { continue }
                
                if c == '"' {
                    self.str_literal = !self.str_literal;
                    //TODO escape \" and newline and \;
                    if self.str_literal { 
                        self.add_word();
                        self.start = i;
                    }else { 
                        self.prepare_line();
                        self.lines[self.cur_line].words.push(self.start..i+1);
                        continue
                    }
                }

                if self.str_literal { continue }

                if is_space(&c) {
                    self.add_word();
                    continue
                }

                match c {
                    ';' => {
                        self.add_word();
                        self.comment = true;
                    }
                    '+' | '-' | '(' | ')' => {
                        self.prepare_line();
                        // Push the previous word
                        if self.has_word {
                            self.lines[self.cur_line].words.push(self.start..i);
                            self.has_word = false;
                        }
                        // Push the character
                        self.lines[self.cur_line].words.push(i..i+1);
                    }
                    _ => {
                        if !self.has_word { self.start = i; }
                        self.has_word = true;
                    }
                }
            }
        }

        Split{ input, lines }
    }

    /// Create a new line if necessary
    fn prepare_line(&mut self) {
        if self.lines.len() < self.cur_line+1 {
            self.lines.push(Line{ number: self.source_line, words: vec![] }); 
            self.cur_line = self.lines.len()-1;
        }
    }

    /// Add word range to lines
    fn add_word(&mut self) {
        if self.has_word {
            let word = input.get(start..i).unwrap();

            if self.dir_if { self.has_symbol = symbols.contains(&word.to_string()); }

            match word {
                "#if" => self.dir_if = true,
                "#else" => self.dir_else = true,
                "endif" => {
                    self.dir_if = false;
                    self.dir_else = false;
                }
                _ => {
                    if !self.dir_if
                        || (self.dir_if && self.has_symbol) 
                        || (self.dir_else && !self.has_symbol)
                    { 
                        self.prepare_line();
                        lines[self.cur_line].1.push(start..i);
                        self.has_word = false;
                    }
                }
            }
        }
    }

}

