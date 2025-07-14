use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

/// Saves the given prompt text to a file with the specified title.
/// The file will be saved with `.txt` extension in the given directory.
pub fn save_prompt_to_file(
    prompt_text: &str,
    output_directory: &str,
    title: &str,
) -> io::Result<()> {
    // Ensure the output directory exists
    std::fs::create_dir_all(output_directory)?;

    // Build the full file path
    let file_name = format!("{}.txt", title);
    let file_path = Path::new(output_directory).join(file_name);

    // Open the file for writing
    let mut file = File::create(&file_path)?;

    // Write the prompt content to the file
    file.write_all(prompt_text.as_bytes())?;

    println!("✅ Prompt successfully saved at: {:?}", file_path);

    Ok(())
}

/// Saves the prompt text to a specific file path chosen by the user
pub fn save_prompt_to_specific_path(prompt_text: &str, file_path: &str) -> io::Result<()> {
    // Open the file for writing
    let mut file = File::create(file_path)?;

    // Write the prompt content to the file
    file.write_all(prompt_text.as_bytes())?;

    println!("✅ Prompt successfully saved at: {}", file_path);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    fn test_save_prompt_to_file_success() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let temp_path = temp_dir.path().to_str().unwrap();
        let prompt_text = "Este é um teste de prompt";
        let title = "teste_prompt";

        let result = save_prompt_to_file(prompt_text, temp_path, title);
        assert!(result.is_ok());

        // Verificar se o arquivo foi criado
        let expected_path = temp_dir.path().join("teste_prompt.txt");
        assert!(expected_path.exists());

        // Verificar o conteúdo do arquivo
        let content = fs::read_to_string(&expected_path).expect("Failed to read file");
        assert_eq!(content, prompt_text);
    }

    #[test]
    fn test_save_prompt_to_file_creates_directory() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let nested_path = temp_dir.path().join("nested").join("directory");
        let nested_path_str = nested_path.to_str().unwrap();
        let prompt_text = "Teste com diretório aninhado";
        let title = "nested_test";

        let result = save_prompt_to_file(prompt_text, nested_path_str, title);
        assert!(result.is_ok());

        // Verificar se o diretório foi criado
        assert!(nested_path.exists());

        // Verificar se o arquivo foi criado
        let expected_path = nested_path.join("nested_test.txt");
        assert!(expected_path.exists());
    }

    #[test]
    fn test_save_prompt_to_file_empty_content() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let temp_path = temp_dir.path().to_str().unwrap();
        let prompt_text = "";
        let title = "empty_prompt";

        let result = save_prompt_to_file(prompt_text, temp_path, title);
        assert!(result.is_ok());

        let expected_path = temp_dir.path().join("empty_prompt.txt");
        assert!(expected_path.exists());

        let content = fs::read_to_string(&expected_path).expect("Failed to read file");
        assert_eq!(content, "");
    }

    #[test]
    fn test_save_prompt_to_file_special_characters() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let temp_path = temp_dir.path().to_str().unwrap();
        let prompt_text =
            "Texto com acentos: açúcar, coração, não\nE caracteres especiais: @#$%&*()";
        let title = "special_chars";

        let result = save_prompt_to_file(prompt_text, temp_path, title);
        assert!(result.is_ok());

        let expected_path = temp_dir.path().join("special_chars.txt");
        let content = fs::read_to_string(&expected_path).expect("Failed to read file");
        assert_eq!(content, prompt_text);
    }

    #[test]
    fn test_save_prompt_to_specific_path_success() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let file_path = temp_dir.path().join("specific_test.txt");
        let file_path_str = file_path.to_str().unwrap();
        let prompt_text = "Teste de caminho específico";

        let result = save_prompt_to_specific_path(prompt_text, file_path_str);
        assert!(result.is_ok());

        assert!(file_path.exists());
        let content = fs::read_to_string(&file_path).expect("Failed to read file");
        assert_eq!(content, prompt_text);
    }

    #[test]
    fn test_save_prompt_to_specific_path_large_content() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let file_path = temp_dir.path().join("large_content.txt");
        let file_path_str = file_path.to_str().unwrap();
        let large_content = "Linha de texto ".repeat(1000);

        let result = save_prompt_to_specific_path(&large_content, file_path_str);
        assert!(result.is_ok());

        assert!(file_path.exists());
        let content = fs::read_to_string(&file_path).expect("Failed to read file");
        assert_eq!(content, large_content);
    }

    #[test]
    fn test_save_prompt_to_specific_path_overwrites_existing() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let file_path = temp_dir.path().join("overwrite_test.txt");
        let file_path_str = file_path.to_str().unwrap();

        // Primeiro conteúdo
        let first_content = "Primeiro conteúdo";
        let result1 = save_prompt_to_specific_path(first_content, file_path_str);
        assert!(result1.is_ok());

        // Segundo conteúdo (sobrescrever)
        let second_content = "Segundo conteúdo";
        let result2 = save_prompt_to_specific_path(second_content, file_path_str);
        assert!(result2.is_ok());

        // Verificar que foi sobrescrito
        let content = fs::read_to_string(&file_path).expect("Failed to read file");
        assert_eq!(content, second_content);
        assert_ne!(content, first_content);
    }

    #[test]
    fn test_save_prompt_to_file_multiline_content() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let temp_path = temp_dir.path().to_str().unwrap();
        let multiline_content = "Linha 1\nLinha 2\nLinha 3\n\nLinha 5 após linha vazia";
        let title = "multiline";

        let result = save_prompt_to_file(multiline_content, temp_path, title);
        assert!(result.is_ok());

        let expected_path = temp_dir.path().join("multiline.txt");
        let content = fs::read_to_string(&expected_path).expect("Failed to read file");
        assert_eq!(content, multiline_content);
        assert!(content.contains("Linha 1"));
        assert!(content.contains("Linha 5"));
    }

    #[test]
    fn test_save_prompt_invalid_path_fails() {
        let invalid_path = if cfg!(windows) {
            "Z:\\invalid\\path\\that\\does\\not\\exist\\file.txt"
        } else {
            "/invalid/path/that/does/not/exist/file.txt"
        };
        let prompt_text = "Teste com caminho inválido";

        let result = save_prompt_to_specific_path(prompt_text, invalid_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_file_path_construction() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let temp_path = temp_dir.path().to_str().unwrap();
        let title = "test_file_name";

        // Este teste verifica indiretamente a construção do caminho
        let result = save_prompt_to_file("test content", temp_path, title);
        assert!(result.is_ok());

        let expected_path = temp_dir.path().join("test_file_name.txt");
        assert!(expected_path.exists());
    }
}
