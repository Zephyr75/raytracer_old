use pixel_canvas::{input::MouseState, Canvas, Color, Image, XY};

const WIDTH: usize = 500;
const HEIGHT: usize = 500;
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
    for i in 0..HEIGHT {
        for j in 0..WIDTH {

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
