use std::collections::HashMap;

use anyhow::{Context, Result};
use eframe::egui::{Label, Ui};

use crate::{data::Step, inventory::Inventory, log::Logs};

#[derive(Default)]
pub struct Game {
    pub title: String,
    pub author: String,
    pub data: HashMap<String, Step>,
    pub start_step_name: String,
    pub logs: Logs,
}
impl Game {
    pub fn show_and_update(
        &mut self,
        ui: &mut Ui,
        current_step_name: &mut String,
        inventory: &mut Inventory,
    ) {
        match self.current_step(current_step_name).unwrap() {
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
                        *current_step_name = opt.goto.clone();
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
                    *current_step_name = f.goto.clone();
                }
            }
            Step::T(t) => {
                ui.add(Label::new(&format!("Not written yet: {}", t.name)).wrap(true));

                if ui.button("Restart").clicked() {
                    inventory.clear();
                    *current_step_name = self.start_step_name.clone();
                }
            }
            Step::E(e) => {
                ui.add(
                    Label::new(format!("Stage name: {}", e.name))
                        .wrap(true)
                        .small(),
                );
                ui.separator();
                ui.add(Label::new(&e.text).wrap(true));

                if ui.button("Restart").clicked() {
                    inventory.clear();
                    *current_step_name = self.start_step_name.clone();
                }
            }
        };
    }

    fn current_step(&self, step_name: &str) -> Result<&Step> {
        self.data
            .get(step_name) //.unwrap_or_else(|| panic!());
            .with_context(|| format!("Couldn't find step {}", step_name))
    }
}
