use std::cmp::Ordering;

use crate::utils;

#[derive(Eq)]
struct Elf {
    calories: usize,
}

impl Elf {
    fn new(calories: usize) -> Self {
        Self { calories }
    }
}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> Ordering {
        self.calories.cmp(&other.calories)
    }
}

impl PartialEq for Elf {
    fn eq(&self, other: &Self) -> bool {
        self.calories == other.calories
    }
}

fn get_elves() -> Vec<Elf> {
    let mut calories_for_this_elf = 0;

    utils::read_lines("input/day-1.txt")
        .iter()
        .filter_map(|line| match line.as_str() {
            "" => {
                let calories = calories_for_this_elf;
                calories_for_this_elf = 0;
                Some(Elf::new(calories))
            }
            _ => {
                calories_for_this_elf += line.parse::<usize>().unwrap();
                None
            }
        })
        .collect()
}

pub fn part_1() -> usize {
    get_elves().iter().max().unwrap().calories
}

pub fn part_2() -> usize {
    let mut elves = get_elves();
    // bubble our top 3 elves to the back
    for _ in 0..3 {
        for i in 0..elves.len() - 1 {
            if elves[i] > elves[i + 1] {
                elves.swap(i, i + 1)
            }
        }
    }
    elves.iter().rev().take(3).map(|e| e.calories).sum()
}
