use anyhow::Result;
use block_mesh::{visible_block_faces, Voxel, VoxelVisibility};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Block {
    Empty,
    Lava,
}

fn main() -> Result<()> {
    let content = std::fs::read_to_string("day18/example.txt")?;

    Ok(())
}

impl Voxel for Block {
    fn get_visibility(&self) -> block_mesh::VoxelVisibility {
        match self {
            Self::Empty => VoxelVisibility::Empty,
            _ => VoxelVisibility::Opaque,
        }
    }
}
