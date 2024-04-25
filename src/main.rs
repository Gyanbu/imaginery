use rand::{rngs::StdRng, SeedableRng};
use rand::Rng;
use std::{ops::RangeInclusive, time::Instant};
use image::{self, ImageBuffer, Rgb};
use rounded_div;

const IMG_WIDTH: u32 = 1000;
const IMG_HEIGHT: u32 = 1000;
const RANDOMIZATION_RANGE: RangeInclusive<i16> = -10..=10;

static mut RNG: Option<StdRng> = None;
unsafe fn rng() -> &'static mut StdRng {
    if RNG.is_none() {
        RNG = Some(rand::rngs::StdRng::from_entropy());
    }
    
    RNG.as_mut().unwrap()
}

fn main() {
    let mut img = image::RgbImage::new(IMG_WIDTH, IMG_HEIGHT);
    let mut visited_pixels: Box<[[bool; IMG_HEIGHT as usize]; IMG_WIDTH as usize]> = Box::new([[false; IMG_HEIGHT as usize]; IMG_WIDTH as usize]);
    let mut edge_pixels: Vec<(u32, u32)> = Vec::new();
    
    let starting_pixels: Vec<(u32, u32)> = vec!((IMG_WIDTH / 2, IMG_HEIGHT / 2));
    // let starting_pixels: Vec<(u32, u32)> = vec!((0, 0));
    // let starting_pixels: Vec<(u32, u32)> = vec!((0, 0), (0, IMG_HEIGHT - 1), (IMG_WIDTH - 1, 0), (IMG_WIDTH - 1, IMG_HEIGHT - 1));
    for (x, y) in starting_pixels {
        edge_pixels.extend(get_neighbors(x, y));
        visited_pixels[x as usize][y as usize] = true;
        img.put_pixel(x, y, Rgb([255, 255, 255]));
    }

    // let mut rng = rand::rngs::StdRng::seed_from_u64(0);
    let rng = unsafe {rng()};

    let start_time = Instant::now();
    while !edge_pixels.is_empty() {
        let random_edge = rng.gen_range(0..edge_pixels.len());
        let pixel = edge_pixels.swap_remove(random_edge);
        // let pixel = edge_pixels.pop().unwrap();
        visited_pixels[pixel.0 as usize][pixel.1 as usize] = true;

        let color = generate_color(&img, pixel.0, pixel.1, &visited_pixels);
        img.put_pixel(pixel.0, pixel.1, color);

        for neighbor in get_neighbors(pixel.0, pixel.1) {
            if visited_pixels[neighbor.0 as usize][neighbor.1 as usize] {
                continue;
            }
            edge_pixels.push(neighbor);
        }
    }
    let elapsed_time = start_time.elapsed();
    println!("Image generated in: {:?}", elapsed_time);

    img.save("image.png").unwrap();
}

const NEIGHBOR_OFFSETS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn get_neighbors(x: u32, y: u32) -> impl Iterator<Item = (u32, u32)> {
    NEIGHBOR_OFFSETS.iter().filter_map(move |&(dx, dy)| {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if nx >= 0 && ny >= 0 && nx < IMG_WIDTH as i32 && ny < IMG_HEIGHT as i32 {
            Some((nx as u32, ny as u32))
        } else {
            None
        }
    })
}

fn generate_color(img: &ImageBuffer<Rgb<u8>, Vec<u8>>, x: u32, y: u32, visited_pixels: &Box<[[bool; IMG_HEIGHT as usize]; IMG_WIDTH as usize]>) -> Rgb<u8> {
    let neighbors = get_neighbors(x, y);
    let mut alive_neighbors: u16 = 0;

    let neighbors_color_sum: [u16; 3] = neighbors.fold([0, 0, 0], |acc: [u16; 3], (x, y)| {
        let color = img.get_pixel(x, y).0;
        // println!("({}, {}) -> {:?}", x, y, color);
        if visited_pixels[x as usize][y as usize] {
            alive_neighbors += 1;
        }
        [acc[0] + color[0] as u16, acc[1] + color[1] as u16, acc[2] + color[2] as u16]
    });
    
    if alive_neighbors == 0 {
        alive_neighbors = 1;
    }
    let mut color = [
        rounded_div::u16(neighbors_color_sum[0], alive_neighbors) as u8,
        rounded_div::u16(neighbors_color_sum[1], alive_neighbors) as u8,
        rounded_div::u16(neighbors_color_sum[2], alive_neighbors) as u8
    ];
    // println!("({}, {}) {:?} <- {}", x, y, color, alive_neighbors);
    
    let rng = unsafe {rng()};
    color.iter_mut().for_each(|value| *value = (*value as i16 + rng.gen_range(RANDOMIZATION_RANGE)).min(255).max(0) as u8);
    Rgb(color)
}