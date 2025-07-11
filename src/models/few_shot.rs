/// Represents the "Few-shot" section of the prompt.
/// Used to provide a question and its reasoning in a single paragraph.
#[derive(Debug, Clone)]
pub struct FewShot {
    /// The question and reasoning text as a single paragraph
    pub content: String,
}

impl FewShot {
    /// Creates a new empty FewShot section
    pub fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    /// Generates the formatted text for this section
    pub fn generate_text(&self) -> String {
        format!(
            "<START_FEW_SHOT>\n{}\n<END_FEW_SHOT>\n",
            self.content
        )
    }
}
