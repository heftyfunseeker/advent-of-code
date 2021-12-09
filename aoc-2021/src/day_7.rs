pub mod day_7 {
    use crate::utils;

    pub fn part_1() -> i64 {
        let mut crab_positions: Vec<i64> = utils::read_lines("input/day-7.txt")
            .pop()
            .unwrap()
            .split(',')
            .map(|pos| pos.parse::<i64>().unwrap())
            .collect();

        crab_positions.sort();

        let median = if crab_positions.len() % 2 == 0 {
            let median_index = crab_positions.len() / 2;
            (crab_positions[median_index] + crab_positions[median_index - 1]) / 2
        } else {
            crab_positions[crab_positions.len() / 2]
        };

        crab_positions.iter().fold(0, |fuel_cost, crab_pos| {
            fuel_cost + (median - crab_pos).abs()
        })
    }

    pub fn part_2() -> i64 {
        let crab_positions: Vec<i64> = utils::read_lines("input/day-7.txt")
            .pop()
            .unwrap()
            .split(',')
            .map(|pos| pos.parse::<i64>().unwrap())
            .collect();

        let sum: i64 = crab_positions.iter().sum();
        let len = crab_positions.len() as f64;
        let average = (sum as f64 / len).floor();

        crab_positions.iter().fold(0, |fuel_cost, crab_pos| {
            let diff = (average as i64 - crab_pos).abs();
            let mut this_cost = 0;
            for fuel in 0..diff {
                this_cost += fuel + 1;
            }
            fuel_cost + this_cost
        })
    }
}
