use super::token::Token as Token;
use super::token::TokenType as TokenType;

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
            '+' => {
                Token::new(TokenType::Plus, self.position, self.current_char)
            },
            '-' => {
                Token::new(TokenType::Minus, self.position, self.current_char)
            },
            '*' => {
                Token::new(TokenType::Multiply, self.position, self.current_char)
            }
            '/' => {
                Token::new(TokenType::Divide, self.position, self.current_char)
            },
            '=' => {
                Token::new(TokenType::Assign, self.position, self.current_char)
            },
            '!' => {
                Token::new(TokenType::Bang, self.position, self.current_char)
            },
            '>' => {
                Token::new(TokenType::GreaterThan, self.position, self.current_char)
            },
            '<' => {
                Token::new(TokenType::LessThan, self.position, self.current_char)
            },
            '&' => {
                Token::new(TokenType::And, self.position, self.current_char)
            },
            '|' => {
                Token::new(TokenType::Pipe, self.position, self.current_char)
            },
            ';' => {
                Token::new(TokenType::Semicolon, self.position, self.current_char)
            },
            '(' => {
                Token::new(TokenType::LeftParen, self.position, self.current_char)
            },
            ')' => {
                Token::new(TokenType::RightParen, self.position, self.current_char)
            },
            '{' => {
                Token::new(TokenType::LeftBracket, self.position, self.current_char)
            },
            '}' => {
                Token::new(TokenType::RightBracket, self.position, self.current_char)
            },
            ':' => {
                Token::new(TokenType::Colon, self.position, self.current_char)
            },
            '[' => {
                Token::new(TokenType::LeftSquare, self.position, self.current_char)
            },
            ']' => {
                Token::new(TokenType::RightSquare, self.position, self.current_char)
            },
            '"' => {
                Token::new(TokenType::Quote, self.position, self.current_char)
            },
            ',' => {
                Token::new(TokenType::Comma, self.position, self.current_char)
            },
            '?' => {
                Token::new(TokenType::QuestionMark, self.position, self.current_char)
            },
            '.' => {
                Token::new(TokenType::Period, self.position, self.current_char)
            },
            '_' => {
                Token::new(TokenType::Underscore, self.position, self.current_char)
            },
            '#' => {
                Token::new(TokenType::Pound, self.position, self.current_char)
            },
            '$' => {
                Token::new(TokenType::Dollar, self.position, self.current_char)
            },
            '%' => {
                Token::new(TokenType::Percent, self.position, self.current_char)
            },
            _ => {
                if is_letter(self.current_char) {
                    Token::new(TokenType::Character, self.position, self.current_char)
                }else if is_number(self.current_char) {
                    Token::new(TokenType::Number, self.position, self.current_char)
                }else{
                    Token::new(TokenType::Unknown, self.position, self.current_char)
                }
            },
        }
    }
}
