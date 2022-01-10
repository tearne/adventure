use std::collections::HashSet;

use eframe::egui::{self, Label, RichText};

pub struct Inventory {
    items: HashSet<String>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            items: HashSet::new(),
        }
    }

    pub fn insert(&mut self, item: String) {
        self.items.insert(item);
    }

    pub fn contains(&self, item: &str) -> bool {
        self.items.contains(item)
    }

    pub fn remove(&mut self, item: &str) {
        self.items.remove(item);
    }

    pub fn clear(&mut self) {
        self.items.clear()
    }

    pub fn show(&mut self, ctx: &egui::CtxRef, is_open: &mut bool) {
        egui::Window::new("Inventory")
            .open(is_open)
            .default_height(300.0)
            .show(ctx, |ui| self.ui(ui));
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        if self.items.is_empty() {
            ui.add(Label::new(RichText::new(" - Empty - ").monospace()));
        } else {
            for item in self.items.iter() {
                ui.add(Label::new(format!("• {}", &item)).wrap(true).small());
            }
        };
    }
}
