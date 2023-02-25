// This is the space that the clouds of points are generated in

pub struct Space {
    pub xmin: f64,
    pub xmax: f64,
    pub ymin: f64,
    pub ymax: f64,
}

impl Space {
    pub fn new(xmin: f64, xmax: f64, ymin: f64, ymax: f64) -> Space {
        Space { xmin, xmax, ymin, ymax }
    }
}