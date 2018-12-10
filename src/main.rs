#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate multimap;
extern crate regex;

use std::env;

mod day1;
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
        _ => {}
    }
}
