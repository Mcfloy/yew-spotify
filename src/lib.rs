#![recursion_limit="256"]

mod utils;

pub use utils::get_access_token;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::utils::log;

mod app;
mod types;
mod route;
mod components;
mod pages;
mod api;


#[wasm_bindgen(start)]
pub fn run_app() {
    App::<app::App>::new().mount_to_body();
}

#[wasm_bindgen]
pub fn hello() {
    log(format!("Hello !"));
}