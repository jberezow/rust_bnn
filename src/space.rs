// This is the space that the clouds of points are generated in

pub struct Space {
    pub xmin: f32,
    pub xmax: f32,
    pub ymin: f32,
    pub ymax: f32,
}

impl Space {
    pub fn new(xmin: f32, xmax: f32, ymin: f32, ymax: f32) -> Space {
        Space { xmin, xmax, ymin, ymax }
    }
}