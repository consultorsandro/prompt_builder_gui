#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::rc::Rc;
use std::cell::RefCell;
use copypasta::{ClipboardContext, ClipboardProvider};
use rfd::FileDialog;

mod models;
mod services;

use models::{
    few_shot::FewShot,
    context::Context,
    main_content::MainContent,
    auxiliary_content::AuxiliaryContent,
    limitations::Limitations,
    refactoring::Refactoring,
    guidance::Guidance,
    tests::Tests,
    output_format::OutputFormat,
};
use services::{
    prompt_generator::PromptData,
    file_service::save_prompt_to_specific_path,
};

slint::include_modules!();

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
            few_shot.example_input = few_shot_val;
            few_shot.expected_output = "".to_string();
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
        
        // Generate the prompt with section markers for better visibility in preview
        let generated_prompt = data.build_prompt(true);
        
        // Debug: print generated prompt to console
        println!("📝 Prompt gerado ({} caracteres):\n{}", generated_prompt.len(), generated_prompt);
        
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
            Ok(mut ctx) => {
                match ctx.set_contents(prompt_text) {
                    Ok(_) => println!("✅ Prompt copiado para a área de transferência!"),
                    Err(e) => eprintln!("❌ Erro ao copiar: {}", e),
                }
            }
            Err(e) => eprintln!("❌ Erro ao acessar área de transferência: {}", e),
        }
    });

    ui.run()?;

    Ok(())
}

