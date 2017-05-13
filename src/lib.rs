extern crate imagefmt;
use imagefmt::{ColFmt, ColType};

#[derive(Clone, Copy)]
pub struct RGB(pub u8, pub u8, pub u8);

impl RGB {
    pub fn white() -> RGB {
        RGB(255, 255, 255)
    }

    pub fn black() -> RGB {
        RGB(0, 0, 0)
    }

    pub fn red() -> RGB {
        RGB(255, 0, 0)
    }

    pub fn green() -> RGB {
        RGB(0, 255, 0)
    }

    pub fn blue() -> RGB {
        RGB(0, 0, 255)
    }
}

pub struct Image {
    data: Vec<RGB>,
    width: usize,
    height: usize,
}

fn swap<T: Clone>(i: &mut T, j: &mut T) {
    let tmp = i.clone();
    *i = j.clone();
    *j = tmp;
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let mut data = Vec::with_capacity(width * height);
        for _ in 0..width*height {
            data.push(RGB(0, 0, 0));
        }
        assert_eq!(data.len(), width * height);
        Image {
            data: data,
            width: width,
            height: height,
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: RGB) {
        assert!(x < self.width);
        assert!(y < self.height);
        let index = (self.height - y - 1) * self.width + x;
        self.data[index] = color;
    }

    pub fn draw_line(&mut self, (mut x0, mut y0): (usize, usize), (mut x1, mut y1): (usize, usize), color: RGB) {
        assert!(x0 < self.width);
        assert!(x1 < self.width);
        assert!(y0 < self.height);
        assert!(y1 < self.height);
        let mut steep = false;
        if (x0 as i32 - x1 as i32).abs() < (y0 as i32 - y1 as i32).abs() {
            steep = true;
            swap(&mut x0, &mut y0);
            swap(&mut x1, &mut y1);
        }
        if x0 > x1 {
            swap(&mut x0, &mut x1);
            swap(&mut y0, &mut y1);
        }
        let dx: i32 = x1 as i32 - x0 as i32;
        let dy: i32 = y1 as i32 - y0 as i32;
        let derror = dy.abs() * 2;
        let mut error = 0;
        let mut y = y0;
        for x in x0..x1 {
            if steep {
                self.set_pixel(y, x, color);
            } else {
                self.set_pixel(x, y, color);
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

    pub fn save_to_file(&self, filename: &str) {
        let mut buf = Vec::with_capacity(self.data.len() * 3);
        for p in &self.data {
            buf.push(p.0);
            buf.push(p.1);
            buf.push(p.2);
        }
        imagefmt::write(filename, self.width, self.height, ColFmt::RGB, &buf, ColType::Color).unwrap();
    }
}