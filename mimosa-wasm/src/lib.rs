#![recursion_limit = "1024"]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::eval_order_dependence)]

pub mod app;
pub mod components;
pub mod errors;
pub mod models;
pub mod routes;
pub mod services;

use wasm_bindgen::prelude::*;
use web_logger;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    web_logger::init();
    // yew::start_app::<App>();
    Ok(())
}
