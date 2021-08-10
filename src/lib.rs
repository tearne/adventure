mod data;
mod app;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::Request;
use web_sys::RequestInit;
use web_sys::RequestMode;
use web_sys::Response;
use crate::data::*;

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

    let data = load().await;

    let app = app::TemplateApp::new(data);
    eframe::start_web(&canvas_id, Box::new(app)).unwrap();

    // log!("{:?}", data["start"]);
}

async fn load() -> Game {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::SameOrigin);

    let url = "data/chapter_1.json";

    let request = Request::new_with_str_and_init(&url, &opts).unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await.expect("b");

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().expect("d");

    let json = JsFuture::from(resp.json().unwrap()).await.expect("a");

    json.into_serde::<RawData>()
        .unwrap()
        .to_game()
}