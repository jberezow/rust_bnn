// This represents the generating distribution for a single cloud of points
use rand_distr::{Normal, Distribution};
use na::core::Vector2;

use crate::point2::Point2;

pub trait Dataset {
    fn generate(&self, n: usize) -> Vec<Point2>;
}

pub struct Cloud {
    pub mean: Vector2<f64>,
    pub std: Vector2<f64>,
    pub y: i8,
    pub points: Option<Vec<Point2>>,
}

impl Cloud {
    pub fn new(mean: Vector2<f64>, std: Vector2<f64>, n: usize, y: i8) -> Self {
        let points = Some(Self::generate(&Self { mean, std, y, points: None }, n));
        Self { mean, std, y, points }
    }

    // Generate a cloud of n points
    pub fn generate(&self, n: usize) -> Vec<Point2> {
        // Create a normal distribution for each dimension
        let x0_dist = Normal::new(self.mean[0], self.std[0]).unwrap();
        let x1_dist = Normal::new(self.mean[1], self.std[1]).unwrap();

        // Generate n points
        let mut points = Vec::new();
        for _ in 0..n {
            let x0 = x0_dist.sample(&mut rand::thread_rng());
            let x1 = x1_dist.sample(&mut rand::thread_rng());
            let p = Point2::new(Vector2::new(x0, x1), self.y);
            points.push(p);
        }

        points
    }
}

// Implement Dataset trait for Cloud
impl Dataset for Cloud {
    fn generate(&self, n: usize) -> Vec<Point2> {
        self.generate(n)
    }
}