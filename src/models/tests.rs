/// Represents testing requirements for developer-related AI outputs.
#[derive(Debug, Clone)]
pub struct Tests {
    /// Testing instructions like unit tests, integration tests, etc.
    pub text: String,
}

impl Tests {
    pub fn new() -> Self {
        Self {
            text: String::new(),
        }
    }

    pub fn generate_text(&self) -> String {
        format!(
            "<START_TESTS>\n{}\n<END_TESTS>\n",
            self.text
        )
    }
}
