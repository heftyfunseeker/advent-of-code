pub mod day_5 {
    use crate::utils;

    #[derive(Clone, Copy)]
    struct Point(i32, i32);

    impl Point {
        pub fn new<'a, I>(mut point_tokens: I) -> Self
        where
            I: Iterator<Item = &'a str>,
        {
            Point {
                0: point_tokens.next().unwrap().parse::<i32>().unwrap(),
                1: point_tokens.next().unwrap().parse::<i32>().unwrap(),
            }
        }
    }

    const GRID_SIZE: i32 = 1000;
    struct Grid {
        points: Vec<i32>,
    }

    impl Grid {
        pub fn new() -> Self {
            Grid {
                // we 'could' use a hashmap to avoid wasted memory,
                // but ripping through a linear array might/probalby faster.
                // We also optimize for worst case where every grid point
                // is hit.
                points: vec![0; (GRID_SIZE * GRID_SIZE) as usize],
            }
        }

        pub fn add_line_segment(&mut self, p1: &Point, p2: &Point) {
            let (mut start_point, end_point) = if p1.0 < p2.0 {
                (p1.clone(), p2)
            } else {
                (p2.clone(), p1)
            };

            if p1.1 == p2.1 {
                let x0 = std::cmp::min(p1.0, p2.0);
                let x1 = std::cmp::max(p1.0, p2.0);
                for x in x0..=x1 {
                    let index = x + p1.1 * GRID_SIZE;
                    self.points[index as usize] += 1;
                }
            } else if p1.0 == p2.0 {
                let y0 = std::cmp::min(p1.1, p2.1);
                let y1 = std::cmp::max(p1.1, p2.1);
                for y in y0..=y1 {
                    let index = p1.0 + y * GRID_SIZE;
                    self.points[index as usize] += 1;
                }
            } else {
                let slope =
                    (start_point.1 - end_point.1) as f32 / (start_point.0 - end_point.0) as f32;
                if slope.abs() == 1.0 {
                    let step = slope as i32;
                    while start_point.0 <= end_point.0 {
                        let index = start_point.0 + start_point.1 * GRID_SIZE;
                        self.points[index as usize] += 1;
                        start_point.0 += 1;
                        start_point.1 += step;
                    }
                }
            }
        }

        pub fn num_overlapping_points(&self) -> i32 {
            self.points.iter().fold(0, |total_overlaps, &overlaps| {
                if overlaps > 1 {
                    total_overlaps + 1
                } else {
                    total_overlaps
                }
            })
        }
    }

    pub fn num_overlapping_points() -> i32 {
        let line_segments: Vec<(Point, Point)> = utils::read_lines("input/day-5.txt")
            .iter()
            .map(|line| {
                let mut tokens = line.split_whitespace();
                let point1 = tokens.next().unwrap().split(",");
                assert!(tokens.next().unwrap() == "->"); // eat point separator ->
                let point2 = tokens.next().unwrap().split(",");
                (Point::new(point1), Point::new(point2))
            })
            .collect();

        let mut grid = Grid::new();
        line_segments.iter().for_each(|segment| {
            grid.add_line_segment(&segment.0, &segment.1);
        });

        grid.num_overlapping_points()
    }
}
