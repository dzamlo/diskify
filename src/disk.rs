use image::{GenericImage, Rgb};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Disk {
    top: u32,
    left: u32,
    width: u32,
    color: Rgb<u8>,
}

impl Disk {
    pub fn new(top: u32, left: u32, width: u32, color: Rgb<u8>) -> Disk {
        Disk {
            top: top,
            left: left,
            width: width,
            color: color,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn inside(&self, x: u32, y: u32) -> bool {
        let center_x = self.left + self.width / 2;
        let center_y = self.top + self.width / 2;
        let center_square = (self.width / 2) * (self.width / 2);
        square_abs_diff(x, center_x) + square_abs_diff(y, center_y) <= center_square
    }

    pub fn set_color(&mut self, color: Rgb<u8>) {
        self.color = color;
    }

    pub fn choose_color<I>(&self, img: &I) -> Rgb<u8>
        where I: GenericImage<Pixel = Rgb<u8>>
    {
        let mut sums = [0u32; 3];
        for x in self.left..(self.left + self.width) {
            for y in self.top..(self.top + self.width) {
                let pixel = img.get_pixel(x, y);
                for i in 0..sums.len() {
                    sums[i] += pixel[i] as u32;
                }
            }
        }

        for channel in &mut sums {
            *channel /= self.width * self.width;
        }

        Rgb { data: [sums[0] as u8, sums[1] as u8, sums[2] as u8] }
    }

    pub fn cost<I>(&self, img: &I) -> f64
        where I: GenericImage<Pixel = Rgb<u8>>
    {
        let mut sum = 0u64;

        for x in self.left..(self.left + self.width) {
            for y in self.top..(self.top + self.width) {
                let pixel = img.get_pixel(x, y);
                for (channel_pixel, channel_self) in pixel.data
                    .into_iter()
                    .zip(&self.color.data) {
                    sum += square_abs_diff(*channel_pixel as u32, *channel_self as u32) as u64
                }
            }
        }

        sum as f64 / (self.width * self.width * 3) as f64
    }

    pub fn split(&self) -> [Disk; 4] {
        let split_at = self.width / 2;
        [Disk::new(self.top, self.left, split_at, self.color),
         Disk::new(self.top + split_at, self.left, split_at, self.color),
         Disk::new(self.top, self.left + split_at, split_at, self.color),
         Disk::new(self.top + split_at,
                   self.left + split_at,
                   split_at,
                   self.color)]
    }

    pub fn draw<I>(&self, img: &mut I)
        where I: GenericImage<Pixel = Rgb<u8>> + 'static
    {
        for x in self.left..(self.left + self.width) {
            for y in self.top..(self.top + self.width) {
                if self.inside(x, y) {
                    img.put_pixel(x, y, self.color);
                }
            }
        }

    }

    pub fn backgrounds_sums<I>(&self, img: &I) -> (u64, [u64; 3])
        where I: GenericImage<Pixel = Rgb<u8>>
    {
        let mut sums = [0u64; 3];
        let mut count = 0;
        for x in self.left..(self.left + self.width) {
            for y in self.top..(self.top + self.width) {
                let pixel = img.get_pixel(x, y);
                if !self.inside(x, y) {
                    count += 1;
                    for i in 0..sums.len() {
                        sums[i] += pixel[i] as u64;
                    }
                }
            }
        }

        (count, sums)
    }
}

fn abs_diff(a: u32, b: u32) -> u32 {
    if a < b { (b - a) } else { (a - b) }
}

fn square_abs_diff(a: u32, b: u32) -> u32 {
    let diff = abs_diff(a, b);
    diff * diff
}
