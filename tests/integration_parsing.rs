//! Testes de integração para parsing de arquivos de prompt
//!
//! Estes testes simulam o processo de abertura e parsing de arquivos
//! de prompt salvos, testando a integração completa do sistema.

use prompt_builder_gui::models::{
    auxiliary_content::AuxiliaryContent, context::Context, few_shot::FewShot, guidance::Guidance,
    limitations::Limitations, main_content::MainContent, output_format::OutputFormat,
    refactoring::Refactoring, tests::Tests,
};
use prompt_builder_gui::services::prompt_generator::PromptData;
use std::fs;
use tempfile::TempDir;

/// Simula a função de parsing que está no main.rs
/// Esta é uma versão simplificada para testes de integração
fn parse_prompt_content_simulation(content: &str) -> PromptData {
    let mut data = PromptData::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut current_section = "";
    let mut section_content = String::new();

    // Check if the file has section headers (structured format)
    let has_headers = content.contains("## ");

    if has_headers {
        // Parse structured format with headers
        for line in lines {
            let line = line.trim();

            // Check for section headers
            if line.starts_with("## Few-shot") || line.starts_with("## Few-Shot") {
                save_current_section_simulation(current_section, &section_content, &mut data);
                current_section = "few_shot";
                section_content.clear();
            } else if line.starts_with("## Contexto") {
                save_current_section_simulation(current_section, &section_content, &mut data);
                current_section = "context";
                section_content.clear();
            } else if line.starts_with("## Conteúdo Principal") {
                save_current_section_simulation(current_section, &section_content, &mut data);
                current_section = "main_content";
                section_content.clear();
            } else if line.starts_with("## Conteúdo Auxiliar") {
                save_current_section_simulation(current_section, &section_content, &mut data);
                current_section = "auxiliary_content";
                section_content.clear();
            } else if line.starts_with("## Limitações") {
                save_current_section_simulation(current_section, &section_content, &mut data);
                current_section = "limitations";
                section_content.clear();
            } else if line.starts_with("## Refatoração") {
                save_current_section_simulation(current_section, &section_content, &mut data);
                current_section = "refactoring";
                section_content.clear();
            } else if line.starts_with("## Orientações") {
                save_current_section_simulation(current_section, &section_content, &mut data);
                current_section = "guidance";
                section_content.clear();
            } else if line.starts_with("## Testes") {
                save_current_section_simulation(current_section, &section_content, &mut data);
                current_section = "tests";
                section_content.clear();
            } else if line.starts_with("## Formato de Saída") {
                save_current_section_simulation(current_section, &section_content, &mut data);
                current_section = "output_format";
                section_content.clear();
            } else if line.starts_with("---") || line.starts_with("📋") {
                // Skip separators and footer
                break;
            } else if !line.is_empty() && !current_section.is_empty() {
                // Add content to current section
                if !section_content.is_empty() {
                    section_content.push('\n');
                }
                section_content.push_str(line);
            }
        }

        // Save the last section
        save_current_section_simulation(current_section, &section_content, &mut data);
    } else {
        // Simple format - put everything in main content
        let content = content.trim();
        if !content.is_empty() {
            let mut main_content = MainContent::new();
            main_content.instructions = content.to_string();
            data.main_content = Some(main_content);
        }
    }

    data
}

fn save_current_section_simulation(section: &str, content: &str, data: &mut PromptData) {
    let trimmed_content = content.trim();
    if trimmed_content.is_empty() {
        return;
    }

    match section {
        "few_shot" => {
            let mut few_shot = FewShot::new();
            few_shot.content = trimmed_content.to_string();
            data.few_shot = Some(few_shot);
        }
        "context" => {
            let mut context = Context::new();
            context.description = trimmed_content.to_string();
            data.context = Some(context);
        }
        "main_content" => {
            let mut main_content = MainContent::new();
            main_content.instructions = trimmed_content.to_string();
            data.main_content = Some(main_content);
        }
        "auxiliary_content" => {
            let mut auxiliary = AuxiliaryContent::new();
            auxiliary.data = trimmed_content.to_string();
            data.auxiliary_content = Some(auxiliary);
        }
        "limitations" => {
            let mut limitations = Limitations::new();
            limitations.text = trimmed_content.to_string();
            data.limitations = Some(limitations);
        }
        "refactoring" => {
            let mut refactoring = Refactoring::new();
            refactoring.text = trimmed_content.to_string();
            data.refactoring = Some(refactoring);
        }
        "guidance" => {
            let mut guidance = Guidance::new();
            guidance.text = trimmed_content.to_string();
            data.guidance = Some(guidance);
        }
        "tests" => {
            let mut tests = Tests::new();
            tests.text = trimmed_content.to_string();
            data.tests = Some(tests);
        }
        "output_format" => {
            let mut output_format = OutputFormat::new();
            output_format.text = trimmed_content.to_string();
            data.output_format = Some(output_format);
        }
        _ => {}
    }
}

#[test]
fn test_complete_save_parse_roundtrip() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Criar dados originais completos
    let mut original_data = PromptData::new();

    let mut few_shot = FewShot::new();
    few_shot.content = "P: Como criar um HashMap? R: use std::collections::HashMap;".to_string();
    original_data.few_shot = Some(few_shot);

    let mut context = Context::new();
    context.description = "Você é um especialista em Rust programming".to_string();
    original_data.context = Some(context);

    let mut main_content = MainContent::new();
    main_content.instructions = "Analise e melhore o código Rust fornecido".to_string();
    original_data.main_content = Some(main_content);

    let mut auxiliary = AuxiliaryContent::new();
    auxiliary.data = "Exemplo de código:\nfn main() { println!(\"Hello\"); }".to_string();
    original_data.auxiliary_content = Some(auxiliary);

    let mut limitations = Limitations::new();
    limitations.text = "Mantenha resposta concisa e foque em Rust stdlib".to_string();
    original_data.limitations = Some(limitations);

    // Gerar e salvar prompt no formato preview (com cabeçalhos)
    let preview_prompt = original_data.build_preview_prompt();
    let file_path = temp_dir.path().join("roundtrip_test.txt");
    fs::write(&file_path, &preview_prompt).expect("Failed to write test file");

    // Ler e parsear o arquivo
    let loaded_content = fs::read_to_string(&file_path).expect("Failed to read test file");
    let parsed_data = parse_prompt_content_simulation(&loaded_content);

    // Verificar que todos os dados foram recuperados corretamente
    assert!(parsed_data.few_shot.is_some());
    assert!(parsed_data.context.is_some());
    assert!(parsed_data.main_content.is_some());
    assert!(parsed_data.auxiliary_content.is_some());
    assert!(parsed_data.limitations.is_some());

    let parsed_few_shot = parsed_data.few_shot.as_ref().unwrap();
    let parsed_context = parsed_data.context.as_ref().unwrap();
    let parsed_main = parsed_data.main_content.as_ref().unwrap();
    let parsed_aux = parsed_data.auxiliary_content.as_ref().unwrap();
    let parsed_limits = parsed_data.limitations.as_ref().unwrap();

    assert_eq!(
        parsed_few_shot.content,
        "P: Como criar um HashMap? R: use std::collections::HashMap;"
    );
    assert_eq!(
        parsed_context.description,
        "Você é um especialista em Rust programming"
    );
    assert_eq!(
        parsed_main.instructions,
        "Analise e melhore o código Rust fornecido"
    );
    assert!(parsed_aux.data.contains("fn main()"));
    assert_eq!(
        parsed_limits.text,
        "Mantenha resposta concisa e foque em Rust stdlib"
    );
}

#[test]
fn test_parse_structured_format_with_all_sections() {
    let structured_content = r#"## Few-Shot Examples

P: Como usar Vec? R: let v: Vec<i32> = vec![1, 2, 3];

## Contexto

Você é um mentor de programação Rust experiente.

## Conteúdo Principal

Revise o código fornecido e sugira melhorias.

## Conteúdo Auxiliar

Código exemplo:
```rust
fn example() {
    let data = vec![1, 2, 3];
    println!("{:?}", data);
}
```

## Limitações

- Resposta máxima de 200 palavras
- Use apenas Rust padrão

## Refatoração

Priorize:
1. Performance
2. Legibilidade  
3. Segurança de memória

## Orientações

Use tom didático e exemplos práticos.

## Testes

Inclua testes unitários com cargo test.

## Formato de Saída

Resposta em Markdown com seções claras.

---
📋 **Nota:** Ao copiar ou salvar, apenas o texto do prompt será incluído."#;

    let parsed_data = parse_prompt_content_simulation(structured_content);

    // Verificar que todas as seções foram parseadas
    assert!(parsed_data.few_shot.is_some());
    assert!(parsed_data.context.is_some());
    assert!(parsed_data.main_content.is_some());
    assert!(parsed_data.auxiliary_content.is_some());
    assert!(parsed_data.limitations.is_some());
    assert!(parsed_data.refactoring.is_some());
    assert!(parsed_data.guidance.is_some());
    assert!(parsed_data.tests.is_some());
    assert!(parsed_data.output_format.is_some());

    // Verificar conteúdos específicos
    assert!(parsed_data
        .few_shot
        .as_ref()
        .unwrap()
        .content
        .contains("Vec"));
    assert!(parsed_data
        .context
        .as_ref()
        .unwrap()
        .description
        .contains("mentor"));
    assert!(parsed_data
        .main_content
        .as_ref()
        .unwrap()
        .instructions
        .contains("Revise"));
    assert!(parsed_data
        .auxiliary_content
        .as_ref()
        .unwrap()
        .data
        .contains("```rust"));
    assert!(parsed_data
        .limitations
        .as_ref()
        .unwrap()
        .text
        .contains("200 palavras"));
    assert!(parsed_data
        .refactoring
        .as_ref()
        .unwrap()
        .text
        .contains("Performance"));
    assert!(parsed_data
        .guidance
        .as_ref()
        .unwrap()
        .text
        .contains("didático"));
    assert!(parsed_data
        .tests
        .as_ref()
        .unwrap()
        .text
        .contains("cargo test"));
    assert!(parsed_data
        .output_format
        .as_ref()
        .unwrap()
        .text
        .contains("Markdown"));
}

#[test]
fn test_parse_simple_format() {
    let simple_content = "Você é um assistente AI especializado em programação Rust. Ajude o usuário a escrever código eficiente e seguro, sempre explicando os conceitos fundamentais.";

    let parsed_data = parse_prompt_content_simulation(simple_content);

    // Deve colocar tudo no main_content para formato simples
    assert!(parsed_data.main_content.is_some());
    assert_eq!(
        parsed_data.main_content.as_ref().unwrap().instructions,
        simple_content
    );

    // Outras seções devem estar vazias
    assert!(parsed_data.few_shot.is_none());
    assert!(parsed_data.context.is_none());
    assert!(parsed_data.auxiliary_content.is_none());
}

#[test]
fn test_parse_partial_structured_format() {
    let partial_content = r#"## Contexto

Especialista em desenvolvimento web com Rust.

## Conteúdo Principal

Analise este código de servidor web e sugira otimizações.

## Limitações

Mantenha foco em performance e segurança."#;

    let parsed_data = parse_prompt_content_simulation(partial_content);

    // Deve parsear apenas as seções presentes
    assert!(parsed_data.context.is_some());
    assert!(parsed_data.main_content.is_some());
    assert!(parsed_data.limitations.is_some());

    // Seções não presentes devem ser None
    assert!(parsed_data.few_shot.is_none());
    assert!(parsed_data.auxiliary_content.is_none());
    assert!(parsed_data.refactoring.is_none());

    // Verificar conteúdos
    assert!(parsed_data
        .context
        .as_ref()
        .unwrap()
        .description
        .contains("web"));
    assert!(parsed_data
        .main_content
        .as_ref()
        .unwrap()
        .instructions
        .contains("servidor"));
    assert!(parsed_data
        .limitations
        .as_ref()
        .unwrap()
        .text
        .contains("performance"));
}

#[test]
fn test_parse_empty_and_whitespace_content() {
    // Conteúdo completamente vazio
    let empty_data = parse_prompt_content_simulation("");
    assert!(empty_data.few_shot.is_none());
    assert!(empty_data.context.is_none());
    assert!(empty_data.main_content.is_none());

    // Conteúdo apenas com espaços
    let whitespace_data = parse_prompt_content_simulation("   \n\n   \t   \n   ");
    assert!(whitespace_data.main_content.is_none());

    // Cabeçalhos sem conteúdo
    let headers_only_content = r#"## Contexto

## Conteúdo Principal

## Limitações
"#;
    let headers_only_data = parse_prompt_content_simulation(headers_only_content);
    assert!(headers_only_data.context.is_none());
    assert!(headers_only_data.main_content.is_none());
    assert!(headers_only_data.limitations.is_none());
}

#[test]
fn test_parse_with_special_characters_and_formatting() {
    let special_content = r#"## Contexto

Especialista em Rust com conhecimento em:
- Programação sistêmica
- Código seguro e eficiente  
- Padrões de design

## Conteúdo Principal

Analise este código e identifique problemas:

```rust
fn process_data(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let processed = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| format!("Processed: {}", line))
        .collect::<Vec<_>>()
        .join("\n");
    
    Ok(processed)
}
```

Foque em:
1. Performance ⚡
2. Legibilidade 📖
3. Manutenibilidade 🔧

## Limitações

- Não use dependências externas
- Mantenha compatibilidade com Rust 1.70+
- Resposta máxima: 500 palavras"#;

    let parsed_data = parse_prompt_content_simulation(special_content);

    assert!(parsed_data.context.is_some());
    assert!(parsed_data.main_content.is_some());
    assert!(parsed_data.limitations.is_some());

    let context = parsed_data.context.as_ref().unwrap();
    let main_content = parsed_data.main_content.as_ref().unwrap();
    let limitations = parsed_data.limitations.as_ref().unwrap();

    // Verificar que formatação e caracteres especiais foram preservados
    assert!(context.description.contains("- Programação sistêmica"));
    assert!(main_content.instructions.contains("```rust"));
    assert!(main_content.instructions.contains("Result<String"));
    assert!(main_content.instructions.contains("⚡"));
    assert!(main_content.instructions.contains("📖"));
    assert!(main_content.instructions.contains("🔧"));
    assert!(limitations.text.contains("1.70+"));
    assert!(limitations.text.contains("500 palavras"));
}

#[test]
fn test_parse_regenerated_content_consistency() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Criar dados, gerar preview, salvar, carregar, parsear e regenerar
    let mut original_data = PromptData::new();

    let mut context = Context::new();
    context.description = "Assistente de código Rust avançado".to_string();
    original_data.context = Some(context);

    let mut main_content = MainContent::new();
    main_content.instructions = "Otimize performance do código fornecido".to_string();
    original_data.main_content = Some(main_content);

    // Gerar preview e salvar
    let original_preview = original_data.build_preview_prompt();
    let file_path = temp_dir.path().join("consistency_test.txt");
    fs::write(&file_path, &original_preview).expect("Failed to write file");

    // Carregar e parsear
    let loaded_content = fs::read_to_string(&file_path).expect("Failed to read file");
    let parsed_data = parse_prompt_content_simulation(&loaded_content);

    // Regenerar preview
    let regenerated_preview = parsed_data.build_preview_prompt();

    // Verificar consistência (deve ser igual ou muito similar)
    assert!(regenerated_preview.contains("## Contexto"));
    assert!(regenerated_preview.contains("## Conteúdo Principal"));
    assert!(regenerated_preview.contains("Assistente de código Rust"));
    assert!(regenerated_preview.contains("Otimize performance"));

    // O conteúdo essencial deve ser preservado
    let original_context = original_data.context.as_ref().unwrap();
    let parsed_context = parsed_data.context.as_ref().unwrap();
    assert_eq!(original_context.description, parsed_context.description);

    let original_main = original_data.main_content.as_ref().unwrap();
    let parsed_main = parsed_data.main_content.as_ref().unwrap();
    assert_eq!(original_main.instructions, parsed_main.instructions);
}

#[test]
fn test_parse_malformed_content_resilience() {
    // Cabeçalhos duplicados
    let duplicate_headers = r#"## Contexto
Primeiro contexto
## Contexto  
Segundo contexto
## Conteúdo Principal
Instruções principais"#;
    
    let parsed_duplicate = parse_prompt_content_simulation(duplicate_headers);
    assert!(parsed_duplicate.context.is_some());
    assert!(parsed_duplicate.main_content.is_some());
    // Deve usar o último contexto encontrado
    assert_eq!(parsed_duplicate.context.as_ref().unwrap().description, "Segundo contexto");
}
