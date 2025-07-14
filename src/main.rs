#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use copypasta::{ClipboardContext, ClipboardProvider};
use rfd::FileDialog;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

mod models;
mod services;

use models::{
    auxiliary_content::AuxiliaryContent, context::Context, few_shot::FewShot, guidance::Guidance,
    limitations::Limitations, main_content::MainContent, output_format::OutputFormat,
    refactoring::Refactoring, tests::Tests,
};
use services::{file_service::save_prompt_to_specific_path, prompt_generator::PromptData};

slint::include_modules!();

// Parse saved prompt content and populate UI fields
fn parse_prompt_content(content: &str, ui: &AppWindow, data: &mut PromptData) {
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
            if line.starts_with("## Few-shot") {
                save_current_section(current_section, &section_content, ui, data);
                current_section = "few_shot";
                section_content.clear();
            } else if line.starts_with("## Contexto") {
                save_current_section(current_section, &section_content, ui, data);
                current_section = "context";
                section_content.clear();
            } else if line.starts_with("## Conteúdo Principal") {
                save_current_section(current_section, &section_content, ui, data);
                current_section = "main_content";
                section_content.clear();
            } else if line.starts_with("## Conteúdo Auxiliar") {
                save_current_section(current_section, &section_content, ui, data);
                current_section = "auxiliary_content";
                section_content.clear();
            } else if line.starts_with("## Limitações") {
                save_current_section(current_section, &section_content, ui, data);
                current_section = "limitations";
                section_content.clear();
            } else if line.starts_with("## Refatoração") {
                save_current_section(current_section, &section_content, ui, data);
                current_section = "refactoring";
                section_content.clear();
            } else if line.starts_with("## Orientações") {
                save_current_section(current_section, &section_content, ui, data);
                current_section = "guidance";
                section_content.clear();
            } else if line.starts_with("## Testes") {
                save_current_section(current_section, &section_content, ui, data);
                current_section = "tests";
                section_content.clear();
            } else if line.starts_with("## Formato de Saída") {
                save_current_section(current_section, &section_content, ui, data);
                current_section = "output_format";
                section_content.clear();
            } else if line.starts_with("---") || line.starts_with("*Prompt gerado") {
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
        save_current_section(current_section, &section_content, ui, data);
    } else {
        // Parse simple format - try to intelligently distribute content
        let content = content.trim();
        if !content.is_empty() {
            println!("📝 Carregando formato simples - tentando distribuir inteligentemente");
            parse_simple_format(content, ui, data);
        }
    }
}

// Parse simple format files by trying to intelligently distribute content
fn parse_simple_format(content: &str, ui: &AppWindow, data: &mut PromptData) {
    let paragraphs: Vec<&str> = content
        .split("\n\n")
        .filter(|p| !p.trim().is_empty())
        .collect();

    if paragraphs.len() == 1 {
        // Single paragraph - put in main content
        let text = paragraphs[0].trim();
        println!("📝 Conteúdo único - carregando como conteúdo principal");
        ui.set_main_content_text(text.into());
        let mut main_content = MainContent::new();
        main_content.instructions = text.to_string();
        data.main_content = Some(main_content);
    } else {
        // Multiple paragraphs - distribute intelligently
        println!(
            "📝 Múltiplos parágrafos ({}) - distribuindo inteligentemente",
            paragraphs.len()
        );

        for (i, paragraph) in paragraphs.iter().enumerate() {
            let text = paragraph.trim();
            if text.is_empty() {
                continue;
            }

            // Distribute based on position and content
            match i {
                0 => {
                    // First paragraph - likely context or introduction
                    if text.len() > 200
                        && (text.contains("você é")
                            || text.contains("assistente")
                            || text.contains("especializado"))
                    {
                        // Looks like context
                        println!("📝 Parágrafo 1 -> Contexto (detecção automática)");
                        ui.set_context_text(text.into());
                        let mut context = Context::new();
                        context.description = text.to_string();
                        data.context = Some(context);
                    } else {
                        // Short or different content - main content
                        println!("📝 Parágrafo 1 -> Conteúdo Principal");
                        ui.set_main_content_text(text.into());
                        let mut main_content = MainContent::new();
                        main_content.instructions = text.to_string();
                        data.main_content = Some(main_content);
                    }
                }
                1 => {
                    // Second paragraph
                    if data.main_content.is_none() {
                        // If main content wasn't set, use this
                        println!("📝 Parágrafo 2 -> Conteúdo Principal");
                        ui.set_main_content_text(text.into());
                        let mut main_content = MainContent::new();
                        main_content.instructions = text.to_string();
                        data.main_content = Some(main_content);
                    } else {
                        // Use as auxiliary content
                        println!("📝 Parágrafo 2 -> Conteúdo Auxiliar");
                        ui.set_auxiliary_content_text(text.into());
                        let mut auxiliary = AuxiliaryContent::new();
                        auxiliary.data = text.to_string();
                        data.auxiliary_content = Some(auxiliary);
                    }
                }
                2 => {
                    // Third paragraph - auxiliary or guidance
                    if text.contains("sempre") || text.contains("quando") || text.contains("use") {
                        println!("📝 Parágrafo 3 -> Orientações (detecção automática)");
                        ui.set_guidance_text(text.into());
                        let mut guidance = Guidance::new();
                        guidance.text = text.to_string();
                        data.guidance = Some(guidance);
                    } else {
                        println!("📝 Parágrafo 3 -> Conteúdo Auxiliar");
                        ui.set_auxiliary_content_text(text.into());
                        let mut auxiliary = AuxiliaryContent::new();
                        auxiliary.data = text.to_string();
                        data.auxiliary_content = Some(auxiliary);
                    }
                }
                _ => {
                    // Additional paragraphs - put in auxiliary content
                    println!("📝 Parágrafo {} -> Conteúdo Auxiliar", i + 1);
                    let current_aux = ui.get_auxiliary_content_text().to_string();
                    let new_content = if current_aux.is_empty() {
                        text.to_string()
                    } else {
                        format!("{}\n\n{}", current_aux, text)
                    };
                    ui.set_auxiliary_content_text(new_content.clone().into());
                    let mut auxiliary = AuxiliaryContent::new();
                    auxiliary.data = new_content;
                    data.auxiliary_content = Some(auxiliary);
                }
            }
        }
    }
}

// Helper function to save section content to UI and data
fn save_current_section(section: &str, content: &str, ui: &AppWindow, data: &mut PromptData) {
    let trimmed_content = content.trim();
    if trimmed_content.is_empty() {
        return;
    }

    match section {
        "few_shot" => {
            ui.set_few_shot_text(trimmed_content.into());
            let mut few_shot = FewShot::new();
            few_shot.content = trimmed_content.to_string();
            data.few_shot = Some(few_shot);
        }
        "context" => {
            ui.set_context_text(trimmed_content.into());
            let mut context = Context::new();
            context.description = trimmed_content.to_string();
            data.context = Some(context);
        }
        "main_content" => {
            ui.set_main_content_text(trimmed_content.into());
            let mut main_content = MainContent::new();
            main_content.instructions = trimmed_content.to_string();
            data.main_content = Some(main_content);
        }
        "auxiliary_content" => {
            ui.set_auxiliary_content_text(trimmed_content.into());
            let mut auxiliary = AuxiliaryContent::new();
            auxiliary.data = trimmed_content.to_string();
            data.auxiliary_content = Some(auxiliary);
        }
        "limitations" => {
            ui.set_limitations_text(trimmed_content.into());
            let mut limitations = Limitations::new();
            limitations.text = trimmed_content.to_string();
            data.limitations = Some(limitations);
        }
        "refactoring" => {
            ui.set_refactoring_text(trimmed_content.into());
            let mut refactoring = Refactoring::new();
            refactoring.text = trimmed_content.to_string();
            data.refactoring = Some(refactoring);
        }
        "guidance" => {
            ui.set_guidance_text(trimmed_content.into());
            let mut guidance = Guidance::new();
            guidance.text = trimmed_content.to_string();
            data.guidance = Some(guidance);
        }
        "tests" => {
            ui.set_tests_text(trimmed_content.into());
            let mut tests = Tests::new();
            tests.text = trimmed_content.to_string();
            data.tests = Some(tests);
        }
        "output_format" => {
            ui.set_output_format_text(trimmed_content.into());
            let mut output_format = OutputFormat::new();
            output_format.text = trimmed_content.to_string();
            data.output_format = Some(output_format);
        }
        _ => {}
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    // Initialize prompt data in a shared state
    let prompt_data = Rc::new(RefCell::new(PromptData::new()));

    // Set up callbacks for UI events
    let ui_weak = ui.as_weak();
    let prompt_data_clone = prompt_data.clone();

    // Generate prompt callback
    ui.on_generate_prompt(move || {
        let ui = ui_weak.unwrap();
        let mut data = prompt_data_clone.borrow_mut();

        // Clear previous data
        *data = PromptData::new();

        println!("🔄 Gerando preview do prompt...");

        // Collect all field values first
        let few_shot_val = ui.get_few_shot_text().to_string();
        let context_val = ui.get_context_text().to_string();
        let main_content_val = ui.get_main_content_text().to_string();
        let auxiliary_val = ui.get_auxiliary_content_text().to_string();
        let limitations_val = ui.get_limitations_text().to_string();
        let refactoring_val = ui.get_refactoring_text().to_string();
        let guidance_val = ui.get_guidance_text().to_string();
        let tests_val = ui.get_tests_text().to_string();
        let output_format_val = ui.get_output_format_text().to_string();

        // Debug: Check if UI values are being read
        println!("📝 Valores dos campos:");
        println!("  Few-shot: '{}'", few_shot_val);
        println!("  Context: '{}'", context_val);
        println!("  Main content: '{}'", main_content_val);
        println!("  Auxiliary: '{}'", auxiliary_val);
        println!("  Limitations: '{}'", limitations_val);
        println!("  Refactoring: '{}'", refactoring_val);
        println!("  Guidance: '{}'", guidance_val);
        println!("  Tests: '{}'", tests_val);
        println!("  Output format: '{}'", output_format_val);

        // Update prompt data with UI values
        if !few_shot_val.trim().is_empty() {
            println!("✅ Adicionando Few-shot");
            let mut few_shot = FewShot::new();
            few_shot.content = few_shot_val;
            data.few_shot = Some(few_shot);
        }

        if !context_val.trim().is_empty() {
            println!("✅ Adicionando Context");
            let mut context = Context::new();
            context.description = context_val;
            data.context = Some(context);
        }

        if !main_content_val.trim().is_empty() {
            println!("✅ Adicionando Main Content");
            let mut main_content = MainContent::new();
            main_content.instructions = main_content_val;
            data.main_content = Some(main_content);
        }

        if !auxiliary_val.trim().is_empty() {
            println!("✅ Adicionando Auxiliary Content");
            let mut aux_content = AuxiliaryContent::new();
            aux_content.data = auxiliary_val;
            data.auxiliary_content = Some(aux_content);
        }

        if !limitations_val.trim().is_empty() {
            println!("✅ Adicionando Limitations");
            let mut limitations = Limitations::new();
            limitations.text = limitations_val;
            data.limitations = Some(limitations);
        }

        if !refactoring_val.trim().is_empty() {
            println!("✅ Adicionando Refactoring");
            let mut refactoring = Refactoring::new();
            refactoring.text = refactoring_val;
            data.refactoring = Some(refactoring);
        }

        if !guidance_val.trim().is_empty() {
            println!("✅ Adicionando Guidance");
            let mut guidance = Guidance::new();
            guidance.text = guidance_val;
            data.guidance = Some(guidance);
        }

        if !tests_val.trim().is_empty() {
            println!("✅ Adicionando Tests");
            let mut tests = Tests::new();
            tests.text = tests_val;
            data.tests = Some(tests);
        }

        if !output_format_val.trim().is_empty() {
            println!("✅ Adicionando Output Format");
            let mut output_format = OutputFormat::new();
            output_format.text = output_format_val;
            data.output_format = Some(output_format);
        }

        // Generate the prompt with clean formatting for preview
        let generated_prompt = data.build_preview_prompt();

        // Debug: print generated prompt to console
        println!(
            "📝 Prompt gerado ({} caracteres):\n{}",
            generated_prompt.len(),
            generated_prompt
        );

        // Force update the preview regardless of content
        println!("🔄 Atualizando preview...");
        ui.set_preview_text(generated_prompt.into());
        println!("✅ Preview atualizado!");
    });

    // Save prompt callback
    let ui_weak2 = ui.as_weak();
    let prompt_data_clone2 = prompt_data.clone();
    ui.on_save_prompt(move || {
        let _ui = ui_weak2.unwrap();
        let data = prompt_data_clone2.borrow();

        let prompt_text = data.build_prompt(false); // Save without markers

        // Open file dialog to choose save location
        if let Some(file_path) = FileDialog::new()
            .set_title("Salvar Prompt")
            .set_file_name("generated_prompt.txt")
            .add_filter("Arquivo de Texto", &["txt"])
            .add_filter("Todos os Arquivos", &["*"])
            .save_file()
        {
            match save_prompt_to_specific_path(&prompt_text, &file_path.to_string_lossy()) {
                Ok(_) => println!("✅ Prompt salvo com sucesso em: {:?}", file_path),
                Err(e) => eprintln!("❌ Erro ao salvar prompt: {}", e),
            }
        } else {
            println!("💭 Salvamento cancelado pelo usuário");
        }
    });

    // Clear all callback
    let ui_weak3 = ui.as_weak();
    let prompt_data_clone3 = prompt_data.clone();
    ui.on_clear_all(move || {
        let ui = ui_weak3.unwrap();
        let mut data = prompt_data_clone3.borrow_mut();

        // Clear all data
        *data = PromptData::new();

        // Clear UI fields
        ui.set_few_shot_text("".into());
        ui.set_context_text("".into());
        ui.set_main_content_text("".into());
        ui.set_auxiliary_content_text("".into());
        ui.set_limitations_text("".into());
        ui.set_refactoring_text("".into());
        ui.set_guidance_text("".into());
        ui.set_tests_text("".into());
        ui.set_output_format_text("".into());
        ui.set_preview_text("O preview do prompt aparecerá aqui...".into());
    });

    // Copy to clipboard callback
    let ui_weak4 = ui.as_weak();
    let prompt_data_clone4 = prompt_data.clone();
    ui.on_copy_to_clipboard(move || {
        let _ui = ui_weak4.unwrap();
        let data = prompt_data_clone4.borrow();

        let prompt_text = data.build_prompt(false); // Copy without markers

        // Try to copy to clipboard
        match ClipboardContext::new() {
            Ok(mut ctx) => match ctx.set_contents(prompt_text) {
                Ok(_) => println!("✅ Prompt copiado para a área de transferência!"),
                Err(e) => eprintln!("❌ Erro ao copiar: {}", e),
            },
            Err(e) => eprintln!("❌ Erro ao acessar área de transferência: {}", e),
        }
    });

    // Open prompt callback
    let ui_weak5 = ui.as_weak();
    let prompt_data_clone5 = prompt_data.clone();
    ui.on_open_prompt(move || {
        let ui = ui_weak5.unwrap();
        let mut data = prompt_data_clone5.borrow_mut();

        println!("📂 Abrindo dialog para selecionar arquivo...");

        // Open file dialog
        if let Some(path) = FileDialog::new()
            .add_filter("Arquivos de texto", &["txt"])
            .set_title("Abrir Prompt Salvo")
            .pick_file()
        {
            println!("📄 Arquivo selecionado: {:?}", path);

            // Read file content
            match std::fs::read_to_string(&path) {
                Ok(content) => {
                    println!("✅ Arquivo carregado com sucesso!");

                    // Clear current data and UI
                    *data = PromptData::new();
                    ui.set_few_shot_text("".into());
                    ui.set_context_text("".into());
                    ui.set_main_content_text("".into());
                    ui.set_auxiliary_content_text("".into());
                    ui.set_limitations_text("".into());
                    ui.set_refactoring_text("".into());
                    ui.set_guidance_text("".into());
                    ui.set_tests_text("".into());
                    ui.set_output_format_text("".into());
                    ui.set_preview_text("O preview do prompt aparecerá aqui...".into());

                    // Parse the content to extract sections
                    parse_prompt_content(&content, &ui, &mut data);

                    // Generate preview automatically after loading
                    println!("� Gerando preview automaticamente...");
                    let generated_prompt = data.build_preview_prompt();
                    ui.set_preview_text(generated_prompt.into());

                    println!("�📝 Prompt carregado, campos preenchidos e preview atualizado!");
                }
                Err(e) => {
                    eprintln!("❌ Erro ao ler arquivo: {}", e);
                }
            }
        } else {
            println!("❌ Nenhum arquivo selecionado.");
        }
    });

    ui.run()?;

    Ok(())
}
