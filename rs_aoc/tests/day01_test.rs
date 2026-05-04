use rs_aoc::days::day01::Day01;
use rs_aoc::days::Solution;
use rs_aoc::{read_input, read_sample_input};

#[test]
fn test_day01_part1_sample() {
    let input = read_sample_input(1);
    let result = Day01.part1(&input);
    assert_eq!(result, "3");
}

#[test]
fn test_day01_part2_sample() {
    let input = read_sample_input(1);
    let result = Day01.part2(&input);
    assert_eq!(result, "6");
}

#[test]
fn test_day01_part1_real() {
    let input = read_input(1);
    let result = Day01.part1(&input);
    assert_eq!(result, "1040");
}

#[test]
fn test_day01_part2_real() {
    let input = read_input(1);
    let result = Day01.part2(&input);
    assert_eq!(result, "6027");
