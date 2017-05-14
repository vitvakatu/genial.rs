extern crate imagefmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RGB {
    data: [u8; 3],
}

pub fn rgb(r: u8, g: u8, b: u8) -> RGB {
    RGB { data: [r, g, b] }
}

pub mod color {
    use ::*;
    pub const WHITE: RGB = RGB { data: [255, 255, 255] };
    pub const BLACK: RGB = RGB { data: [0, 0, 0] };
    pub const RED: RGB = RGB { data: [255, 0, 0] };
    pub const LIME: RGB = RGB { data: [0, 255, 0] };
    pub const BLUE: RGB = RGB { data: [0, 0, 255] };
    pub const YELLOW: RGB = RGB { data: [255, 255, 0] };
    pub const CYAN: RGB = RGB { data: [0, 255, 255] };
    pub const MAGENTA: RGB = RGB { data: [255, 0, 255] };
    pub const SILVER: RGB = RGB { data: [192, 192, 192] };
    pub const GRAY: RGB = RGB { data: [128, 128, 128] };
    pub const MAROON: RGB = RGB { data: [218, 0, 0] };
    pub const OLIVE: RGB = RGB { data: [128, 128, 0] };
    pub const GREEN: RGB = RGB { data: [0, 128, 0] };
    pub const PURPLE: RGB = RGB { data: [128, 0, 128] };
    pub const TEAL: RGB = RGB { data: [0, 128, 128] };
    pub const NAVY: RGB = RGB { data: [0, 0, 128] };
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ColorFormat {
    Y,
    YA,
    AY,
    RGB,
    RGBA,
    ARGB,
    BGR,
    BGRA,
    ABGR,
}

impl ColorFormat {
    fn channels(&self) -> usize {
        match *self {
            ColorFormat::Y => 1,
            ColorFormat::YA | ColorFormat::AY => 2,
            ColorFormat::RGB | ColorFormat::BGR => 3,
            ColorFormat::ABGR | ColorFormat::ARGB | ColorFormat::BGRA | ColorFormat::RGBA => 4,
        }
    }
}

pub trait Pixel {
    fn from_rgb(u8, u8, u8) -> Self;
    fn color_fmt(&self) -> ColorFormat;
    fn as_slice(&self) -> &[u8];
}

impl Pixel for RGB {
    fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        rgb(r, g, b)
    }
    fn color_fmt(&self) -> ColorFormat {
        ColorFormat::RGB
    }
    fn as_slice(&self) -> &[u8] {
        &self.data
    }
}

#[derive(Clone)]
pub struct Image {
    data: Vec<u8>,
    width: usize,
    height: usize,
    format: ColorFormat,
}

fn swap<T: Clone>(i: &mut T, j: &mut T) {
    let tmp = i.clone();
    *i = j.clone();
    *j = tmp;
}

impl Image {
    pub fn new(width: usize, height: usize, format: ColorFormat) -> Self {
        let mut data = Vec::with_capacity(width as usize * height as usize *
                                          format.channels() as usize);
        for _ in 0..width * height * format.channels() {
            data.push(0);
        }
        debug_assert_eq!(data.len(), width * height * format.channels());
        Image {
            data: data,
            width: width,
            height: height,
            format: format,
        }
    }

    pub fn from_file(filename: &str) -> Option<Self> {
        let image = imagefmt::read(filename, imagefmt::ColFmt::Auto);
        match image {
            Ok(image) => {
                let (width, height) = (image.w, image.h);
                let format = match image.fmt {
                    imagefmt::ColFmt::ABGR => ColorFormat::ABGR,
                    imagefmt::ColFmt::ARGB => ColorFormat::ARGB,
                    imagefmt::ColFmt::AY => ColorFormat::AY,
                    imagefmt::ColFmt::Y => ColorFormat::Y,
                    imagefmt::ColFmt::YA => ColorFormat::YA,
                    imagefmt::ColFmt::RGB => ColorFormat::RGB,
                    imagefmt::ColFmt::RGBA => ColorFormat::RGBA,
                    imagefmt::ColFmt::BGR => ColorFormat::BGR,
                    imagefmt::ColFmt::BGRA => ColorFormat::BGRA,
                    imagefmt::ColFmt::Auto => unimplemented!(),
                };
                Some(Image {
                         width: width,
                         height: height,
                         data: image.buf,
                         format: format,
                     })
            }
            Err(_) => None,
        }
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn set_pixel<P: Pixel>(&mut self, x: i32, y: i32, color: P) {
        debug_assert!(x <= self.width as i32);
        debug_assert!(y <= self.height as i32);
        let index = ((self.height as i32 - y - 1) * self.width as i32 + x) as usize *
                    self.format.channels();
        let channels = self.format.channels() as usize;
        if color.color_fmt() == self.format {
            for i in index..(index + channels) {
                self.data[i] = color.as_slice()[i - index];
            }
        }
    }

    pub fn get_pixel<P: Pixel>(&self, x: i32, y: i32) -> P {
        debug_assert!(x <= self.width as i32);
        debug_assert!(y <= self.height as i32);
        let index = ((self.height as i32 - y - 1) * self.width as i32 + x) as usize *
                    self.format.channels() as usize;
        match self.format {
            ColorFormat::RGB => {
                P::from_rgb(self.data[index], self.data[index + 1], self.data[index + 2])
            }
            _ => unimplemented!(),
        }
    }


    pub fn save_to_file(&self, filename: &str) {
        imagefmt::write(filename,
                        self.width as usize,
                        self.height as usize,
                        imagefmt::ColFmt::RGB,
                        &self.data,
                        imagefmt::ColType::Color)
                .unwrap();
    }
}

pub mod ops {
    use ::*;
    pub fn flip_vertical(image: &Image) -> Image {
        let (width, height) = image.dimensions();
        let mut result = Image::new(width, height, image.format);
        for y in 0..height {
            for x in 0..width {
                let p: RGB = image.get_pixel(x as i32, y as i32);
                result.set_pixel(x as i32, (height - y) as i32 - 1, p);
            }
        }
        result
    }

    pub fn flip_horizontal(image: &Image) -> Image {
        let (width, height) = image.dimensions();
        let mut result = Image::new(width, height, image.format);
        for y in 0..height {
            for x in 0..width {
                let p: RGB = image.get_pixel(x as i32, y as i32);
                result.set_pixel((width - x) as i32 - 1, y as i32, p);
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
        debug_assert!(x0 <= dimensions.0 as i32);
        debug_assert!(x1 <= dimensions.0 as i32);
        debug_assert!(y0 <= dimensions.1 as i32);
        debug_assert!(y1 <= dimensions.1 as i32);
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
