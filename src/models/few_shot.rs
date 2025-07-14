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
        format!("<START_FEW_SHOT>\n{}\n<END_FEW_SHOT>\n", self.content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_few_shot_new() {
        let few_shot = FewShot::new();
        assert!(few_shot.content.is_empty());
    }

    #[test]
    fn test_few_shot_with_content() {
        let mut few_shot = FewShot::new();
        few_shot.content = "Exemplo de few-shot learning".to_string();
        assert_eq!(few_shot.content, "Exemplo de few-shot learning");
    }

    #[test]
    fn test_few_shot_generate_text_empty() {
        let few_shot = FewShot::new();
        let generated = few_shot.generate_text();
        assert_eq!(generated, "<START_FEW_SHOT>\n\n<END_FEW_SHOT>\n");
    }

    #[test]
    fn test_few_shot_generate_text_with_content() {
        let mut few_shot = FewShot::new();
        few_shot.content =
            "Pergunta: Como resolver X?\nRaciocínio: Primeiro analise Y, depois Z.".to_string();

        let generated = few_shot.generate_text();
        let expected = "<START_FEW_SHOT>\nPergunta: Como resolver X?\nRaciocínio: Primeiro analise Y, depois Z.\n<END_FEW_SHOT>\n";
        assert_eq!(generated, expected);
    }

    #[test]
    fn test_few_shot_content_mutation() {
        let mut few_shot = FewShot::new();
        assert!(few_shot.content.is_empty());

        few_shot.content = "Primeiro exemplo".to_string();
        assert_eq!(few_shot.content, "Primeiro exemplo");

        few_shot.content.push_str("\nSegundo exemplo");
        assert!(few_shot.content.contains("Primeiro exemplo"));
        assert!(few_shot.content.contains("Segundo exemplo"));
    }

    #[test]
    fn test_few_shot_clone() {
        let mut original = FewShot::new();
        original.content = "Conteúdo original".to_string();

        let cloned = original.clone();
        assert_eq!(original.content, cloned.content);

        // Verificar que são independentes
        // (Como String é clonada profundamente, modificar um não afeta o outro)
    }

    #[test]
    fn test_few_shot_debug_format() {
        let mut few_shot = FewShot::new();
        few_shot.content = "Teste debug".to_string();

        let debug_str = format!("{:?}", few_shot);
        assert!(debug_str.contains("FewShot"));
        assert!(debug_str.contains("Teste debug"));
    }
}
