pub mod day_6 {
    use crate::utils;

    // use a rotating array to batch tick da fishes
    fn _simulate_lantern_spawns(fish_counts: &mut [usize; 9], days_to_simulate: usize) -> usize {
        for _day in 0..days_to_simulate {
            let new_spawns = fish_counts[0];
            fish_counts.rotate_left(1);
            fish_counts[6] += new_spawns;
        }
        fish_counts.iter().sum()
    }

    pub fn simulate_lantern_spawns() -> usize {
        let mut fish_counts: [usize; 9] = utils::read_lines("input/day-6.txt")
            .pop()
            .unwrap()
            .split(',')
            .map(|fish_timer| fish_timer.parse::<usize>().unwrap())
            .fold([0; 9], |mut fish_counts, fishy_timer| {
                fish_counts[fishy_timer] += 1;
                fish_counts
            });
        _simulate_lantern_spawns(&mut fish_counts, 256)
    }
}
