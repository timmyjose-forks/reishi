#[derive(Debug, PartialEq)]
pub enum Token {
    Character(char),
    Number(char),
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
    Unknown{c: char, position: usize,},
}