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

    /// Helper function to remove section markers like <START_> and <END_>
    fn remove_markers(text: &str) -> String {
        let mut cleaned_lines: Vec<String> = Vec::new();
        for line in text.lines() {
            if !line.starts_with("<START_") && !line.starts_with("<END_") {
                cleaned_lines.push(line.to_string());
            }
        }
        cleaned_lines.join("\n")
    }

    /// Builds a clean prompt for preview with section titles instead of markers
    pub fn build_preview_prompt(&self) -> String {
        let mut sections: Vec<String> = Vec::new();

        if let Some(section) = &self.few_shot {
            if !section.content.trim().is_empty() {
                sections.push(format!("## Few-Shot Examples\n\n{}", section.content.trim()));
            }
        }

        if let Some(section) = &self.context {
            if !section.description.trim().is_empty() {
                sections.push(format!("## Contexto\n\n{}", section.description.trim()));
            }
        }

        if let Some(section) = &self.main_content {
            if !section.instructions.trim().is_empty() {
                sections.push(format!("## Conteúdo Principal\n\n{}", section.instructions.trim()));
            }
        }

        if let Some(section) = &self.auxiliary_content {
            if !section.data.trim().is_empty() {
                sections.push(format!("## Conteúdo Auxiliar\n\n{}", section.data.trim()));
            }
        }

        if let Some(section) = &self.limitations {
            if !section.text.trim().is_empty() {
                sections.push(format!("## Limitações\n\n{}", section.text.trim()));
            }
        }

        if let Some(section) = &self.refactoring {
            if !section.text.trim().is_empty() {
                sections.push(format!("## Refatoração (Código)\n\n{}", section.text.trim()));
            }
        }

        if let Some(section) = &self.guidance {
            if !section.text.trim().is_empty() {
                sections.push(format!("## Orientações\n\n{}", section.text.trim()));
            }
        }

        if let Some(section) = &self.tests {
            if !section.text.trim().is_empty() {
                sections.push(format!("## Testes\n\n{}", section.text.trim()));
            }
        }

        if let Some(section) = &self.output_format {
            if !section.text.trim().is_empty() {
                sections.push(format!("## Formato de Saída\n\n{}", section.text.trim()));
            }
        }

        if sections.is_empty() {
            "Nenhum campo foi preenchido ainda.".to_string()
        } else {
            // Add user-friendly message at the end
            sections.push("---".to_string());
            sections.push("📋 **Nota:** Ao copiar ou salvar, apenas o texto do prompt será incluído, sem os subtítulos ou marcações acima.".to_string());
            sections.join("\n\n")
        }
    }
}
