extern crate genial;

use genial::Image;
use genial::draw;
use genial::color;

fn main() {
    let mut image = Image::new(100, 100);
    draw::draw_line(&mut image, (0, 0), (100, 100), color::CYAN);
    draw::draw_circle(&mut image, (50, 50), 30, color::LIME);
    draw::draw_filled_circle(&mut image, (50, 50), 20, color::MAROON);
    image.save_to_file("drawing.bmp");
}
