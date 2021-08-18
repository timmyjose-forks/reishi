pub mod reishi;
use crate::reishi::*;


#[cfg(test)]
mod tests {
    #[test]
    fn basic_token_test() {
        let mut l = crate::reishi::lexer::Lexer::new(".".to_string());
        let test_token = l.read_char();
        assert_eq!(test_token, crate::reishi::token::Token::Period)
    }
    #[test]
    fn is_number_test() {
        let mut l = crate::reishi::lexer::Lexer::new('1'.to_string());
        let test_token = l.read_char();
        assert_eq!(test_token, crate::reishi::token::Token::Number('1'))
    }

    #[test]
    fn is_letter_test() {
        let mut l = crate::reishi::lexer::Lexer::new('c'.to_string());
        let test_token = l.read_char();
        assert_eq!(test_token, crate::reishi::token::Token::Character('c'))
    }

    #[test]
    fn detect_unknown_test() {
        let mut l = crate::reishi::lexer::Lexer::new('@'.to_string());
        let test_token = l.read_char();
        assert_eq!(test_token, crate::reishi::token::Token::Unknown{c: '@', position: 0})
    }
}