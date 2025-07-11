/// Represents guidance on how the AI output should be presented.
#[derive(Debug, Clone)]
pub struct Guidance {
    /// Instructions on tone, style, target audience, etc.
    pub text: String,
}

impl Guidance {
    pub fn new() -> Self {
        Self {
            text: String::new(),
        }
    }

    pub fn generate_text(&self) -> String {
        format!(
            "<START_GUIDANCE>\n{}\n<END_GUIDANCE>\n",
            self.text
        )
    }
}
