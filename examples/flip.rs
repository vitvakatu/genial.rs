extern crate genial;

use genial::Image;
use genial::ops::ImageOps;

fn main() {
    Image::from_file("example.jpg")
        .unwrap()
        .flip_vertical()
        .flip_horizontal()
        .save_to_file("flipped.bmp");
}
