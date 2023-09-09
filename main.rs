// Random Color Cat
// By: Sina Tashakkori, QVLx Labs

use rand::Rng;
use std::io::{self, Read};

fn main() {
    // Read input from standard input (stdin)
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    print_colorful(input);
}

fn ansi_rgb_string(s: String, r: u32, g: u32, b: u32) -> String {
    if r > 6 || g > 6 || b > 6 {
        return s;
    }
    let mut ansi_code = String::from("\x1b[48;5;");
    let color = 16 + (r * 36) + (g * 6) + b;
    ansi_code.push_str(&color.to_string());
    ansi_code.push_str("m");
    ansi_code.push_str(&s);
    ansi_code.push_str("\x1b[0m");
    ansi_code
}

fn print_colorful(s: String) {
    let ansi_code = String::from("\x1b[38;5;");
    let mut colored_string = String::new();
    let raw = s.chars();
    for (i, character) in raw.enumerate() {
        let r = rand::thread_rng().gen_range(1..6);
        let g = rand::thread_rng().gen_range(1..6);
        let b = rand::thread_rng().gen_range(1..6);
        let non_system = 16 + (r * 36) + (g * 6) + b;
        let system_color = rand::thread_rng().gen_range(1..9);
        let shuffler = rand::thread_rng().gen_range(0..7);
        let mut code = non_system;
        if shuffler > 5 {
            code = system_color;
        }
        colored_string.push_str(&ansi_code);
        colored_string.push_str(&code.to_string());
        colored_string.push_str("m");
        colored_string.push(character);
        colored_string.push_str("\x1b[0m");
    }
    println!("{}", colored_string);
}
