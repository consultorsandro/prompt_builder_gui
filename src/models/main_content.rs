/// Represents the main content or primary instructions for the AI.
#[derive(Debug, Clone)]
pub struct MainContent {
    /// Main instructions or task for the AI
    pub instructions: String,
}

impl MainContent {
    /// Creates a new empty main content
    pub fn new() -> Self {
        Self {
            instructions: String::new(),
        }
    }

    /// Generates the formatted text for this section
    pub fn generate_text(&self) -> String {
        format!(
            "<START_MAIN_CONTENT>\n{}\n<END_MAIN_CONTENT>\n",
            self.instructions
        )
    }
}
