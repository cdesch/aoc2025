use rs_aoc::days::day09::Day09;
use rs_aoc::days::Solution;
use rs_aoc::{read_input, read_sample_input};

#[test]
fn test_day09_part1_sample() {
    let input = read_sample_input(9);
    let result = Day09.part1(&input);
    assert_eq!(result, "50");
}

#[test]
#[ignore] // Part 2 not yet revealed
fn test_day09_part2_sample() {
    let input = read_sample_input(9);
    let result = Day09.part2(&input);
    assert_eq!(result, "expected");
}

#[test]
#[ignore] // remove once you have the real answer
fn test_day09_part1_real() {
    let input = read_input(9);
    let result = Day09.part1(&input);
    assert_eq!(result, "expected"); // TODO: replace with expected real answer
}

#[test]
#[ignore] // remove once you have the real answer
fn test_day09_part2_real() {
    let input = read_input(9);
    let result = Day09.part2(&input);
    assert_eq!(result, "expected"); // TODO: replace with expected real answer
}
