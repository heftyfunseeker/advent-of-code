pub mod day_10 {
    use crate::utils;
    use std::ops::ControlFlow;

    fn get_error_score(c: char) -> usize {
        match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("unexpected error character: {}", c),
        }
    }

    fn get_closing_chunk(c: char) -> char {
        match c {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => panic!("unexpected error character: {}", c),
        }
    }
    pub fn part_1() -> usize {
        let b: Vec<ControlFlow<usize>> = utils::read_lines("input/day-10.txt")
            .iter()
            .map(|line| {
                let mut open_chunks = vec![];
                line.chars().try_for_each(|c| match c {
                    '(' | '[' | '{' | '<' => {
                        open_chunks.push(c);
                        ControlFlow::Continue(())
                    }
                    ')' | ']' | '}' | '>' => {
                        if let Some(open_c) = open_chunks.pop() {
                            if get_closing_chunk(open_c) != c {
                                ControlFlow::Break(get_error_score(c))
                            } else {
                                ControlFlow::Continue(())
                            }
                        } else {
                            ControlFlow::Continue(())
                        }
                    }
                    _ => panic!("wat"),
                })
            })
            .collect();

        b.iter().fold(0, |acc, r| {
            if let ControlFlow::Break(x) = r {
                acc + x
            } else {
                acc
            }
        })
    }

    fn get_completion_score(c: char) -> usize {
        match c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => panic!("unexpected error character: {}", c),
        }
    }
    pub fn part_2() -> usize {
        let mut incomplete: Vec<usize> = utils::read_lines("input/day-10.txt")
            .iter()
            .filter_map(|line| {
                let mut open_chunks = vec![];
                let r = line.chars().try_for_each(|c| match c {
                    '(' | '[' | '{' | '<' => {
                        open_chunks.push(c);
                        ControlFlow::Continue(())
                    }
                    ')' | ']' | '}' | '>' => {
                        if let Some(open_c) = open_chunks.pop() {
                            if get_closing_chunk(open_c) != c {
                                ControlFlow::Break(get_error_score(c))
                            } else {
                                ControlFlow::Continue(())
                            }
                        } else {
                            ControlFlow::Continue(())
                        }
                    }
                    _ => panic!("wat"),
                });

                if r == ControlFlow::Continue(()) {
                    Some(
                        open_chunks
                            .iter()
                            .rev()
                            .fold(0, |acc, c| acc * 5 + get_completion_score(*c)),
                    )
                } else {
                    None
                }
            })
            .collect();

        incomplete.sort();
        let index = incomplete.len() / 2;
        incomplete[index]
    }
}
