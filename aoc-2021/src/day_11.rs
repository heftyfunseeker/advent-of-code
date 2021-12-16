pub mod day_11 {
    use crate::utils;

    // 10x10
    const WIDTH: usize = 10;
    const HEIGHT: usize = 10;

    // returns num flashes
    fn step_octos(octos: &mut Vec<u32>) -> usize {
        let mut num_flashes = 0;
        // increase energy by 1
        octos.iter_mut().for_each(|o| {
            *o += 1;
        });

        // get seed flashes
        let mut new_flashes = Vec::new();
        octos.iter_mut().enumerate().for_each(|(index, o)| {
            if *o > 9 {
                new_flashes.push(index);
                // mark flashed
                *o = u32::MAX;
            }
        });

        while new_flashes.len() > 0 {
            num_flashes += new_flashes.len();

            while new_flashes.len() > 0 {
                let octo_index = new_flashes.pop().unwrap();

                // mark flashed
                octos[octo_index] = u32::MAX;

                let x = octo_index % WIDTH;
                let y = octo_index / WIDTH;

                if x > 0 {
                    let left = x - 1 + y * WIDTH;
                    if octos[left] != u32::MAX {
                        octos[left] += 1;
                    }
                    if y > 0 {
                        let top_left = x - 1 + (y - 1) * WIDTH;
                        if octos[top_left] != u32::MAX {
                            octos[top_left] += 1;
                        }
                    }
                    if y < HEIGHT - 1 {
                        let bottom_left = x - 1 + (y + 1) * WIDTH;
                        if octos[bottom_left] != u32::MAX {
                            octos[bottom_left] += 1;
                        }
                    }
                }
                if x < WIDTH - 1 {
                    let right = x + 1 + y * WIDTH;
                    if octos[right] != u32::MAX {
                        octos[right] += 1;
                    }
                    if y > 0 {
                        let top_right = x + 1 + (y - 1) * WIDTH;
                        if octos[top_right] != u32::MAX {
                            octos[top_right] += 1;
                        }
                    }
                    if y < HEIGHT - 1 {
                        let bottom_right = x + 1 + (y + 1) * WIDTH;
                        if octos[bottom_right] != u32::MAX {
                            octos[bottom_right] += 1;
                        }
                    }
                }
                if y > 0 {
                    let top = x + (y - 1) * WIDTH;
                    if octos[top] != u32::MAX {
                        octos[top] += 1;
                    }
                }
                if y < HEIGHT - 1 {
                    let bottom = x + (y + 1) * WIDTH;
                    if octos[bottom] != u32::MAX {
                        octos[bottom] += 1;
                    }
                }
            }

            octos.iter_mut().enumerate().for_each(|(index, o)| {
                if *o != u32::MAX && *o > 9 {
                    new_flashes.push(index);
                    // mark flashed
                    *o = u32::MAX;
                }
            });
        }

        octos.iter_mut().for_each(|o| {
            if *o == u32::MAX {
                *o = 0;
            }
        });

        return num_flashes;
    }

    pub fn part_1() -> usize {
        let mut octos = utils::read_lines("input/day-11.txt")
            .iter()
            .flat_map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<u32>>();

        let mut flashes = 0;
        for _ in 0..100 {
            flashes += step_octos(&mut octos);
        }
        flashes
    }

    pub fn part_2() -> usize {
        let mut octos = utils::read_lines("input/day-11.txt")
            .iter()
            .flat_map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<u32>>();

        let mut step = 0;
        while step_octos(&mut octos) != octos.len() {
            step += 1;
        }
        step
    }
}
