use eframe::egui::{self, Label, RichText};

#[derive(Default)]
pub struct Logs {
    items: Vec<String>,
}

impl Logs {
    pub fn new(items: Vec<String>) -> Self {
        Self { items }
    }

    pub fn show(&mut self, ctx: &egui::CtxRef, is_open: &mut bool) {
        egui::Window::new("Logs")
            .open(is_open)
            .default_height(300.0)
            .show(ctx, |ui| self.ui(ui));
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        if self.items.is_empty() {
            ui.add(Label::new(RichText::new(" - Empty - ").monospace()));
        } else {
            for item in self.items.iter() {
                ui.add(
                    Label::new(
                        RichText::new(format!("• {}", &item))
                            .color(egui::Color32::RED)
                            .small(),
                    )
                    .wrap(true),
                );
            }
        }
    }
}
