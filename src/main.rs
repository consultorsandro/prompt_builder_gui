#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::rc::Rc;
use std::cell::RefCell;
use copypasta::{ClipboardContext, ClipboardProvider};

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
    file_service::save_prompt_to_file,
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
        
        // Update prompt data with UI values
        if !ui.get_few_shot_text().is_empty() {
            let mut few_shot = FewShot::new();
            few_shot.example_input = ui.get_few_shot_text().to_string();
            few_shot.expected_output = "".to_string();
            data.few_shot = Some(few_shot);
        }
        
        if !ui.get_context_text().is_empty() {
            let mut context = Context::new();
            context.description = ui.get_context_text().to_string();
            data.context = Some(context);
        }
        
        if !ui.get_main_content_text().is_empty() {
            let mut main_content = MainContent::new();
            main_content.instructions = ui.get_main_content_text().to_string();
            data.main_content = Some(main_content);
        }
        
        if !ui.get_auxiliary_content_text().is_empty() {
            let mut aux_content = AuxiliaryContent::new();
            aux_content.data = ui.get_auxiliary_content_text().to_string();
            data.auxiliary_content = Some(aux_content);
        }
        
        if !ui.get_limitations_text().is_empty() {
            let mut limitations = Limitations::new();
            limitations.text = ui.get_limitations_text().to_string();
            data.limitations = Some(limitations);
        }
        
        if !ui.get_refactoring_text().is_empty() {
            let mut refactoring = Refactoring::new();
            refactoring.text = ui.get_refactoring_text().to_string();
            data.refactoring = Some(refactoring);
        }
        
        if !ui.get_guidance_text().is_empty() {
            let mut guidance = Guidance::new();
            guidance.text = ui.get_guidance_text().to_string();
            data.guidance = Some(guidance);
        }
        
        if !ui.get_tests_text().is_empty() {
            let mut tests = Tests::new();
            tests.text = ui.get_tests_text().to_string();
            data.tests = Some(tests);
        }
        
        if !ui.get_output_format_text().is_empty() {
            let mut output_format = OutputFormat::new();
            output_format.text = ui.get_output_format_text().to_string();
            data.output_format = Some(output_format);
        }
        
        // Generate the prompt
        let generated_prompt = data.build_prompt(false);
        ui.set_preview_text(generated_prompt.into());
    });

    // Save prompt callback
    let ui_weak2 = ui.as_weak();
    let prompt_data_clone2 = prompt_data.clone();
    ui.on_save_prompt(move || {
        let ui = ui_weak2.unwrap();
        let data = prompt_data_clone2.borrow();
        
        let prompt_text = data.build_prompt(false);
        let output_dir = std::env::current_dir().unwrap_or_default();
        let output_dir_str = output_dir.to_string_lossy();
        
        match save_prompt_to_file(&prompt_text, &output_dir_str, "generated_prompt") {
            Ok(_) => println!("✅ Prompt salvo com sucesso!"),
            Err(e) => eprintln!("❌ Erro ao salvar prompt: {}", e),
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
        let ui = ui_weak4.unwrap();
        let data = prompt_data_clone4.borrow();
        
        let prompt_text = data.build_prompt(false);
        
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

