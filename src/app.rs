use crate::camera::Camera;
use crate::camera_controller::CameraController;
use crate::voxel;
use crate::voxel::{Voxel, VoxelGrid};
use crate::webgpu;
use winit::event::WindowEvent;

pub struct App {
    renderer: webgpu::Renderer,
    camera: Camera,
    camera_controller: CameraController,
    world: VoxelGrid,
}

impl webgpu::App for App {
    fn new(ctx: &webgpu::Context) -> Self {
        let camera = Camera {
            // position the camera 1 unit up and 2 units back
            // +z is out of the screen
            eye: (0.0, 1.0, 2.0).into(),
            // have it look at the origin
            target: (0.0, 0.0, 0.0).into(),
            // which way is "up"
            up: cgmath::Vector3::unit_y(),
            aspect: ctx.width_f32() / ctx.height_f32(),
            fov_y: 45.0,
            z_near: 0.1,
            z_far: 100.0,
        };
        let camera_controller = CameraController::new(0.2);

        let renderer = webgpu::Renderer::new(ctx, &camera);

        let voxels: Vec<Voxel> = vec![
            Voxel {
                position: (0, 0, 0).into(),
                kind: voxel::Kind::Terrain,
            },
            Voxel {
                position: (0, 0, 1).into(),
                kind: voxel::Kind::Terrain,
            },
            Voxel {
                position: (0, 0, 2).into(),
                kind: voxel::Kind::Terrain,
            },
        ];

        App {
            renderer,
            camera,
            camera_controller,
            world: VoxelGrid { voxels },
        }
    }

    fn update(&mut self, ctx: &webgpu::Context) {
        self.camera_controller.update_camera(&mut self.camera);
        self.renderer.set_camera(ctx, &self.camera);
    }

    fn render(&mut self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        self.renderer.render(encoder, view, &self.world);
    }

    fn input(&mut self, event: &WindowEvent) {
        self.camera_controller.process_event(event);
    }
}
