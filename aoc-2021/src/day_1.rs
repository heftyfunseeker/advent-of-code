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
            .windows(4)
            .fold(0, |acc, window| {
                if window[3] > window[0] {
                    acc + 1
                }
                else {
                    acc
                }
            })
    }
}
