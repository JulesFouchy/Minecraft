pub mod mesh;
pub use crate::webgpu::mesh::*;
pub mod texture;
pub use crate::webgpu::texture::*;
pub mod renderer;
pub use crate::webgpu::renderer::*;

use std::iter;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use winit::{event::*, event_loop::EventLoop, window::Window, window::WindowBuilder};

pub trait App {
    fn new(webgpu_context: &Context) -> Self;
    fn update(&mut self);
    fn render(&mut self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView);
}

pub struct Context<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    // The window must be declared after the surface so
    // it gets dropped after it as the surface contains
    // unsafe references to the window's resources.
    window: &'a Window,
}

impl<'a> Context<'a> {
    pub async fn new(window: &'a Window) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            ..Default::default()
        });

        let surface = instance.create_surface(window).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    required_limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    memory_hints: Default::default(),
                },
                // Some(&std::path::Path::new("trace")), // Trace path
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an Srgb surface texture. Using a different
        // one will result all the colors coming out darker. If you want to support non
        // Srgb surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            desired_maximum_frame_latency: 2,
            view_formats: vec![],
        };

        Self {
            surface,
            device,
            queue,
            config,
            size,
            window,
        }
    }

    pub fn render<T>(&mut self, do_rendering: T)
    where
        T: FnOnce(&mut wgpu::CommandEncoder, &wgpu::TextureView),
    {
        match self.surface.get_current_texture() {
            Ok(output) => {
                let view = output
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                let mut encoder =
                    self.device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                            label: Some("Render Encoder"),
                        });

                do_rendering(&mut encoder, &view);

                self.queue.submit(iter::once(encoder.finish()));
                output.present();
            }

            // Reconfigure the surface if it's lost or outdated
            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => self.resize(self.size),

            // The system is out of memory, we should probably quit
            Err(wgpu::SurfaceError::OutOfMemory) => {
                log::error!("OutOfMemory");
                // control_flow.exit();
            }

            // This happens when the a frame takes too long to present
            Err(wgpu::SurfaceError::Timeout) => {
                log::warn!("Surface timeout")
            }
        }
    }

    pub fn window(&self) -> &Window {
        self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    #[allow(unused_variables)]
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }
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

    let mut webgpu_context = Context::new(&window).await;
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
