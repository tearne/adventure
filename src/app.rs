use std::collections::HashSet;

use eframe::{egui::{self, Checkbox, FontDefinitions, FontFamily, Label, TextStyle, RichText}, epi};

use crate::{data::{Game, Step, RawData}, editor::Editor, inventory::Inventory};

pub struct App {
    //https://stackoverflow.com/questions/32300132/why-cant-i-store-a-value-and-a-reference-to-that-value-in-the-same-struct
    editor: Editor,
    game: Game,
    step_name: String,
    inventory: Inventory,
    show_inventory: bool,
    show_editor: bool,
}

impl App {
    pub fn new(source: String) -> Self {
        Self {
            editor: Editor::new(&source),
            game: serde_json::from_str::<RawData>(&source).unwrap().to_game(),
            step_name: String::from("start"),
            inventory: Inventory::new(),
            show_inventory: false,
            show_editor: true,
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
            .insert(TextStyle::Small, (FontFamily::Proportional, 32.0));

        ctx.set_fonts(fonts);

        let Self {
            editor,
            game,
            step_name,
            inventory,
            show_inventory,
            show_editor,
        } = self;

        inventory.show(ctx, show_inventory);

        if let Some(new_game) = editor.show(ctx, show_editor) {
            *game = new_game;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let checkbox = Checkbox::new(show_inventory, RichText::new("Show inventory").small());
            ui.add(checkbox);

            let checkbox = Checkbox::new(show_editor, RichText::new("Show editor").small());
            ui.add(checkbox);

            for warning in &game.warnings {
                ui.add(Label::new(
                    RichText::new(warning).color(egui::Color32::RED).small()
                ).wrap(true));
            }

            let step = &game.data[step_name];
            match step {
                Step::D(d) => {
                    ui.add(
                        Label::new(format!("You are at stage: {}", &d.name))
                            .wrap(true)
                            .small(),
                    );
                    ui.separator();
                    ui.add(Label::new(&d.text).wrap(true));
                    ui.separator();
                    d.acquire.iter().for_each(|item| {
                        let _ = inventory.insert(item.clone());
                    });
                    for opt in d.opts.iter() {
                        let is_available = opt
                            .requires
                            .as_ref()
                            .map(|item| inventory.contains(item))
                            .unwrap_or(true);

                        if is_available && ui.button(&opt.text).clicked() {
                            opt.requires.iter().for_each(|item| {
                                let _ = inventory.remove(item);
                            });
                            *step_name = opt.goto.clone();
                        }
                    }
                }
                Step::F(f) => {
                    ui.add(
                        Label::new(format!("Stage name: {}", &f.name))
                            .wrap(true)
                            .small(),
                    );
                    ui.separator();
                    ui.add(Label::new(&f.text).wrap(true));
                    ui.separator();
                    f.acquire.iter().for_each(|item| {
                        let _ = inventory.insert(item.clone());
                    });
                    if ui.button("Continue").clicked() {
                        *step_name = f.goto.clone();
                    }
                }
                Step::T(t) => {
                    ui.add(Label::new(&format!("Not written yet: {}", t.name)).wrap(true));

                    if ui.button("Restart").clicked() {
                        inventory.clear();
                        *step_name = "start".into();
                    }
                }
            };
        });
    }
}
