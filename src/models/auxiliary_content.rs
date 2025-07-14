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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auxiliary_content_new() {
        let aux_content = AuxiliaryContent::new();
        assert!(aux_content.data.is_empty());
    }

    #[test]
    fn test_auxiliary_content_with_data() {
        let mut aux_content = AuxiliaryContent::new();
        aux_content.data = "Dados auxiliares para apoiar a resposta".to_string();
        assert_eq!(aux_content.data, "Dados auxiliares para apoiar a resposta");
    }

    #[test]
    fn test_auxiliary_content_generate_text_empty() {
        let aux_content = AuxiliaryContent::new();
        let generated = aux_content.generate_text();
        assert_eq!(
            generated,
            "<START_AUXILIARY_CONTENT>\n\n<END_AUXILIARY_CONTENT>\n"
        );
    }

    #[test]
    fn test_auxiliary_content_generate_text_with_data() {
        let mut aux_content = AuxiliaryContent::new();
        aux_content.data = "Exemplo de código:\nfn main() { println!(\"Hello\"); }".to_string();

        let generated = aux_content.generate_text();
        let expected = "<START_AUXILIARY_CONTENT>\nExemplo de código:\nfn main() { println!(\"Hello\"); }\n<END_AUXILIARY_CONTENT>\n";
        assert_eq!(generated, expected);
    }

    #[test]
    fn test_auxiliary_content_with_examples() {
        let mut aux_content = AuxiliaryContent::new();
        aux_content.data = "Exemplo 1: Input -> Output\nExemplo 2: Caso especial".to_string();

        let generated = aux_content.generate_text();
        assert!(generated.contains("Exemplo 1"));
        assert!(generated.contains("Exemplo 2"));
        assert!(generated.starts_with("<START_AUXILIARY_CONTENT>"));
        assert!(generated.ends_with("<END_AUXILIARY_CONTENT>\n"));
    }

    #[test]
    fn test_auxiliary_content_with_large_data() {
        let mut aux_content = AuxiliaryContent::new();
        let large_data = "Documentação extensa: ".to_string() + &"linha ".repeat(100);
        aux_content.data = large_data.clone();

        let generated = aux_content.generate_text();
        assert!(generated.contains(&large_data));
        assert!(generated.len() > large_data.len()); // Deve incluir as tags
    }

    #[test]
    fn test_auxiliary_content_clone() {
        let mut original = AuxiliaryContent::new();
        original.data = "Dados originais".to_string();

        let cloned = original.clone();
        assert_eq!(original.data, cloned.data);
    }

    #[test]
    fn test_auxiliary_content_debug_format() {
        let mut aux_content = AuxiliaryContent::new();
        aux_content.data = "Teste debug".to_string();

        let debug_str = format!("{:?}", aux_content);
        assert!(debug_str.contains("AuxiliaryContent"));
        assert!(debug_str.contains("Teste debug"));
    }

    #[test]
    fn test_auxiliary_content_data_append() {
        let mut aux_content = AuxiliaryContent::new();
        aux_content.data = "Dados iniciais".to_string();
        aux_content.data.push_str("\nDados adicionais");

        assert!(aux_content.data.contains("Dados iniciais"));
        assert!(aux_content.data.contains("Dados adicionais"));
    }

    #[test]
    fn test_auxiliary_content_empty_after_clear() {
        let mut aux_content = AuxiliaryContent::new();
        aux_content.data = "Conteúdo temporário".to_string();
        aux_content.data.clear();
        assert!(aux_content.data.is_empty());
    }
}
