#![allow(dead_code)]

use std::env;

mod day1;
mod day2;
mod day3;
mod grid;
mod input;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {:?} puzzle filename", std::env::current_exe());
        return;
    }
    let puzzle = &args[1];
    let filename = &args[2];
    match puzzle.as_ref() {
        "1.1" => day1::run1(filename),
        "1.2" => day1::run2(filename),
        "2.1" => day2::run1(filename),
        "2.2" => day2::run2(filename),
        "3.1" => day3::run1(filename),
        _ => {}
    }
}
