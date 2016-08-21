extern crate clap;
extern crate image;

use std::path::Path;

use clap::{Arg, App};
use image::{ImageBuffer, Rgb};

mod disk;
use disk::Disk;

const BLACK: Rgb<u8> = Rgb::<u8> { data: [0, 0, 0] };

fn validate_min_width(val: String) -> Result<(), String> {
    if val.parse::<u32>().is_err() {
        Err(String::from("the miminum width must an integer in the range 0-4294967295"))
    } else {
        Ok(())
    }
}

fn validate_iterations(val: String) -> Result<(), String> {
    if val.parse::<u32>().is_err() {
        Err(String::from("the number of iterations must an integer in the range 0-4294967295"))
    } else {
        Ok(())
    }
}

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
    let app = App::new("diskify")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(Arg::with_name("min_width")
            .short("m")
            .long("min-width")
            .help("minimum width of the disks")
            .takes_value(true)
            .validator(validate_min_width)
            .default_value("8"))
        .arg(Arg::with_name("iterations")
            .short("i")
            .long("iterations")
            .help("the number of iterations")
            .takes_value(true)
            .validator(validate_iterations)
            .default_value("1024"))
        .arg(Arg::with_name("input")
            .help("input image")
            .required(true))
        .arg(Arg::with_name("output")
            .help("where to save the result")
            .required(true));
    let matches = app.get_matches();
    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();
    let min_width = matches.value_of("min_width").unwrap().parse().unwrap();
    let iterations = matches.value_of("iterations").unwrap().parse().unwrap();


    let img = image::open(&Path::new(input)).unwrap().to_rgb();
    assert!(img.width() == img.height(), "The image must be a square");
    let mut initial_disk = Disk::new(0, 0, img.width(), BLACK);
    let color = initial_disk.choose_color(&img);
    initial_disk.set_color(color);
    let initial_disk_cost = initial_disk.cost(&img);
    let mut disks = vec![(initial_disk_cost, initial_disk)];

    for i in 0..iterations {
        println!("iteration {}/{}", i + 1, iterations);
        disks.sort_by(|&(cost1, _), &(cost2, _)| cost1.partial_cmp(&cost2).unwrap());
        let (_, d) = disks.pop().unwrap();
        if d.width() <= min_width {
            disks.push((0.0, d));
            break;
        }
        let mut split = d.split();
        for disk in &mut split {
            let color = disk.choose_color(&img);
            disk.set_color(color);
            let cost = if disk.width() <= min_width {
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

    img.save(output).unwrap();
}
