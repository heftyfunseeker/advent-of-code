pub mod day_8 {
    use crate::utils;

    // len -> digits
    // 2 => 1
    // 4 => 4
    // 3 => 7
    // 5 => 2, 3, 5,
    // 6 => 0, 6, 9
    // 7 => 8

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

    use std::collections::HashSet;
    pub fn part_2() -> usize {
        let mut digit_to_signal = vec![HashSet::new(); 10];

        utils::read_lines("input/day-8.txt")
            .iter()
            .fold(0, |sum, entry| {
                entry
                    .split_whitespace()
                    .take(10)
                    .for_each(|digit_str| match digit_str.len() {
                        2 => digit_to_signal[1] = digit_str.chars().collect(),
                        4 => digit_to_signal[4] = digit_str.chars().collect(),
                        3 => digit_to_signal[7] = digit_str.chars().collect(),
                        7 => digit_to_signal[8] = digit_str.chars().collect(),
                        _ => {}
                    });

                entry.split_whitespace().take(10).for_each(|digit_str| {
                    let signal = digit_str.chars().collect();
                    let one = &digit_to_signal[1];

                    let index: i32 = match digit_str.len() {
                        6 => {
                            let mut four_u_seven = digit_to_signal[4].clone();
                            four_u_seven.extend(&digit_to_signal[7]);

                            if one.is_subset(&signal) == false {
                                6
                            } else if four_u_seven.is_subset(&signal) {
                                9
                            } else {
                                0
                            }
                        }
                        5 => {
                            let mut two_u_four = digit_to_signal[4].clone();
                            two_u_four.extend(&signal);
                            if one.is_subset(&signal) {
                                3
                            } else if two_u_four == digit_to_signal[8] {
                                2
                            } else {
                                5
                            }
                        }
                        _ => -1,
                    };

                    if index != -1 {
                        digit_to_signal[index as usize] = signal;
                    }
                });

                sum + entry.split_whitespace().skip(11).fold(0, |sum, digit_str| {
                    let signal = digit_str.chars().collect::<HashSet<char>>();
                    let digit = digit_to_signal.iter().position(|d| *d == signal).unwrap();
                    sum * 10 + digit
                })
            })
    }
}
