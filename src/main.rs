use rand::{rngs::StdRng, SeedableRng};
use rand::Rng;
use std::time::Instant;
use image::{self, ImageBuffer, Rgb};
use rounded_div;

static mut RNG: Option<StdRng> = None;
unsafe fn rng() -> &'static mut StdRng {
    if RNG.is_none() {
        RNG = Some(rand::rngs::StdRng::from_entropy());
    }
    
    RNG.as_mut().unwrap()
}

struct Imaginery {
    width: u32,
    height: u32,
    randomness: i16,
    save_path: String,
    img: ImageBuffer<Rgb<u8>, Vec<u8>>,
    visited_pixels: Vec<bool>
}

impl Imaginery {
    fn new(width: u32, height: u32, randomness: i16, save_path: &str) -> Imaginery {
        let save_path = save_path.to_owned();
        let img = image::RgbImage::new(width, height);
        let visited_pixels: Vec<bool> = vec![false; (width * height) as usize];

        Imaginery {
            width,
            height,
            randomness,
            save_path,
            img,
            visited_pixels
        }
    }

    fn generate_image(&mut self) {
        let mut edge_pixels: Vec<(u32, u32)> = Vec::new();
        let starting_pixels: Vec<(u32, u32)> = vec!((self.height / 2, self.height / 2));
        // let starting_pixels: Vec<(u32, u32)> = vec!((0, 0));
        // let starting_pixels: Vec<(u32, u32)> = vec!((0, 0), (0, IMG_HEIGHT - 1), (IMG_WIDTH - 1, 0), (IMG_WIDTH - 1, IMG_HEIGHT - 1));
        for (x, y) in starting_pixels {
            edge_pixels.extend(self.get_neighbors(x, y));
            self.visited_pixels[(x * self.width + y) as usize] = true;
            self.img.put_pixel(x, y, Rgb([255, 255, 255]));
        }

        // let mut rng = rand::rngs::StdRng::seed_from_u64(0);
        let rng = unsafe {rng()};

        let start_time = Instant::now();
        while !edge_pixels.is_empty() {
            let random_edge = rng.gen_range(0..edge_pixels.len());
            let pixel = edge_pixels.swap_remove(random_edge);
            // let pixel = edge_pixels.pop().unwrap();
            self.visited_pixels[(pixel.0 * self.width + pixel.1) as usize] = true;

            let color = self.generate_color(pixel.0, pixel.1);
            self.img.put_pixel(pixel.0, pixel.1, color);

            for neighbor in self.get_neighbors(pixel.0, pixel.1) {
                if self.visited_pixels[(neighbor.0 * self.width + neighbor.1) as usize] {
                    continue;
                }
                edge_pixels.push(neighbor);
            }
        }
        let elapsed_time = start_time.elapsed();
        println!("Image generated in: {:?}", elapsed_time);

        self.img.save(self.save_path.clone()).unwrap();
    }

    fn get_neighbors(&self, x: u32, y: u32) -> impl Iterator<Item = (u32, u32)> + '_ {
        const NEIGHBOR_OFFSETS: [(i32, i32); 4] = [(0,  1), (-1, 0), (1,  0), (0, -1)];
        NEIGHBOR_OFFSETS.iter().filter_map(move |&(dx, dy)| {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && ny >= 0 && nx < self.width as i32 && ny < self.height as i32 {
                Some((nx as u32, ny as u32))
            } else {
                None
            }
        })
    }

    fn generate_color(&self, x: u32, y: u32) -> Rgb<u8> {
        let neighbors = self.get_neighbors(x, y);
        let mut alive_neighbors: u16 = 0;

        let neighbors_color_sum: [u16; 3] = neighbors.fold([0, 0, 0], |acc: [u16; 3], (x, y)| {
            let color = self.img.get_pixel(x, y).0;
            // println!("({}, {}) -> {:?}", x, y, color);
            if self.visited_pixels[(x * self.width + y) as usize] {
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
        color.iter_mut().for_each(|value| *value = (*value as i16 + rng.gen_range(-self.randomness..=self.randomness)).min(255).max(0) as u8);
        Rgb(color)
    }
}

fn main() {
    for i in 1..=40 {
        Imaginery::new(4096, 4096, i, &format!("images/{}.png", i)).generate_image();
    }
}
