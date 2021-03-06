#![feature(zero_one)]

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

mod conway;
mod utils;

use piston::window::WindowSettings;
use piston::event_loop::Events;
use piston::input::{RenderEvent, UpdateEvent, RenderArgs, UpdateArgs};
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use graphics::Transformed;

pub struct App {
    gl: GlGraphics,
    game: conway::GameOfLife,
    clock_remainder: f64,
    tick_period: f64,
}

impl App {
    fn new(opengl: OpenGL) -> App {
        let mut game = conway::GameOfLife::new(100, 100);
        game.randomize();

        App {
            gl: GlGraphics::new(opengl),
            game: game,
            clock_remainder: 0.0,
            tick_period: 0.3,
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        const ALIVE_RED: [f32; 4] = [0.9, 0.2, 0.2, 1.0];
        const ALIVE_BLUE: [f32; 4] = [0.2, 0.2, 0.9, 1.0];
        const ALIVE_NEUTRAL: [f32; 4] = [0.9, 0.9, 0.9, 1.0];
        const DEAD: [f32; 4] = [0.1, 0.1, 0.1, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let size = 6.0;
        let square = graphics::rectangle::square(0.0, 0.0, size);
        let cells = self.game.cells.iter();

        self.gl.draw(args.viewport(), |ctx, gl| {
            // Clear the screen.
            graphics::clear(BLACK, gl);

            for (x, y, cell) in cells {
                let transform = ctx.transform.trans(x as f64 * size + 20.0,
                                                    y as f64 * size + 20.0);
                let color = match (cell.alive, cell.team) {
                    (true, conway::Team::Red) => ALIVE_RED,
                    (true, conway::Team::Blue) => ALIVE_BLUE,
                    (true, conway::Team::Neutral) => ALIVE_NEUTRAL,
                    (false, _) => DEAD,
                };
                graphics::rectangle(color, square, transform, gl);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.clock_remainder += args.dt;
        while self.clock_remainder >= self.tick_period {
            self.clock_remainder -= self.tick_period;
            self.game.tick();
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "Conway's Game of War",
            [200, 200]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App::new(opengl);

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
