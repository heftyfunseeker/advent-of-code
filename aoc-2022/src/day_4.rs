use crate::utils;

struct ElfSectionAssigment {
    start_id: usize,
    end_id: usize,
}

impl From<&str> for ElfSectionAssigment {
    fn from(section_ids: &str) -> Self {
        let mut sections = section_ids.split('-');
        Self {
            start_id: sections.next().unwrap().parse().unwrap(),
            end_id: sections.next().unwrap().parse().unwrap(),
        }
    }
}

impl ElfSectionAssigment {
    fn overlaps(&self, other: &Self) -> bool {
        (self.start_id >= other.start_id && self.end_id <= other.end_id)
            || (other.start_id >= self.start_id && other.end_id <= self.end_id)
    }

    fn is_partial_overlap(&self, other: &Self) -> bool {
        (self.end_id >= other.start_id && self.end_id <= other.end_id)
            || (other.end_id >= self.start_id && other.end_id <= self.end_id)
    }
}

struct ElfAssigmentPair {
    elf_1: ElfSectionAssigment,
    elf_2: ElfSectionAssigment,
}

impl From<&String> for ElfAssigmentPair {
    fn from(pair: &String) -> Self {
        let mut sections = pair.split(',');
        Self {
            elf_1: sections.next().unwrap().into(),
            elf_2: sections.next().unwrap().into(),
        }
    }
}

impl ElfAssigmentPair {
    fn overlaps(&self) -> bool {
        self.elf_1.overlaps(&self.elf_2)
    }

    fn is_partial_overlap(&self) -> bool {
        self.elf_1.is_partial_overlap(&self.elf_2)
    }
}

fn iter_elf_assignments(lines: &Vec<String>) -> impl Iterator<Item = ElfAssigmentPair> + '_ {
    lines.iter().map(|line| line.into())
}

pub fn part_1() -> usize {
    iter_elf_assignments(&utils::read_lines("input/day-4.txt"))
        .fold(0, |num_complete_overlaps, pair| {
            num_complete_overlaps + pair.overlaps() as usize
        })
}

pub fn part_2() -> usize {
    iter_elf_assignments(&utils::read_lines("input/day-4.txt"))
        .fold(0, |num_partial_overlaps, pair| {
            num_partial_overlaps + pair.is_partial_overlap() as usize
        })
}
