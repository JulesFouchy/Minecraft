mod app;
mod camera;
mod camera_controller;
mod voxel;
mod webgpu;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    webgpu::run::<app::App>().await;
}
