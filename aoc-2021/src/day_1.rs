pub mod day_1 {
    use crate::utils;

    pub fn part_1() -> usize {
        let lines: Vec<usize> = utils::read_lines("input/day-1-1.txt")
            .iter()
            .map(|l| l.parse::<usize>().unwrap())
            .collect();

        lines.iter().zip(lines.iter().skip(1)).fold(
            0,
            |acc, (prev, curr)| {
                if curr > prev {
                    acc + 1
                } else {
                    acc
                }
            },
        )
    }

    pub fn part_2() -> usize {
        let lines: Vec<usize> = utils::read_lines("input/day-1-1.txt")
            .iter()
            .map(|l| l.parse::<usize>().unwrap())
            .collect();

        lines
            .windows(3)
            .zip(lines[1..].windows(3))
            .fold(0, |acc, (prev_window, curr_window)| {
                let prev_sum: usize = prev_window.iter().sum();
                let curr_sum: usize = curr_window.iter().sum();
                if curr_sum > prev_sum {
                    acc + 1
                } else {
                    acc
                }
            })
    }

    // reduces redundant sums using memoization and sliding window
    pub fn part_2_fast() -> usize {
        let lines: Vec<usize> = utils::read_lines("input/day-1-1.txt")
            .iter()
            .map(|l| l.parse::<usize>().unwrap())
            .collect();

        lines
            .windows(4)
            .fold((0, 0), |acc, window| {
                let prev_sum: usize;
                let curr_sum: usize;
                // initialize previous sum (only runs once)
                if acc.0 == 0 {
                    prev_sum = window[..=2].iter().sum();
                } else {
                    prev_sum = acc.0;
                }
                curr_sum = prev_sum - window[0] + window[3];

                if curr_sum > prev_sum {
                    (curr_sum, acc.1 + 1)
                } else {
                    (curr_sum, acc.1)
                }
            })
            .1
    }
}
