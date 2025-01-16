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
                        while self.current < self.source.len()
                            && self.source.chars().nth(self.current) != Some('\n')
                        {
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
