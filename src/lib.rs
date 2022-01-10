mod app;
mod data;
pub mod editor;
pub mod game;
pub mod inventory;
pub mod log;

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
    use crate::app::App;

    let game_data = load_source().await;
    let app = match game_data {
        Ok(d) => App::new(d),
        Err(_) => {
            log!("Failed to load game data");
            App::new(Default::default())
        }
    };

    eframe::start_web(&canvas_id, Box::new(app)).unwrap();
}

async fn load_source() -> anyhow::Result<String> {
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

    // log!("Loaded this: {:?}", json);

    let parsed = json.into_serde::<RawData>()?;
    let string = serde_json::to_string_pretty(&parsed)?;
    Ok(string)
}
