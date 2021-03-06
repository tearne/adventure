use eframe::{
    egui::{self, Checkbox, FontDefinitions, FontFamily, RichText, TextStyle},
    epi,
};

use crate::{data::RawData, editor::Editor, game::Game, inventory::Inventory};

pub struct App {
    //https://stackoverflow.com/questions/32300132/why-cant-i-store-a-value-and-a-reference-to-that-value-in-the-same-struct
    editor: Editor,
    game: Game,
    inventory: Inventory,
    show_inventory: bool,
    show_editor: bool,
    show_logs: bool,
    current_step_name: String,
}

impl App {
    pub fn new(source: String) -> Self {
        let raw_data = serde_json::from_str::<RawData>(&source).unwrap();
        let game = raw_data.to_game();
        let current_step_name = game.start_step_name.clone();

        Self {
            editor: Editor::new(&source),
            game,
            current_step_name,
            inventory: Inventory::new(),
            show_inventory: false,
            show_editor: false,
            show_logs: false,
        }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "egui template"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
        let mut fonts = FontDefinitions::default();

        // Large button text:
        fonts
            .family_and_size
            .insert(TextStyle::Button, (FontFamily::Proportional, 40.0));
        fonts
            .family_and_size
            .insert(TextStyle::Body, (FontFamily::Proportional, 40.0));
        fonts
            .family_and_size
            .insert(TextStyle::Small, (FontFamily::Proportional, 28.0));

        ctx.set_fonts(fonts);

        let Self {
            editor,
            game,
            inventory,
            show_inventory,
            show_editor,
            show_logs,
            current_step_name,
        } = self;

        inventory.show(ctx, show_inventory);

        if let Some(new_game) = editor.show(ctx, show_editor) {
            *game = new_game;
            *current_step_name = game.start_step_name.clone();
        }

        game.logs.show(ctx, show_logs);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                let checkbox =
                    Checkbox::new(show_inventory, RichText::new("Show inventory").small());
                ui.add(checkbox);
                ui.separator();
                let checkbox = Checkbox::new(show_editor, RichText::new("Show editor").small());
                ui.add(checkbox);
                ui.separator();
                let checkbox = Checkbox::new(show_logs, RichText::new("Show logs").small());
                ui.add(checkbox);
            });
            ui.separator();

            game.show_and_update(ui, current_step_name, inventory);
        });
    }
}
