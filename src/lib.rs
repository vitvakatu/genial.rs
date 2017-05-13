extern crate imagefmt;
use imagefmt::{ColFmt, ColType};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RGB(pub u8, pub u8, pub u8);

pub mod color {
    use ::*;
    pub const WHITE: RGB = RGB(255, 255, 255);
    pub const BLACK: RGB = RGB(0, 0, 0);
    pub const RED: RGB = RGB(255, 0, 0);
    pub const LIME: RGB = RGB(0, 255, 0);
    pub const BLUE: RGB = RGB(0, 0, 255);
    pub const YELLOW: RGB = RGB(255, 255, 0);
    pub const CYAN: RGB = RGB(0, 255, 255);
    pub const MAGENTA: RGB = RGB(255, 0, 255);
    pub const SILVER: RGB = RGB(192, 192, 192);
    pub const GRAY: RGB = RGB(128, 128, 128);
    pub const MAROON: RGB = RGB(218, 0, 0);
    pub const OLIVE: RGB = RGB(128, 128, 0);
    pub const GREEN: RGB = RGB(0, 128, 0);
    pub const PURPLE: RGB = RGB(128, 0, 128);
    pub const TEAL: RGB = RGB(0, 128, 128);
    pub const NAVY: RGB = RGB(0, 0, 128);
}

#[derive(Clone)]
pub struct Image {
    data: Vec<RGB>,
    width: i32,
    height: i32,
}

fn swap<T: Clone>(i: &mut T, j: &mut T) {
    let tmp = i.clone();
    *i = j.clone();
    *j = tmp;
}

impl Image {
    pub fn new(width: i32, height: i32) -> Self {
        let mut data = Vec::with_capacity(width as usize * height as usize);
        for _ in 0..width * height {
            data.push(RGB(0, 0, 0));
        }
        debug_assert_eq!(data.len() as i32, width * height);
        Image {
            data: data,
            width: width,
            height: height,
        }
    }

    pub fn from_file(filename: &str) -> Option<Self> {
        let image = imagefmt::read(filename, ColFmt::Auto);
        match image {
            Ok(image) => {
                let (width, height) = (image.w, image.h);
                if image.fmt != ColFmt::RGB {
                    return None;
                }
                let mut buf = Vec::with_capacity(width * height);
                for c in image.buf.chunks(3) {
                    let color = RGB(c[0], c[1], c[2]);
                    buf.push(color);
                }
                Some(Image {
                         width: width as i32,
                         height: height as i32,
                         data: buf,
                     })
            }
            Err(_) => None,
        }
    }

    pub fn dimensions(&self) -> (i32, i32) {
        (self.width, self.height)
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: RGB) {
        debug_assert!(x <= self.width);
        debug_assert!(y <= self.height);
        let index = (self.height - y - 1) * self.width + x;
        self.data[index as usize] = color;
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> RGB {
        debug_assert!(x <= self.width);
        debug_assert!(y <= self.height);
        let index = (self.height - y - 1) * self.width + x;
        self.data[index as usize]
    }


    pub fn save_to_file(&self, filename: &str) {
        let mut buf = Vec::with_capacity(self.data.len() * 3);
        for p in &self.data {
            buf.push(p.0);
            buf.push(p.1);
            buf.push(p.2);
        }
        imagefmt::write(filename,
                        self.width as usize,
                        self.height as usize,
                        ColFmt::RGB,
                        &buf,
                        ColType::Color)
                .unwrap();
    }
}

pub mod ops {
    use ::*;
    pub fn flip_vertical(image: &Image) -> Image {
        let (width, height) = image.dimensions();
        let mut result = Image::new(width, height);
        for y in 0..height {
            for x in 0..width {
                let p = image.get_pixel(x, y);
                result.set_pixel(x, height - y - 1, p);
            }
        }
        result
    }

    pub fn flip_horizontal(image: &Image) -> Image {
        let (width, height) = image.dimensions();
        let mut result = Image::new(width, height);
        for y in 0..height {
            for x in 0..width {
                let p = image.get_pixel(x, y);
                result.set_pixel(width - x - 1, y, p);
            }
        }
        result
    }
}

pub mod draw {
    use ::*;
    pub fn draw_line(image: &mut Image,
                     (mut x0, mut y0): (i32, i32),
                     (mut x1, mut y1): (i32, i32),
                     color: RGB) {
        let dimensions = image.dimensions();
        debug_assert!(x0 <= dimensions.0);
        debug_assert!(x1 <= dimensions.0);
        debug_assert!(y0 <= dimensions.1);
        debug_assert!(y1 <= dimensions.1);
        let mut steep = false;
        if (x0 - x1).abs() < (y0 - y1).abs() {
            steep = true;
            swap(&mut x0, &mut y0);
            swap(&mut x1, &mut y1);
        }
        if x0 > x1 {
            swap(&mut x0, &mut x1);
            swap(&mut y0, &mut y1);
        }
        let dx = x1 - x0;
        let dy = y1 - y0;
        let derror = dy.abs() * 2;
        let mut error = 0;
        let mut y = y0;
        for x in x0..x1 {
            if steep {
                image.set_pixel(y, x, color);
            } else {
                image.set_pixel(x, y, color);
            }
            error += derror;
            if error > dx {
                if y1 > y0 {
                    y += 1;
                } else {
                    y -= 1;
                }
                error -= dx * 2;
            }
        }
    }

    pub fn draw_circle(image: &mut Image, (x0, y0): (i32, i32), r: i32, color: RGB) {
        let mut x: i32 = r;
        let mut y: i32 = 0;
        let mut err = 0;
        while x >= y {
            image.set_pixel(x0 + x, y0 + y, color);
            image.set_pixel(x0 + y, y0 + x, color);
            image.set_pixel(x0 - y, y0 + x, color);
            image.set_pixel(x0 - x, y0 + y, color);
            image.set_pixel(x0 - x, y0 - y, color);
            image.set_pixel(x0 - y, y0 - x, color);
            image.set_pixel(x0 + y, y0 - x, color);
            image.set_pixel(x0 + x, y0 - y, color);

            y += 1;
            if err <= 0 {
                err += 2 * y + 1;
            }
            if err > 0 {
                x -= 1;
                err -= 2 * x + 1;
            }
        }
    }

    pub fn draw_filled_circle(image: &mut Image, (x0, y0): (i32, i32), r: i32, color: RGB) {
        let r2 = r * r;
        let area = r2 << 2;
        let rr = r << 1;

        for i in 0..area {
            let tx = (i % rr) - r;
            let ty = (i / rr) - r;

            if tx * tx + ty * ty <= r2 {
                image.set_pixel(x0 + tx, y0 + ty, color);
            }
        }
        draw_circle(image, (x0, y0), r, color);
    }
}
