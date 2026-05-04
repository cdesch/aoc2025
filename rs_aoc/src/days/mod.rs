// Each day module is registered here.
// The new_day.sh script will append `pub mod dayXX;` lines below.

pub mod day01;
pub mod day02;
pub mod day09;

/// Trait that every day's solution implements.
pub trait Solution {
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;
}

/// Run a given day's solution against the real input.
pub fn run_day(day: u32) {
    let input = crate::read_input(day);

    match day {
        // The new_day.sh script will insert match arms here.
        1 => run_solution(1, &day01::Day01, &input),
        2 => run_solution(2, &day02::Day02, &input),
        9 => run_solution(9, &day09::Day09, &input),
        // MATCH_ARMS
        _ => eprintln!("Day {:02} is not implemented yet.", day),
    }
}

fn run_solution(day: u32, sol: &dyn Solution, input: &str) {
    println!("=== Day {:02} ===", day);
    println!("Part 1: {}", sol.part1(input));
    println!("Part 2: {}", sol.part2(input));
}
