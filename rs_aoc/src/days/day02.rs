use std::collections::HashSet;

use crate::days::Solution;

pub struct Day02;

/// Parse the input into a list of (start, end) ranges.
fn parse_ranges(input: &str) -> Vec<(u128, u128)> {
    input
        .trim()
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|range| {
            let parts: Vec<&str> = range.split('-').collect();
            let start: u128 = parts[0].trim().parse().unwrap();
            let end: u128 = parts[1].trim().parse().unwrap();
            (start, end)
        })
        .collect()
}

/// Generate all "double" numbers: a k-digit base repeated exactly 2 times.
fn doubles_with_half_len(half_len: u32) -> impl Iterator<Item = u128> {
    let lo = if half_len == 1 {
        1
    } else {
        10u128.pow(half_len - 1)
    };
    let hi = 10u128.pow(half_len);
    let multiplier = hi;
    (lo..hi).map(move |n| n * multiplier + n)
}

/// Generate all numbers formed by repeating a k-digit base exactly `reps` times.
fn repeats_with_base_len(base_len: u32, reps: u32) -> impl Iterator<Item = u128> {
    let lo = if base_len == 1 {
        1
    } else {
        10u128.pow(base_len - 1)
    };
    let hi = 10u128.pow(base_len);
    let bl = base_len;
    let r = reps;
    (lo..hi).map(move |n| {
        let mut result = 0u128;
        for i in 0..r {
            result += n * 10u128.pow(bl * i);
        }
        result
    })
}

/// Find the maximum number of digits across all range endpoints.
fn max_digits(ranges: &[(u128, u128)]) -> u32 {
    ranges
        .iter()
        .map(|(_, end)| end.to_string().len() as u32)
        .max()
        .unwrap_or(0)
}

/// Check if a number falls within any of the given ranges.
fn in_any_range(n: u128, ranges: &[(u128, u128)]) -> bool {
    ranges.iter().any(|&(start, end)| n >= start && n <= end)
}

impl Solution for Day02 {
    fn part1(&self, input: &str) -> String {
        let ranges = parse_ranges(input);
        let max_d = max_digits(&ranges);
        let max_half = (max_d + 1) / 2;

        let mut total: u128 = 0;

        for half_len in 1..=max_half {
            for double in doubles_with_half_len(half_len) {
                if in_any_range(double, &ranges) {
                    total += double;
                }
            }
        }

        total.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let ranges = parse_ranges(input);
        let max_d = max_digits(&ranges);

        // Collect all invalid IDs (deduplicated) that fall in any range.
        // For each base length k and repetition count r >= 2 where k*r <= max_d,
        // generate all k-digit numbers repeated r times.
        let mut invalid_ids: HashSet<u128> = HashSet::new();

        for base_len in 1..=max_d {
            // reps must be >= 2, and base_len * reps <= max_d
            if base_len * 2 > max_d {
                break;
            }
            let max_reps = max_d / base_len;
            for reps in 2..=max_reps {
                for val in repeats_with_base_len(base_len, reps) {
                    if in_any_range(val, &ranges) {
                        invalid_ids.insert(val);
                    }
                }
            }
        }

        let total: u128 = invalid_ids.iter().sum();
        total.to_string()
    }
}
