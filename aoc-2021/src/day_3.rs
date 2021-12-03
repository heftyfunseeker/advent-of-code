pub mod day_3 {
    use crate::utils;

    pub fn part_1() -> u32 {
        const NUM_BITS: usize = 12;
        let mut one_bit_counts: [usize; NUM_BITS] = [0; NUM_BITS];

        // I feel like I should just be using a xor bitwise op or something?
        let lines = utils::read_lines("input/day-3.txt");
        lines.iter().for_each(|l| {
            l.chars().enumerate().for_each(|(bit_index, bit)| {
                one_bit_counts[bit_index] += bit.to_digit(10).unwrap() as usize
            })
        });

        let count_threshold = lines.len() / 2;
        let mut gamma: u32 = 0;
        one_bit_counts
            .into_iter()
            .enumerate()
            .for_each(|(bit_index, bit_count)| {
                let bit = if bit_count > count_threshold { 1 } else { 0 };
                gamma |= bit << (NUM_BITS - 1) - bit_index;
            });

        let epsilon = !gamma & 0xFFF;
        gamma * epsilon
    }
}
