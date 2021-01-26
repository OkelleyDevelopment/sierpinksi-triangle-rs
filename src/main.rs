//! Sierpinski Triangle Generator in Rust
//!
//! Author: Nicholas O'Kelley
//! Date finished: Jan 26, 2021
extern crate image;
extern crate rand;

use rand::{thread_rng, Rng};
use std::io;
use std::process;

/// A point struct used to build out the triangles
pub struct Point {
    x: u32,
    y: u32,
}

/// Width of the generated image
const WIDTH: u32 = 800;
/// Height of the generated image
const HEIGHT: u32 = 600;

/// This will begin the construction of a Sierpinski Triangle using the
/// Chaos Method found on the the wiki page (linked in the readme)
pub fn main() {
    println!("Welcome to the Sierpinksi Triangle\nEnter the output file pathname> ");
    let out_file: String = String::from("./images/") + &get_input();

    // Following the image-rs readme, this will create our canvas
    let mut img = image::ImageBuffer::from_fn(WIDTH, HEIGHT, |x, y| {
        if x == 0 && y == 0 {
            image::Luma([0u8])
        } else {
            image::Luma([255u8])
        }
    });

    // a reuseable thread_rng variable
    let mut rng = thread_rng();

    // The number of iterations
    let mut iterations: usize = 100000;

    // The array of points forming the largest triangle
    let pts: [Point; 3] = [
        Point { x: WIDTH / 2, y: 0 },
        Point { x: 0, y: HEIGHT },
        Point {
            x: WIDTH,
            y: HEIGHT,
        },
    ];

    // A random starting point
    let mut random_point = Point {
        x: rng.gen_range(0..WIDTH),
        y: rng.gen_range(0..HEIGHT),
    };

    // Sourcing of the starting pixel
    let pixel = img[(0, 0)];

    while iterations > 0 {
        iterations = iterations - 1;
        let num = rng.gen_range(0..3);
        random_point.x = (random_point.x + pts[num].x) / 2;
        random_point.y = (random_point.y + pts[num].y) / 2;
        img.put_pixel(random_point.x, random_point.y, pixel);
    }
    println!("Saving image... ");
    img.save(out_file).unwrap();
    println!("Done.");
    process::exit(0);
}

fn get_input() -> String {
    let mut input = String::new();

    while input == String::new() {
        io::stdin()
            .read_line(&mut input)
            .expect("Error with reading input");
    }
    return input.trim().to_string();
}
