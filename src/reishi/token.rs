pub enum Token {
    Identifier(Vec<char>),
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
    Apostrophe,
    Comma,
    QuestionMark,
    Period,
    Underscore,
    Pound,
    Dollar,
    Percent,
}