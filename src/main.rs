extern crate rand;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use rand::Rng;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent};
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

    fn clear(&mut self, args: &RenderArgs) {
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(WHITE, gl);
        });
    }

    fn render_point(&mut self, args: &RenderArgs, x: f64, y: f64, size: f64, color: [f32; 4]) {
        let square = graphics::rectangle::square(x, y, size);
    
        self.gl.draw(args.viewport(), |c, gl| {
            graphics::ellipse(color, square, c.transform, gl);
        });
    }

    fn render_line(&mut self, args: &RenderArgs) {
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    
        self.gl.draw(args.viewport(), |c, gl| {
            graphics::line(BLACK, 0.5, [0.0, 0.0, 500.0, 500.0], c.transform, gl);
        });
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
}

/* Perceptron ----------------------------------------------------- */

struct Perceptron {
    weights: Vec<f64>,
    lr: f64
}

impl Perceptron {
    fn new() -> Perceptron {
        Perceptron {
            weights: (0..2).map(|_x| rand::thread_rng().gen_range(-1.0, 1.0)).collect(),
            lr: 0.1
        }
    }

    fn guess(&self, inputs: [f64; 2]) -> i32 {
        let mut sum: f64 = 0.0;

        for i in 0..self.weights.len() {
            sum += inputs[i] * self.weights[i];
        }
        return sum.signum() as i32;
    }

    fn train(&mut self, inputs: [f64; 2], target: i32) {
        let guess: i32 = self.guess(inputs);
        let error: i32 = target - guess;

        for i in 0..self.weights.len() {
            self.weights[i] += error as f64 * inputs[i] * self.lr;
        }
    }
}

/* Functions ------------------------------------------------------ */

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("Perceptron", [500, 500]).graphics_api(opengl).exit_on_esc(true).build().unwrap();
    let mut display = Display::new(opengl);

    let mut p: Perceptron = Perceptron::new();
    let rd_points: Vec<RandomPoint> = (0..100).map(|_x| RandomPoint::new(500.0)).collect();

    let mut train_count = 0;
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            display.clear(&args);
            display.render_line(&args);

            for point in rd_points.iter() {
                display.render_point(&args, point.x, point.y, 10.0, [0.0, 0.0, 0.0, 1.0]);

                let inputs: [f64; 2] = [point.x, point.y];
                let target: i32 = point.label;
                p.train(inputs, target);

                let guess: i32 = p.guess(inputs);
                let color = if guess == target { [0.0, 1.0, 0.0, 1.0] } else { [1.0, 0.0, 0.0, 1.0] };
                display.render_point(&args, point.x, point.y, 5.0, color);
            }

            eprintln!("Nb trains: {}", train_count);
            train_count += 1;
        }
    }
}
