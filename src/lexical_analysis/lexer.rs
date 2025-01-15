use crate::lexical_analysis::error::LexicalError;
use crate::lexical_analysis::token::Token;
use crate::lexical_analysis::token_type::TokenType;

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
                _ => errors.push(LexicalError::new(ch.to_string(), self.line)),
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
}
