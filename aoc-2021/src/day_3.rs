pub mod day_3 {
    use crate::utils;

    const NUM_BITS: usize = 12;
    fn get_words() -> Vec<u32> {
        utils::read_lines("input/day-3.txt")
            .iter()
            .map(|l| u32::from_str_radix(l, 2).unwrap())
            .collect()
    }

    fn get_bit_counts(words: &Vec<u32>) -> [u32; NUM_BITS] {
        let mut bit_counts: [u32; NUM_BITS] = [0; NUM_BITS];
        words.iter().for_each(|word| {
            for i in 0..NUM_BITS {
                bit_counts[i] += u32::from((word & (1 << (NUM_BITS - 1 - i))) != 0)
            }
        });
        return bit_counts;
    }

    fn get_gamma(bit_counts: [u32; NUM_BITS], total_words: u32) -> u32 {
        let count_threshold = total_words / 2;
        let mut gamma: u32 = 0;
        bit_counts
            .into_iter()
            .enumerate()
            .for_each(|(bit_index, bit_count)| {
                gamma |= u32::from(bit_count >= count_threshold) << (NUM_BITS - 1) - bit_index;
            });
        return gamma;
    }

    fn get_epsilon(gamma: u32) -> u32 {
        !gamma & 0xFFF
    }

    pub fn part_1() -> u32 {
        let words = get_words();
        let bit_counts = get_bit_counts(&words);
        let gamma = get_gamma(bit_counts, words.len() as u32);
        let epsilon = get_epsilon(gamma);
        gamma * epsilon
    }

    pub fn apply_filter_mask(words: Vec<u32>, mask: u32, bit_index: usize) -> Vec<u32> {
        words
            .iter()
            .cloned()
            .filter(|w| {
                let mask_bit = mask & 1 << bit_index;
                let word_bit = w & 1 << bit_index;
                word_bit == mask_bit
            })
            .collect()
    }

    pub fn part_2() -> u32 {
        let mut o2_gen_rating = get_words();
        let mut co2_scrubber_rating = o2_gen_rating.clone();

        for bit_index in (0..NUM_BITS).rev() {
            if o2_gen_rating.len() == 1 {
                break;
            }
            let bit_counts = get_bit_counts(&o2_gen_rating);
            let gamma = get_gamma(bit_counts, o2_gen_rating.len() as u32);
            o2_gen_rating = apply_filter_mask(o2_gen_rating, gamma, bit_index);
        }

        for bit_index in (0..NUM_BITS).rev() {
            if co2_scrubber_rating.len() == 1 {
                break;
            }
            let bit_counts = get_bit_counts(&co2_scrubber_rating);
            let gamma = get_gamma(bit_counts, co2_scrubber_rating.len() as u32);
            let epsilon = get_epsilon(gamma);
            co2_scrubber_rating = apply_filter_mask(co2_scrubber_rating, epsilon, bit_index);
        }

        o2_gen_rating.pop().unwrap() * co2_scrubber_rating.pop().unwrap()
    }
}
