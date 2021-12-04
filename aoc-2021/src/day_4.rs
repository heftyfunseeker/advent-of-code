pub mod day_4 {
    use crate::utils;
    use std::{borrow::BorrowMut, collections::hash_map::*};

    const BOARD_SIZE: u8 = 5;
    // row, col, value
    #[derive(Copy, Clone)]
    struct BoardPos(u8, u8, u8);

    struct Board {
        marked_positions: Vec<BoardPos>,
        board_sum: u16, // sum of all values on board
        rows_tally: [u8; BOARD_SIZE as usize],
        cols_tally: [u8; BOARD_SIZE as usize],
    }

    impl Board {
        fn new() -> Self {
            Board {
                marked_positions: vec![],
                board_sum: 0,
                rows_tally: [0; BOARD_SIZE as usize],
                cols_tally: [0; BOARD_SIZE as usize],
            }
        }
    }

    struct BoardLoc {
        board_index: usize,
        pos: BoardPos,
    }

    impl BoardLoc {
        fn new(board_index: usize, pos: BoardPos) -> Self {
            BoardLoc { board_index, pos }
        }
    }

    fn get_winning_score(bingo_board: &Board, last_call: u8) -> u16 {
        let mut score = bingo_board.board_sum;
        bingo_board
            .marked_positions
            .iter()
            .for_each(|mark| score -= mark.2 as u16);
        return score * last_call as u16;
    }

    pub fn part_1() -> u16 {
        let lines = utils::read_lines("input/day-4.txt");
        let mut line_reader = lines.iter();
        let draw: Vec<u8> = line_reader
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.parse::<u8>().unwrap())
            .collect();

        // build our boards and lut
        let mut board_locations: HashMap<u8, Vec<BoardLoc>> = HashMap::new();
        let mut boards = vec![];

        let mut row = 0;
        line_reader.for_each(|line| {
            if line == "" {
                assert!(row == 0);
                boards.push(Board::new());
            } else {
                let mut board = boards.pop().unwrap();
                line.split_whitespace()
                    .map(|n| n.parse::<u8>().unwrap())
                    .enumerate()
                    .for_each(|(col, n)| {
                        board.board_sum += n as u16;
                        let locations = board_locations.entry(n).or_insert_with(|| vec![]);
                        locations.push(BoardLoc::new(boards.len(), BoardPos(row, col as u8, n)))
                    });
                row = (row + 1) % BOARD_SIZE;
                boards.push(board);
            }
        });

        for draw_index in 0..draw.len() {
            let draw = draw[draw_index];
            if let Some(locations) = board_locations.get(&draw) {
                for loc_index in 0..locations.len() {
                    let loc = &locations[loc_index];
                    let board = &mut boards[loc.board_index];
                    board.marked_positions.push(loc.pos);
                    let row_index = loc.pos.0 as usize;
                    let col_index = loc.pos.1 as usize;
                    board.rows_tally[row_index] += 1;
                    board.cols_tally[col_index] += 1;

                    if board.rows_tally[row_index] == BOARD_SIZE
                        || board.cols_tally[col_index] == BOARD_SIZE
                    {
                        return get_winning_score(&board, draw);
                    }
                }
            }
        }
        0
    }
}
