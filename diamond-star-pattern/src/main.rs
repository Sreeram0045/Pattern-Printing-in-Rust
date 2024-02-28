// In order to print a diamond pattern you need to combine both the star pyramid programs

use std::io;

fn straight_pyramid(h: i32) {
    let height = h;
    for i in 0..height {
        for _ in (0..height - i - 1).rev() {
            print!(" ");
        }
        for _ in 0..(i * 2 + 1) {
            print!("*");
        }
        println!();
    }
}

fn inverted_pyramid(h: i32) {
    let height = h;
    for i in 0..height {
        for _ in 0..i {
            print!(" ");
        }
        for _ in 0..(2 * height - 2 * i - 1) {
            print!("*");
        }
        println!();
    }
}

fn main() {
    let mut input_string = String::new();
    print!("Enter the height of the pyramid: ");
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to fetch input");
    
    let input: i32 = input_string.trim().parse().expect("Failed to parse");
    
    straight_pyramid(input);

    inverted_pyramid(input);
}
