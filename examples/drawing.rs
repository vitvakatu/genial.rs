extern crate genial;

use genial::Image;
use genial::draw::Draw;
use genial::color;

fn main() {
    Image::new(100, 100, genial::ColorFormat::RGB)
        .line()
        .from(0, 0)
        .to(100, 100)
        .with_color(color::WHITE)
        .draw()
        .circle()
        .origin(50, 50)
        .radius(30)
        .with_color(color::CYAN)
        .draw()
        .save_to_file("drawing.bmp");
}
