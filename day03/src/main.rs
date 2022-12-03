use std::collections::HashSet;

const REAL_INPUT: &str = include_str!("real_input.txt");
#[allow(dead_code)]
const TEST_INPUT: &str = include_str!("test_input.txt");

pub fn part1() -> String {
    part1_inner(REAL_INPUT)
}

fn part1_inner(input: &str) -> String {
    input
        .lines()
        .map(|rucksack: &str| {
            let size = rucksack.len();
            let (comp1, comp2) = rucksack.split_at(size / 2);
            let set1: HashSet<char> = comp1.chars().collect();
            let set2: HashSet<char> = comp2.chars().collect();
            let mut intersection = set1.intersection(&set2);
            intersection.nth(0).unwrap().clone()
        })
        .map(|c| {
            let offset: u32 = if c.is_uppercase() { 38 } else { 96 };
            c as u32 - offset
        })
        .sum::<u32>()
        .to_string()
}

#[test]
fn test_part1() {
    assert_eq!("157", part1_inner(TEST_INPUT))
}

// ---

pub fn part2() -> String {
    part2_inner(REAL_INPUT)
}

use itertools::Itertools;

fn part2_inner(input: &str) -> String {
    input
        .lines()
        .tuples()
        .map(|(s1, s2, s3)| {
            let set1: HashSet<char> = s1.chars().collect();
            let set2: HashSet<char> = s2.chars().collect();
            let set3: HashSet<char> = s3.chars().collect();
            let mut intersection = set1.iter().filter(|x| set2.contains(x) && set3.contains(x));
            intersection.nth(0).unwrap().clone()
        })
        .map(|c| {
            let offset: u32 = if c.is_uppercase() { 38 } else { 96 };
            c as u32 - offset
        })
        .sum::<u32>()
        .to_string()
}

#[test]
fn test_part2() {
    assert_eq!("70", part2_inner(TEST_INPUT))
}

// ---

fn main() {
    println!("part1: {}", part1());
    println!("part2: {}", part2());
}
