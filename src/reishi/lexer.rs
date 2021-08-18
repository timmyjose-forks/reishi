use super::token::Token as Token;

pub fn is_number(input: char) -> bool {
    '0' <= input && input <= '9'
}

pub fn is_letter(input: char) -> bool {
    'a' <= input && input <= 'z' || 'A' <= input && input <= 'Z'
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: char,
}

impl Lexer {
    
    pub fn new(unprocessed_input: String) -> Self {
        Self { 
            input: unprocessed_input.chars().collect(),
            position: 0,
            current_char: unprocessed_input.chars().nth(0).unwrap(),
        }
    }

    pub fn next_char(&mut self) {
        self.position = self.position + 1;
        self.current_char = self.input[self.position];
    }

    pub fn read_char(&mut self) -> Token {
        match self.current_char {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Multiply,
            '/' => Token::Divide,
            '=' => Token::Assign,
            '!' => Token::Bang,
            '>' => Token::GreaterThan,
            '<' => Token::LessThan,
            '&' => Token::And,
            '|' => Token::Pipe,
            ';' => Token::Semicolon,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftBracket,
            '}' => Token::RightBracket,
            ':' => Token::Colon,
            '[' => Token::LeftSquare,
            ']' => Token::RightSquare,
            '"' => Token::Quote,
            ',' => Token::Comma,
            '?' => Token::QuestionMark,
            '.' => Token::Period,
            '_' => Token::Underscore,
            '#' => Token::Pound,
            '$' => Token::Dollar,
            '%' => Token::Percent,
            _ => {
                if is_letter(self.current_char) {
                    Token::Character(self.current_char)
                }else if is_number(self.current_char) {
                    Token::Number(self.current_char)
                }else{
                    Token::Unknown{c: self.current_char, position: self.position}
                }
            },
        }
    }
}
