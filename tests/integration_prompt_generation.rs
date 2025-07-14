//! Testes de integração para o módulo de prompt generation
//!
//! Estes testes verificam a integração entre diferentes módulos
//! e o funcionamento completo do sistema de geração de prompts.

use prompt_builder_gui::models::{
    auxiliary_content::AuxiliaryContent, context::Context, few_shot::FewShot, guidance::Guidance,
    limitations::Limitations, main_content::MainContent, output_format::OutputFormat,
    refactoring::Refactoring, tests::Tests,
};
use prompt_builder_gui::services::prompt_generator::PromptData;
use std::fs;
use tempfile::TempDir;

/// Cria uma instância completa de PromptData para testes
fn create_full_prompt_data() -> PromptData {
    let mut data = PromptData::new();

    let mut few_shot = FewShot::new();
    few_shot.content = "Q: Como implementar um HashMap em Rust?\nA: Use std::collections::HashMap, defina tipos, insira com insert()".to_string();
    data.few_shot = Some(few_shot);

    let mut context = Context::new();
    context.description = "Você é um especialista em Rust com 10 anos de experiência, focado em performance e segurança de memória.".to_string();
    data.context = Some(context);

    let mut main_content = MainContent::new();
    main_content.instructions = "Analise o código Rust fornecido e identifique possíveis melhorias de performance, segurança e legibilidade.".to_string();
    data.main_content = Some(main_content);

    let mut auxiliary = AuxiliaryContent::new();
    auxiliary.data = "Código de exemplo:\n```rust\nfn process_data(data: Vec<String>) -> Vec<String> {\n    data.iter().map(|s| s.clone()).collect()\n}\n```".to_string();
    data.auxiliary_content = Some(auxiliary);

    let mut limitations = Limitations::new();
    limitations.text = "- Limite a resposta a 300 palavras\n- Evite jargões excessivamente técnicos\n- Cite apenas soluções padrão do Rust".to_string();
    data.limitations = Some(limitations);

    let mut refactoring = Refactoring::new();
    refactoring.text = "Priorize:\n1. Eliminação de clones desnecessários\n2. Uso de borrowing quando possível\n3. Implementação de iteradores eficientes".to_string();
    data.refactoring = Some(refactoring);

    let mut guidance = Guidance::new();
    guidance.text =
        "Mantenha tom didático, use exemplos práticos e explique o 'porquê' de cada sugestão."
            .to_string();
    data.guidance = Some(guidance);

    let mut tests = Tests::new();
    tests.text = "Inclua sugestões de testes unitários usando cargo test e proptest para property-based testing.".to_string();
    data.tests = Some(tests);

    let mut output_format = OutputFormat::new();
    output_format.text = "Resposta em Markdown com:\n## Análise\n## Sugestões de Melhoria\n## Código Refatorado\n## Testes Sugeridos".to_string();
    data.output_format = Some(output_format);

    data
}

#[test]
fn test_complete_prompt_generation_workflow() {
    let data = create_full_prompt_data();

    // Testar geração com marcadores
    let prompt_with_markers = data.build_prompt(true);
    assert!(prompt_with_markers.len() > 100);
    assert!(prompt_with_markers.contains("<START_FEW_SHOT>"));
    assert!(prompt_with_markers.contains("<START_CONTEXT>"));
    assert!(prompt_with_markers.contains("<START_MAIN_CONTENT>"));

    // Testar geração sem marcadores
    let prompt_without_markers = data.build_prompt(false);
    assert!(prompt_without_markers.len() > 100);
    assert!(!prompt_without_markers.contains("<START_FEW_SHOT>"));
    assert!(!prompt_without_markers.contains("<END_CONTEXT>"));

    // Verificar que o conteúdo principal está presente em ambos
    assert!(prompt_with_markers.contains("HashMap em Rust"));
    assert!(prompt_without_markers.contains("HashMap em Rust"));
    assert!(prompt_with_markers.contains("especialista em Rust"));
    assert!(prompt_without_markers.contains("especialista em Rust"));
}

#[test]
fn test_preview_generation_with_all_sections() {
    let data = create_full_prompt_data();
    let preview = data.build_preview_prompt();

    // Verificar presença de todos os cabeçalhos
    assert!(preview.contains("## Few-Shot Examples"));
    assert!(preview.contains("## Contexto"));
    assert!(preview.contains("## Conteúdo Principal"));
    assert!(preview.contains("## Conteúdo Auxiliar"));
    assert!(preview.contains("## Limitações"));
    assert!(preview.contains("## Refatoração"));
    assert!(preview.contains("## Orientações"));
    assert!(preview.contains("## Testes"));
    assert!(preview.contains("## Formato de Saída"));

    // Verificar presença de conteúdos específicos
    assert!(preview.contains("HashMap em Rust"));
    assert!(preview.contains("10 anos de experiência"));
    assert!(preview.contains("300 palavras"));
    assert!(preview.contains("clones desnecessários"));
    assert!(preview.contains("tom didático"));
    assert!(preview.contains("cargo test"));
    assert!(preview.contains("Markdown"));
    assert!(preview.contains("📋 **Nota:**"));
}

#[test]
fn test_section_interaction_and_dependencies() {
    let mut data = PromptData::new();

    // Adicionar seções que interagem entre si
    let mut context = Context::new();
    context.description = "Especialista em otimização de código".to_string();
    data.context = Some(context);

    let mut main_content = MainContent::new();
    main_content.instructions = "Otimize este código para melhor performance".to_string();
    data.main_content = Some(main_content);

    let mut refactoring = Refactoring::new();
    refactoring.text = "Foque em algoritmos O(n) ao invés de O(n²)".to_string();
    data.refactoring = Some(refactoring);

    let prompt = data.build_prompt(false);

    // Verificar que as seções se complementam logicamente
    assert!(prompt.contains("otimização"));
    assert!(prompt.contains("performance"));
    assert!(prompt.contains("O(n)"));

    // Verificar ordem lógica no prompt
    let context_pos = prompt.find("Especialista em otimização").unwrap();
    let main_pos = prompt.find("Otimize este código").unwrap();
    let refactor_pos = prompt.find("algoritmos O(n)").unwrap();

    assert!(context_pos < main_pos);
    assert!(main_pos < refactor_pos);
}

#[test]
fn test_empty_and_partial_data_handling() {
    // Dados completamente vazios
    let empty_data = PromptData::new();
    let empty_prompt = empty_data.build_prompt(true);
    let empty_preview = empty_data.build_preview_prompt();

    assert_eq!(empty_prompt, "Nenhum campo foi preenchido ainda.");
    assert_eq!(empty_preview, "Nenhum campo foi preenchido ainda.");

    // Dados parciais - apenas algumas seções
    let mut partial_data = PromptData::new();

    let mut context = Context::new();
    context.description = "Contexto mínimo".to_string();
    partial_data.context = Some(context);

    let mut output_format = OutputFormat::new();
    output_format.text = "Resposta simples".to_string();
    partial_data.output_format = Some(output_format);

    let partial_prompt = partial_data.build_prompt(false);
    let partial_preview = partial_data.build_preview_prompt();

    assert!(partial_prompt.contains("Contexto mínimo"));
    assert!(partial_prompt.contains("Resposta simples"));
    assert!(partial_preview.contains("## Contexto"));
    assert!(partial_preview.contains("## Formato de Saída"));

    // Não deve conter seções não preenchidas
    assert!(!partial_preview.contains("## Few-Shot Examples"));
    assert!(!partial_preview.contains("## Conteúdo Principal"));
}

#[test]
fn test_data_consistency_across_operations() {
    let original_data = create_full_prompt_data();

    // Clonar dados
    let cloned_data = original_data.clone();

    // Verificar que as operações produzem os mesmos resultados
    let original_prompt = original_data.build_prompt(true);
    let cloned_prompt = cloned_data.build_prompt(true);
    assert_eq!(original_prompt, cloned_prompt);

    let original_preview = original_data.build_preview_prompt();
    let cloned_preview = cloned_data.build_preview_prompt();
    assert_eq!(original_preview, cloned_preview);

    // Verificar que modificar o clone não afeta o original
    let mut modified_clone = cloned_data;
    if let Some(ref mut context) = modified_clone.context {
        context.description = "Contexto modificado".to_string();
    }

    let modified_prompt = modified_clone.build_prompt(false);
    let original_prompt_after = original_data.build_prompt(false);

    assert!(modified_prompt.contains("Contexto modificado"));
    assert!(!original_prompt_after.contains("Contexto modificado"));
    assert!(original_prompt_after.contains("especialista em Rust"));
}

#[test]
fn test_marker_removal_consistency() {
    let data = create_full_prompt_data();

    // Gerar prompt com marcadores e depois removê-los manualmente
    let prompt_with_markers = data.build_prompt(true);
    let manually_cleaned = prompt_with_markers
        .lines()
        .filter(|line| !line.starts_with("<START_") && !line.starts_with("<END_"))
        .collect::<Vec<_>>()
        .join("\n");

    // Gerar prompt sem marcadores diretamente
    let prompt_without_markers = data.build_prompt(false);

    // Remover espaços em branco extras para comparação justa
    let manually_cleaned = manually_cleaned.trim();
    let direct_clean = prompt_without_markers.trim();

    // Ambos devem conter o mesmo conteúdo essencial
    assert!(manually_cleaned.contains("HashMap em Rust"));
    assert!(direct_clean.contains("HashMap em Rust"));
    assert!(manually_cleaned.contains("especialista em Rust"));
    assert!(direct_clean.contains("especialista em Rust"));
}

#[test]
fn test_large_content_handling() {
    let mut data = PromptData::new();

    // Criar conteúdo grande
    let large_text = "Esta é uma linha de texto repetida. ".repeat(100);

    let mut main_content = MainContent::new();
    main_content.instructions = large_text.clone();
    data.main_content = Some(main_content);

    let mut auxiliary = AuxiliaryContent::new();
    auxiliary.data = large_text.clone();
    data.auxiliary_content = Some(auxiliary);

    // Verificar que o sistema lida bem com conteúdo grande
    let prompt = data.build_prompt(false);
    let preview = data.build_preview_prompt();

    assert!(prompt.len() > 5000); // Deve ser um texto grande
    assert!(preview.len() > 5000);
    assert!(prompt.contains("linha de texto repetida"));
    assert!(preview.contains("## Conteúdo Principal"));
    assert!(preview.contains("## Conteúdo Auxiliar"));
}

#[test]
fn test_special_characters_and_encoding() {
    let mut data = PromptData::new();

    let special_content = "Texto com acentos: ação, coração, não.\nCaracteres especiais: @#$%^&*()[]{}|\\\n\nEmojis: 🦀 🚀 ⚡\nUnicode: αβγδε ñáéíóú";

    let mut context = Context::new();
    context.description = special_content.to_string();
    data.context = Some(context);

    let prompt = data.build_prompt(false);
    let preview = data.build_preview_prompt();

    // Verificar que caracteres especiais são preservados
    assert!(prompt.contains("ação"));
    assert!(prompt.contains("coração"));
    assert!(prompt.contains("@#$%^&*()"));
    assert!(prompt.contains("🦀"));
    assert!(prompt.contains("αβγδε"));
    assert!(prompt.contains("ñáéíóú"));

    assert!(preview.contains("## Contexto"));
    assert!(preview.contains("🦀"));
}

#[test]
fn test_whitespace_and_formatting_preservation() {
    let mut data = PromptData::new();

    let formatted_content = "Código com indentação:\n    fn main() {\n        println!(\"Hello\");\n    }\n\nLista:\n- Item 1\n  - Subitem 1.1\n  - Subitem 1.2\n- Item 2";

    let mut auxiliary = AuxiliaryContent::new();
    auxiliary.data = formatted_content.to_string();
    data.auxiliary_content = Some(auxiliary);

    let prompt = data.build_prompt(false);
    let preview = data.build_preview_prompt();

    // Verificar que a formatação é preservada
    assert!(prompt.contains("    fn main()"));
    assert!(prompt.contains("        println!"));
    assert!(prompt.contains("- Item 1"));
    assert!(prompt.contains("  - Subitem"));

    assert!(preview.contains("    fn main()"));
    assert!(preview.contains("## Conteúdo Auxiliar"));
}
