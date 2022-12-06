const REAL_INPUT: &str = include_str!("real_input.txt");
#[allow(dead_code)]
const TEST_INPUT: &str = include_str!("test_input.txt");

use std::collections::HashSet;

fn all_distinct(marker: &[u8]) -> bool {
    let mut seen: HashSet<u8> = HashSet::new();
    for b in marker {
        match seen.get(b) {
            Some(_b) => return false,
            None => seen.insert(*b),
        };
    }
    return true;
}

pub fn part1() -> String {
    part1_inner(REAL_INPUT)
}

fn part1_inner(input: &str) -> String {
    const MARKER_SIZE: u32 = 4;
    let mut acc: u32 = MARKER_SIZE;
    for marker in input.as_bytes().windows(MARKER_SIZE as usize) {
        if all_distinct(marker) {
            break;
        }
        acc += 1;
    }
    acc.to_string()
}

#[test]
fn test_part1() {
    assert_eq!("7", part1_inner(TEST_INPUT))
}

// ---

pub fn part2() -> String {
    part2_inner(REAL_INPUT)
}

fn part2_inner(input: &str) -> String {
    const MARKER_SIZE: u32 = 14;
    let mut acc: u32 = MARKER_SIZE;
    for marker in input.as_bytes().windows(MARKER_SIZE as usize) {
        if all_distinct(marker) {
            break;
        }
        acc += 1;
    }
    acc.to_string()
}

#[test]
fn test_part2() {
    assert_eq!("19", part2_inner(TEST_INPUT))
}

// ---

fn main() {
    println!("part1: {}", part1());
    println!("part2: {}", part2());
}
