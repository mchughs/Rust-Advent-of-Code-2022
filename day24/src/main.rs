const REAL_INPUT: &str = include_str!("real_input.txt");
#[allow(dead_code)]
const TEST_INPUT: &str = include_str!("test_input.txt");

pub fn part1() -> String {
    part1_inner(REAL_INPUT)
}

fn part1_inner(_input: &str) -> String {
    "TODO".to_owned()
}

#[test]
fn test_part1() {
    assert_eq!(
        "TODO",
        part1_inner(TEST_INPUT)
    )
}

// ---

pub fn part2() -> String {
    part2_inner(REAL_INPUT)
}

fn part2_inner(_input: &str) -> String {
    "TODO".to_owned()
}

#[test]
fn test_part2() {
    assert_eq!(
        "TODO",
        part2_inner(TEST_INPUT)
    )
}

// ---

fn main() {
    println!("part1: {}", part1());
    println!("part2: {}", part2());
}
