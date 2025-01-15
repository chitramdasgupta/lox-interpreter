#[derive(Debug, Clone)]
pub struct LexicalError {
    character: String,
    line: usize,
}

impl LexicalError {
    pub fn new(character: String, line: usize) -> Self {
        Self { character, line }
    }
}

impl std::fmt::Display for LexicalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[line {}] Error: Unexpected character: {}", self.line, self.character)
    }
}