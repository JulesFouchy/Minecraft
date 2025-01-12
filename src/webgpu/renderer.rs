use super::*;

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.0868241, 0.49240386, 0.0],
        uv: [0.4131759, 0.99240386],
    }, // A
    Vertex {
        position: [-0.49513406, 0.06958647, 0.0],
        uv: [0.0048659444, 0.56958647],
    }, // B
    Vertex {
        position: [-0.21918549, -0.44939706, 0.0],
        uv: [0.28081453, 0.05060294],
    }, // C
    Vertex {
        position: [0.35966998, -0.3473291, 0.0],
        uv: [0.85967, 0.1526709],
    }, // D
    Vertex {
        position: [0.44147372, 0.2347359, 0.0],
        uv: [0.9414737, 0.7347359],
    }, // E
];
const VERTICES2: &[Vertex] = &[
    Vertex {
        position: [-0.0868241, 1. + 0.49240386, 0.0],
        uv: [0.4131759, 0.99240386],
    }, // A
    Vertex {
        position: [-0.49513406, 1. + 0.06958647, 0.0],
        uv: [0.0048659444, 0.56958647],
    }, // B
    Vertex {
        position: [-0.21918549, 1. + -0.44939706, 0.0],
        uv: [0.28081453, 0.05060294],
    }, // C
    Vertex {
        position: [0.35966998, 1. + -0.3473291, 0.0],
        uv: [0.85967, 0.1526709],
    }, // D
    Vertex {
        position: [0.44147372, 1. + 0.2347359, 0.0],
        uv: [0.9414737, 0.7347359],
    }, // E
];

const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];

pub struct Renderer {
    render_pipeline: wgpu::RenderPipeline,
    cube_mesh: Mesh,
    cube_mesh2: Mesh,
    diffuse_texture: Texture,
}

impl Renderer {
    pub fn new(ctx: &Context) -> Self {
        let cube_model = Mesh::new(ctx, VERTICES, INDICES);
        let cube_model2 = Mesh::new(ctx, VERTICES2, INDICES);
        let diffuse_texture = Texture::new(ctx);

        let shader = ctx
            .device
            .create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        let render_pipeline_layout =
            ctx.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[&diffuse_texture.bind_group_layout],
                    push_constant_ranges: &[],
                });

        let render_pipeline = ctx
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    buffers: &[Vertex::desc()],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    // 3.
                    module: &shader,
                    entry_point: Some("fs_main"),
                    targets: &[Some(wgpu::ColorTargetState {
                        // 4.
                        format: ctx.config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw, // 2.
                    cull_mode: Some(wgpu::Face::Back),
                    // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                    polygon_mode: wgpu::PolygonMode::Fill,
                    // Requires Features::DEPTH_CLIP_CONTROL
                    unclipped_depth: false,
                    // Requires Features::CONSERVATIVE_RASTERIZATION
                    conservative: false,
                },
                depth_stencil: None, // 1.
                multisample: wgpu::MultisampleState {
                    count: 1,                         // 2.
                    mask: !0,                         // 3.
                    alpha_to_coverage_enabled: false, // 4.
                },
                multiview: None, // 5.
                cache: None,     // 6.
            });

        Renderer {
            cube_mesh: cube_model,
            cube_mesh2: cube_model2,
            diffuse_texture,
            render_pipeline,
        }
    }

    pub fn render(&self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.diffuse_texture.bind_group, &[]);

        self.cube_mesh.draw(&mut render_pass);
        self.cube_mesh2.draw(&mut render_pass);
    }
}
