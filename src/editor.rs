use eframe::egui::{epaint::Fonts, TextStyle, Color32, Ui, Button, ScrollArea, TextEdit, Window};

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

    pub fn show(&mut self, ctx: &eframe::egui::CtxRef, is_open: &mut bool) -> Option<Game> {
        let result = Window::new("Editor1")
            .open(is_open)
            .vscroll(false)
            .resizable(true)
            .default_height(300.0)
            .show(ctx, |ui| 
                self.ui(ui)
            );


            // Window::new("↔ resizable with TextEdit")
            // .open(open)
            // .vscroll(false)
            // .resizable(true)
            // .default_height(300.0)

        result.map(|i| i.inner).flatten().flatten()
    }

    fn ui(&mut self, ui: &mut Ui) -> Option<Game> {
        let clicked = ui.add(Button::new("Reload Game")).clicked();

        // let mut layouter = |ui: &Ui, string: &str, wrap_width: f32| {
        //     ui.fonts().layout(string.into(), TextStyle::Monospace, Color32::DARK_BLUE, wrap_width)
        // };

        
        ScrollArea::vertical().show(ui, |ui| {
            ui.add_sized(ui.available_size(), TextEdit::multiline(&mut self.text));
        });

        // ui.allocate_space(ui.available_size());

        if clicked {
            serde_json::from_str::<RawData>(&self.text)
                .ok()
                .map(|d|d.to_game())
        } else {
            None
        }
    }
}
