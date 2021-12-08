pub mod day_6 {
    use crate::utils;

    const INIT_SPAWN_DELAY: u32 = 8;
    const SPAWN_RESET_DELAY: u32 = 6;

    struct LanternFish(u32);

    impl LanternFish {
        pub fn new() -> Self {
            LanternFish(INIT_SPAWN_DELAY)
        }

        pub fn tick(&mut self) -> Option<LanternFish> {
            if self.0 == 0 {
                self.0 = SPAWN_RESET_DELAY;
                Some(LanternFish::new())
            } else {
                self.0 -= 1;
                None
            }
        }
    }

    fn simulate_lantern_spawns(fish: &mut Vec<LanternFish>) -> usize {
        const TOTAL_SIM_DAYS: usize = 80;
        for _day in 0..TOTAL_SIM_DAYS {
            let mut spawns = vec![];

            fish.iter_mut().for_each(|fishy| {
                if let Some(new_fish) = fishy.tick() {
                    spawns.push(new_fish);
                }
            });
            fish.append(&mut spawns);
        }
        fish.len()
    }

    pub fn part_1() -> usize {
        let mut fish: Vec<LanternFish> = utils::read_lines("input/day-6.txt")
            .pop()
            .unwrap()
            .split(',')
            .map(|fish_timer| LanternFish(fish_timer.parse::<u32>().unwrap()))
            .collect();
        simulate_lantern_spawns(&mut fish)
    }
}
