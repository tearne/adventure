use std::{borrow::Cow, collections::HashMap};

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub name: String,
    pub steps: Vec<Step>,
}
impl Game {
    pub fn to_map(self) -> HashMap<Cow<'static, str>, Step> {
        self.steps.into_iter().map(|s|{
            (s.name().into(), s)
        }).collect()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Step {
    #[serde(rename = "decision")]
    D(Decision),
    #[serde(rename = "forwarder")]
    F(Forwarder),
}
impl Step{
    pub fn name(&self) -> String {
        match self {
            Step::D(d) => d.name.clone(),
            Step::F(f) => f.name.clone(),
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