pub mod day_1 {
    use crate::utils;

    pub fn part_1() -> usize {
        let lines = utils::read_lines("input/day-1-1.txt");
        lines
            .iter()
            .zip(lines.iter().skip(1))
            .fold(0, |acc, (prev, curr)| {
                let prev_num: usize = prev.parse().unwrap();
                let curr_num: usize = curr.parse().unwrap();
                if curr_num > prev_num {
                    acc + 1
                } else {
                    acc
                }
            })
    }

    pub fn part_2() -> usize {
        let lines = utils::read_lines("input/day-1-1.txt");
        lines
            .windows(3)
            .zip(lines[1..].windows(3))
            .fold(0, |acc, (prev_window, curr_window)| {
                let prev_sum: usize = prev_window
                    .iter()
                    .map(|l| l.parse::<usize>().unwrap())
                    .sum();
                let curr_sum: usize = curr_window
                    .iter()
                    .map(|l| l.parse::<usize>().unwrap())
                    .sum();
                if curr_sum > prev_sum {
                    acc + 1
                } else {
                    acc
                }
            })
    }
}
