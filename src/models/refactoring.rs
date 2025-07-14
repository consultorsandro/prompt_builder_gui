/// Represents refactoring instructions for developers.
/// Used mainly in code-related prompts.
#[derive(Debug, Clone)]
pub struct Refactoring {
    /// Instructions for refactoring
    pub text: String,
}

impl Refactoring {
    pub fn new() -> Self {
        Self {
            text: String::new(),
        }
    }

    pub fn generate_text(&self) -> String {
        format!("<START_REFACTORING>\n{}\n<END_REFACTORING>\n", self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_refactoring_new() {
        let refactoring = Refactoring::new();
        assert!(refactoring.text.is_empty());
    }

    #[test]
    fn test_refactoring_with_text() {
        let mut refactoring = Refactoring::new();
        refactoring.text = "Extraia funções pequenas e reutilizáveis".to_string();
        assert_eq!(refactoring.text, "Extraia funções pequenas e reutilizáveis");
    }

    #[test]
    fn test_refactoring_generate_text_empty() {
        let refactoring = Refactoring::new();
        let generated = refactoring.generate_text();
        assert_eq!(generated, "<START_REFACTORING>\n\n<END_REFACTORING>\n");
    }

    #[test]
    fn test_refactoring_generate_text_with_content() {
        let mut refactoring = Refactoring::new();
        refactoring.text =
            "1. Elimine código duplicado\n2. Melhore legibilidade\n3. Adicione documentação"
                .to_string();

        let generated = refactoring.generate_text();
        let expected = "<START_REFACTORING>\n1. Elimine código duplicado\n2. Melhore legibilidade\n3. Adicione documentação\n<END_REFACTORING>\n";
        assert_eq!(generated, expected);
    }

    #[test]
    fn test_refactoring_code_instructions() {
        let mut refactoring = Refactoring::new();
        refactoring.text = "Refatore usando padrões SOLID:\n- Single Responsibility\n- Open/Closed\n- Liskov Substitution".to_string();

        let generated = refactoring.generate_text();
        assert!(generated.contains("SOLID"));
        assert!(generated.contains("Single Responsibility"));
        assert!(generated.contains("Open/Closed"));
        assert!(generated.contains("Liskov Substitution"));
    }

    #[test]
    fn test_refactoring_performance_tips() {
        let mut refactoring = Refactoring::new();
        refactoring.text = "Otimizações de performance:\n- Use iterators em vez de loops\n- Evite alocações desnecessárias".to_string();

        let generated = refactoring.generate_text();
        assert!(generated.contains("performance"));
        assert!(generated.contains("iterators"));
        assert!(generated.contains("alocações"));
        assert!(generated.starts_with("<START_REFACTORING>"));
        assert!(generated.ends_with("<END_REFACTORING>\n"));
    }

    #[test]
    fn test_refactoring_clone() {
        let mut original = Refactoring::new();
        original.text = "Instruções de refactoring originais".to_string();

        let cloned = original.clone();
        assert_eq!(original.text, cloned.text);
    }

    #[test]
    fn test_refactoring_debug_format() {
        let mut refactoring = Refactoring::new();
        refactoring.text = "Teste debug".to_string();

        let debug_str = format!("{:?}", refactoring);
        assert!(debug_str.contains("Refactoring"));
        assert!(debug_str.contains("Teste debug"));
    }

    #[test]
    fn test_refactoring_append_instructions() {
        let mut refactoring = Refactoring::new();
        refactoring.text = "Instrução inicial".to_string();
        refactoring.text.push_str("\nInstrução adicional");

        assert!(refactoring.text.contains("Instrução inicial"));
        assert!(refactoring.text.contains("Instrução adicional"));
    }

    #[test]
    fn test_refactoring_clear() {
        let mut refactoring = Refactoring::new();
        refactoring.text = "Instruções temporárias".to_string();
        refactoring.text.clear();
        assert!(refactoring.text.is_empty());
    }

    #[test]
    fn test_refactoring_with_code_examples() {
        let mut refactoring = Refactoring::new();
        refactoring.text =
            "Antes:\nlet x = vec![1, 2, 3];\n\nDepois:\nlet x: Vec<i32> = vec![1, 2, 3];"
                .to_string();

        let generated = refactoring.generate_text();
        assert!(generated.contains("Antes:"));
        assert!(generated.contains("Depois:"));
        assert!(generated.contains("vec![1, 2, 3]"));
    }
}
