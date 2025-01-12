use super::WebgpuContext;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use winit::{
    event::*,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

pub trait App {
    fn new(webgpu_context: &WebgpuContext) -> Self;
    fn update(&mut self);
    fn render(&mut self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView);
}

pub async fn run<AppT>()
where
    AppT: App,
{
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Info).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("Minecraft version du turfu")
        .with_maximized(true)
        // .with_fullscreen(Some(Fullscreen::Borderless(None)))
        .build(&event_loop)
        .unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        // Winit prevents sizing with CSS, so we have to set
        // the size manually when on web.
        use winit::dpi::PhysicalSize;

        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| {
                let width = win.inner_width().unwrap().as_f64().unwrap() as u32;
                let height = win.inner_height().unwrap().as_f64().unwrap() as u32;
                let factor = window.scale_factor();
                let logical = winit::dpi::LogicalSize { width, height };
                let PhysicalSize { width, height }: PhysicalSize<u32> = logical.to_physical(factor);
                window.request_inner_size(PhysicalSize::new(width, height));
                win.document()
            })
            .and_then(|doc| {
                let dst = doc.get_element_by_id("minecraft")?;
                let canvas = web_sys::Element::from(window.canvas()?);
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }

    let mut webgpu_context = WebgpuContext::new(&window).await;
    let mut app = AppT::new(&webgpu_context);
    let mut surface_configured = false;

    event_loop
        .run(move |event, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == webgpu_context.window().id() => {
                    if !webgpu_context.input(event) {
                        match event {
                            WindowEvent::CloseRequested => control_flow.exit(),
                            WindowEvent::Resized(physical_size) => {
                                surface_configured = true;
                                webgpu_context.resize(*physical_size);
                            }
                            WindowEvent::RedrawRequested => {
                                // This tells winit that we want another frame after this one
                                webgpu_context.window().request_redraw();

                                if !surface_configured {
                                    return;
                                }

                                app.update();
                                webgpu_context.render(|context, view| {
                                    app.render(context, view);
                                });
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        })
        .unwrap();
}
