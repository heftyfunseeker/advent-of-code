pub mod day_3 {
    use crate::utils;

    const NUM_BITS: usize = 12;
    fn get_bit_counts_and_words() -> ([u32; NUM_BITS], Vec<u32>){
        let mut one_bit_counts: [u32; NUM_BITS] = [0; NUM_BITS];

        let words: Vec<u32> = utils::read_lines("input/day-3.txt")
            .iter()
            .map(|l| u32::from_str_radix(l, 2).unwrap())
            .collect();

        words.iter().for_each(|word| {
            for i in 0..NUM_BITS {
                one_bit_counts[i] += u32::from((word & (1 << (NUM_BITS - 1 - i))) != 0)
            }
        });
        return (one_bit_counts, words);
    }

    fn get_gamma(bit_counts: [u32; NUM_BITS], total_words: u32) -> u32 {
        let count_threshold = total_words / 2;
        let mut gamma: u32 = 0;
        bit_counts
            .into_iter()
            .enumerate()
            .for_each(|(bit_index, bit_count)| {
                let bit = if bit_count > count_threshold { 1 } else { 0 };
                gamma |= bit << (NUM_BITS - 1) - bit_index;
            });
        return gamma;
    }

    fn get_epsilon(gamma: u32) -> u32 {
        !gamma & 0xFFF
    }

    pub fn part_1() -> u32 {
        let (bit_counts, words) = get_bit_counts_and_words();
        let gamma = get_gamma(bit_counts, words.len() as u32);
        let epsilon = get_epsilon(gamma);
        gamma * epsilon
    }

    pub fn part_2() {
    }
}
