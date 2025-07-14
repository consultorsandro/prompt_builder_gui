/// Represents the context or persona configuration for the AI assistant.
#[derive(Debug, Clone)]
pub struct Context {
    /// AI persona or task description
    pub description: String,
}

impl Context {
    /// Creates a new empty context
    pub fn new() -> Self {
        Self {
            description: String::new(),
        }
    }

    /// Generates the formatted text for this section
    pub fn generate_text(&self) -> String {
        format!("<START_CONTEXT>\n{}\n<END_CONTEXT>\n", self.description)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_new() {
        let context = Context::new();
        assert!(context.description.is_empty());
    }

    #[test]
    fn test_context_with_description() {
        let mut context = Context::new();
        context.description = "Você é um assistente especializado em programação".to_string();
        assert_eq!(
            context.description,
            "Você é um assistente especializado em programação"
        );
    }

    #[test]
    fn test_context_generate_text_empty() {
        let context = Context::new();
        let generated = context.generate_text();
        assert_eq!(generated, "<START_CONTEXT>\n\n<END_CONTEXT>\n");
    }

    #[test]
    fn test_context_generate_text_with_description() {
        let mut context = Context::new();
        context.description = "Você é um expert em Rust que ajuda desenvolvedores.".to_string();

        let generated = context.generate_text();
        let expected =
            "<START_CONTEXT>\nVocê é um expert em Rust que ajuda desenvolvedores.\n<END_CONTEXT>\n";
        assert_eq!(generated, expected);
    }

    #[test]
    fn test_context_description_multiline() {
        let mut context = Context::new();
        context.description =
            "Você é um assistente AI.\nEspecializado em código.\nSempre seja útil.".to_string();

        let generated = context.generate_text();
        assert!(generated.contains("Você é um assistente AI."));
        assert!(generated.contains("Especializado em código."));
        assert!(generated.contains("Sempre seja útil."));
    }

    #[test]
    fn test_context_clone() {
        let mut original = Context::new();
        original.description = "Contexto original".to_string();

        let cloned = original.clone();
        assert_eq!(original.description, cloned.description);
    }

    #[test]
    fn test_context_debug_format() {
        let mut context = Context::new();
        context.description = "Teste debug".to_string();

        let debug_str = format!("{:?}", context);
        assert!(debug_str.contains("Context"));
        assert!(debug_str.contains("Teste debug"));
    }

    #[test]
    fn test_context_empty_after_clear() {
        let mut context = Context::new();
        context.description = "Conteúdo temporário".to_string();
        context.description.clear();
        assert!(context.description.is_empty());
    }
}
