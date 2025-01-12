use crate::webgpu;

pub struct App {
    renderer: webgpu::Renderer,
}

impl webgpu::App for App {
    fn new(ctx: &webgpu::Context) -> Self {
        App {
            renderer: webgpu::Renderer::new(ctx),
        }
    }

    fn update(&mut self) {}

    fn render(&mut self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        self.renderer.render(encoder, view);
    }
}
