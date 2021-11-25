/// Empty string
macro_rules! mt { () => { String::new() } }

pub fn is_new_line(c: &char) -> bool {
    const CHARS: [char; 2] = ['\u{000A}', '\u{000D}'];
    CHARS.contains(c)
}

pub fn is_space(c: &char) -> bool {
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

pub fn is_ident_char(c: &char) -> bool {
    is_num(c) || is_ident_first(c)
}

pub fn is_num(c: &char) -> bool {
    const CHARS: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
    CHARS.contains(c)
}

pub fn is_ident_first(c: &char) -> bool {
    const CHARS: [char; 53] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
        'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        '_',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
        'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    CHARS.contains(c)
}

#[cfg(feature = "debug")]
pub fn debug_title(title: &str) {
    let decoration = "=".repeat(79);
    println!("{}\n\t\t\t\t{}\n{}", decoration, title, decoration);
}

pub fn parse_dec(dec: &str) -> usize {
    //TODO handle error.
    str::parse::<usize>(dec).unwrap()
}
