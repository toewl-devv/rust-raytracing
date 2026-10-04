pub struct Vec3 { 
    pub x: f64, 
    pub y: f64, 
    pub z: f64 
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }
}


pub fn dot(vec1: Vec3, vec2: Vec3) -> f64 {
    vec1.x * vec2.x + vec1.y * vec2.y + vec1.z * vec2.z
}

pub fn cross(vec1: Vec3, vec2: Vec3) -> Vec3 {
    Vec3 {
        x: todo!(),
        y: todo!(),
        z: todo!()
    }
}
