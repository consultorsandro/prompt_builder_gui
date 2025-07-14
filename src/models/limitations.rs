/// Represents limitations or constraints to control AI outputs.
#[derive(Debug, Clone)]
pub struct Limitations {
    /// Description of the constraints or boundaries for the AI
    pub text: String,
}

impl Limitations {
    pub fn new() -> Self {
        Self {
            text: String::new(),
        }
    }

    pub fn generate_text(&self) -> String {
        format!("<START_LIMITATIONS>\n{}\n<END_LIMITATIONS>\n", self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_limitations_new() {
        let limitations = Limitations::new();
        assert!(limitations.text.is_empty());
    }

    #[test]
    fn test_limitations_with_text() {
        let mut limitations = Limitations::new();
        limitations.text = "Não forneça informações confidenciais".to_string();
        assert_eq!(limitations.text, "Não forneça informações confidenciais");
    }

    #[test]
    fn test_limitations_generate_text_empty() {
        let limitations = Limitations::new();
        let generated = limitations.generate_text();
        assert_eq!(generated, "<START_LIMITATIONS>\n\n<END_LIMITATIONS>\n");
    }

    #[test]
    fn test_limitations_generate_text_with_content() {
        let mut limitations = Limitations::new();
        limitations.text =
            "Limite a resposta a 500 palavras.\nEvite linguagem técnica complexa.".to_string();

        let generated = limitations.generate_text();
        let expected = "<START_LIMITATIONS>\nLimite a resposta a 500 palavras.\nEvite linguagem técnica complexa.\n<END_LIMITATIONS>\n";
        assert_eq!(generated, expected);
    }

    #[test]
    fn test_limitations_multiple_constraints() {
        let mut limitations = Limitations::new();
        limitations.text =
            "1. Não use jargões\n2. Seja conciso\n3. Cite fontes quando aplicável".to_string();

        let generated = limitations.generate_text();
        assert!(generated.contains("1. Não use jargões"));
        assert!(generated.contains("2. Seja conciso"));
        assert!(generated.contains("3. Cite fontes"));
        assert!(generated.starts_with("<START_LIMITATIONS>"));
        assert!(generated.ends_with("<END_LIMITATIONS>\n"));
    }

    #[test]
    fn test_limitations_with_special_characters() {
        let mut limitations = Limitations::new();
        limitations.text = "Evite: \\n, \\t, e caracteres especiais @#$%".to_string();

        let generated = limitations.generate_text();
        assert!(generated.contains("\\n, \\t"));
        assert!(generated.contains("@#$%"));
    }

    #[test]
    fn test_limitations_clone() {
        let mut original = Limitations::new();
        original.text = "Limitações originais".to_string();

        let cloned = original.clone();
        assert_eq!(original.text, cloned.text);
    }

    #[test]
    fn test_limitations_debug_format() {
        let mut limitations = Limitations::new();
        limitations.text = "Teste debug".to_string();

        let debug_str = format!("{:?}", limitations);
        assert!(debug_str.contains("Limitations"));
        assert!(debug_str.contains("Teste debug"));
    }

    #[test]
    fn test_limitations_text_modification() {
        let mut limitations = Limitations::new();
        limitations.text = "Limitação inicial".to_string();

        limitations.text.push_str("\nLimitação adicional");
        assert!(limitations.text.contains("Limitação inicial"));
        assert!(limitations.text.contains("Limitação adicional"));
    }

    #[test]
    fn test_limitations_clear() {
        let mut limitations = Limitations::new();
        limitations.text = "Limitações temporárias".to_string();
        limitations.text.clear();
        assert!(limitations.text.is_empty());
    }

    #[test]
    fn test_limitations_long_text() {
        let mut limitations = Limitations::new();
        let long_text = "Limitação ".repeat(50);
        limitations.text = long_text.clone();

        let generated = limitations.generate_text();
        assert!(generated.contains(&long_text));
        assert!(generated.len() > long_text.len());
    }
}
