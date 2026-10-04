use crate::vec3::Vec3;
use crate::color::Color;

pub struct Camera {
    pub pos: Vec3,
    pub dir: Vec3
}

const sphere_pos: Vec3 = Vec3{x: 2.0, y: 2.0, z: 2.0};
const sphere_radius: f64 = 2f64;

impl Camera {
    pub fn new() -> Self {
        Camera {
            pos: Vec3{x: 0.0 ,y: 0.0 ,z: 0.0},
            dir: Vec3{x: 1.0 ,y: 0.0 ,z: 0.0}
        }
    }

    pub fn display(&self, x: u32, y: u32) -> Color {
        // first find the position of x and y in coordinates
        // Actually, first make it as if the camera is in origin (use notes from presentation!) and
        // then find the stuff
        
        // then raymarch along that until the point is inside some sphere
        // once it's inside, draw a white point. If it reaches, say t = 2000, draw it black

        Color{r: 1, g: 1, b: 1}

    }
}

