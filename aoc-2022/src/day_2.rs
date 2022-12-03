use crate::utils;

enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

pub fn part_1() -> usize {
    utils::read_lines("input/day-2.txt")
        .iter()
        .fold(0, |score, line| {
            match line.as_str() {
                // Rock
                "A X" => score + Shape::Rock as usize + 3,
                "A Y" => score + Shape::Paper as usize + 6,
                "A Z" => score + Shape::Scissors as usize,

                // Paper
                "B X" => score + Shape::Rock as usize,
                "B Y" => score + Shape::Paper as usize + 3,
                "B Z" => score + Shape::Scissors as usize + 6,

                // Scissors
                "C X" => score + Shape::Rock as usize + 6,
                "C Y" => score + Shape::Paper as usize,
                "C Z" => score + Shape::Scissors as usize + 3,
                _ => panic!("unrecognized strategy")
            }
        })
}

pub fn part_2() -> usize {
    utils::read_lines("input/day-2.txt")
        .iter()
        .fold(0, |score, line| {
            match line.as_str() {
                "A X" => score + Shape::Scissors as usize,
                "A Y" => score + Shape::Rock as usize + 3,
                "A Z" => score + Shape::Paper as usize + 6,

                "B X" => score + Shape::Rock as usize,
                "B Y" => score + Shape::Paper as usize + 3,
                "B Z" => score + Shape::Scissors as usize + 6,

                "C X" => score + Shape::Paper as usize,
                "C Y" => score + Shape::Scissors as usize + 3,
                "C Z" => score + Shape::Rock as usize + 6,
                _ => panic!("unrecognized strategy")
            }
        })
}
