#![recursion_limit = "512"]

mod api;
mod app;
mod article_list;
mod list_page;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<app::App>();
}
