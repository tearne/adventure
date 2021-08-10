use std::{collections::HashMap};

use serde::{Serialize, Deserialize};


pub struct Game{
    pub title: String,
    pub author: String,
    pub data: HashMap<String, Step>,
    pub start_step: String,
    pub warnings: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RawData {
    pub title: String,
    pub author: String,
    pub start_step: String, 
    pub steps: Vec<Step>,
}
impl RawData {
    pub fn to_game(mut self) -> Game {
        let mut data = HashMap::new();
        let mut warnings = Vec::new();

        for step in self.steps {
            let name = step.name();
            if data.contains_key(&name) {
                warnings.push("Name collisions.  Story truncated.".to_string());
                break;
            }
            data.insert(name, step);
        };

        fn ensure_links_resolvable(name: &String, data: &mut HashMap<String, Step>, warnings: &mut Vec<String>){
            if !data.contains_key(name) {
                let todo_step = Step::T(Todo{name: name.clone()});
                data.insert(name.clone(), todo_step);
                warnings.push(format!("Missing stage {}.  Todo inserted in its place.", name));
            } else {
                match &mut data[name] {
                    Step::D(d) => {
                        for opt in d.opts.iter() {
                            ensure_links_resolvable(&opt.goto, data, warnings);
                        }
                    },
                    Step::F(f) => {
                        ensure_links_resolvable(&f.goto, data, warnings);
                    }
                    _ => ()
                }
                
            }
        }


        Game {
            title: self.title,
            author: self.author,
            data,
            start_step: self.start_step,
            warnings,
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
    #[serde(rename = "todo")]
    T(Todo),
}
impl Step{
    pub fn name(&self) -> String {
        match self {
            Step::D(d) => d.name.clone(),
            Step::F(f) => f.name.clone(),
            Step::T(t) => t.name.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Decision {
    pub name: String,
    pub text: String,
    pub opts: Vec<Opt>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Opt{
    pub text: String, 
    pub goto: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Forwarder {
    pub name: String,
    pub text: String,
    pub goto: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub name: String,
}
