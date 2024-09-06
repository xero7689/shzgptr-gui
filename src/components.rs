use eframe::egui;
use egui::Color32;

use crate::markdown::{parse_markdown, Block, BlockType};

pub struct MessageBox {
    pub frame: egui::Frame,
    font_id: egui::FontId,
    text_blocks: Vec<Block>,
}

impl MessageBox {
    pub fn new(text: &String, fill_color: Color32) -> Self {
        let text_blocks = parse_markdown(text);

        Self {
            frame: egui::Frame {
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
                    egui::Color32::from_rgba_unmultiplied(219, 216, 227, 128),
                ),
            },
            font_id: egui::FontId {
                size: 14.0,
                family: egui::FontFamily::Monospace,
            },
            text_blocks,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        self.frame.show(ui, |ui| {
            for block in &self.text_blocks {
                // We use the reference to the text_block
                match &block.block_type {
                    // So we can only match the reference to the block_type,
                    // otherwise the String would be moved which is not allowed
                    BlockType::Text => {
                        ui.label(
                            egui::RichText::new(block.content.clone())
                                .color(Color32::from_rgb(250, 240, 230))
                                .font(self.font_id.clone()),
                        );
                    }
                    BlockType::Code(language) => {
                        let theme =
                            egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx());

                        egui_extras::syntax_highlighting::code_view_ui(
                            ui,
                            &theme,
                            &block.content,
                            language,
                        );
                    }
                    BlockType::Heading(level) => {
                        let heading = match level {
                            1 => egui::FontId {
                                size: 24.0,
                                family: egui::FontFamily::Proportional,
                            },
                            2 => egui::FontId {
                                size: 20.0,
                                family: egui::FontFamily::Proportional,
                            },
                            3 => egui::FontId {
                                size: 16.0,
                                family: egui::FontFamily::Proportional,
                            },
                            _ => egui::FontId {
                                size: 14.0,
                                family: egui::FontFamily::Proportional,
                            },
                        };

                        ui.label(
                            egui::RichText::new(block.content.clone())
                                .color(Color32::from_rgb(250, 240, 230))
                                .font(heading),
                        );
                    }
                };
            }
        });
    }
}
