pub mod reishi;

#[cfg(test)]
mod tests {
    #[test]
    fn basic_token_test() {
        let mut l = crate::reishi::lexer::Lexer::new('.'.to_string());
        let test_token = l.read_char();
        assert_eq!(
            test_token.token_type,
            crate::reishi::token::TokenType::Period
        )
    }
    #[test]
    fn is_number_test() {
        let mut l = crate::reishi::lexer::Lexer::new('1'.to_string());
        let test_token = l.read_char();
        assert_eq!(
            test_token.token_type,
            crate::reishi::token::TokenType::Number
        )
    }

    #[test]
    fn is_letter_test() {
        let mut l = crate::reishi::lexer::Lexer::new('c'.to_string());
        let test_token = l.read_char();
        assert_eq!(
            test_token.token_type,
            crate::reishi::token::TokenType::Character
        )
    }

    #[test]
    fn position_test() {
        let mut l = crate::reishi::lexer::Lexer::new('a'.to_string());
        let test_token = l.read_char();
        assert_eq!(test_token.token_place, 0)
    }
}