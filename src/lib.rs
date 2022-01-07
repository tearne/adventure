mod app;
mod data;
pub mod editor;
pub mod inventory;

use crate::data::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::Request;
use web_sys::RequestInit;
use web_sys::RequestMode;
use web_sys::Response;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn start(canvas_id: String) {
    log!("Hello, world!");

    let game_data = load_source().await;
    // log!("You have {} warnings", game_data.warnings.len());

    let app = app::App::new(game_data);
    eframe::start_web(&canvas_id, Box::new(app)).unwrap();
}

async fn load_source() -> String {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::SameOrigin);

    let url = "data/example.json";

    let request = Request::new_with_str_and_init(&url, &opts).unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .unwrap();

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    let json = JsFuture::from(resp.json().unwrap()).await.unwrap();

    log!("you got this: {:?}", json);

    let parsed = json.into_serde::<RawData>().unwrap();
    serde_json::to_string_pretty(&parsed).unwrap()
}
