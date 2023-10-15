use ggez::{
    conf::WindowMode,
    event::{self, EventHandler},
    glam::*,
    graphics::{Canvas, Color},
    Context, ContextBuilder, GameResult,
};

use pixel_handler::{
    pixel_handler::PixelHandler,
    structs::{grid_position::GridPosition, pixel::Pixel},
};

const CELL_SIZE: (f32, f32) = (15.0, 15.0);

struct MainState {
    pixel_handler: PixelHandler,
}

impl MainState {
    pub fn new(_ctx: &mut Context) -> MainState {
        let mut pixel_handler = PixelHandler::new(CELL_SIZE);

        let pixel = Pixel::new(GridPosition::new(10, 10, CELL_SIZE), Color::BLUE);
        pixel_handler.register_pixel(pixel);

        MainState { pixel_handler }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::WHITE);
        let pixel_handler = &mut self.pixel_handler;

        pixel_handler.draw_grid(ctx, Color::BLACK);
        pixel_handler.display_fps(ctx);

        for (_position, pixel) in pixel_handler.pixels.iter_mut() {
            let next_position = pixel.position + GridPosition::new(0, 1, CELL_SIZE);

            if next_position.is_offscreen(ctx) {
                continue;
            } else {
                pixel.position = next_position;
            }
        }

        pixel_handler.update(&mut canvas, ctx);
        canvas.finish(ctx)
    }
}

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("Simple Render", "")
        .window_setup(ggez::conf::WindowSetup::default().title("Pixel Handler"))
        .build()
        .expect("Could not create ggez context");

    let state = MainState::new(&mut ctx);

    ctx.gfx
        .set_mode(WindowMode {
            resizable: true,
            min_height: 350.0,
            min_width: 350.0,

            ..Default::default()
        })
        .unwrap();

    event::run(ctx, event_loop, state);
}