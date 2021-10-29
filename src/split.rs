use utils;
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

impl<'a> Split<'a> {
    #[cfg(debug)]
    pub fn debug(&self) {
        println!("Split data:");
        for line in &self.lines {
            let mut n = line.number.to_string();
            if n.len() < 6 { n.push_str(&" ".repeat(7-n.len())); }
            print!("    L{}| ", n);

            for word in &line.words {
                print!("{} | ", &self.input[word.start..word.end]);
            }
            println!();
        }
        println!();
    }
}

impl<'a> Split<'a> {
    pub fn new(input: &'a str, symbols: &'a [String]) -> Split<'a> {
        Splitter::new(input, symbols).run()
    }
}

#[derive(Default)]
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
    pub dir_if: usize,
    pub dir_else: bool,

    /// Next word start index
    pub start: usize,
}

impl<'a> Splitter<'a> {
    fn new(input: &'a str, symbols: &'a [String]) -> Self {
        Self { input, symbols, source_line: 1, ..Default::default() }
    }

    fn run(mut self) -> Split<'a> {
        for (i, c) in self.input.chars().enumerate() {
            if utils::is_new_line(&c) {
                self.add_word(i);
                self.cur_line += 1;
                self.comment = false;
                self.source_line += 1;
            }else {
                if self.comment { continue }
                
                if c == '"' {
                    self.str_literal = !self.str_literal;
                    //TODO escape \" and newline and \;
                    if self.str_literal { 
                        self.add_word(i);
                        self.start = i;
                    }else { 
                        self.prepare_line();
                        self.lines[self.cur_line].words.push(self.start..i+1);
                        continue
                    }
                }

                if self.str_literal { continue }

                if utils::is_space(&c) {
                    self.add_word(i);
                    continue
                }

                match c {
                    ';' => {
                        self.add_word(i);
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

        Split{ input: self.input, lines: self.lines }
    }

    /// Create a new line if necessary
    fn prepare_line(&mut self) {
        if self.lines.len() < self.cur_line+1 {
            self.lines.push(Line{ number: self.source_line, words: vec![] }); 
            self.cur_line = self.lines.len()-1;
        }
    }

    /// Add word range to lines
    fn add_word(&mut self, end: usize) {
        if self.has_word {
            let word = self.input.get(self.start..end).unwrap();

            if self.dir_if != 0 { self.has_symbol = self.symbols.contains(&word.to_string()); }

            match word {
                "#if" => self.dir_if = self.cur_line,
                "#else" => self.dir_else = true,
                "#endif" => {
                    self.dir_if = 0;
                    self.dir_else = false;
                }
                _ => {
                    let mut push = self.dir_if == 0 || (self.dir_if == 0 && !self.has_symbol);
                    if self.dir_if != 0 && self.has_symbol && self.dir_if != self.cur_line {
                        push = true;
                    }

                    if push { 
                        self.prepare_line();
                        self.lines[self.cur_line].words.push(self.start..end);
                    }
                }
            }


            self.has_word = false;
        }
    }

}

