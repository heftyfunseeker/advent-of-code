pub mod day_2 {
    use crate::utils;

    pub fn part_1() -> usize {
        // (horizontal, vertical)
        let coords = utils::read_lines("input/day-2.txt")
            .iter()
            .fold((0, 0), |c, l| {
                let mut tokens = l.split(' ');
                let dir = tokens.next().unwrap();
                let value = tokens.next().unwrap().parse::<usize>().unwrap();
                match (dir, value) {
                    ("forward", n) => (c.0 + n, c.1),
                    ("up", n) => (c.0, c.1 - n),
                    ("down", n) => (c.0, c.1 + n),
                    _ => panic!("where we headed?"),
                }
            });

        coords.0 * coords.1
    }

    pub fn part_2() -> usize {
        // (horizontal, vertical)
        let coords = utils::read_lines("input/day-2.txt")
            .iter()
            // horizontal, depth, aim
            .fold((0, 0, 0), |c, l| {
                let mut tokens = l.split(' ');
                let dir = tokens.next().unwrap();
                let value = tokens.next().unwrap().parse::<usize>().unwrap();
                match (dir, value) {
                    ("forward", n) => (c.0 + n, c.1 + c.2 * n, c.2),
                    ("up", n) => (c.0, c.1, c.2 - n),
                    ("down", n) => (c.0, c.1, c.2 + n),
                    _ => panic!("where we headed?"),
                }
            });

        coords.0 * coords.1
    }
}
