use crate::models::{
    auxiliary_content::AuxiliaryContent, context::Context, few_shot::FewShot, guidance::Guidance,
    limitations::Limitations, main_content::MainContent, output_format::OutputFormat,
    refactoring::Refactoring, tests::Tests,
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
            sections.push(if include_section_markers {
                text
            } else {
                Self::remove_markers(&text)
            });
        }

        if let Some(section) = &self.context {
            let text = section.generate_text();
            sections.push(if include_section_markers {
                text
            } else {
                Self::remove_markers(&text)
            });
        }

        if let Some(section) = &self.main_content {
            let text = section.generate_text();
            sections.push(if include_section_markers {
                text
            } else {
                Self::remove_markers(&text)
            });
        }

        if let Some(section) = &self.auxiliary_content {
            let text = section.generate_text();
            sections.push(if include_section_markers {
                text
            } else {
                Self::remove_markers(&text)
            });
        }

        if let Some(section) = &self.limitations {
            let text = section.generate_text();
            sections.push(if include_section_markers {
                text
            } else {
                Self::remove_markers(&text)
            });
        }

        if let Some(section) = &self.refactoring {
            let text = section.generate_text();
            sections.push(if include_section_markers {
                text
            } else {
                Self::remove_markers(&text)
            });
        }

        if let Some(section) = &self.guidance {
            let text = section.generate_text();
            sections.push(if include_section_markers {
                text
            } else {
                Self::remove_markers(&text)
            });
        }

        if let Some(section) = &self.tests {
            let text = section.generate_text();
            sections.push(if include_section_markers {
                text
            } else {
                Self::remove_markers(&text)
            });
        }

        if let Some(section) = &self.output_format {
            let text = section.generate_text();
            sections.push(if include_section_markers {
                text
            } else {
                Self::remove_markers(&text)
            });
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
                sections.push(format!(
                    "## Few-Shot Examples\n\n{}",
                    section.content.trim()
                ));
            }
        }

        if let Some(section) = &self.context {
            if !section.description.trim().is_empty() {
                sections.push(format!("## Contexto\n\n{}", section.description.trim()));
            }
        }

        if let Some(section) = &self.main_content {
            if !section.instructions.trim().is_empty() {
                sections.push(format!(
                    "## Conteúdo Principal\n\n{}",
                    section.instructions.trim()
                ));
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
                sections.push(format!(
                    "## Refatoração (Código)\n\n{}",
                    section.text.trim()
                ));
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

#[cfg(test)]
mod tests {
    use super::*;

    fn create_sample_prompt_data() -> PromptData {
        let mut data = PromptData::new();

        let mut few_shot = FewShot::new();
        few_shot.content = "Exemplo: Pergunta sobre Rust -> Resposta detalhada".to_string();
        data.few_shot = Some(few_shot);

        let mut context = Context::new();
        context.description = "Você é um especialista em Rust".to_string();
        data.context = Some(context);

        let mut main_content = MainContent::new();
        main_content.instructions = "Analise o código fornecido".to_string();
        data.main_content = Some(main_content);

        data
    }

    #[test]
    fn test_prompt_data_new() {
        let data = PromptData::new();
        assert!(data.few_shot.is_none());
        assert!(data.context.is_none());
        assert!(data.main_content.is_none());
        assert!(data.auxiliary_content.is_none());
        assert!(data.limitations.is_none());
        assert!(data.refactoring.is_none());
        assert!(data.guidance.is_none());
        assert!(data.tests.is_none());
        assert!(data.output_format.is_none());
    }

    #[test]
    fn test_prompt_data_clone() {
        let original = create_sample_prompt_data();
        let cloned = original.clone();

        assert!(cloned.few_shot.is_some());
        assert!(cloned.context.is_some());
        assert!(cloned.main_content.is_some());

        // Verificar que os conteúdos são iguais
        assert_eq!(
            original.few_shot.as_ref().unwrap().content,
            cloned.few_shot.as_ref().unwrap().content
        );
    }

    #[test]
    fn test_build_prompt_empty() {
        let data = PromptData::new();
        let prompt_with_markers = data.build_prompt(true);
        let prompt_without_markers = data.build_prompt(false);

        assert_eq!(prompt_with_markers, "Nenhum campo foi preenchido ainda.");
        assert_eq!(prompt_without_markers, "Nenhum campo foi preenchido ainda.");
    }

    #[test]
    fn test_build_prompt_with_markers() {
        let data = create_sample_prompt_data();
        let prompt = data.build_prompt(true);

        assert!(prompt.contains("<START_FEW_SHOT>"));
        assert!(prompt.contains("<END_FEW_SHOT>"));
        assert!(prompt.contains("<START_CONTEXT>"));
        assert!(prompt.contains("<END_CONTEXT>"));
        assert!(prompt.contains("<START_MAIN_CONTENT>"));
        assert!(prompt.contains("<END_MAIN_CONTENT>"));
        assert!(prompt.contains("Exemplo: Pergunta sobre Rust"));
        assert!(prompt.contains("especialista em Rust"));
        assert!(prompt.contains("Analise o código"));
    }

    #[test]
    fn test_build_prompt_without_markers() {
        let data = create_sample_prompt_data();
        let prompt = data.build_prompt(false);

        assert!(!prompt.contains("<START_FEW_SHOT>"));
        assert!(!prompt.contains("<END_FEW_SHOT>"));
        assert!(!prompt.contains("<START_CONTEXT>"));
        assert!(!prompt.contains("<END_CONTEXT>"));
        assert!(!prompt.contains("<START_MAIN_CONTENT>"));
        assert!(!prompt.contains("<END_MAIN_CONTENT>"));

        // Mas ainda deve conter o conteúdo
        assert!(prompt.contains("Exemplo: Pergunta sobre Rust"));
        assert!(prompt.contains("especialista em Rust"));
        assert!(prompt.contains("Analise o código"));
    }

    #[test]
    fn test_remove_markers() {
        let text_with_markers = "<START_TEST>\nConteúdo do teste\nMais uma linha\n<END_TEST>";
        let cleaned = PromptData::remove_markers(text_with_markers);

        assert!(!cleaned.contains("<START_TEST>"));
        assert!(!cleaned.contains("<END_TEST>"));
        assert!(cleaned.contains("Conteúdo do teste"));
        assert!(cleaned.contains("Mais uma linha"));
    }

    #[test]
    fn test_remove_markers_mixed_content() {
        let text = "<START_SECTION>\nLinha válida 1\n<END_SECTION>\nLinha válida 2\n<START_OTHER>\nLinha válida 3";
        let cleaned = PromptData::remove_markers(text);

        assert!(!cleaned.contains("<START_SECTION>"));
        assert!(!cleaned.contains("<END_SECTION>"));
        assert!(!cleaned.contains("<START_OTHER>"));
        assert!(cleaned.contains("Linha válida 1"));
        assert!(cleaned.contains("Linha válida 2"));
        assert!(cleaned.contains("Linha válida 3"));
    }

    #[test]
    fn test_build_preview_prompt_empty() {
        let data = PromptData::new();
        let preview = data.build_preview_prompt();
        assert_eq!(preview, "Nenhum campo foi preenchido ainda.");
    }

    #[test]
    fn test_build_preview_prompt_with_content() {
        let data = create_sample_prompt_data();
        let preview = data.build_preview_prompt();

        assert!(preview.contains("## Few-Shot Examples"));
        assert!(preview.contains("## Contexto"));
        assert!(preview.contains("## Conteúdo Principal"));
        assert!(preview.contains("Exemplo: Pergunta sobre Rust"));
        assert!(preview.contains("especialista em Rust"));
        assert!(preview.contains("Analise o código"));
        assert!(preview.contains("📋 **Nota:**"));
    }

    #[test]
    fn test_build_preview_prompt_skips_empty_sections() {
        let mut data = PromptData::new();

        let mut context = Context::new();
        context.description = "Apenas contexto preenchido".to_string();
        data.context = Some(context);

        // Few-shot vazio
        let mut few_shot = FewShot::new();
        few_shot.content = "".to_string();
        data.few_shot = Some(few_shot);

        let preview = data.build_preview_prompt();

        assert!(preview.contains("## Contexto"));
        assert!(preview.contains("Apenas contexto preenchido"));
        assert!(!preview.contains("## Few-Shot Examples"));
    }

    #[test]
    fn test_build_preview_all_sections() {
        let mut data = PromptData::new();

        let mut few_shot = FewShot::new();
        few_shot.content = "Few-shot content".to_string();
        data.few_shot = Some(few_shot);

        let mut context = Context::new();
        context.description = "Context content".to_string();
        data.context = Some(context);

        let mut main_content = MainContent::new();
        main_content.instructions = "Main content".to_string();
        data.main_content = Some(main_content);

        let mut auxiliary = AuxiliaryContent::new();
        auxiliary.data = "Auxiliary content".to_string();
        data.auxiliary_content = Some(auxiliary);

        let mut limitations = Limitations::new();
        limitations.text = "Limitations content".to_string();
        data.limitations = Some(limitations);

        let mut refactoring = Refactoring::new();
        refactoring.text = "Refactoring content".to_string();
        data.refactoring = Some(refactoring);

        let mut guidance = Guidance::new();
        guidance.text = "Guidance content".to_string();
        data.guidance = Some(guidance);

        let mut tests = Tests::new();
        tests.text = "Tests content".to_string();
        data.tests = Some(tests);

        let mut output_format = OutputFormat::new();
        output_format.text = "Output format content".to_string();
        data.output_format = Some(output_format);

        let preview = data.build_preview_prompt();

        assert!(preview.contains("## Few-Shot Examples"));
        assert!(preview.contains("## Contexto"));
        assert!(preview.contains("## Conteúdo Principal"));
        assert!(preview.contains("## Conteúdo Auxiliar"));
        assert!(preview.contains("## Limitações"));
        assert!(preview.contains("## Refatoração"));
        assert!(preview.contains("## Orientações"));
        assert!(preview.contains("## Testes"));
        assert!(preview.contains("## Formato de Saída"));
    }

    #[test]
    fn test_prompt_data_debug_format() {
        let data = create_sample_prompt_data();
        let debug_str = format!("{:?}", data);
        assert!(debug_str.contains("PromptData"));
        assert!(debug_str.contains("few_shot"));
        assert!(debug_str.contains("context"));
    }

    #[test]
    fn test_build_prompt_section_order() {
        let data = create_sample_prompt_data();
        let prompt = data.build_prompt(true);

        let few_shot_pos = prompt.find("<START_FEW_SHOT>").unwrap_or(usize::MAX);
        let context_pos = prompt.find("<START_CONTEXT>").unwrap_or(usize::MAX);
        let main_content_pos = prompt.find("<START_MAIN_CONTENT>").unwrap_or(usize::MAX);

        // Verificar ordem: few_shot, context, main_content
        assert!(few_shot_pos < context_pos);
        assert!(context_pos < main_content_pos);
    }

    #[test]
    fn test_partial_data_build() {
        let mut data = PromptData::new();

        // Apenas main_content preenchido
        let mut main_content = MainContent::new();
        main_content.instructions = "Só conteúdo principal".to_string();
        data.main_content = Some(main_content);

        let prompt = data.build_prompt(false);
        assert!(prompt.contains("Só conteúdo principal"));
        assert!(!prompt.contains("Nenhum campo foi preenchido"));
    }

    #[test]
    fn test_whitespace_handling() {
        let mut data = PromptData::new();

        let mut context = Context::new();
        context.description = "   Texto com espaços   \n\n   ".to_string();
        data.context = Some(context);

        let preview = data.build_preview_prompt();
        assert!(preview.contains("Texto com espaços"));
        // trim() deve remover espaços desnecessários
        assert!(!preview.contains("   Texto com espaços   "));
    }
}
