use std::collections::HashMap;

use anyhow::{Context, Result};
use eframe::egui::{Label, Ui};
use serde::{Deserialize, Serialize};

use crate::{game::Game, inventory::Inventory, log::Logs};

#[derive(Serialize, Deserialize, Debug)]
pub struct RawData {
    pub title: String,
    pub author: String,
    pub start_step: String,
    pub steps: Vec<Step>,
}
impl RawData {
    pub fn to_game(self) -> Game {
        let mut data = HashMap::new();
        let mut warnings = Vec::new();

        for step in self.steps {
            let name = step.name();
            if data.contains_key(&name) {
                warnings.push("Name collisions.  Story truncated.".to_string());
                break;
            }
            data.insert(name, step);
        }

        //TODO yukky!
        let targetted_keys_and_steps_they_appear_in: Vec<(String, String)> = data
            .values()
            .flat_map(|step| match step {
                Step::D(d) => {
                    let vec = d
                        .opts
                        .iter()
                        .map(|o| (o.goto.clone(), step.name()))
                        .collect::<Vec<_>>();
                    vec
                }
                Step::F(f) => vec![(f.goto.clone(), step.name())],
                _ => Vec::new(),
            })
            .collect::<Vec<_>>();

        for (key, step_name) in targetted_keys_and_steps_they_appear_in {
            if key.is_empty() {
                warnings.push(format!("Step \"{}\" targets an empty key.", step_name));
                // let todo_step = Step::T(Todo { name: key.clone() });
                // data.insert(key, todo_step);
            } else if !data.contains_key(&key) {
                warnings.push(format!(
                    "Missing stage \"{}\".  Todo inserted in its place.",
                    key
                ));
                let todo_step = Step::T(Todo { name: key.clone() });
                data.insert(key, todo_step);
            }
        }

        Game {
            title: self.title,
            author: self.author,
            data,
            start_step_name: self.start_step.clone(),
            // current_step_name: self.start_step.clone(),
            logs: Logs::new(warnings),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Step {
    #[serde(rename = "decision")]
    D(Decision),
    #[serde(rename = "forwarder")]
    F(Forwarder),
    #[serde(rename = "end")]
    E(End),
    #[serde(rename = "todo")]
    T(Todo),
}
impl Step {
    pub fn name(&self) -> String {
        match self {
            Step::D(d) => d.name.clone(),
            Step::F(f) => f.name.clone(),
            Step::T(t) => t.name.clone(),
            Step::E(e) => e.name.clone(),
        }
    }
    // pub fn show(&self, ui: &mut Ui, game: &mut Game, inventory: &mut Inventory) {

    // }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Decision {
    pub name: String,
    pub text: String,
    pub acquire: Option<String>,
    pub opts: Vec<Opt>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Opt {
    pub text: String,
    pub goto: String,
    pub requires: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Forwarder {
    pub name: String,
    pub text: String,
    pub goto: String,
    pub acquire: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub name: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct End {
    pub name: String,
    pub text: String,
}
