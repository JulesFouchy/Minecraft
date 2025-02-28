// use super::Context;
// use wgpu::util::DeviceExt;

// pub struct Mesh {
//     pub vertex_buffer: wgpu::Buffer,
//     pub index_buffer: wgpu::Buffer,
//     pub num_indices: u32,
// }

// impl Mesh {
//     pub fn new(ctx: &Context, vertices: &[Vertex], indices: &[u16]) -> Self {
//         let vertex_buffer = ctx
//             .device
//             .create_buffer_init(&wgpu::util::BufferInitDescriptor {
//                 label: Some("Vertex Buffer"),
//                 contents: bytemuck::cast_slice(vertices),
//                 usage: wgpu::BufferUsages::VERTEX,
//             });
//         let index_buffer = ctx
//             .device
//             .create_buffer_init(&wgpu::util::BufferInitDescriptor {
//                 label: Some("Index Buffer"),
//                 contents: bytemuck::cast_slice(indices),
//                 usage: wgpu::BufferUsages::INDEX,
//             });

//         Mesh {
//             vertex_buffer,
//             index_buffer,
//             num_indices: indices.len() as u32,
//         }
//     }
// }
