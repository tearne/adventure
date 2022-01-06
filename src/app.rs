use std::collections::HashSet;

use eframe::{egui::{self, Checkbox, FontDefinitions, FontFamily, Label, TextStyle}, epi};

use crate::data::{Game, Step};

pub struct TemplateApp {
    //https://stackoverflow.com/questions/32300132/why-cant-i-store-a-value-and-a-reference-to-that-value-in-the-same-struct
    game: Game,
    step_name: String,
    inventory: HashSet<String>,
    show_inventory: bool,
}

impl TemplateApp {
    pub fn new(game: Game) -> Self {
        Self {
            game,
            step_name: String::from("start"),
            inventory: HashSet::new(),
            show_inventory: true,
        }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "egui template"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
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
            game,
            step_name,
            inventory,
            show_inventory,
        } = self;

        if *show_inventory {
            egui::SidePanel::right("inventory_panel")
                .min_width(10.0)
                .resizable(true).show(ctx, |ui| {
                if inventory.is_empty() {
                    ui.add(Label::new(" - Empty - ").monospace());
                } else {
                    for item in inventory.iter() {
                        ui.add(Label::new(format!("• {}",&item)).wrap(true).small());
                    }
                }
             });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let checkbox = Checkbox::new(show_inventory, "Show inventory").text_style(TextStyle::Small);
            ui.add(checkbox);

            for warning in &game.warnings {
                ui.add(Label::new(&warning).text_color(egui::Color32::RED).wrap(true).small());
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
