extern crate image;

use std::path::Path;

use image::Rgb;

mod disk;
use disk::Disk;

const BLACK: Rgb<u8> = Rgb::<u8> { data: [0, 0, 0] };

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
        let mut split = d.split();
        for disk in &mut split {
            let color = disk.choose_color(&img);
            disk.set_color(color);
            let cost = disk.cost(&img);
            disks.push((cost, disk.clone()));
        }
    }

    let mut img = image::ImageBuffer::new(img.width(), img.height());
    for (_, d) in disks {
        d.draw(&mut img);
    }

    img.save("test.png").unwrap();
}
