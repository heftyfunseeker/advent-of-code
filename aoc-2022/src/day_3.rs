use crate::utils;
use std::collections::HashSet;
use std::collections::HashMap;

fn priority(c: char) -> usize {
    match c.is_lowercase() {
        true => c as usize - 'a' as usize + 1,
        false => c as usize - 'A' as usize + 27,
    }
}

pub fn part_1() -> usize {
    utils::read_lines("input/day-3.txt")
        .iter()
        .fold(0, |priority_sum, line| {
            let sack_size = line.len() / 2;
            let items_seen: HashSet<char> = HashSet::from_iter(line.chars().take(sack_size));

            line.chars()
                .skip(sack_size)
                .filter_map(|c| match items_seen.contains(&c) {
                    true => Some(priority(c)),
                    false => None,
                })
                .take(1)
                .sum::<usize>()
                + priority_sum
        })
}

pub fn part_2() -> usize {
    utils::read_lines("input/day-3.txt")
        .chunks(3)
        .fold(0, |priority_sum, group| {
            let mut items_seen = HashMap::new();
            let mut mask = 0;
            for sack in group {
                sack.chars().for_each(|c| {
                    let item_count = items_seen.entry(c).or_insert(0);
                    *item_count |= 1 << mask;
                });
                mask += 1;
            }

            const BADGE_MASK: usize = 7;

            items_seen.iter().filter_map(|(item_type, item_count)| match item_count & BADGE_MASK {
                BADGE_MASK => Some(priority(*item_type)),
                _ => None
            }).sum::<usize>() + priority_sum
        })
}
