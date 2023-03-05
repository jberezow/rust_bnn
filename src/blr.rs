// Bayesian Logistic Regression: basic case for classification
use na::Vector2;
use statrs::distribution::Normal;
use crate::prior::Prior;

pub struct Model {
    pub w: Vector2<f64>,
    pub b: f64,
    pub prior: Prior<Normal>,
}


// Softmax function one output
fn softmax(z: f64) -> f64 {
    
    1.0 / (1.0 + (-z).exp())
}

impl Model {
    #[must_use] pub fn new(w: Vector2<f64>, b: f64, prior: Prior<Normal>) -> Self {
        Self { w, b, prior }
    }

    // Initialize weight and bias parameters from the prior
    pub fn draw_from_prior(&mut self) {
        self.w = self.w.map(|_| self.prior.sample());
        self.b = self.prior.sample();
    }

    #[must_use] pub fn forward(&self, x: Vector2<f64>) -> i8 {
        let z = self.w.dot(&x) + self.b;
        let result: i8 = softmax(z).round() as i8;
        result
    }

    pub fn likelihood(&self, x: Vector2<f64>, y: i8) -> f64 {
        let z = self.w.dot(&x) + self.b;
        let result: f64 = softmax(z);
        if y == 1 {
            result
        } else {
            1.0 - result
        }
    }

    pub fn log_likelihood(&self, x: Vector2<f64>, y: i8) -> f64 {
        let z = self.w.dot(&x) + self.b;
        let result: f64 = softmax(z);
        if y == 1 {
            result.ln()
        } else {
            (1.0 - result).ln()
        }
    }

    // Compute the log posterior
    pub fn log_posterior(&self, x: Vector2<f64>, y: i8) -> f64 {
        let log_prior = self.prior.log_prob(self.w[0]) + self.prior.log_prob(self.w[1]) + self.prior.log_prob(self.b);
        let log_likelihood = self.log_likelihood(x, y);
        log_prior + log_likelihood
    }
}

// Implement Default trait for Model
impl Default for Model {
    fn default() -> Self {
        let w: Vector2<f64> = Vector2::new(0.0, 0.0);
        let b: f64 = 0.0;
        let normal_dist = Normal::new(0.0, 1.0).unwrap();
        let prior = Prior::new(normal_dist);
        Self::new(w,b,prior)
    }
}