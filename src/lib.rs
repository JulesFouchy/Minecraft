mod app;
mod webgpu;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    webgpu::run::<app::App>().await;
}
