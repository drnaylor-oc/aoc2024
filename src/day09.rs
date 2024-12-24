use std::collections::VecDeque;
use crate::util::Errors::NoImplementationError;
use crate::util::{load_from, Errors};
use crate::Day;

pub struct Day09 {}

impl Day for Day09 {
    fn part_1(&self) -> Result<String, Errors> {
        let data = load_from("day09a.txt")?;
        let mut initial = parse_map(&data);
        let compacted = compact(&mut initial);
        let checksum = checksum(&compacted);
        Ok(checksum.to_string())
    }

    fn part_2(&self) -> Result<String, Errors> {
        Err(NoImplementationError)
    }

    fn create_day() -> Box<dyn Day> where Self: Sized {
        Box::new(Day09 {})
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum DiskEntry {
    /// Used space with its ID and length
    Used { id: usize, length: u64 },
    /// Free space with length
    Free { length: u64 },
}

fn parse_map(entry: &str) -> VecDeque<DiskEntry> {
    let mut result = VecDeque::new();
    let mut next_id = 0usize;
    let mut is_free = false;
    let mut iter = entry.chars().flat_map(|c| c.to_digit(10).map(|x| x as u64));
    while let Some(digit) = iter.next() {
        if digit != 0 {
            if is_free {
                result.push_back(DiskEntry::Free { length: digit });
            } else {
                result.push_back(DiskEntry::Used { id: next_id, length: digit });
                next_id += 1;
            }
        }
        is_free = !is_free;
    }

    result
}

fn compact(initial: &mut VecDeque<DiskEntry>) -> Vec<DiskEntry> {
    let mut new_layout: Vec<DiskEntry> = Vec::new();
    let mut holding: Option<DiskEntry> = None;
    while let Some(entry) = initial.pop_front() {
        match entry {
            DiskEntry::Used { .. } => {
                new_layout.push(entry);
            }
            DiskEntry::Free { length } => {
                let mut remaining_length = length;
                while remaining_length > 0 {
                    let next = holding.take().or_else(|| select_next_used(initial));
                    if let Some(DiskEntry::Used { id, length }) = next {
                        if length <= remaining_length {
                            remaining_length -= length;
                            new_layout.push(DiskEntry::Used { id, length });
                        } else {
                            holding = Some(DiskEntry::Used {id, length: length - remaining_length});
                            new_layout.push(DiskEntry::Used { id, length: remaining_length });
                            remaining_length = 0;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }

    if let Some(DiskEntry::Used { id: i_id, length: i_length }) = holding {
        if let Some(&DiskEntry::Used { id: r_id, length: r_length }) = new_layout.last() {
            if i_id == r_id {
                new_layout.pop(); // basically overwriting
                new_layout.push(DiskEntry::Used { id: i_id, length: i_length + r_length });
            } else {
                new_layout.push(DiskEntry::Used { id: i_id, length: i_length });
            }
        }
    }

    new_layout
}

fn checksum(check: &Vec<DiskEntry>) -> u64 {
    let mut count: u64 = 0;
    let mut accumulator: u64 = 0;
    for entry in check {
        match *entry {
            DiskEntry::Used { id, length } => {
                for _ in 0..length {
                    accumulator += id as u64 * count;
                    count += 1;
                }
            },
            _ => panic!("Nope")
        }
    }
    accumulator
}

fn select_next_used(initial: &mut VecDeque<DiskEntry>) -> Option<DiskEntry> {
    while let Some(entry) = initial.pop_back() {
        if let DiskEntry::Used { .. } = entry {
            return Some(entry);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use crate::day09::DiskEntry::{Free, Used};
    use crate::day09::{checksum, compact, parse_map, DiskEntry};
    use lazy_static::lazy_static;

    const TEST_INPUT: &str = "2333133121414131402";

    // 00...111...2...333.44.5555.6666.777.888899
    lazy_static! {
        static ref TEST_MAP: VecDeque<DiskEntry> = VecDeque::from([
            Used { id: 0, length: 2 },
            Free { length: 3 },
            Used { id: 1, length: 3 },
            Free { length: 3 },
            Used { id: 2, length: 1 },
            Free { length: 3 },
            Used { id: 3, length: 3 },
            Free { length: 1 },
            Used { id: 4, length: 2 },
            Free { length: 1 },
            Used { id: 5, length: 4 },
            Free { length: 1 },
            Used { id: 6, length: 4 },
            Free { length: 1 },
            Used { id: 7, length: 3 },
            Free { length: 1 },
            Used { id: 8, length: 4 },
            Used { id: 9, length: 2 },
        ]);

        // 0099811188827773336446555566
        static ref COMPACTED_TEST_MAP: Vec<DiskEntry> = vec![
            Used { id: 0, length: 2 },
            Used { id: 9, length: 2 },
            Used { id: 8, length: 1 },
            Used { id: 1, length: 3 },
            Used { id: 8, length: 3 },
            Used { id: 2, length: 1 },
            Used { id: 7, length: 3 },
            Used { id: 3, length: 3 },
            Used { id: 6, length: 1 },
            Used { id: 4, length: 2 },
            Used { id: 6, length: 1 },
            Used { id: 5, length: 4 },
            Used { id: 6, length: 2 },
        ];
    }

    #[test]
    fn test_parse_map() {
        assert_eq!(parse_map(TEST_INPUT), *TEST_MAP);
    }

    #[test]
    fn test_compact() {
        let mut clone = TEST_MAP.clone();
        let compacted = compact(&mut clone);
        assert_eq!(compacted, *COMPACTED_TEST_MAP);
    }

    #[test]
    fn test_checksum() {
        assert_eq!(checksum(&*COMPACTED_TEST_MAP), 1928);
    }
}