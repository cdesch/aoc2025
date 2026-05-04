use rs_aoc::days::day01::Day01;
use rs_aoc::days::Solution;
use rs_aoc::{read_input, read_sample_input};

#[test]
fn test_day01_part1_sample() {
    let input = read_sample_input(1);
    let result = Day01.part1(&input);
    assert_eq!(result, "expected"); // TODO: replace with expected sample answer
}

#[test]
fn test_day01_part2_sample() {
    let input = read_sample_input(1);
    let result = Day01.part2(&input);
    assert_eq!(result, "expected"); // TODO: replace with expected sample answer
}

#[test]
#[ignore] // remove once you have the real answer
fn test_day01_part1_real() {
    let input = read_input(1);
    let result = Day01.part1(&input);
    assert_eq!(result, "expected"); // TODO: replace with expected real answer
}

#[test]
#[ignore] // remove once you have the real answer
fn test_day01_part2_real() {
    let input = read_input(1);
    let result = Day01.part2(&input);
    assert_eq!(result, "expected"); // TODO: replace with expected real answer
}
