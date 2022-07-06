/*
use pixel_canvas::{input::MouseState, Canvas, Color, Image, XY, Vec3};
mod vector3;
use vector3::Vector3;
mod point3;
use point3::Point3;
mod ray;
use ray::Ray;

const WIDTH: usize = 800;
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const HEIGHT: usize = 450;

fn main() {
    let canvas = Canvas::new(WIDTH, HEIGHT)
        .title("Tile")
        .state(MouseState::new())
        .input(MouseState::handle_input);

    canvas.render(|mouse, image| {
        let mut pixels = vec![Color::BLACK; WIDTH * HEIGHT];

        pixels = get_pixels();

        let width = image.width() as usize;
        for (y, row) in image.chunks_mut(width).enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                let dx = x as i32 - mouse.x;
                let dy = y as i32 - mouse.y;
                let _dist = dx * dx + dy * dy;
                *pixel = pixels[x * HEIGHT + y];
            }
        }
    });
}

fn get_pixels() -> Vec<Color>{
    let mut pixels = vec![Color::BLACK; WIDTH * HEIGHT];

    // Viewport settings
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO as f32 * viewport_height;
    let focal_length = 1.0;

    let origin: Point3 = Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let horizontal: Vector3 = Vector3 {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };

    let vertical: Vector3 = Vector3 {
        x: 0.0,
        y: viewport_height,
        z: 0.0,
    };

    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - vector3 :: Vector3{x: 0.0, y: 0.0, z: focal_length};

    for j in 0..HEIGHT {
        //println!("Lines remaining: {}", HEIGHT - i);
        for i in 0..WIDTH {

            let u: f32 = i as f32 / (WIDTH as f32 - 1.0);
            let v: f32 = j as f32 / (HEIGHT as f32 - 1.0);
            
            let r: Ray = Ray { origin: origin, direction: lower_left_corner - origin + horizontal * u + vertical * v };

            let mut color = ray_color(r);

            let center: Point3 = Point3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            };

            if (hit_sphere(center, 0.5, r)){
                color = Color::BLACK;
            }

            pixels[i * HEIGHT + j] = color;
        }
    }
    return pixels;
}



*/