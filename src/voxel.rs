pub enum Kind {
    Terrain,
    Trunk,
    Leaves,
}

pub struct Voxel {
    pub position: cgmath::Vector3<i32>,
    pub kind: Kind,
}

pub struct VoxelGrid {
    pub voxels: Vec<Voxel>,
}
