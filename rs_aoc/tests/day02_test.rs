use rs_aoc::days::day02::Day02;
use rs_aoc::days::Solution;
use rs_aoc::{read_input, read_sample_input};

#[test]
fn test_day02_part1_sample() {
    let input = read_sample_input(2);
    let result = Day02.part1(&input);
    assert_eq!(result, "expected"); // TODO: replace with expected sample answer
}

#[test]
fn test_day02_part2_sample() {
    let input = read_sample_input(2);
    let result = Day02.part2(&input);
    assert_eq!(result, "expected"); // TODO: replace with expected sample answer
}

#[test]
#[ignore] // remove once you have the real answer
fn test_day02_part1_real() {
    let input = read_input(2);
    let result = Day02.part1(&input);
    assert_eq!(result, "expected"); // TODO: replace with expected real answer
}

#[test]
#[ignore] // remove once you have the real answer
fn test_day02_part2_real() {
    let input = read_input(2);
    let result = Day02.part2(&input);
    assert_eq!(result, "expected"); // TODO: replace with expected real answer
}
