use token::Token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    current_char: char,
}

impl Lexer {
    
    pub fn new(unprocessed_input: String) -> Self {
        Self { 
            input: unprocessed_input.chars().collect(),
            position: 0,
            read_position: 0,
            current_char: char::None,
        }
    }

}