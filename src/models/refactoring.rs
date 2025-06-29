/// Represents refactoring instructions for developers.
/// Used mainly in code-related prompts.
#[derive(Debug, Clone)]
pub struct Refactoring {
    /// Instructions for refactoring
    pub text: String,
}

impl Refactoring {
    pub fn new() -> Self {
        Self {
            text: String::new(),
        }
    }

    pub fn generate_text(&self) -> String {
        format!(
            "<START_REFACTORING>\n{}\n<END_REFACTORING>\n",
            self.text
        )
    }
}
