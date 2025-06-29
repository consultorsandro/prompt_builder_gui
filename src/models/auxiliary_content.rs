/// Represents additional auxiliary data for improving AI responses.
#[derive(Debug, Clone)]
pub struct AuxiliaryContent {
    /// Supporting information, examples, or attachments description
    pub data: String,
}

impl AuxiliaryContent {
    pub fn new() -> Self {
        Self {
            data: String::new(),
        }
    }

    pub fn generate_text(&self) -> String {
        format!(
            "<START_AUXILIARY_CONTENT>\n{}\n<END_AUXILIARY_CONTENT>\n",
            self.data
        )
    }
}
