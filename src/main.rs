use std::collections::HashSet;
use std::ops::RangeInclusive;
use image::{self, ImageBuffer, Rgb};
use rand::{Rng, SeedableRng};

const IMG_WIDTH: u32 = 1920;
const IMG_HEIGHT: u32 = 1080;
const RANDOMIZATION_RANGE: RangeInclusive<i16> = -1..=1;

const BLANK: &Rgb<u8> = &Rgb([0, 0, 0]);

fn main() {
    let center = (IMG_WIDTH / 2, IMG_HEIGHT / 2);
    // let center = (0, 0);
    let mut visited_pixels: HashSet<(u32, u32)> = HashSet::new();
    visited_pixels.insert((center.0, center.1));
    
    let mut edge_pixels: Vec<(u32, u32)> = get_neighbors(center.0, center.1);
    // println!("{:?}", edge_pixels);

    let mut img = image::RgbImage::new(IMG_WIDTH, IMG_HEIGHT);
    img.put_pixel(center.0, center.1, Rgb([255, 255, 255]));

    // let mut rng = rand::rngs::StdRng::seed_from_u64(0);
    let mut rng = rand::thread_rng();
    while edge_pixels.len() != 0 {
        let random_edge = rng.gen_range(0..edge_pixels.len());
        let pixel = edge_pixels.remove(random_edge);
        visited_pixels.insert(pixel);

        let color = generate_color(&img, pixel.0, pixel.1);
        img.put_pixel(pixel.0, pixel.1, color);

        for neighbor in get_neighbors(pixel.0, pixel.1) {
            if visited_pixels.contains(&neighbor) {
                continue;
            }
            edge_pixels.push(neighbor);
        }
    }

    img.save("image.png").unwrap();
}

fn get_neighbors(x: u32, y: u32) -> Vec<(u32, u32)> {
    let mut neighbors: Vec<(u32, u32)> = Vec::new();

    for off_x in [-1, 1] {
        let off_x = (x as i32) + off_x;
        if off_x < 0 {
            continue;
        }
        let off_x: u32 = off_x.try_into().unwrap();
        if off_x >= IMG_WIDTH {
            continue;
        }

        neighbors.push((off_x, y));
    }

    for off_y in [-1, 1] {
        let off_y = (y as i32) + off_y;
        if off_y < 0 {
            continue;
        }
        let off_y: u32 = off_y.try_into().unwrap();
        if off_y >= IMG_HEIGHT {
            continue;
        }

        neighbors.push((x, off_y));
    }

    neighbors
}

fn generate_color(img: &ImageBuffer<Rgb<u8>, Vec<u8>>, x: u32, y: u32) -> Rgb<u8> {
    let neighbors = get_neighbors(x, y);
    let mut alive_neighbors = neighbors.len() as u16;

    let neighbors_color_sum: [u16; 3] = neighbors.iter().fold([0, 0, 0], |acc: [u16; 3], (x, y)| {
        let color = img.get_pixel(*x, *y).0;
        // println!("({}, {}) -> {:?}", x, y, color);
        if color == BLANK.0 {
            alive_neighbors -= 1
        }
        [acc[0] + color[0] as u16, acc[1] + color[1] as u16, acc[2] + color[2] as u16]
    });
    
    if alive_neighbors == 0 {
        alive_neighbors = 1;
    }
    let mut color = [(neighbors_color_sum[0] / alive_neighbors) as u8, (neighbors_color_sum[1] / alive_neighbors) as u8, (neighbors_color_sum[2] / alive_neighbors) as u8];
    // println!("({}, {}) {:?} <- {}", x, y, color, alive_neighbors);
    
    let mut rng = rand::thread_rng();
    color.iter_mut().for_each(|value| *value = (*value as i16 + rng.gen_range(RANDOMIZATION_RANGE)).min(255).max(0) as u8);
    Rgb(color)
}