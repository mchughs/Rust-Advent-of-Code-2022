const REAL_INPUT: &str = include_str!("real_input.txt");
#[allow(dead_code)]
const TEST_INPUT: &str = include_str!("test_input.txt");

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

fn outcome(opponent_sym: char, your_sym: char) -> Outcome {
    match (opponent_sym, your_sym) {
        ('A', 'X') => Outcome::Draw,
        ('B', 'Y') => Outcome::Draw,
        ('C', 'Z') => Outcome::Draw,
        ('C', 'X') => Outcome::Win,
        ('A', 'Y') => Outcome::Win,
        ('B', 'Z') => Outcome::Win,
        _ => Outcome::Loss,
    }
}

fn score(opponent_sym: char, your_sym: char) -> u32 {
    let outcome: Outcome = outcome(opponent_sym, your_sym);
    let shape_points: u32 = match your_sym {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => 0,
    };
    outcome as u32 + shape_points
}

pub fn part1() -> String {
    part1_inner(REAL_INPUT)
}

fn part1_inner(input: &str) -> String {
    input
        .lines()
        .map(|s| {
            let opponent_sym = s.chars().nth(0).unwrap();
            let your_sym = s.chars().nth(2).unwrap();
            return score(opponent_sym, your_sym);
        })
        .sum::<u32>()
        .to_string()
}

#[test]
fn test_part1() {
    assert_eq!("15", part1_inner(TEST_INPUT))
}

// ---

fn shape_points(opponent_sym: char, your_outcome: Outcome) -> u32 {
    match (opponent_sym, your_outcome) {
        ('A', Outcome::Draw) => 1,
        ('B', Outcome::Draw) => 2,
        ('C', Outcome::Draw) => 3,
        ('C', Outcome::Win) => 1,
        ('A', Outcome::Win) => 2,
        ('B', Outcome::Win) => 3,
        ('C', Outcome::Loss) => 2,
        ('A', Outcome::Loss) => 3,
        ('B', Outcome::Loss) => 1,
        _ => 0,
    }
}

fn score_2(opponent_sym: char, your_sym: char) -> u32 {
    let outcome: Outcome = match your_sym {
        'X' => Outcome::Loss,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        _ => Outcome::Draw
    };
    outcome as u32 + shape_points(opponent_sym, outcome)
}

pub fn part2() -> String {
    part2_inner(REAL_INPUT)
}

fn part2_inner(input: &str) -> String {
    input
    .lines()
    .map(|s| {
        let opponent_sym = s.chars().nth(0).unwrap();
        let your_sym = s.chars().nth(2).unwrap();
        return score_2(opponent_sym, your_sym);
    })
    .sum::<u32>()
    .to_string()
}

#[test]
fn test_part2() {
    assert_eq!("12", part2_inner(TEST_INPUT))
}

// ---

fn main() {
    println!("part1: {}", part1());
    println!("part2: {}", part2());
}
