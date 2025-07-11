use crate::models::{
    auxiliary_content::AuxiliaryContent,
    context::Context,
    few_shot::FewShot,
    guidance::Guidance,
    limitations::Limitations,
    main_content::MainContent,
    output_format::OutputFormat,
    refactoring::Refactoring,
    tests::Tests,
};

/// Struct to hold all prompt sections before generating the final prompt
#[derive(Debug, Clone)]
pub struct PromptData {
    pub few_shot: Option<FewShot>,
    pub context: Option<Context>,
    pub main_content: Option<MainContent>,
    pub auxiliary_content: Option<AuxiliaryContent>,
    pub limitations: Option<Limitations>,
    pub refactoring: Option<Refactoring>,
    pub guidance: Option<Guidance>,
    pub tests: Option<Tests>,
    pub output_format: Option<OutputFormat>,
}

impl PromptData {
    /// Creates a new empty PromptData with all sections set to None
    pub fn new() -> Self {
        Self {
            few_shot: None,
            context: None,
            main_content: None,
            auxiliary_content: None,
            limitations: None,
            refactoring: None,
            guidance: None,
            tests: None,
            output_format: None,
        }
    }

    /// Builds the final prompt text by concatenating all available sections
    pub fn build_prompt(&self, include_section_markers: bool) -> String {
        let mut sections: Vec<String> = Vec::new();

        if let Some(section) = &self.few_shot {
            let text = section.generate_text();
            sections.push(if include_section_markers { text } else { Self::remove_markers(&text) });
        }

        if let Some(section) = &self.context {
            let text = section.generate_text();
            sections.push(if include_section_markers { text } else { Self::remove_markers(&text) });
        }

        if let Some(section) = &self.main_content {
            let text = section.generate_text();
            sections.push(if include_section_markers { text } else { Self::remove_markers(&text) });
        }

        if let Some(section) = &self.auxiliary_content {
            let text = section.generate_text();
            sections.push(if include_section_markers { text } else { Self::remove_markers(&text) });
        }

        if let Some(section) = &self.limitations {
            let text = section.generate_text();
            sections.push(if include_section_markers { text } else { Self::remove_markers(&text) });
        }

        if let Some(section) = &self.refactoring {
            let text = section.generate_text();
            sections.push(if include_section_markers { text } else { Self::remove_markers(&text) });
        }

        if let Some(section) = &self.guidance {
            let text = section.generate_text();
            sections.push(if include_section_markers { text } else { Self::remove_markers(&text) });
        }

        if let Some(section) = &self.tests {
            let text = section.generate_text();
            sections.push(if include_section_markers { text } else { Self::remove_markers(&text) });
        }

        if let Some(section) = &self.output_format {
            let text = section.generate_text();
            sections.push(if include_section_markers { text } else { Self::remove_markers(&text) });
        }

        if sections.is_empty() {
            "Nenhum campo foi preenchido ainda.".to_string()
        } else {
            sections.join("\n\n")
        }
    }

    /// Helper function to remove section markers like <START_SECTION> and <END_SECTION>
    fn remove_markers(text: &str) -> String {
        let mut cleaned_lines: Vec<String> = Vec::new();
        for line in text.lines() {
            if !line.starts_with("<START_") && !line.starts_with("<END_") {
                cleaned_lines.push(line.to_string());
            }
        }
        cleaned_lines.join("\n")
    }
}
