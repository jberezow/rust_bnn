// Bayesian Logistic Regression: basic case for classification

use rand_distrs::{Normal, Distribution};

pub struct Model {
    pub w: Vector2<f64>,
    pub b: f64,
    pub prior: Normal<f64>,
}

// Softmax for a vector with two classes
fn softmax(x: Vector2<f64>) -> Vector2<f64> {
    let exp_x = x.map(|x| x.exp());
    let sum_exp_x = exp_x.sum();
    exp_x / sum_exp_x
}

impl Model {
    pub fn new() -> Model {
        let w = Vector2::new(0.0, 0.0);
        let b = 0.0;
        Model { w, b }
    }

    pub fn draw_from_prior(&self) -> Model {
        let w = self.w.map(|w| self.prior.sample(&mut rand::thread_rng()));
        let b = self.prior.sample(&mut rand::thread_rng());
        Model { w, b }
    }

    pub fn forward(&self, x: Vector2<f64>) -> Vector2<f64> {
        let z = self.w.dot(&x) + self.b;
        softmax(z)
    }
}