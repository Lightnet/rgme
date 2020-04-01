


use ggez;
use ggez::event;
use ggez::event::{ Axis, Button, GamepadId, KeyCode, KeyMods, MouseButton};
use ggez::graphics;
use ggez::input;
use ggez::nalgebra as na;

struct MainState {
    // Your state here...
    pos_x: f32,
    pos_y: f32,
    mouse_down: bool,
}

impl MainState {
    // Load/create resources such as images here.

    fn new() -> ggez::GameResult<MainState> {
        let s = MainState { 
            pos_x: 0.0,
            pos_y: 100.0,
            mouse_down: false,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        
        if input::keyboard::is_key_pressed(ctx, KeyCode::A) {
            println!("A press");
        }

        self.pos_x = self.pos_x % 800.0 + 1.0;



        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(self.pos_x, 380.0),
            100.0,
            2.0,
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &circle, (na::Point2::new(0.0, 0.0),))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> ggez::GameResult {
    // Make a Context.
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let (ctx, event_loop) = &mut cb.build()?;


    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}