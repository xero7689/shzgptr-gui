pub mod openai;

use crate::openai::OpenAIClient;
use eframe::egui;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::{env, thread};

#[derive(Clone, PartialEq, Debug)]
enum Role {
    User,
    Assistant,
}

#[derive(Clone, Debug)]
struct Message {
    role: Role,
    message: String,
}

pub struct MyApp {
    user_prompt: String,
    assistant_prompt: Arc<Mutex<String>>,
    llm_client_triggered: Arc<Mutex<bool>>,
    openai_api_key: String,
    chat_history: Vec<Message>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            user_prompt: "".to_string(),
            assistant_prompt: Arc::new(Mutex::new("".to_string())),
            llm_client_triggered: Arc::new(Mutex::new(false)),
            openai_api_key: env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set"),
            chat_history: vec![],
        }
    }
}

impl fmt::Display for MyApp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "User Prompt: {}\nAssistant Prompt: {:?}\nLLM Triggered: {:?}\nChat History: {:?}",
            self.user_prompt, self.assistant_prompt, self.llm_client_triggered, self.chat_history
        )
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
                let llm_client = OpenAIClient::new(openai_api_key, None, None, None);

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
        // Check if assistant_prompt has been updated by the thread
        if let Ok(mut prompt) = self.assistant_prompt.lock() {
            if !prompt.is_empty() && self.chat_history.last().unwrap().role != Role::Assistant {
                println!("Found Assistant Prompt is not empty");
                let assistant_message = Message {
                    role: Role::Assistant,
                    message: prompt.clone(),
                };
                self.chat_history.push(assistant_message);
                prompt.clear();

                println!("My App State: {}", self);
            }
        }

        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("SHZ-GPT-R - OpenAI Chatbot");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical(|ui| {
                    egui::Frame::default().show(ui, |ui| {
                        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
                        for item in &self.chat_history {
                            let (fill_color, layout) = match item.role {
                                Role::User => (
                                    egui::Color32::from_rgba_unmultiplied(92, 84, 112, 128),
                                    egui::Layout::top_down(egui::Align::Max),
                                ),
                                Role::Assistant => (
                                    egui::Color32::from_rgba_unmultiplied(53, 47, 68, 128),
                                    egui::Layout::top_down(egui::Align::Min),
                                ),
                            };
                            let text = format!("{}", item.message);

                            ui.with_layout(layout, |ui| {
                                egui::Frame::default()
                                    .rounding(ui.visuals().widgets.noninteractive.rounding)
                                    .show(ui, |ui| {
                                        let frame = egui::Frame {
                                            inner_margin: 12.0.into(),
                                            outer_margin: 12.0.into(),
                                            rounding: 14.0.into(),
                                            shadow: egui::Shadow {
                                                offset: [4.0, 8.0].into(),
                                                blur: 16.0,
                                                spread: 0.0,
                                                color: egui::Color32::from_black_alpha(180),
                                            },
                                            fill: fill_color,
                                            stroke: egui::Stroke::new(
                                                0.0,
                                                egui::Color32::from_rgba_unmultiplied(
                                                    219, 216, 227, 128,
                                                ),
                                            ),
                                        };
                                        frame.show(ui, |ui| {
                                            //ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                                            ui.label(
                                                egui::RichText::new(text)
                                                    .color(egui::Color32::WHITE),
                                            );
                                        });
                                    });
                            });
                        }
                    });
                })
            })
        });

        egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
            let mut layout = egui::Layout::top_down_justified(egui::Align::Center);
            layout.cross_justify = true;

            ui.horizontal(|ui| {
                ui.with_layout(layout, |ui| {
                    ui.text_edit_singleline(&mut self.user_prompt);

                    if ui.button("Send").clicked() {
                        if self.user_prompt.is_empty() {
                            return;
                        }

                        let user_message = Message {
                            role: Role::User,
                            message: self.user_prompt.to_string(),
                        };
                        self.chat_history.push(user_message);
                        self.trigger_llm_client();
                        self.user_prompt.clear();
                    }
                });
            });
        });
    }
}
