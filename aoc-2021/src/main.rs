#![allow(dead_code)]

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod utils;

// * all four-digit displays use same connections
// so we really just want to look at the number of segments turned on initially
// * we use the 10 signal patterns of each entry to decode the 4 digit number
//
fn main() {
    print!("answer: {}", day_8::day_8::part_2());
}
