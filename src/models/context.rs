/// Represents the context or persona configuration for the AI assistant.
#[derive(Debug, Clone)]
pub struct Context {
    /// AI persona or task description
    pub description: String,
}

impl Context {
    /// Creates a new empty context
    pub fn new() -> Self {
        Self {
            description: String::new(),
        }
    }

    /// Generates the formatted text for this section
    pub fn generate_text(&self) -> String {
        format!(
            "<START_CONTEXT>\n{}\n<END_CONTEXT>\n",
            self.description
        )
    }
}
