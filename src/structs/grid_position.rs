use std::{
    hash::{Hash, Hasher},
    ops::{Add, AddAssign, Mul},
};

use ggez::{glam::Vec2, graphics};

#[derive(Debug, Copy, Clone)]
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

    fn round_down(&self, value: f32, multiple: f32) -> f32 {
        (value / multiple).floor() * multiple
    }

    fn round_up(&self, value: f32, multiple: f32) -> f32 {
        (value / multiple).ceil() * multiple
    }

    pub fn from_vec2(vec: Vec2, cell_size: (f32, f32)) -> Self {
        let x = (vec.x / cell_size.0).floor() as i32;
        let y = (vec.y / cell_size.1).floor() as i32;

        Self::new(x, y, cell_size)
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

    pub fn is_offscreen(&self, ctx: &mut ggez::Context) -> bool {
        let window_size = ctx.gfx.drawable_size();

        let top = 0.0;
        let left = 0.0;
        let bottom = window_size.1 - self.cell_size.1;
        let right = window_size.0 - self.cell_size.0;

        let position = self.as_vec2();

        return position.x < left 
            || position.x > right 
            || position.y < top 
            || position.y > bottom
    }
}

impl AddAssign for GridPosition {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Add for GridPosition {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            cell_size: self.cell_size,
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul for GridPosition {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            cell_size: self.cell_size,
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}
