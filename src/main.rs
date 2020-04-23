extern crate rand;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use rand::Rng;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

/* Display -------------------------------------------------------- */

pub struct Display {
    gl: GlGraphics,
}

impl Display {
    fn new(opengl: OpenGL) -> Display {
        return Display {
            gl: GlGraphics::new(opengl)
        }
    }

    fn render(&mut self, args: &RenderArgs, points: [RandomPoint; 100]) {
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            graphics::clear(WHITE, gl);
        });

        for point in points.iter() {
            point.render(&mut self.gl, args);
        }
    }
}

/* Training ------------------------------------------------------- */

#[derive(Clone, Copy)]
struct RandomPoint {
    x: f64,
    y: f64,
    label: i32,
}

impl RandomPoint {
    fn new(range: f64) -> RandomPoint {
        let x: f64 = rand::thread_rng().gen_range(0.0, range);
        let y: f64 = rand::thread_rng().gen_range(0.0, range);
        let label = if x > y { 1 } else { - 1};
        return RandomPoint { x, y, label };
    }

    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        let square = graphics::rectangle::square(self.x, self.y, 5.0);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(BLACK, square, transform, gl);
        });
    }
}

/* Perceptron ----------------------------------------------------- */

struct Perceptron {
    weights: [f64; 2],
}

impl Perceptron {
    fn new() -> Perceptron {
        let mut p: Perceptron = Perceptron { weights: [0.0; 2] };
        for i in 0..p.weights.len() {
            p.weights[i] = rand::thread_rng().gen_range(-1.0, 1.0);
        }
        return p;
    }

    fn guess(&self, inputs: [f64; 2]) -> i32 {
        let mut sum: f64 = 0.0 ;

        for i in 0..self.weights.len() {
            sum += inputs[i] * self.weights[i];
        }
        return sum.signum() as i32;
    }
}

/* Functions ------------------------------------------------------ */

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("Perceptron", [200, 200]).graphics_api(opengl).exit_on_esc(true).build().unwrap();
    let mut display = Display::new(opengl);

    let p: Perceptron = Perceptron::new();
    let mut rdPoints: [RandomPoint; 100] = [RandomPoint {x: 0.0, y: 0.0, label: 0}; 100];

    for i in 0..rdPoints.len() {
        rdPoints[i] = RandomPoint::new(200.0);
        eprintln!("{} {}", rdPoints[i].x, rdPoints[i].y);
    }

    let inputs: [f64; 2] = [-1.0, 0.5];
    let guess: i32 = p.guess(inputs);
    println!("{}", guess);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            display.render(&args, rdPoints);
        }
    }
}
