use std::{ops::AddAssign, hash::Hasher};
use std::hash::Hash;

use ggez::{glam::Vec2, graphics};

#[derive(Copy, Clone)]
pub struct GridPosition {
    pub cell_size: (f32, f32),
    pub x: i32,
    pub y: i32,
}

impl PartialEq for GridPosition {
    fn eq(&self, other: &Self) -> bool {
        self.cell_size == other.cell_size && self.x == other.x && self.y == other.y
    }
}

impl Eq for GridPosition {}

impl Hash for GridPosition {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.cell_size.0.to_bits().hash(state);
        self.cell_size.1.to_bits().hash(state);
        self.x.hash(state);
        self.y.hash(state);
    }
}


impl GridPosition {
    pub fn new(x: i32, y: i32, cell_size: (f32, f32)) -> Self {
        let x = x * cell_size.0 as i32;
        let y = y * cell_size.1 as i32;

        Self { cell_size, x, y }
    }

    pub fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }

    pub fn as_rect(&self) -> graphics::Rect {
        graphics::Rect::new(
            self.x as f32,
            self.y as f32,
            self.cell_size.0,
            self.cell_size.1,
        )
    }
}

impl AddAssign for GridPosition {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}