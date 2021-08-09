use std::{collections::HashMap};

use eframe::{egui::{self, FontDefinitions, FontFamily, Label, TextStyle}, epi};

use crate::data::Step;


pub struct TemplateApp {
    //https://stackoverflow.com/questions/32300132/why-cant-i-store-a-value-and-a-reference-to-that-value-in-the-same-struct
    data: HashMap<String, Step>,
    step_name: String,
}

impl TemplateApp {
    pub fn new(data: HashMap<String, Step>) -> Self {
        Self {
            data,
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
            (FontFamily::Proportional, 28.0)
        );
        fonts.family_and_size.insert(
            TextStyle::Body,
            (FontFamily::Proportional, 28.0)
        );
        
        ctx.set_fonts(fonts);


        let Self { data, step_name } = self;

        let step = &data[step_name];

        match step {
            Step::D(d) => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.add(Label::new(&d.text).wrap(true));
                    for opt in d.opts.iter() {
                        if ui.button(&opt.text).clicked() {
                            *step_name = opt.goto.clone();
                        }
                    }
                });
            },
            Step::F(f) => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.add(Label::new(&f.text).wrap(true));
                
                    if ui.button("Continue").clicked() {
                        *step_name = f.goto.clone();
                    }
                });
            },
        };
    }
}
