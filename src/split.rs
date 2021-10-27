use std::ops::Range;

pub struct Line {
    pub number: usize,
    pub words: Vec<Range<usize>>,
}

/// Split input into lines and words.
pub struct Split {
    pub lines: Vec<Line>,
}

struct Splitter<'a> {
    /// Source file
    pub input: &'a str,
    /// Conditional compilation
    pub symbols: &'a [String],
    
    /// Holds source line number and range for every words
    pub lines: Vec<(usize, Vec<Range<usize>>)>,

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

impl<'a> Splitter {
    pub fn new(input: &'a str, symbols: &'a [String]) -> Self {
        Self {
            input,
            symbols,
            lines: vec![],
            cur_line: 0,
            source_line: 1,
        }
    }

    pub fn run(mut self) -> Split {
        for (i, c) in input.chars().enumerate() {
            if is_new_line(&c) {
                self.add_word();
                cur_line += 1;
                comment = false;
                source_line += 1;
            }else {
                if comment { continue }
                
                if c == '"' {
                    str_literal = !str_literal;
                    //TODO escape \" and newline and \;
                    if str_literal { 
                        self.add_word();
                        start = i;
                    }else { 
                        self.prepare_line();
                        lines[cur_line].1.push(start..i+1);
                        continue
                    }
                }

                if str_literal { continue }

                if is_space(&c) {
                    self.add_word();
                    continue
                }

                match c {
                    ';' => {
                        self.add_word();
                        comment = true;
                    }
                    '+' | '-' | '(' | ')' => {
                        self.prepare_line();
                        // Push the previous word
                        if has_word {
                            lines[cur_line].1.push(start..i);
                            has_word = false;
                        }
                        // Push the character
                        lines[cur_line].1.push(i..i+1);
                    }
                    _ => {
                        if !has_word { start = i; }
                        has_word = true;
                    }
                }
            }
        }

        Split{ lines }
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
                    if !self.dir_if || (self.dir_if && self.has_symbol) || (self.dir_else && !self.has_symbol) { 
                        self.prepare_line();
                        lines[self.cur_line].1.push(start..i);
                        self.has_word = false;
                    }
                }
            }
        }
    }

}

fn is_new_line(c: &char) -> bool {
    const CHARS: [char; 2] = ['\u{000A}', '\u{000D}'];
    CHARS.contains(c)
}

fn is_space(c: &char) -> bool {
    const CHARS: [char; 29] = [
        '\u{0009}', '\u{000B}', '\u{000C}', '\u{0020}', '\u{0085}',
        '\u{00A0}', '\u{1680}', '\u{180E}', '\u{2000}', '\u{2001}', 
        '\u{2002}', '\u{2003}', '\u{2004}', '\u{2005}', '\u{2006}',
        '\u{2007}', '\u{2008}', '\u{2009}', '\u{200A}', '\u{200B}',
        '\u{200C}', '\u{200D}', '\u{2028}', '\u{2029}', '\u{202F}',
        '\u{205F}', '\u{2060}', '\u{3000}', '\u{FEFF}',
    ];
    CHARS.contains(c)
}

fn is_ident_char(c: &char) -> bool {
    const CHARS: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
    CHARS.contains(c) || is_ident_first(c)
}

fn is_ident_first(c: &char) -> bool {
    const CHARS: [char; 53] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
        'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        '_',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
        'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    CHARS.contains(c)
}
