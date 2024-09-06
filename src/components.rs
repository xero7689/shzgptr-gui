use eframe::egui;
use egui::Color32;

pub struct MessageBox {
    pub frame: egui::Frame,
}

impl MessageBox {
    pub fn new(fill_color: Color32) -> Self {
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
        }
    }
}
