/// Represents the desired output format for the AI response.
/// Example: plain text, HTML, Markdown, etc.
#[derive(Debug, Clone)]
pub struct OutputFormat {
    /// Output format specification
    pub text: String,
}

impl OutputFormat {
    pub fn new() -> Self {
        Self {
            text: String::new(),
        }
    }

    pub fn generate_text(&self) -> String {
        format!(
            "<START_OUTPUT_FORMAT>\n{}\n<END_OUTPUT_FORMAT>\n",
            self.text
        )
    }
}
