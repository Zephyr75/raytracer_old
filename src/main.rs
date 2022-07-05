use pixel_canvas::{input::MouseState, Canvas, Color, Image, XY, Vec3};
mod vector3;
mod point3;
mod ray;

const WIDTH: usize = 800;
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

    let origin: point3::Point3 = point3::Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let direction: vector3::Vector3 = vector3::Vector3 {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };

    let mut a = ray::Ray {
        origin: origin,
        direction: direction,
    };

    println!("{}", a);

    std::process::exit(0);

    for i in 0..WIDTH {
        //println!("Lines remaining: {}", HEIGHT - i);
        for j in 0..HEIGHT {

            let r: u8 = (255 * i / (WIDTH - 1)) as u8;
            let g: u8 = (255 * j / (HEIGHT - 1)) as u8;
            let b: u8 = (60) as u8;
            pixels[i * HEIGHT + j] = Color {
                r: r,
                g: g,
                b: b,
            }; 
        }
    }
    return pixels;
}
