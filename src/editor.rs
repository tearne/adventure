use eframe::egui;

use crate::{data::RawData, game::Game};

pub struct Editor {
    text: String,
}

impl Editor {
    pub fn new(source: &str) -> Self {
        Self {
            text: source.into(),
        }
    }

    pub fn show(&mut self, ctx: &egui::CtxRef, is_open: &mut bool) -> Option<Game> {
        let result = egui::Window::new("Editor")
            .open(is_open)
            .default_height(300.0)
            .show(ctx, |ui| self.ui(ui));

        result.map(|i| i.inner).flatten().flatten()
    }

    fn ui(&mut self, ui: &mut egui::Ui) -> Option<Game> {
        let clicked = ui.add(egui::Button::new("Reload Game")).clicked();

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(&mut self.text)
                    .text_style(egui::TextStyle::Monospace) // for cursor height
                    .code_editor()
                    .desired_rows(10)
                    .lock_focus(true)
                    .desired_width(f32::INFINITY),
            );
        });

        if clicked {
            serde_json::from_str::<RawData>(&self.text)
                .ok()
                .map(|d|d.to_game())
        } else {
            None
        }
    }
}
