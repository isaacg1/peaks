extern crate rand;
use rand::{distributions::{normal::LogNormal, IndependentSample},
           thread_rng,
           Rng};

extern crate image;
use image::{ImageBuffer, Rgb};

use std::env::args;

fn add_color(c: f64, d: f64) -> f64 {
    (c + d).max(0.).min(255.)
}
fn draw_peaks(
    size: usize,
    num_peaks: usize,
    radius_center: f64,
    intensity: f64,
) -> Vec<Vec<(f64, f64, f64)>> {
    let mut board = vec![vec![(128., 128., 128.); size]; size];
    let dist = LogNormal::new(radius_center.ln(), 2.);
    for _ in 0..num_peaks {
        let row = thread_rng().gen_range(0, size);
        let col = thread_rng().gen_range(0, size);
        let offset = (
            thread_rng().gen_range(-intensity, intensity),
            thread_rng().gen_range(-intensity, intensity),
            thread_rng().gen_range(-intensity, intensity),
        );
        let radius = dist.ind_sample(&mut thread_rng()).min(size as f64);
        for dr in -(radius as isize)..radius as isize {
            for dc in -(radius as isize)..radius as isize {
                let new_row = row as isize + dr;
                let new_col = col as isize + dc;
                if new_row >= 0 && new_row < size as isize && new_col >= 0
                    && new_col < size as isize
                {
                    let new_row = new_row as usize;
                    let new_col = new_col as usize;
                    let multiplier = (-(dr * dr + dc * dc) as f64 * 3. / (radius * radius)).exp();
                    if multiplier > 0.04 {
                        let delta = (
                            offset.0 * multiplier,
                            offset.1 * multiplier,
                            offset.2 * multiplier,
                        );
                        let old_color = board[new_row][new_col];
                        board[new_row][new_col] = (
                            add_color(old_color.0, delta.0),
                            add_color(old_color.1, delta.1),
                            add_color(old_color.2, delta.2),
                        );
                    }
                }
            }
        }
    }
    board
}

fn main() {
    let size = args().nth(1).unwrap().parse().unwrap();
    let num_peaks = args().nth(2).unwrap().parse().unwrap();
    let radius_center = args().nth(3).unwrap().parse().unwrap();
    let intensity = args().nth(4).unwrap().parse().unwrap();
    let name = args().nth(5).unwrap();
    let result = draw_peaks(size, num_peaks, radius_center, intensity);

    let mut image = ImageBuffer::new(size as u32, size as u32);
    for i in 0..size {
        for j in 0..size {
            let color = result[i][j];
            let pixel = Rgb([color.0 as u8, color.1 as u8, color.2 as u8]);
            image.put_pixel(i as u32, j as u32, pixel);
        }
    }

    image.save(name).expect("Saved successfully");
}
