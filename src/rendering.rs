use crate::vec3::Vec3;
use crate::color::Color;

pub struct Camera {
    pub pos: Vec3,
    pub dir: Vec3
}

const sphere_pos = Vec3{x: 2.0, y: 2.0, z: 2.0};
const sphere_radius = 2f64;

impl Camera {
    pub fn display(x: u32, y: u32) -> Color {
        // first find the position of x and y in coordinates


        // then raymarch along that until the point is inside some sphere
        // once it's inside, draw a white point. If it reaches, say t = 2000, draw it black

    }
}

