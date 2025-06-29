/// Represents limitations or constraints to control AI outputs.
#[derive(Debug, Clone)]
pub struct Limitations {
    /// Description of the constraints or boundaries for the AI
    pub text: String,
}

impl Limitations {
    pub fn new() -> Self {
        Self {
            text: String::new(),
        }
    }

    pub fn generate_text(&self) -> String {
        format!(
            "<START_LIMITATIONS>\n{}\n<END_LIMITATIONS>\n",
            self.text
        )
    }
}
