// This represents the generating distribution for a single cloud of points

use rand::distributions::{Distribution, Normal};
use rand::Rng;
use na::Vector2;

use crate::point2::Point2;

pub struct Cloud {
    pub mean: Vector2,
    pub std: Vector2,
    pub y: i8,
}

impl Cloud {
    pub fn new(mean: Vector2, std: Vector2, n: usize, y: i8) -> Cloud {
        Cloud { mean, std, y }
    }

    // Generate a cloud of n points
    pub fn generate(&self, n: usize) -> Vec<Point2> {
        let mut rng = rand::thread_rng();
        let mut points: Vec<Point2> = Vec::new();
        let x_dist = Normal::new(self.mean[0], self.std[0]);
        let y_dist = Normal::new(self.mean[1], self.std[1]);
        for _ in 0..n {
            let x = Vector2::new(x_dist.sample(&mut rng), y_dist.sample(&mut rng));
            let p = Point2::new(x, self.y);
            points.push(p);
        }
        points
    }
}