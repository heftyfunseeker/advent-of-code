pub mod day_9 {
    use crate::utils;
    use std::collections::hash_set::HashSet;

    struct Point(u32, u32);

    const MAP_COLUMNS: u32 = 100;
    const MAP_ROWS: u32 = 100;

    struct HeightMap {
        map: Vec<u32>,
    }

    impl HeightMap {
        pub fn new(map: Vec<u32>) -> Self {
            HeightMap { map }
        }

        pub fn get_height(&self, x: u32, y: u32) -> u32 {
            self.map[self.get_height_index(x, y)]
        }

        pub fn get_height_index(&self, x: u32, y: u32) -> usize {
            (x + y * MAP_COLUMNS) as usize
        }

        pub fn is_low_point(&self, x: u32, y: u32) -> bool {
            let this_height = self.get_height(x, y);
            if x > 0 && this_height >= self.get_height(x - 1, y) {
                return false;
            } else if x < MAP_COLUMNS - 1 && this_height >= self.get_height(x + 1, y) {
                return false;
            } else if y > 0 && this_height >= self.get_height(x, y - 1) {
                return false;
            } else if y < MAP_ROWS - 1 && this_height >= self.get_height(x, y + 1) {
                return false;
            }
            return true;
        }
    }

    pub fn part_1() -> u32 {
        let height_map: Vec<u32> = utils::read_lines("input/day-9.txt")
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .flatten()
            .collect();

        let height_map = HeightMap::new(height_map);

        let mut low_points = vec![];
        for y in 0..MAP_ROWS {
            for x in 0..MAP_COLUMNS {
                if height_map.is_low_point(x, y) {
                    low_points.push(height_map.get_height(x, y));
                }
            }
        }

        low_points.iter().map(|p| p + 1).sum()
    }

    pub fn part_2() -> usize {
        let height_map: Vec<u32> = utils::read_lines("input/day-9.txt")
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .flatten()
            .collect();

        let height_map = HeightMap::new(height_map);

        let mut low_points = vec![];
        for y in 0..MAP_ROWS {
            for x in 0..MAP_COLUMNS {
                if height_map.is_low_point(x, y) {
                    low_points.push(Point(x, y));
                }
            }
        }

        let mut basin_sizes = low_points
            .iter()
            .map(|p| get_basin_size(&height_map, p))
            .collect::<Vec<usize>>();

        basin_sizes.sort();
        basin_sizes.reverse();
        basin_sizes
            .iter()
            .take(3)
            .fold(1, |total, size| total * size)
    }

    fn get_basin_size_helper(
        point: &Point,
        seen_heights: &mut HashSet<usize>,
        height_map: &HeightMap,
    ) {
        let x = point.0;
        let y = point.1;

        let this_index = height_map.get_height_index(x, y);
        if seen_heights.contains(&this_index) {
            return;
        }

        seen_heights.insert(this_index);
        if x > 0 && height_map.get_height(x - 1, y) != 9 {
            get_basin_size_helper(&Point(x - 1, y), seen_heights, height_map);
        }
        if x < MAP_COLUMNS - 1 && height_map.get_height(x + 1, y) != 9 {
            get_basin_size_helper(&Point(x + 1, y), seen_heights, height_map);
        }
        if y > 0 && height_map.get_height(x, y - 1) != 9 {
            get_basin_size_helper(&Point(x, y - 1), seen_heights, height_map);
        }
        if y < MAP_ROWS - 1 && height_map.get_height(x, y + 1) != 9 {
            get_basin_size_helper(&Point(x, y + 1), seen_heights, height_map);
        }
    }

    fn get_basin_size(height_map: &HeightMap, low_point: &Point) -> usize {
        let mut seen_heights = HashSet::new();
        get_basin_size_helper(low_point, &mut seen_heights, height_map);
        seen_heights.len()
    }
}
