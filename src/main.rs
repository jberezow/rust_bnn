mod point2;

pub use na::Vector2;
pub use point2::Point2;

fn main() {
    // Generate a random vector for making a point
    let x = Vector2::new(1.0, 2.0);
    let y: i8 = 1;
    let p = Point2::new(x, y);
    println!("{}", p);
}
