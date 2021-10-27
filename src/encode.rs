use ast::Token;

fn validate_ast(root: Token) -> bool {
    let mut valid = true; 
    //let mut opcodes = opcodes::get_opcodes();



    valid
}

fn hex_to_byte(hex: &str) -> u8 {
    let hex = hex.chars().collect::<Vec<_>>();
    (hex[0].to_digit(16).unwrap() * 16 + hex[1].to_digit(16).unwrap()) as u8
}

pub fn encode(ast: Token) -> Vec<u8> {
    todo!();
}
