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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_content_new() {
        let main_content = MainContent::new();
        assert!(main_content.instructions.is_empty());
    }

    #[test]
    fn test_main_content_with_instructions() {
        let mut main_content = MainContent::new();
        main_content.instructions = "Analise o código e forneça sugestões de melhoria".to_string();
        assert_eq!(
            main_content.instructions,
            "Analise o código e forneça sugestões de melhoria"
        );
    }

    #[test]
    fn test_main_content_generate_text_empty() {
        let main_content = MainContent::new();
        let generated = main_content.generate_text();
        assert_eq!(generated, "<START_MAIN_CONTENT>\n\n<END_MAIN_CONTENT>\n");
    }

    #[test]
    fn test_main_content_generate_text_with_instructions() {
        let mut main_content = MainContent::new();
        main_content.instructions =
            "Revise este código Rust e identifique possíveis problemas.".to_string();

        let generated = main_content.generate_text();
        let expected = "<START_MAIN_CONTENT>\nRevise este código Rust e identifique possíveis problemas.\n<END_MAIN_CONTENT>\n";
        assert_eq!(generated, expected);
    }

    #[test]
    fn test_main_content_long_instructions() {
        let mut main_content = MainContent::new();
        let long_instruction = "Esta é uma instrução muito longa que deve ser preservada completamente, incluindo detalhes específicos sobre como processar dados, aplicar transformações e gerar resultados de alta qualidade.".to_string();
        main_content.instructions = long_instruction.clone();

        let generated = main_content.generate_text();
        assert!(generated.contains(&long_instruction));
        assert!(generated.starts_with("<START_MAIN_CONTENT>"));
        assert!(generated.ends_with("<END_MAIN_CONTENT>\n"));
    }

    #[test]
    fn test_main_content_multiline_instructions() {
        let mut main_content = MainContent::new();
        main_content.instructions = "Primeira linha\nSegunda linha\nTerceira linha".to_string();

        let generated = main_content.generate_text();
        assert!(generated.contains("Primeira linha"));
        assert!(generated.contains("Segunda linha"));
        assert!(generated.contains("Terceira linha"));
    }

    #[test]
    fn test_main_content_clone() {
        let mut original = MainContent::new();
        original.instructions = "Instruções originais".to_string();

        let cloned = original.clone();
        assert_eq!(original.instructions, cloned.instructions);
    }

    #[test]
    fn test_main_content_debug_format() {
        let mut main_content = MainContent::new();
        main_content.instructions = "Teste debug".to_string();

        let debug_str = format!("{:?}", main_content);
        assert!(debug_str.contains("MainContent"));
        assert!(debug_str.contains("Teste debug"));
    }

    #[test]
    fn test_main_content_instructions_modification() {
        let mut main_content = MainContent::new();
        main_content.instructions = "Instrução inicial".to_string();

        main_content.instructions.push_str(" - adição");
        assert!(main_content.instructions.contains("Instrução inicial"));
        assert!(main_content.instructions.contains("adição"));
    }
}
