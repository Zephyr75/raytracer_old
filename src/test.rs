use pixel_canvas::{Canvas, Color, input::MouseState};

fn main() {
    const WIDTH: usize = 500;
    const HEIGHT: usize = 500;

    let canvas = Canvas::new(WIDTH, HEIGHT)
        .title("Tile")
        .state(MouseState::new())
        .input(MouseState::handle_input);

    let mut pixels: [[Color; WIDTH]; HEIGHT] = [[Color::BLACK; WIDTH]; HEIGHT];

    for i in 0..HEIGHT {
        pixels[i][i] = Color::WHITE;
    }
        
    canvas.render(|mouse, image| {
        let width = image.width() as usize;
        for (y, row) in image.chunks_mut(width).enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                let dx = x as i32 - mouse.x;
                let dy = y as i32 - mouse.y;
                let dist = dx * dx + dy * dy;
                *pixel = pixels[y][x];
            }
        }
    });
}