const REAL_INPUT: &str = include_str!("real_input.txt");
#[allow(dead_code)]
const TEST_INPUT: &str = include_str!("test_input.txt");

pub fn part1() -> String {
    part1_inner(REAL_INPUT)
}

use itertools::Itertools;
use std::str::FromStr;

fn split_range(range: &str) -> (u32, u32) {
    range
        .splitn(2, '-')
        .map(|n| FromStr::from_str(n).unwrap())
        .collect_tuple()
        .unwrap()
}

fn fully_contains(ranges: &(u32, u32, u32, u32)) -> bool {
    let (start_a, end_a, start_b, end_b) = ranges;
    (start_a >= start_b && end_a <= end_b) || (start_a <= start_b && end_a >= end_b)
}

fn part1_inner(input: &str) -> String {
    input
        .lines()
        .map(|s| {
            let (range_a, range_b) = s.splitn(2, ',').collect_tuple().unwrap();
            let (start_a, end_a) = split_range(range_a);
            let (start_b, end_b) = split_range(range_b);
            return (start_a, end_a, start_b, end_b);
        })
        .filter(|x| fully_contains(x))
        .count()
        .to_string()
}

#[test]
fn test_part1() {
    assert_eq!("2", part1_inner(TEST_INPUT))
}

// ---

pub fn part2() -> String {
    part2_inner(REAL_INPUT)
}

fn partially_overlaps(ranges: &(u32, u32, u32, u32)) -> bool {
    let (start_a, end_a, start_b, end_b) = ranges;
    (end_a >= start_b && end_a <= end_b) || (start_a <= end_b && end_a >= end_b)
}

fn part2_inner(input: &str) -> String {
    input
        .lines()
        .map(|s| {
            let (range_a, range_b) = s.splitn(2, ',').collect_tuple().unwrap();
            let (start_a, end_a) = split_range(range_a);
            let (start_b, end_b) = split_range(range_b);
            return (start_a, end_a, start_b, end_b);
        })
        .filter(|x| fully_contains(x) || partially_overlaps(x))
        .count()
        .to_string()
}

#[test]
fn test_part2() {
    assert_eq!("4", part2_inner(TEST_INPUT))
}

// ---

fn main() {
    println!("part1: {}", part1());
    println!("part2: {}", part2());
}
