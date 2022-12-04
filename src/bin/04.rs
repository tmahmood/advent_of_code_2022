use std::ops::Range;
use advent_of_code_2022::read_from_file;

const REAL_INPUT: &str = "inputs/day_four/input";
const TEST_INPUT: &str = "inputs/day_four/test";

#[derive(PartialEq, Eq, Debug)]
pub enum OverlapState {
    Fully,
    Partial,
    NoOverlaps,
}

fn main() {
    let k1 = count_pairs_that_contains_overlaps(TEST_INPUT);
    let k2 = count_pairs_that_contains_overlaps(REAL_INPUT);
    println!("{} {}", k1, k2);
}

fn get_ranges(p1: &str, p2: &str) -> (Range<i32>, Range<i32>) {
    let (a1, a2) = p1.split_once("-").unwrap();
    let (s1_start, s1_end) = (a1.parse::<i32>().unwrap(), a2.parse::<i32>().unwrap());
    let (s1, s2) = p2.split_once("-").unwrap();
    let (s2_start, s2_end) = (s1.parse::<i32>().unwrap(), s2.parse::<i32>().unwrap());
    (s1_start..s1_end + 1, s2_start..s2_end + 1)
}

pub fn is_any_contained<'a>(p1: &'a str, p2: &'a str) -> OverlapState {
    let (r1, r2) = get_ranges(p1, p2);
    if r1.contains(&r2.start) && r1.contains(&(r2.end - 1)) {
        return OverlapState::Fully;
    } else if r2.contains(&r1.start) && r2.contains(&(r1.end - 1)) {
        return OverlapState::Fully;
    } else if r1.contains(&r2.start) || r1.contains(&(r2.end - 1)) || r2.contains(&r1.start) || r2.contains(&(r1.end -1)) {
        return OverlapState::Partial;
    }
    OverlapState::NoOverlaps
}

fn count_pairs_that_contains_one_fully(input_file: &str) -> i32 {
    let c = read_from_file(input_file);
    c.lines().into_iter().map(|line: &str| {
        let (r1, r2) = line.split_once(",").unwrap();
        match is_any_contained(r1, r2) {
            OverlapState::Fully => 1,
            _ => 0
        }
    }).collect::<Vec<i32>>().iter().sum::<i32>()
}

fn count_pairs_that_contains_overlaps(input_file: &str) -> i32 {
    let c = read_from_file(input_file);
    c.lines().into_iter().map(|line: &str| {
        let (r1, r2) = line.split_once(",").unwrap();
        match is_any_contained(r1, r2) {
            OverlapState::Partial => 1,
            OverlapState::Fully => 1,
            _ => 0
        }
    }).collect::<Vec<i32>>().iter().sum::<i32>()
}

#[cfg(test)]
mod tests {
    use crate::read_from_file;
    use super::*;

    #[test]
    fn fully_contained_in_one() {
        assert_eq!(is_any_contained("2-8", "3-7"), OverlapState::Fully);
        assert_eq!(is_any_contained("2-3", "4-5"), OverlapState::NoOverlaps);
        assert_eq!(is_any_contained("2-3", "3-5"), OverlapState::Partial);
    }

    #[test]
    fn how_many_pairs_contains_fully() {
        let k = count_pairs_that_contains_one_fully(TEST_INPUT);
        assert_eq!(2, k);
        let k = count_pairs_that_contains_one_fully(REAL_INPUT);
        assert_eq!(483, k);
    }

    #[test]
    fn how_many_pairs_contains_overlaps() {
        let k = count_pairs_that_contains_overlaps(TEST_INPUT);
        assert_eq!(4, k);
        let k = count_pairs_that_contains_overlaps(REAL_INPUT);
        assert_eq!(483, k);
    }
}