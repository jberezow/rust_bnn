// Bayesian Logistic Regression: basic case for classification

use rand_distr::{Normal, Distribution};
use na::Vector2;

pub struct Model {
    pub w: Vector2<f64>,
    pub b: f64,
    pub prior: Normal<f64>,
}

// Softmax function one output
fn softmax(z: f64) -> f64 {
    
    1.0 / (1.0 + (-z).exp())
}

impl Model {
    #[must_use] pub fn new() -> Self {
        let w = Vector2::new(0.0, 0.0);
        let b = 0.0;
        let prior = Normal::new(0.0, 1.0).unwrap();
        Self { w, b, prior }
    }

    // Initialize weight and bias parameters from the prior
    pub fn draw_from_prior(&mut self) {
        self.w = self.w.map(|_| self.prior.sample(&mut rand::thread_rng()));
        self.b = self.prior.sample(&mut rand::thread_rng());
    }

    #[must_use] pub fn forward(&self, x: Vector2<f64>) -> i8 {
        let z = self.w.dot(&x) + self.b;
        let result: i8 = softmax(z).round() as i8;
        // println!("z: {:?}, result: {:?}", z, result);
        result
    }

    // Print model details
    pub fn print(&self) {
        println!("w: {:?}", self.w);
        println!("b: {:?}", self.b);
    }
}

// Implement Default trait for Model
impl Default for Model {
    fn default() -> Self {
        Self::new()
    }
}