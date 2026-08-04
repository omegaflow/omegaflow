#![allow(mixed_script_confusables)]
pub mod core;

#[cfg(target_arch = "wasm32")]
pub mod wasm;
