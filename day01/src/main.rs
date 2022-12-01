const REAL_INPUT: &str = include_str!("real_input.txt");
#[allow(dead_code)]
const TEST_INPUT: &str = include_str!("test_input.txt");

fn get_calories_counts(input: &str) -> impl Iterator<Item = i32> + '_{
    input.split("\n\n").map(|slice| {
        slice.lines().fold(0, |acc, s| acc + s.parse::<i32>().unwrap())
    })
}

pub fn part1() -> String {
    part1_inner(REAL_INPUT)
}

fn part1_inner(input: &str) -> String {
    let counts = get_calories_counts(input);
    counts.max().unwrap().to_string()
}

#[test]
fn test_part1() {
    assert_eq!(
        "24000",
        part1_inner(TEST_INPUT)
    )
}

// ---

pub fn part2() -> String {
    part2_inner(REAL_INPUT)
}

fn part2_inner(input: &str) -> String {
    let mut counts = get_calories_counts(input).collect::<Vec<i32>>();
    counts.sort();
    counts.reverse();
    counts.iter().take(3).sum::<i32>().to_string()
}

#[test]
fn test_part2() {
    assert_eq!(
        "45000",
        part2_inner(TEST_INPUT)
    )
}

// ---

fn main() {
    println!("part1: {}", part1());
    println!("part2: {}", part2());
}
