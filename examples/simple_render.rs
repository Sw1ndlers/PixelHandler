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

const CELL_SIZE: (f32, f32) = (3.0, 3.0);

struct MainState {
    pixel_handler: PixelHandler,
}

impl MainState {
    pub fn new(_ctx: &mut Context) -> MainState {
        let mut pixel_handler = PixelHandler::new(CELL_SIZE);

        let window_size = _ctx.gfx.size();
        let total_pixels = (window_size.0 / CELL_SIZE.0 * window_size.1 / CELL_SIZE.0) as i32;

        println!("Total pixels: {}", total_pixels);

        for i in 0..total_pixels {
            let pixel = Pixel::new(GridPosition::new(i, i, CELL_SIZE), Color::BLUE);
            pixel_handler.register_pixel(pixel);
        }

        MainState { pixel_handler }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::WHITE);

        self.pixel_handler.draw_grid(ctx, Color::BLACK);
        self.pixel_handler.display_fps(ctx);

        for (_position, pixel) in self.pixel_handler.pixels.iter_mut() {
            pixel.position += GridPosition::new(0, -1, CELL_SIZE);
        }

        self.pixel_handler.update(&mut canvas, ctx);
        canvas.finish(ctx)
    }
}

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("pixel manager", "sw1ndler")
        .window_setup(ggez::conf::WindowSetup::default().title("real"))
        .build()
        .expect("Could not create ggez context");

    let state = MainState::new(&mut ctx);

    ctx.gfx
        .set_mode(WindowMode {
            resizable: true,
            min_height: 280.0,
            min_width: 350.0,

            ..Default::default()
        })
        .unwrap();

    event::run(ctx, event_loop, state);
}