extern crate image;

use std::path::Path;

use image::{ImageBuffer, Rgb};

mod disk;
use disk::Disk;

const BLACK: Rgb<u8> = Rgb::<u8> { data: [0, 0, 0] };
const MIN_WIDTH: u32 = 8;

pub fn choose_background<'a, D>(disks: D, img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Rgb<u8>
    where D: IntoIterator<Item = &'a Disk>
{
    let mut sums = [0u64; 3];
    let mut count = 0;
    for d in disks {
        let (disk_count, disk_sums) = d.backgrounds_sums(img);
        count += disk_count;
        for i in 0..sums.len() {
            sums[i] += disk_sums[i];
        }
    }
    for channel in &mut sums {
        if count > 0 {
            *channel /= count;
        }
    }

    Rgb { data: [sums[0] as u8, sums[1] as u8, sums[2] as u8] }
}

fn main() {
    let img = image::open(&Path::new("test.jpg")).unwrap().to_rgb();
    assert!(img.width() == img.height(), "The image must be a square");
    let mut initial_disk = Disk::new(0, 0, img.width(), BLACK);
    let color = initial_disk.choose_color(&img);
    initial_disk.set_color(color);
    let initial_disk_cost = initial_disk.cost(&img);
    let mut disks = vec![(initial_disk_cost, initial_disk)];

    for i in 0..4096 {
        println!("{}", i);
        disks.sort_by(|&(cost1, _), &(cost2, _)| cost1.partial_cmp(&cost2).unwrap());
        let (_, d) = disks.pop().unwrap();
        if d.width() <= MIN_WIDTH {
            disks.push((0.0, d));
            break;
        }
        let mut split = d.split();
        for disk in &mut split {
            let color = disk.choose_color(&img);
            disk.set_color(color);
            let cost = if disk.width() <= MIN_WIDTH {
                0.0
            } else {
                disk.cost(&img)
            };
            disks.push((cost, disk.clone()));
        }
    }
    let only_disks = disks.iter().map(|&(_, ref d)| d);
    let background = choose_background(only_disks, &img);
    let mut img = image::ImageBuffer::from_pixel(img.width(), img.height(), background);
    for &(_, ref d) in &disks {
        d.draw(&mut img);
    }

    img.save("test.png").unwrap();
}
