extern crate rand;

use rand::Rng;

/* Training ------------------------------------------------------- */

#[derive(Clone, Copy)]
struct RandomPoint {
    x: f64,
    y: f64,
    label: i32,
}

impl RandomPoint {
    fn new(range: f64) -> RandomPoint {
        let x: f64 = rand::thread_rng().gen_range(-(range / 2.0), range / 2.0);
        let y: f64 = rand::thread_rng().gen_range(-(range / 2.0), range / 2.0);
        let label = if x > y { 1 } else { - 1};
        return RandomPoint { x, y, label };
    }

    /*fn show(&self) {
        if (self.label == 1) {

        }
    }*/
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
    let p: Perceptron = Perceptron::new();
    let rdPoints: [RandomPoint; 100] = [RandomPoint::new(200.0); 100];

    let inputs: [f64; 2] = [-1.0, 0.5];
    let guess: i32 = p.guess(inputs);

    println!("{}", guess);
}
