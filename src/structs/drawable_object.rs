// Wrapper for drawable objects

use ggez::graphics::{self, Canvas, Drawable, Image, Mesh, Text};

pub enum DrawableObject {
    Text(Text),
    Image(Image),
    Mesh(Mesh),
}

impl Drawable for DrawableObject {
    fn draw(&self, canvas: &mut Canvas, param: impl Into<graphics::DrawParam>) {
        match self {
            DrawableObject::Text(text) => {
                canvas.draw(text, param);
            }
            DrawableObject::Image(image) => {
                canvas.draw(image, param);
            }
            DrawableObject::Mesh(mesh) => {
                canvas.draw(mesh, param);
            }
        }
    }

    fn dimensions(
        &self,
        gfx: &impl ggez::context::Has<graphics::GraphicsContext>,
    ) -> Option<graphics::Rect> {
        match self {
            DrawableObject::Text(text) => text.dimensions(gfx),
            DrawableObject::Image(image) => image.dimensions(gfx),
            DrawableObject::Mesh(mesh) => mesh.dimensions(gfx),
        }
    }
}
