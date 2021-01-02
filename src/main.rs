//! Basic hello world example.

use cgmath;
use ggez;

use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::{Context, GameResult};
use std::env;
use std::path;
use std::vec::Vec;

mod simulation;

// First we make a structure to contain the game's state
struct Scaffolding {
    simulation: simulation::world::Simulation,
}

impl Scaffolding {
    fn new(ctx: &mut Context) -> GameResult<Scaffolding> {
        let image = graphics::Image::new(ctx, "/wing.bmp")?;
        let simulation = simulation::world::Simulation::new(
            image.width().into(),
            image.height().into(),
            image.to_rgba8(ctx)?);
        let s = Scaffolding { simulation };
        Ok(s)
    }

    fn render(&self, ctx: &mut Context) -> GameResult<graphics::Image> {
        let mut rgba = Vec::new();
        for y in 0..self.simulation.grid.height {
           for x in 0..self.simulation.grid.width {
                if self.simulation.is_blocked(&simulation::world::Position::new(x, y)) {
                    rgba.push(0);
                    rgba.push(0);
                    rgba.push(0);
                    rgba.push(255);
                } else {
                    rgba.push(255);
                    rgba.push(255);
                    rgba.push(255);
                    rgba.push(255);
                }
            }
        }
        graphics::Image::from_rgba8(ctx, self.simulation.grid.width as u16, self.simulation.grid.height as u16, &rgba)
    }
}



// Then we implement the `ggez:event::EventHandler` trait on it, which
// requires callbacks for updating and drawing the game state each frame.
//
// The `EventHandler` trait also contains callbacks for event handling
// that you can override if you wish, but the defaults are fine.
impl EventHandler for Scaffolding {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        let image = &self.render(ctx)?;
        graphics::draw(ctx, image, graphics::DrawParam::default().dest(cgmath::Point2::new(0.0, 0.0)))?;

        graphics::present(ctx)?;

        Ok(())
    }
}

// Now our main function, which does three things:
//
// * First, create a new `ggez::ContextBuilder`
// object which contains configuration info on things such
// as screen resolution and window title.
// * Second, create a `ggez::game::Game` object which will
// do the work of creating our MainState and running our game.
// * Then, just call `game.run()` which runs the `Game` mainloop.
pub fn main() -> GameResult {
    // We add the CARGO_MANIFEST_DIR/resources to the resource paths
    // so that ggez will look in our cargo project directory for files.
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("helloworld", "ggez").add_resource_path(resource_dir);
    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut Scaffolding::new(ctx)?;
    event::run(ctx, event_loop, state)
}
