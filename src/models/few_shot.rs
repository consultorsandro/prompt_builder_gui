/// Represents the "Few-shot" section of the prompt.
/// Used to provide example input-output pairs for the AI.
#[derive(Debug, Clone)]
pub struct FewShot {
    /// Example input text
    pub example_input: String,

    /// Expected output text
    pub expected_output: String,
}

impl FewShot {
    /// Creates a new empty FewShot section
    pub fn new() -> Self {
        Self {
            example_input: String::new(),
            expected_output: String::new(),
        }
    }

    /// Generates the formatted text for this section
    pub fn generate_text(&self) -> String {
        format!(
            "<START_FEW_SHOT>\nInput Example:\n{}\nExpected Output:\n{}\n<END_FEW_SHOT>\n",
            self.example_input, self.expected_output
        )
    }
}
