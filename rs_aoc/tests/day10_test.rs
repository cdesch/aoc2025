use rs_aoc::days::day10::Day10;
use rs_aoc::days::Solution;
use rs_aoc::{read_input, read_sample_input};

#[test]
fn test_day10_part1_sample() {
    let input = read_sample_input(10);
    let result = Day10.part1(&input);
    assert_eq!(result, "7");
}

#[test]
fn test_day10_part2_sample() {
    let input = read_sample_input(10);
    let result = Day10.part2(&input);
    assert_eq!(result, "33");
}

#[test]
fn test_day10_part1_real() {
    let input = read_input(10);
    let result = Day10.part1(&input);
    assert_eq!(result, "507");
}

#[test]
fn test_day10_part2_real() {
    let input = read_input(10);
    let result = Day10.part2(&input);
    assert_eq!(result, "18982");
}
