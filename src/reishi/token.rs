pub struct Token {
    pub token_type: TokenType,
    pub token_place: usize,
    pub token_char: char,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Character,
    Number,
    Plus,
    Minus,
    Multiply,
    Divide,
    Assign,
    Bang,
    GreaterThan,
    LessThan,
    And,
    Pipe,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Colon,
    LeftSquare,
    RightSquare,
    Quote,
    Comma,
    QuestionMark,
    Period,
    Underscore,
    Pound,
    Dollar,
    Percent,
    Unknown,
}

impl Token {
    
    pub fn new(token_type: TokenType, token_place: usize, token_char: char) -> Self {
        Self {
            token_type,
            token_place,
            token_char,
        }
    }

}