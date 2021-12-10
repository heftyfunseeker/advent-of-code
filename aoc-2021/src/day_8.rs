pub mod day_8 {
    use crate::utils;

    pub fn part_1() -> usize {
        utils::read_lines("input/day-8.txt")
            .iter()
            .fold([0; 10], |mut digits, entry| {
                entry
                    .split_whitespace()
                    .skip(11) // 10 patterns + | separator
                    .for_each(|digit| match digit.len() {
                        2 => digits[1] += 1,
                        4 => digits[4] += 1,
                        3 => digits[7] += 1,
                        7 => digits[8] += 1,
                        _ => {}
                    });
                digits
            })
            .iter()
            .sum()
    }
}
