// This nonsense is required to have benchmarking coexist while allowing
// the normal program to run on stable. The "unstable" feature is not a magic
// keyword, it's just a bogus feature defined in my Cargo.toml.
#![cfg_attr(feature = "unstable", feature(test))]

use std::env;
#[macro_use] extern crate lazy_static;

#[cfg(feature = "unstable")]
extern crate test;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].parse().expect("enter a number");
    match day {
        1 => day1::day1(),
        2 => day2::day2(),
        3 => day3::day3(),
        4 => day4::day4(),
        5 => day5::day5(),
        6 => day6::day6(),
        7 => day7::day7(),
        8 => day8::day8(),
        9 => day9::day9(),
        10 => day10::day10(),
        11 => day11::day11(),
        12 => day12::day12(),
        13 => day13::day13(),
        14 => day14::day14(),
        15 => day15::day15(),
        16 => day16::day16(),
        17 => day17::day17(),
        x => println!("unimplemented day {}", x),
    }
}