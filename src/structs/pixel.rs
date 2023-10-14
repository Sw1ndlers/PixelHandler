use super::grid_position::GridPosition;

use ggez::graphics::{self, Color, MeshBuilder};

#[derive(Clone)]
pub struct Pixel {
    pub position: GridPosition,
    pub color: Color,
}

impl Pixel {
    pub fn new(position: GridPosition, color: Color) -> Self {
        Self { position, color }
    }

    pub fn append_to_mesh(&mut self, mesh_builder: &mut MeshBuilder) {
        mesh_builder
            .rectangle(
                graphics::DrawMode::fill(),
                self.position.as_rect(),
                self.color,
            )
            .unwrap();
    }
}
