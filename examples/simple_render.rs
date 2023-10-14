use ggez::{
    conf::WindowMode,
    event::{self, EventHandler, MouseButton},
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
        let pixel_handler = PixelHandler::new(CELL_SIZE);
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

        self.pixel_handler.update(&mut canvas, ctx);
        canvas.finish(ctx)
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        let position = GridPosition::from_vec2(Vec2::new(x, y), CELL_SIZE);

        println!("Adding pixel at {:?}", position);

        let pixel = Pixel::new(position, Color::new(0.0, 0.0, 0.0, 1.0));
        self.pixel_handler.pixels.insert(position, pixel);

        Ok(())
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
            min_height: 280.0,
            min_width: 350.0,

            ..Default::default()
        })
        .unwrap();

    event::run(ctx, event_loop, state);
}