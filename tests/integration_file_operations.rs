//! Testes de integração para operações de arquivo
//!
//! Estes testes verificam a integração entre geração de prompts,
//! salvamento de arquivos e carregamento de prompts salvos.

use prompt_builder_gui::models::{context::Context, few_shot::FewShot, main_content::MainContent};
use prompt_builder_gui::services::{
    file_service::{save_prompt_to_file, save_prompt_to_specific_path},
    prompt_generator::PromptData,
};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

/// Cria dados de prompt para testes de arquivo
fn create_test_prompt_data() -> PromptData {
    let mut data = PromptData::new();

    let mut context = Context::new();
    context.description = "Você é um assistente de programação especializado em Rust.".to_string();
    data.context = Some(context);

    let mut main_content = MainContent::new();
    main_content.instructions =
        "Ajude o usuário a escrever código Rust eficiente e seguro.".to_string();
    data.main_content = Some(main_content);

    let mut few_shot = FewShot::new();
    few_shot.content = "P: Como criar um vetor? R: let v: Vec<i32> = vec![1, 2, 3];".to_string();
    data.few_shot = Some(few_shot);

    data
}

#[test]
fn test_complete_save_and_verify_workflow() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let temp_path = temp_dir.path().to_str().unwrap();

    // Criar dados de prompt
    let data = create_test_prompt_data();
    let prompt_text = data.build_prompt(false);

    // Salvar arquivo
    let result = save_prompt_to_file(&prompt_text, temp_path, "test_integration");
    assert!(result.is_ok());

    // Verificar se o arquivo existe
    let file_path = temp_dir.path().join("test_integration.txt");
    assert!(file_path.exists());

    // Ler e verificar conteúdo
    let saved_content = fs::read_to_string(&file_path).expect("Failed to read saved file");
    assert_eq!(saved_content, prompt_text);
    assert!(saved_content.contains("assistente de programação"));
    assert!(saved_content.contains("código Rust eficiente"));
    assert!(saved_content.contains("vec![1, 2, 3]"));
}

#[test]
fn test_save_and_load_round_trip() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Dados originais
    let original_data = create_test_prompt_data();
    let original_prompt = original_data.build_prompt(false);

    // Salvar
    let file_path = temp_dir.path().join("round_trip_test.txt");
    let file_path_str = file_path.to_str().unwrap();
    let save_result = save_prompt_to_specific_path(&original_prompt, file_path_str);
    assert!(save_result.is_ok());

    // Carregar
    let loaded_content = fs::read_to_string(&file_path).expect("Failed to load file");

    // Verificar integridade
    assert_eq!(original_prompt, loaded_content);
    assert!(loaded_content.contains("assistente de programação"));
    assert!(loaded_content.contains("código Rust eficiente"));
}

#[test]
fn test_save_with_complex_formatting() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let mut data = PromptData::new();

    let mut main_content = MainContent::new();
    main_content.instructions = "Instruções com formatação:\n\n1. Primeiro passo\n   - Subitem A\n   - Subitem B\n\n2. Segundo passo\n\nCódigo exemplo:\n```rust\nfn main() {\n    println!(\"Hello, World!\");\n}\n```".to_string();
    data.main_content = Some(main_content);

    let formatted_prompt = data.build_prompt(false);

    // Salvar
    let file_path = temp_dir.path().join("formatted_test.txt");
    let file_path_str = file_path.to_str().unwrap();
    let result = save_prompt_to_specific_path(&formatted_prompt, file_path_str);
    assert!(result.is_ok());

    // Verificar que a formatação foi preservada
    let saved_content = fs::read_to_string(&file_path).expect("Failed to read file");
    assert!(saved_content.contains("1. Primeiro passo"));
    assert!(saved_content.contains("   - Subitem A"));
    assert!(saved_content.contains("```rust"));
    assert!(saved_content.contains("fn main()"));
    assert!(saved_content.contains("    println!"));
}

#[test]
fn test_save_empty_and_minimal_content() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Teste com dados vazios
    let empty_data = PromptData::new();
    let empty_prompt = empty_data.build_prompt(false);

    let empty_file_path = temp_dir.path().join("empty_test.txt");
    let empty_file_path_str = empty_file_path.to_str().unwrap();
    let result = save_prompt_to_specific_path(&empty_prompt, empty_file_path_str);
    assert!(result.is_ok());

    let saved_empty = fs::read_to_string(&empty_file_path).expect("Failed to read empty file");
    assert_eq!(saved_empty, "Nenhum campo foi preenchido ainda.");

    // Teste com dados mínimos
    let mut minimal_data = PromptData::new();
    let mut context = Context::new();
    context.description = "Contexto simples".to_string();
    minimal_data.context = Some(context);

    let minimal_prompt = minimal_data.build_prompt(false);

    let minimal_file_path = temp_dir.path().join("minimal_test.txt");
    let minimal_file_path_str = minimal_file_path.to_str().unwrap();
    let result = save_prompt_to_specific_path(&minimal_prompt, minimal_file_path_str);
    assert!(result.is_ok());

    let saved_minimal =
        fs::read_to_string(&minimal_file_path).expect("Failed to read minimal file");
    assert!(saved_minimal.contains("Contexto simples"));
}

#[test]
fn test_save_multiple_files_same_directory() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let temp_path = temp_dir.path().to_str().unwrap();

    // Criar dados diferentes
    let data1 = create_test_prompt_data();
    let prompt1 = data1.build_prompt(false);

    let mut data2 = PromptData::new();
    let mut context2 = Context::new();
    context2.description = "Segundo contexto diferente".to_string();
    data2.context = Some(context2);
    let prompt2 = data2.build_prompt(false);

    // Salvar múltiplos arquivos
    let result1 = save_prompt_to_file(&prompt1, temp_path, "arquivo1");
    let result2 = save_prompt_to_file(&prompt2, temp_path, "arquivo2");

    assert!(result1.is_ok());
    assert!(result2.is_ok());

    // Verificar que ambos os arquivos existem
    let file1_path = temp_dir.path().join("arquivo1.txt");
    let file2_path = temp_dir.path().join("arquivo2.txt");

    assert!(file1_path.exists());
    assert!(file2_path.exists());

    // Verificar conteúdos diferentes
    let content1 = fs::read_to_string(&file1_path).expect("Failed to read file 1");
    let content2 = fs::read_to_string(&file2_path).expect("Failed to read file 2");

    assert!(content1.contains("assistente de programação"));
    assert!(content2.contains("Segundo contexto diferente"));
    assert_ne!(content1, content2);
}

#[test]
fn test_save_with_preview_vs_clean_format() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let data = create_test_prompt_data();

    // Gerar formatos diferentes
    let clean_prompt = data.build_prompt(false); // Sem marcadores
    let preview_prompt = data.build_preview_prompt(); // Com cabeçalhos

    // Salvar ambos
    let clean_path = temp_dir.path().join("clean_format.txt");
    let preview_path = temp_dir.path().join("preview_format.txt");

    let clean_path_str = clean_path.to_str().unwrap();
    let preview_path_str = preview_path.to_str().unwrap();

    let result1 = save_prompt_to_specific_path(&clean_prompt, clean_path_str);
    let result2 = save_prompt_to_specific_path(&preview_prompt, preview_path_str);

    assert!(result1.is_ok());
    assert!(result2.is_ok());

    // Ler e comparar
    let saved_clean = fs::read_to_string(&clean_path).expect("Failed to read clean file");
    let saved_preview = fs::read_to_string(&preview_path).expect("Failed to read preview file");

    // Clean não deve ter cabeçalhos, preview deve ter
    assert!(!saved_clean.contains("## Contexto"));
    assert!(saved_preview.contains("## Contexto"));
    assert!(saved_preview.contains("## Conteúdo Principal"));
    assert!(saved_preview.contains("📋 **Nota:**"));

    // Ambos devem ter o conteúdo essencial
    assert!(saved_clean.contains("assistente de programação"));
    assert!(saved_preview.contains("assistente de programação"));
}

#[test]
fn test_save_large_content() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let mut data = PromptData::new();

    // Criar conteúdo muito grande
    let large_text = "Esta é uma linha de texto muito longa que será repetida muitas vezes para testar o salvamento de arquivos grandes. ".repeat(1000);

    let mut main_content = MainContent::new();
    main_content.instructions = large_text.clone();
    data.main_content = Some(main_content);

    let large_prompt = data.build_prompt(false);

    // Salvar arquivo grande
    let large_file_path = temp_dir.path().join("large_file.txt");
    let large_file_path_str = large_file_path.to_str().unwrap();
    let result = save_prompt_to_specific_path(&large_prompt, large_file_path_str);
    assert!(result.is_ok());

    // Verificar que o arquivo foi salvo corretamente
    assert!(large_file_path.exists());
    let saved_large = fs::read_to_string(&large_file_path).expect("Failed to read large file");
    assert_eq!(saved_large.len(), large_prompt.len());
    assert!(saved_large.contains("linha de texto muito longa"));

    // Verificar tamanho do arquivo
    let metadata = fs::metadata(&large_file_path).expect("Failed to get file metadata");
    assert!(metadata.len() > 50000); // Deve ser um arquivo grande
}

#[test]
fn test_save_with_special_filename_characters() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let temp_path = temp_dir.path().to_str().unwrap();

    let data = create_test_prompt_data();
    let prompt = data.build_prompt(false);

    // Nomes de arquivo com caracteres especiais (seguros para sistemas de arquivo)
    let safe_names = vec![
        "prompt-com-hifens",
        "prompt_com_underscores",
        "prompt.com.pontos",
        "prompt123com456numeros",
        "PROMPT_MAIUSCULO",
        "prompt_misto_CasE",
    ];

    for name in safe_names {
        let result = save_prompt_to_file(&prompt, temp_path, name);
        assert!(result.is_ok(), "Failed to save file with name: {}", name);

        let file_path = temp_dir.path().join(format!("{}.txt", name));
        assert!(file_path.exists(), "File does not exist: {}", name);

        let content = fs::read_to_string(&file_path).expect("Failed to read file");
        assert!(content.contains("assistente de programação"));
    }
}

#[test]
fn test_directory_creation_on_save() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Criar um caminho com múltiplos níveis de diretório
    let nested_dir = temp_dir.path().join("level1").join("level2").join("level3");
    let nested_path = nested_dir.to_str().unwrap();

    let data = create_test_prompt_data();
    let prompt = data.build_prompt(false);

    // Salvar no diretório aninhado (que não existe ainda)
    let result = save_prompt_to_file(&prompt, nested_path, "nested_test");
    assert!(result.is_ok());

    // Verificar que os diretórios foram criados
    assert!(nested_dir.exists());
    assert!(nested_dir.is_dir());

    // Verificar que o arquivo foi criado no local correto
    let file_path = nested_dir.join("nested_test.txt");
    assert!(file_path.exists());

    let content = fs::read_to_string(&file_path).expect("Failed to read nested file");
    assert!(content.contains("assistente de programação"));
}

#[test]
fn test_overwrite_existing_file() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("overwrite_test.txt");
    let file_path_str = file_path.to_str().unwrap();

    // Primeiro salvamento
    let data1 = create_test_prompt_data();
    let prompt1 = data1.build_prompt(false);
    let result1 = save_prompt_to_specific_path(&prompt1, file_path_str);
    assert!(result1.is_ok());

    // Verificar primeiro conteúdo
    let content1 = fs::read_to_string(&file_path).expect("Failed to read first content");
    assert!(content1.contains("assistente de programação"));

    // Segundo salvamento (sobrescrever)
    let mut data2 = PromptData::new();
    let mut context2 = Context::new();
    context2.description = "Contexto completamente diferente para sobrescrever".to_string();
    data2.context = Some(context2);
    let prompt2 = data2.build_prompt(false);

    let result2 = save_prompt_to_specific_path(&prompt2, file_path_str);
    assert!(result2.is_ok());

    // Verificar que foi sobrescrito
    let content2 = fs::read_to_string(&file_path).expect("Failed to read overwritten content");
    assert!(content2.contains("completamente diferente"));
    assert!(!content2.contains("assistente de programação"));
}
