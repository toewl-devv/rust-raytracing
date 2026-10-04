mod rendering;
mod vec3;
mod color;
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;


fn main() {
    let camera = rendering::Camera::new();

    let mut buffer = vec![0u32; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Ray Tracer",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap();

    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {

        // Your renderer goes here
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let pixelcolor = &camera.display(x as u32, y as u32);
                let r = &pixelcolor.r;
                let g = &pixelcolor.g;
                let b = &pixelcolor.b;
                /*
                let r = (x * 255 / WIDTH) as u32;
                let g = (y * 255 / HEIGHT) as u32;
                let b = 100;
                */

                buffer[y * WIDTH + x] =
                    (r << 16) | (g << 8) | b;
            }
        }

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
