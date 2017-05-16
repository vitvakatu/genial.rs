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
    fn format(&self) -> ColorFormat;
    fn as_slice(&self) -> &[u8];
}

impl Pixel for RGB {
    fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        rgb(r, g, b)
    }
    fn format(&self) -> ColorFormat {
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
        let (width, height) = self.dimensions();
        if x > width as i32 || y > height as i32 || x < 0 || y < 0 {
            return;
        }
        let index = ((self.height as i32 - y - 1) * self.width as i32 + x) as usize *
                    self.format.channels();
        let channels = self.format.channels() as usize;
        if color.format() == self.format {
            for i in index..(index + channels) {
                self.data[i] = color.as_slice()[i - index];
            }
        }
    }

    pub fn get_pixel<P: Pixel>(&self, x: i32, y: i32) -> P {
        let (width, height) = self.dimensions();
        if x > width as i32 || y > height as i32 || x < 0 || y < 0 {
            return P::from_rgb(0, 0, 0);
        }
        let index = ((height as i32 - y - 1) * width as i32 + x) as usize *
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

    pub trait ImageOps {
        fn flip_vertical(&mut self) -> &mut Image;
        fn flip_horizontal(&mut self) -> &mut Image;
    }

    impl ImageOps for Image {
        fn flip_vertical(&mut self) -> &mut Image {
            let (width, height) = self.dimensions();
            let mut pixel1: RGB;
            let mut pixel2: RGB;
            for y in 0..height / 2 {
                for x in 0..width {
                    pixel1 = self.get_pixel(x as i32, y as i32);
                    pixel2 = self.get_pixel(x as i32, (height - y - 1) as i32);
                    self.set_pixel(x as i32, y as i32, pixel2);
                    self.set_pixel(x as i32, (height - y - 1) as i32, pixel1);
                }
            }
            self
        }

        fn flip_horizontal(&mut self) -> &mut Image {
            let (width, height) = self.dimensions();
            let mut pixel1: RGB;
            let mut pixel2: RGB;
            for x in 0..width / 2 {
                for y in 0..height {
                    pixel1 = self.get_pixel(x as i32, y as i32);
                    pixel2 = self.get_pixel((width - x - 1) as i32, y as i32);
                    self.set_pixel(x as i32, y as i32, pixel2);
                    self.set_pixel((width - x - 1) as i32, y as i32, pixel1);
                }
            }
            self
        }
    }
}

pub mod draw {
    use ::*;

    pub struct LineBuilder<'a> {
        image: &'a mut Image,
        from: (i32, i32),
        to: (i32, i32),
        color: RGB,
    }

    impl<'a> LineBuilder<'a> {
        pub fn from(mut self, x: i32, y: i32) -> Self {
            self.from = (x, y);
            self
        }

        pub fn to(mut self, x: i32, y: i32) -> Self {
            self.to = (x, y);
            self
        }

        pub fn with_color(mut self, color: RGB) -> Self {
            self.color = color;
            self
        }

        pub fn draw(self) -> &'a mut Image {
            draw_line(self.image, self.from, self.to, self.color);
            self.image
        }
    }

    pub struct CircleBuilder<'a> {
        image: &'a mut Image,
        filled: bool,
        origin: (i32, i32),
        radius: i32,
        color: RGB,
    }

    impl<'a> CircleBuilder<'a> {
        pub fn origin(mut self, x: i32, y: i32) -> Self {
            self.origin = (x, y);
            self
        }

        pub fn filled(mut self, filled: bool) -> Self {
            self.filled = filled;
            self
        }

        pub fn radius(mut self, radius: i32) -> Self {
            self.radius = radius;
            self
        }

        pub fn with_color(mut self, color: RGB) -> Self {
            self.color = color;
            self
        }

        pub fn draw(self) -> &'a mut Image {
            if self.filled {
                draw_filled_circle(self.image, self.origin, self.radius, self.color);
            } else {
                draw_circle(self.image, self.origin, self.radius, self.color);
            }
            self.image
        }
    }

    pub trait Draw {
        fn line(&mut self) -> LineBuilder;
        fn circle(&mut self) -> CircleBuilder;
    }

    impl Draw for Image {
        fn line(&mut self) -> LineBuilder {
            LineBuilder {
                image: self,
                from: (0, 0),
                to: (0, 0),
                color: rgb(0, 0, 0),
            }
        }

        fn circle(&mut self) -> CircleBuilder {
            CircleBuilder {
                image: self,
                filled: false,
                origin: (0, 0),
                radius: 0,
                color: rgb(0, 0, 0),
            }
        }
    }

    fn draw_line(image: &mut Image,
                 (x_start, y_start): (i32, i32),
                 (x_end, y_end): (i32, i32),
                 color: RGB) {
        let mut steep = false;
        let mut x0: i32;
        let mut x1: i32;
        let mut y0: i32;
        let mut y1: i32;
        if (x_start - x_end).abs() < (y_start - y_end).abs() {
            steep = true;
            // swap x and y
            x0 = y_start;
            x1 = y_end;
            y0 = x_start;
            y1 = x_end;
        } else {
            // normal
            x0 = x_start;
            x1 = x_end;
            y0 = y_start;
            y1 = y_end;
        }
        if x_start > x_end {
            // swap the beginning and the end of the line
            let mut tmp = x1;
            x1 = x0;
            x0 = tmp;
            tmp = y1;
            y1 = y0;
            y0 = tmp;
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

    fn draw_circle(image: &mut Image, (x0, y0): (i32, i32), r: i32, color: RGB) {
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

    fn draw_filled_circle(image: &mut Image, (x0, y0): (i32, i32), r: i32, color: RGB) {
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
