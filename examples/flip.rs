extern crate genial;

use genial::Image;
use genial::ops;

fn main() {
    let image = Image::from_file("example.jpg").unwrap();
    let flipped = ops::flip_horizontal(&image);
    flipped.save_to_file("flip.bmp");
}
