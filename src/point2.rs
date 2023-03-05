// This ties a 2d vector in the cartesian space to a label

use na::Vector2;

pub struct Point2 {
  pub x: Vector2<f64>,
  pub y: i8,
}

impl Point2 {
  #[must_use] pub fn new(x: Vector2<f64>, y: i8) -> Self {
    Self { x, y }
  }
}

// Print trait for Point2
impl std::fmt::Display for Point2 {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "Point: ({}, {}) -> {}", self.x[0], self.x[1], self.y)
  }
}

// Copy trait
impl Clone for Point2 {
  fn clone(&self) -> Self {
    Self { x: self.x, y: self.y }
  }
}
impl Copy for Point2 {}