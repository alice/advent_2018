#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate cpuprofiler;
extern crate multimap;
extern crate regex;

use std::env;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod grid;
mod input;
mod multi_grid;

fn main() {
    let args: Vec<String> = env::args().collect();
    let puzzle = &args[1];
    let mut filename = &"".to_string();
    if args.len() > 2 {
        filename = &args[2];
    }
    match puzzle.as_ref() {
        "1.1" => day1::run1(filename),
        "1.2" => day1::run2(filename),
        "2.1" => day2::run1(filename),
        "2.2" => day2::run2(filename),
        "3.1" => day3::run1(filename),
        "4.1" => day4::run(filename),
        "4.2" => day4::run(filename),
        "5.1" => day5::run1(filename),
        "5.2" => day5::run2(filename),
        "6.1" => day6::run1(filename),
        "6.2" => day6::run2(filename),
        "7.1" => day7::run1(filename),
        "7.2" => day7::run2(filename),
        "8.1" => day8::run1(filename),
        "8.2" => day8::run2(filename),
        "9.1" => day9::run1(),
        "9.2" => day9::run2(),
        "10.1" => day10::run(filename),
        "10.2" => day10::run(filename),
        "11.1" => day11::run1(),
        "11.2" => day11::run2(),
        "12.1" => day12::run1(filename),
        "12.2" => day12::run2(filename),
        "13.1" => day13::run1(filename),
        "13.2" => day13::run2(filename),
        "14.1" => day14::run1(),
        "14.2" => day14::run2(),
        _ => {}
    }
}
