use std::fs;
use std::io;

use crate::days::{day1, day2, day3, day4, day5, day6, day7, day8};

pub mod days;

fn main() {
    println!("Enter the day please");
    let mut day_inp = String::new();
    io::stdin().read_line(&mut day_inp).expect("oh dear!");
    println!("Enter which part please");
    let mut part = String::new();
    io::stdin().read_line(&mut part).expect("aww nah");
    day_inp = day_inp.trim().to_string();
    let inp = fs::read_to_string("input/".to_owned() + &day_inp)
        .expect("couldn't find the file");

    let fun = match (day_inp.as_str(), part.trim()) {
        ("1", "1") => day1::part_1,
        ("1", "2") => day1::part_2,
        ("2", "1") => day2::part_1,
        ("2", "2") => day2::part_2,
        ("3", "1") => day3::part_1,
        ("3", "2") => day3::part_2,
        ("4", "1") => day4::part_1,
        ("4", "2") => day4::part_2,
        ("5", "1") => day5::part_1,
        ("5", "2") => day5::part_2,
        ("6", "1") => day6::part_1,
        ("6", "2") => day6::part_2,
        ("7", "1") => day7::part_1,
        ("7", "2") => day7::part_2,
        ("8", "1") => day8::part_1,
        ("8", "2") => day8::part_2,
        _ => panic!("invalid day")
    };
    println!("{}", fun(&inp))
}
