use image::{self, ImageBuffer, Rgb};
use rand::{self, Rng};

const IMG_WIDTH: u32 = 200;
const IMG_HEIGHT: u32 = 200;
const CHANGE_FORCE: i16 = 0;

const BLANK: &Rgb<u8> = &Rgb([0, 0, 0]);

fn main() {
    let mut img = image::RgbImage::new(IMG_WIDTH, IMG_HEIGHT);

    let center = (IMG_WIDTH / 2, IMG_HEIGHT / 2);

    img.put_pixel(center.0, center.1, Rgb([255, 255, 255]));
    img = spread(img, center.0, center.1);

    img.save("image.png").unwrap();
}

fn spread(mut img: ImageBuffer<Rgb<u8>, Vec<u8>>, x: u32, y: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut stack: Vec<(u32, u32)> = Vec::new();
    stack.push((x, y));

    while let Some((x, y)) = stack.pop() {
        let mut neighbors: Vec<(u32, u32, Rgb<u8>)> = Vec::new();

        for &off in &[-1, 1] {
            let off_x = (x as i32) + off;
            if off_x < 0 || off_x >= img.width() as i32 {
                continue;
            }
            let off_x = off_x as u32;

            let pixel = img.get_pixel(off_x, y);
            neighbors.push((off_x, y, *pixel));
        }

        for &off in &[-1, 1] {
            let off_y = (y as i32) + off;
            if off_y < 0 || off_y >= img.height() as i32 {
                continue;
            }
            let off_y = off_y as u32;

            let pixel = img.get_pixel(x, off_y);
            neighbors.push((x, off_y, *pixel));
        }

        let color = img.get_pixel(x, y);
        if color == BLANK {
            let neighbors_color = neighbors.iter().fold((0, 0, 0), |acc, (_, _, color)| {
                (
                    acc.0 + color.0[0] as u16,
                    acc.1 + color.0[1] as u16,
                    acc.2 + color.0[2] as u16,
                )
            });

            let mut alive_neighbors: u8 = neighbors.iter().fold(0, |acc, (_, _, color)| {
                if color.0[0] + color.0[1] + color.0[2] == 0 {
                    acc
                } else {
                    acc + 1
                }
            });

            if alive_neighbors == 0 {
                alive_neighbors = 1
            }

            let mut color = [
                (neighbors_color.0 as u8) / alive_neighbors,
                (neighbors_color.1 as u8) / alive_neighbors,
                (neighbors_color.2 as u8) / alive_neighbors,
            ];

            let mut rng = rand::thread_rng();
            color.iter_mut().for_each(|value| {
                *value = (*value as i16 + rng.gen_range(-CHANGE_FORCE..=CHANGE_FORCE))
                    .min(255)
                    .max(0) as u8;
            });

            img.put_pixel(x, y, Rgb(color));
        }

        for neighbor in &neighbors {
            if &neighbor.2 == BLANK {
                stack.push((neighbor.0, neighbor.1));
            }
        }
    }

    img
}