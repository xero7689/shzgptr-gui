#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(rustdoc::missing_crate_level_docs)]

pub mod openai;

use dotenvy::dotenv;
use eframe::egui;
use std::sync::{Arc, Mutex};
use std::{env, thread};

use crate::openai::OpenAIClient;

#[tokio::main]
async fn main() -> eframe::Result {
    dotenv().ok();

    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };

    eframe::run_native(
        "SHZ-GPT-R - OpenAI Chatbot",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    user_prompt: String,
    assistant_prompt: Arc<Mutex<String>>,
    llm_client_triggered: Arc<Mutex<bool>>,
    openai_api_key: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            user_prompt: "".to_string(),
            assistant_prompt: Arc::new(Mutex::new("".to_string())),
            llm_client_triggered: Arc::new(Mutex::new(false)),
            openai_api_key: env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set"),
        }
    }
}

impl MyApp {
    fn trigger_llm_client(&self) {
        let mut triggered = self.llm_client_triggered.lock().unwrap();

        if !*triggered {
            *triggered = true;

            let llm_client_triggered = Arc::clone(&self.llm_client_triggered);
            let user_prompt = self.user_prompt.clone();
            let assistant_prompt = Arc::clone(&self.assistant_prompt);
            let openai_api_key = self.openai_api_key.clone();

            thread::spawn(move || {
                println!("Function Triggered in a seprate thread!");

                let llm_client = OpenAIClient {
                    api_key: openai_api_key,
                    model_id: "gpt-4o-mini".into(),
                };

                if let Ok(response) = llm_client.chat_completions_in_thread(user_prompt) {
                    let assistant_message = response.choices[0].message.content.clone();
                    println!("Assistant Prompt: {}", assistant_message);

                    let mut prompt = assistant_prompt.lock().unwrap();
                    *prompt = assistant_message;
                } else {
                    println!("Error occured while processing the request");
                }

                let mut triggered = llm_client_triggered.lock().unwrap();
                *triggered = false;
            });
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("Assistant Prompt",));
            ui.vertical(|ui| {
                egui::Frame::default().show(ui, |ui| {
                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
                    let text = self.assistant_prompt.lock().unwrap().clone();
                    ui.label(egui::RichText::new(text).code());
                });
            })
        });

        egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let prompt_label = ui.label("User prompt: ");
                ui.text_edit_singleline(&mut self.user_prompt)
                    .labelled_by(prompt_label.id);

                if ui.button("Send").clicked() {
                    self.trigger_llm_client();
                }
            });
        });
    }
}
