#![feature(stmt_expr_attributes)]

pub mod animations;
pub mod expositors;
pub mod game;
pub mod renderers;
pub mod utils;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn init() -> Result<(), ()> {
    utils::graphics::init()
}

pub fn get_version() -> Vec<&'static str> {
    VERSION.split('.').collect()
}
