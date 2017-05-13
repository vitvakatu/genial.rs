extern crate genial;

use genial::Image;
use genial::draw;
use genial::ops;
use genial::color;

fn main() {
    let mut image = Image::new(100, 100);
    draw::draw_line(&mut image, (0, 0), (100, 100), color::CYAN);
    let flipped = ops::flip_vertical(&image);
    flipped.save_to_file("flip.bmp");
}
