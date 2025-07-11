use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

/// Saves the given prompt text to a file with the specified title.
/// The file will be saved with `.txt` extension in the given directory.
pub fn save_prompt_to_file(prompt_text: &str, output_directory: &str, title: &str) -> io::Result<()> {
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
