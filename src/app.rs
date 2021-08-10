use eframe::{egui::{self, FontDefinitions, FontFamily, Label, TextStyle}, epi};

use crate::data::{Game, Step};


pub struct TemplateApp {
    //https://stackoverflow.com/questions/32300132/why-cant-i-store-a-value-and-a-reference-to-that-value-in-the-same-struct
    game: Game,
    step_name: String,
}

impl TemplateApp {
    pub fn new(game: Game) -> Self {
        Self {
            game,
            step_name: String::from("start"),
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
        fonts.family_and_size.insert(
            TextStyle::Button,
            (FontFamily::Proportional, 24.0)
        );
        fonts.family_and_size.insert(
            TextStyle::Body,
            (FontFamily::Proportional, 24.0)
        );
        fonts.family_and_size.insert(
            TextStyle::Small,
            (FontFamily::Proportional, 16.0)
        );
        
        ctx.set_fonts(fonts);


        let Self { game, step_name } = self;

        let step = &game.data[step_name];

        match step {
            Step::D(d) => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.add(Label::new(format!("Stage name: {}",&d.name)).wrap(true).small());
                    ui.separator();
                    ui.add(Label::new(&d.text).wrap(true));
                    ui.separator();
                    for opt in d.opts.iter() {
                        if ui.button(&opt.text).clicked() {
                            *step_name = opt.goto.clone();
                        }
                    }
                });
            },
            Step::F(f) => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.add(Label::new(format!("Stage name: {}",&f.name)).wrap(true).small());
                    ui.separator();
                    ui.add(Label::new(&f.text).wrap(true));
                    ui.separator();
                    if ui.button("Continue").clicked() {
                        *step_name = f.goto.clone();
                    }
                });
            },
            Step::T(t) => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.add(Label::new(&format!("Not written yet: {}",t.name)).wrap(true));
                
                    if ui.button("Restart").clicked() {
                        *step_name = "start".into();
                    }
                });
            }
        };
    }
}
