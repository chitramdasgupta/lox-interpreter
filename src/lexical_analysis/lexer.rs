use crate::lexical_analysis::error::{LexicalError, LexicalErrorType};
use crate::lexical_analysis::token::{Literal, Token};
use crate::lexical_analysis::token_type::TokenType;
use std::str::FromStr;

#[allow(unused)]
pub struct Lexer {
    /// The entire source code string.
    source: String,
    /// The index of the first character in the lexeme being processed.
    start: usize,
    /// The index of the current character in the lexeme being processed.
    current: usize,
    /// The line number to which the current lexeme belongs.
    line: usize,
    /// A vector to store any lexical errors encountered during scanning.
    errors: Vec<LexicalError>,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            start: 0,
            current: 0,
            line: 1,
            errors: Vec::new(),
        }
    }

    #[allow(unused)]
    pub fn scan_tokens(&mut self) -> (Vec<Token>, Vec<LexicalError>) {
        let mut tokens = Vec::new();
        let mut errors = Vec::new();

        while self.current < self.source.len() {
            self.start = self.current;
            let ch = self.advance();

            match ch {
                '(' => tokens.push(Token::new(
                    TokenType::LeftParen,
                    ch.to_string(),
                    None,
                    self.line,
                )),
                ')' => tokens.push(Token::new(
                    TokenType::RightParen,
                    ch.to_string(),
                    None,
                    self.line,
                )),
                '{' => tokens.push(Token::new(
                    TokenType::LeftBrace,
                    ch.to_string(),
                    None,
                    self.line,
                )),
                '}' => tokens.push(Token::new(
                    TokenType::RightBrace,
                    ch.to_string(),
                    None,
                    self.line,
                )),
                ',' => tokens.push(Token::new(
                    TokenType::Comma,
                    ch.to_string(),
                    None,
                    self.line,
                )),
                '.' => tokens.push(Token::new(TokenType::Dot, ch.to_string(), None, self.line)),
                '-' => tokens.push(Token::new(
                    TokenType::Minus,
                    ch.to_string(),
                    None,
                    self.line,
                )),
                '+' => tokens.push(Token::new(TokenType::Plus, ch.to_string(), None, self.line)),
                ';' => tokens.push(Token::new(
                    TokenType::Semicolon,
                    ch.to_string(),
                    None,
                    self.line,
                )),
                '*' => tokens.push(Token::new(TokenType::Star, ch.to_string(), None, self.line)),
                '=' => {
                    if self.advance_if_equal('=') {
                        tokens.push(Token::new(
                            TokenType::EqualEqual,
                            "==".to_string(),
                            None,
                            self.line,
                        ))
                    } else {
                        tokens.push(Token::new(
                            TokenType::Equal,
                            ch.to_string(),
                            None,
                            self.line,
                        ))
                    }
                }
                '!' => {
                    if self.advance_if_equal('=') {
                        tokens.push(Token::new(
                            TokenType::BangEqual,
                            "!=".to_string(),
                            None,
                            self.line,
                        ))
                    } else {
                        tokens.push(Token::new(TokenType::Bang, ch.to_string(), None, self.line))
                    }
                }
                '>' => {
                    if self.advance_if_equal('=') {
                        tokens.push(Token::new(
                            TokenType::GreaterEqual,
                            ">=".to_string(),
                            None,
                            self.line,
                        ))
                    } else {
                        tokens.push(Token::new(
                            TokenType::Greater,
                            ch.to_string(),
                            None,
                            self.line,
                        ))
                    }
                }
                '<' => {
                    if self.advance_if_equal('=') {
                        tokens.push(Token::new(
                            TokenType::LessEqual,
                            "<=".to_string(),
                            None,
                            self.line,
                        ))
                    } else {
                        tokens.push(Token::new(TokenType::Less, ch.to_string(), None, self.line))
                    }
                }
                '/' => {
                    if self.advance_if_equal('/') {
                        while self.current < self.source.len() && !self.current_char_matches('\n') {
                            self.current += 1;
                        }
                    } else {
                        tokens.push(Token::new(
                            TokenType::Slash,
                            ch.to_string(),
                            None,
                            self.line,
                        ))
                    }
                }
                '\n' => self.line += 1,
                ch if ch.is_whitespace() => continue,
                '"' => {
                    while self.current < self.source.len() && !self.current_char_matches('"') {
                        // Support multi-line strings
                        if self.current_char_matches('\n') {
                            self.line += 1;
                        }

                        self.advance();
                    }

                    if self.current >= self.source.len() {
                        errors.push(LexicalError::new(
                            LexicalErrorType::UnterminatedString,
                            self.line,
                        ))
                    } else {
                        self.advance();

                        tokens.push(Token::new(
                            TokenType::String,
                            self.source[self.start..self.current].to_string(),
                            Some(Literal::String(
                                self.source[self.start + 1..self.current - 1].to_string(),
                            )),
                            self.line,
                        ))
                    }
                }
                ch if ch.is_ascii_digit() => {
                    while self.current < self.source.len()
                        && self
                            .source
                            .chars()
                            .nth(self.current)
                            .unwrap()
                            .is_ascii_digit()
                    {
                        self.advance();
                    }

                    if self.current_char_matches('.')
                        && self.current + 1 < self.source.len()
                        && self
                            .source
                            .chars()
                            .nth(self.current + 1)
                            .unwrap()
                            .is_ascii_digit()
                    {
                        self.advance();

                        while self.current < self.source.len()
                            && self
                                .source
                                .chars()
                                .nth(self.current)
                                .unwrap()
                                .is_ascii_digit()
                        {
                            self.advance();
                        }
                    }

                    tokens.push(Token::new(
                        TokenType::Number,
                        self.source[self.start..self.current].to_string(),
                        match f64::from_str(&self.source[self.start..self.current]) {
                            Ok(number) => Some(Literal::Number(number)),
                            Err(_) => None,
                        },
                        self.line,
                    ))
                }
                ch if ch.is_alphanumeric() || ch == '_' => {
                    while self.current < self.source.len()
                        && (self.current_char_matches('_')
                            || self
                                .source
                                .chars()
                                .nth(self.current)
                                .unwrap()
                                .is_alphanumeric())
                    {
                        self.advance();
                    }

                    let text = &self.source[self.start..self.current];
                    let typ =
                        TokenType::from_keyword(text).unwrap_or_else(|| TokenType::Identifier);

                    tokens.push(Token::new(typ, text.to_string(), None, self.line))
                }
                _ => errors.push(LexicalError::new(
                    LexicalErrorType::UnexpectedCharacter(ch),
                    self.line,
                )),
            }
        }

        tokens.push(Token::new(TokenType::Eof, "".to_string(), None, self.line));

        (tokens, errors)
    }

    fn advance(&mut self) -> char {
        let curr_char = self.source.chars().nth(self.current).unwrap();
        self.current += 1;

        curr_char
    }

    fn advance_if_equal(&mut self, expected: char) -> bool {
        if self.current >= self.source.len() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() == expected {
            self.current += 1;
            true
        } else {
            false
        }
    }

    fn current_char_matches(&mut self, expected: char) -> bool {
        if self.current >= self.source.len()
            || self.source.chars().nth(self.current) != Some(expected)
        {
            false
        } else {
            true
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lexical_analysis::token::{Literal, Token};
    use crate::lexical_analysis::token_type::TokenType;

    fn assert_tokens(source: &str, expected_tokens: Vec<Token>) {
        let mut lexer = Lexer::new(source);
        let (tokens, errors) = lexer.scan_tokens();

        assert!(errors.is_empty(), "Unexpected lexical errors: {:?}", errors);
        assert_eq!(
            tokens.len(),
            expected_tokens.len(),
            "Token count mismatch. Expected {} tokens, got {}",
            expected_tokens.len(),
            tokens.len()
        );

        for (actual, expected) in tokens.iter().zip(expected_tokens.iter()) {
            assert_eq!(
                actual, expected,
                "\nExpected token: {:?}\nActual token: {:?}",
                expected, actual
            );
        }
    }

    #[test]
    fn test_single_character_tokens() {
        let source = "(){},.-+;*";
        let expected = vec![
            Token::new(TokenType::LeftParen, "(".to_string(), None, 1),
            Token::new(TokenType::RightParen, ")".to_string(), None, 1),
            Token::new(TokenType::LeftBrace, "{".to_string(), None, 1),
            Token::new(TokenType::RightBrace, "}".to_string(), None, 1),
            Token::new(TokenType::Comma, ",".to_string(), None, 1),
            Token::new(TokenType::Dot, ".".to_string(), None, 1),
            Token::new(TokenType::Minus, "-".to_string(), None, 1),
            Token::new(TokenType::Plus, "+".to_string(), None, 1),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 1),
            Token::new(TokenType::Star, "*".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];
        assert_tokens(source, expected);
    }

    #[test]
    fn test_one_or_two_character_tokens() {
        let source = "! != = == > >= < <= //";
        let expected = vec![
            Token::new(TokenType::Bang, "!".to_string(), None, 1),
            Token::new(TokenType::BangEqual, "!=".to_string(), None, 1),
            Token::new(TokenType::Equal, "=".to_string(), None, 1),
            Token::new(TokenType::EqualEqual, "==".to_string(), None, 1),
            Token::new(TokenType::Greater, ">".to_string(), None, 1),
            Token::new(TokenType::GreaterEqual, ">=".to_string(), None, 1),
            Token::new(TokenType::Less, "<".to_string(), None, 1),
            Token::new(TokenType::LessEqual, "<=".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];
        assert_tokens(source, expected);
    }

    #[test]
    fn test_string_literals() {
        let source = r#""Hello, World!" "Multi
line
string""#;
        let expected = vec![
            Token::new(
                TokenType::String,
                "\"Hello, World!\"".to_string(),
                Some(Literal::String("Hello, World!".to_string())),
                1,
            ),
            Token::new(
                TokenType::String,
                "\"Multi\nline\nstring\"".to_string(),
                Some(Literal::String("Multi\nline\nstring".to_string())),
                3,
            ),
            Token::new(TokenType::Eof, "".to_string(), None, 3),
        ];
        assert_tokens(source, expected);
    }

    #[test]
    fn test_number_literals() {
        let source = "123 123.456 0.123";
        let expected = vec![
            Token::new(
                TokenType::Number,
                "123".to_string(),
                Some(Literal::Number(123.0)),
                1,
            ),
            Token::new(
                TokenType::Number,
                "123.456".to_string(),
                Some(Literal::Number(123.456)),
                1,
            ),
            Token::new(
                TokenType::Number,
                "0.123".to_string(),
                Some(Literal::Number(0.123)),
                1,
            ),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];
        assert_tokens(source, expected);
    }

    #[test]
    fn test_keywords_and_identifiers() {
        let source = "var number = 42;";
        let expected = vec![
            Token::new(TokenType::Var, "var".to_string(), None, 1),
            Token::new(TokenType::Identifier, "number".to_string(), None, 1),
            Token::new(TokenType::Equal, "=".to_string(), None, 1),
            Token::new(
                TokenType::Number,
                "42".to_string(),
                Some(Literal::Number(42.0)),
                1,
            ),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];
        assert_tokens(source, expected);
    }
}
