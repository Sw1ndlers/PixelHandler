use std::{
    collections::HashMap,
    time::{self, Instant},
};

use ggez::{
    glam::*,
    graphics::{self, Canvas, Color, DrawParam, Drawable, Mesh, Rect, Text, PxScale},
    Context,
};

use crate::structs::{drawable_object::DrawableObject, grid_position::GridPosition, pixel::Pixel};

pub struct PixelHandler {
    pub pixels: HashMap<GridPosition, Pixel>,
    frame_time: time::Instant,
    draw_stack: Vec<(DrawParam, DrawableObject)>,
    cell_size: (f32, f32),
}

impl Default for PixelHandler {
    fn default() -> Self {
        Self::new((20.0, 20.0))
    }
}

impl PixelHandler {
    pub fn new(cell_size: (f32, f32)) -> Self {
        Self {
            draw_stack: Vec::new(),
            pixels: HashMap::new(),
            frame_time: Instant::now(),
            cell_size,
        }
    }

    fn add_to_draw_stack(&mut self, drawable: DrawableObject, params: DrawParam) {
        self.draw_stack.push((params, drawable));
    }

    pub fn draw_grid(&mut self, ctx: &mut Context, grid_color: Color) {
        let window_size = ctx.gfx.size();
        let cell_size = self.cell_size;

        let mut mesh_builder = graphics::MeshBuilder::new();

        let horizontal_lines = (window_size.1 / cell_size.1) as i32; 
        let vertical_lines = (window_size.0 / cell_size.0) as i32; 

        // creating lines from left to right 
        for y in 0..horizontal_lines {
            let y_column = y as f32 * cell_size.1;

            let left_point = Vec2::new(0.0, y_column);
            let right_point = Vec2::new(window_size.0, y_column);

            mesh_builder
                .line(&[left_point, right_point], 1.0, grid_color)
                .unwrap();
        }

        // creating lines from top to bottom
        for x in 0..vertical_lines {
            let x_row = x as f32 * cell_size.0;

            let top_point = Vec2::new(x_row, 0.0);
            let bottom_point = Vec2::new(x_row, window_size.1);

            mesh_builder
                .line(&[top_point, bottom_point], 1.0, grid_color)
                .unwrap();
        }

        let mesh_data = mesh_builder.build();
        let mesh = Mesh::from_data(ctx, mesh_data);

        self.add_to_draw_stack(DrawableObject::Mesh(mesh), DrawParam::default());
    }

    pub fn get_fps(&self) -> f32 {
        let frame_duration = self.frame_time.elapsed();
        let fps = 1.0 / frame_duration.as_secs_f32();

        fps
    }

    pub fn display_fps(&mut self, ctx: &mut Context) {
        let fps = ctx.time.fps().round();
        let bounds = Vec2::new(80.0, 20.0);
        let rect = Rect::new(0.0, 0.0, bounds.x, bounds.y);

        let mut fps_text = Text::new(format!("Fps: {}", fps));

        fps_text.set_bounds(bounds);
        fps_text.set_scale(PxScale::from(18.0));

        let fps_background =
            Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, Color::BLACK).unwrap();

        self.add_to_draw_stack(DrawableObject::Mesh(fps_background), DrawParam::default());
        self.add_to_draw_stack(DrawableObject::Text(fps_text), DrawParam::default());
    }

    pub fn register_pixel(&mut self, pixel: Pixel) {
        self.pixels.insert(pixel.position, pixel);
    }

    pub fn position_occupied(&self, position: &GridPosition) -> bool {
        self.pixels.contains_key(position)
    }

    pub fn get_pixel(&self, position: &GridPosition) -> Option<&Pixel> {
        self.pixels.get(&position)
    }

    pub fn update(&mut self, canvas: &mut Canvas, ctx: &mut Context) {
        let mut mesh_builder = graphics::MeshBuilder::new();
        let mut new_pixels = HashMap::new();

        for (_old_pos, pixel) in self.pixels.iter_mut() {
            pixel.append_to_mesh(&mut mesh_builder);
            new_pixels.insert(pixel.position, pixel.clone());
        }

        let mesh = Mesh::from_data(ctx, mesh_builder.build());
        canvas.draw(&mesh, DrawParam::default());

        for (params, drawable) in &self.draw_stack {
            drawable.draw(canvas, *params);
        }

        self.pixels = new_pixels;

        self.draw_stack.clear();
        self.frame_time = Instant::now();
    }
}
