// Import statrs
use statrs::statistics::Distribution;
use statrs::distribution::Continuous;
use rand::thread_rng;

// Prior struct that accepts arbitrary distribution
pub struct Prior<T: Distribution<f64> + Continuous<f64, f64>> {
    pub dist: T,
    pub rng: rand::rngs::ThreadRng,
}

impl <T: Distribution<f64> + Continuous<f64, f64>> Prior<T> {
    pub fn new(dist: T) -> Self {
        Self { dist, rng: thread_rng() }
    }

    pub fn sample(&mut self) -> f64 {
        self.dist.sample(&mut self.rng)
    }

    pub fn log_prob(&self, x: f64) -> f64 {
        self.dist.ln_pdf(x)
    }
}

