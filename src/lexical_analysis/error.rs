#[derive(Debug, Clone)]
pub struct LexicalError {
    typ: LexicalErrorType,
    line: usize,
}

#[derive(Debug, Clone)]
pub enum LexicalErrorType {
    UnexpectedCharacter(char),
    UnterminatedString,
}

impl LexicalError {
    pub fn new(typ: LexicalErrorType, line: usize) -> Self {
        Self { typ, line }
    }
}

impl std::fmt::Display for LexicalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.typ {
            LexicalErrorType::UnexpectedCharacter(c) => {
                write!(f, "[line {}] Error: Unexpected character: {}", self.line, c)
            }
            LexicalErrorType::UnterminatedString => {
                write!(f, "[line {}] Error: Unterminated string.", self.line)
            }
        }
    }
}
