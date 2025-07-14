/// Represents guidance on how the AI output should be presented.
#[derive(Debug, Clone)]
pub struct Guidance {
    /// Instructions on tone, style, target audience, etc.
    pub text: String,
}

impl Guidance {
    pub fn new() -> Self {
        Self {
            text: String::new(),
        }
    }

    pub fn generate_text(&self) -> String {
        format!("<START_GUIDANCE>\n{}\n<END_GUIDANCE>\n", self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guidance_new() {
        let guidance = Guidance::new();
        assert!(guidance.text.is_empty());
    }

    #[test]
    fn test_guidance_with_text() {
        let mut guidance = Guidance::new();
        guidance.text = "Use tom profissional e linguagem clara".to_string();
        assert_eq!(guidance.text, "Use tom profissional e linguagem clara");
    }

    #[test]
    fn test_guidance_generate_text_empty() {
        let guidance = Guidance::new();
        let generated = guidance.generate_text();
        assert_eq!(generated, "<START_GUIDANCE>\n\n<END_GUIDANCE>\n");
    }

    #[test]
    fn test_guidance_generate_text_with_content() {
        let mut guidance = Guidance::new();
        guidance.text = "Mantenha tom amigável.\nExplique conceitos complexos de forma simples.\nUse exemplos práticos.".to_string();

        let generated = guidance.generate_text();
        let expected = "<START_GUIDANCE>\nMantenha tom amigável.\nExplique conceitos complexos de forma simples.\nUse exemplos práticos.\n<END_GUIDANCE>\n";
        assert_eq!(generated, expected);
    }

    #[test]
    fn test_guidance_tone_instructions() {
        let mut guidance = Guidance::new();
        guidance.text = "Tom: Profissional mas acessível\nAudiência: Desenvolvedores iniciantes\nEstilo: Didático com exemplos".to_string();

        let generated = guidance.generate_text();
        assert!(generated.contains("Profissional mas acessível"));
        assert!(generated.contains("Desenvolvedores iniciantes"));
        assert!(generated.contains("Didático com exemplos"));
    }

    #[test]
    fn test_guidance_formatting_rules() {
        let mut guidance = Guidance::new();
        guidance.text = "Formatação:\n- Use markdown para código\n- Destaque pontos importantes\n- Organize em seções".to_string();

        let generated = guidance.generate_text();
        assert!(generated.contains("markdown"));
        assert!(generated.contains("Destaque pontos"));
        assert!(generated.contains("Organize em seções"));
        assert!(generated.starts_with("<START_GUIDANCE>"));
        assert!(generated.ends_with("<END_GUIDANCE>\n"));
    }

    #[test]
    fn test_guidance_audience_specific() {
        let mut guidance = Guidance::new();
        guidance.text = "Para públicos específicos:\n- Iniciantes: Evite jargões\n- Experts: Seja direto\n- Acadêmicos: Cite referências".to_string();

        let generated = guidance.generate_text();
        assert!(generated.contains("Iniciantes"));
        assert!(generated.contains("Experts"));
        assert!(generated.contains("Acadêmicos"));
    }

    #[test]
    fn test_guidance_clone() {
        let mut original = Guidance::new();
        original.text = "Orientações originais".to_string();

        let cloned = original.clone();
        assert_eq!(original.text, cloned.text);
    }

    #[test]
    fn test_guidance_debug_format() {
        let mut guidance = Guidance::new();
        guidance.text = "Teste debug".to_string();

        let debug_str = format!("{:?}", guidance);
        assert!(debug_str.contains("Guidance"));
        assert!(debug_str.contains("Teste debug"));
    }

    #[test]
    fn test_guidance_text_modification() {
        let mut guidance = Guidance::new();
        guidance.text = "Orientação inicial".to_string();
        guidance.text.push_str("\nOrientação adicional");

        assert!(guidance.text.contains("Orientação inicial"));
        assert!(guidance.text.contains("Orientação adicional"));
    }

    #[test]
    fn test_guidance_clear() {
        let mut guidance = Guidance::new();
        guidance.text = "Orientações temporárias".to_string();
        guidance.text.clear();
        assert!(guidance.text.is_empty());
    }

    #[test]
    fn test_guidance_comprehensive_style() {
        let mut guidance = Guidance::new();
        guidance.text = "Estilo de resposta:\n1. Seja claro e objetivo\n2. Use exemplos relevantes\n3. Mantenha consistência\n4. Valide com o usuário".to_string();

        let generated = guidance.generate_text();
        assert!(generated.contains("1. Seja claro"));
        assert!(generated.contains("2. Use exemplos"));
        assert!(generated.contains("3. Mantenha consistência"));
        assert!(generated.contains("4. Valide"));
    }
}
