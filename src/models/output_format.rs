/// Represents the desired output format for the AI response.
/// Example: plain text, HTML, Markdown, etc.
#[derive(Debug, Clone)]
pub struct OutputFormat {
    /// Output format specification
    pub text: String,
}

impl OutputFormat {
    pub fn new() -> Self {
        Self {
            text: String::new(),
        }
    }

    pub fn generate_text(&self) -> String {
        format!(
            "<START_OUTPUT_FORMAT>\n{}\n<END_OUTPUT_FORMAT>\n",
            self.text
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_format_new() {
        let output_format = OutputFormat::new();
        assert!(output_format.text.is_empty());
    }

    #[test]
    fn test_output_format_with_text() {
        let mut output_format = OutputFormat::new();
        output_format.text = "Formato: Markdown com blocos de código".to_string();
        assert_eq!(output_format.text, "Formato: Markdown com blocos de código");
    }

    #[test]
    fn test_output_format_generate_text_empty() {
        let output_format = OutputFormat::new();
        let generated = output_format.generate_text();
        assert_eq!(generated, "<START_OUTPUT_FORMAT>\n\n<END_OUTPUT_FORMAT>\n");
    }

    #[test]
    fn test_output_format_generate_text_with_content() {
        let mut output_format = OutputFormat::new();
        output_format.text =
            "Resposta em JSON:\n{\n  \"resultado\": \"valor\",\n  \"status\": \"sucesso\"\n}"
                .to_string();

        let generated = output_format.generate_text();
        let expected = "<START_OUTPUT_FORMAT>\nResposta em JSON:\n{\n  \"resultado\": \"valor\",\n  \"status\": \"sucesso\"\n}\n<END_OUTPUT_FORMAT>\n";
        assert_eq!(generated, expected);
    }

    #[test]
    fn test_output_format_markdown_specification() {
        let mut output_format = OutputFormat::new();
        output_format.text = "Formato Markdown:\n# Título Principal\n## Subtítulo\n- Lista item 1\n- Lista item 2\n```rust\ncódigo aqui\n```".to_string();

        let generated = output_format.generate_text();
        assert!(generated.contains("# Título Principal"));
        assert!(generated.contains("## Subtítulo"));
        assert!(generated.contains("```rust"));
    }

    #[test]
    fn test_output_format_json_specification() {
        let mut output_format = OutputFormat::new();
        output_format.text =
            "JSON estruturado:\n{\n  \"data\": [],\n  \"metadata\": {},\n  \"errors\": null\n}"
                .to_string();

        let generated = output_format.generate_text();
        assert!(generated.contains("JSON estruturado"));
        assert!(generated.contains("\"data\": []"));
        assert!(generated.contains("\"metadata\": {}"));
        assert!(generated.starts_with("<START_OUTPUT_FORMAT>"));
        assert!(generated.ends_with("<END_OUTPUT_FORMAT>\n"));
    }

    #[test]
    fn test_output_format_html_specification() {
        let mut output_format = OutputFormat::new();
        output_format.text = "HTML com estrutura:\n<div class=\"resultado\">\n  <h1>Título</h1>\n  <p>Conteúdo</p>\n</div>".to_string();

        let generated = output_format.generate_text();
        assert!(generated.contains("<div class=\"resultado\">"));
        assert!(generated.contains("<h1>Título</h1>"));
        assert!(generated.contains("<p>Conteúdo</p>"));
    }

    #[test]
    fn test_output_format_mixed_formats() {
        let mut output_format = OutputFormat::new();
        output_format.text =
            "Formatos aceitos:\n1. Markdown\n2. JSON\n3. Plain text\n4. HTML\n5. XML".to_string();

        let generated = output_format.generate_text();
        assert!(generated.contains("Markdown"));
        assert!(generated.contains("JSON"));
        assert!(generated.contains("Plain text"));
        assert!(generated.contains("HTML"));
        assert!(generated.contains("XML"));
    }

    #[test]
    fn test_output_format_clone() {
        let mut original = OutputFormat::new();
        original.text = "Formato original".to_string();

        let cloned = original.clone();
        assert_eq!(original.text, cloned.text);
    }

    #[test]
    fn test_output_format_debug_format() {
        let mut output_format = OutputFormat::new();
        output_format.text = "Teste debug".to_string();

        let debug_str = format!("{:?}", output_format);
        assert!(debug_str.contains("OutputFormat"));
        assert!(debug_str.contains("Teste debug"));
    }

    #[test]
    fn test_output_format_text_modification() {
        let mut output_format = OutputFormat::new();
        output_format.text = "Formato inicial".to_string();
        output_format.text.push_str("\nFormato adicional");

        assert!(output_format.text.contains("Formato inicial"));
        assert!(output_format.text.contains("Formato adicional"));
    }

    #[test]
    fn test_output_format_clear() {
        let mut output_format = OutputFormat::new();
        output_format.text = "Formato temporário".to_string();
        output_format.text.clear();
        assert!(output_format.text.is_empty());
    }

    #[test]
    fn test_output_format_complex_specification() {
        let mut output_format = OutputFormat::new();
        output_format.text = "Resposta estruturada:\n1. Resumo executivo\n2. Análise detalhada\n3. Recomendações\n4. Próximos passos\n\nCada seção deve ter no máximo 200 palavras.".to_string();

        let generated = output_format.generate_text();
        assert!(generated.contains("Resumo executivo"));
        assert!(generated.contains("Análise detalhada"));
        assert!(generated.contains("Recomendações"));
        assert!(generated.contains("200 palavras"));
    }
}
