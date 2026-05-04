use crate::days::Solution;

pub struct Day02;

/// Check if a number is "invalid": its decimal representation is some
/// sequence of digits repeated exactly twice (e.g. 55, 6464, 123123).
/// The number must have an even number of digits and the first half
/// must equal the second half. No leading zeroes allowed (handled
/// naturally since we work with actual numbers).
fn is_double(n: u128) -> bool {
    let s = n.to_string();
    let len = s.len();
    if len % 2 != 0 {
        return false;
    }
    let half = len / 2;
    s[..half] == s[half..]
}

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

/// Generate all "double" numbers with exactly `2 * half_len` digits.
/// A double number is formed by taking a `half_len`-digit number and
/// concatenating it with itself. E.g. half_len=2 gives 1010, 1111, ..., 9999.
fn doubles_with_half_len(half_len: u32) -> impl Iterator<Item = u128> {
    let lo = if half_len == 1 { 1 } else { 10u128.pow(half_len - 1) };
    let hi = 10u128.pow(half_len); // exclusive
    let multiplier = hi; // 10^half_len
    (lo..hi).map(move |n| n * multiplier + n)
}

/// Find the maximum number of digits across all range endpoints.
fn max_digits(ranges: &[(u128, u128)]) -> u32 {
    ranges
        .iter()
        .map(|(_, end)| end.to_string().len() as u32)
        .max()
        .unwrap_or(0)
}

impl Solution for Day02 {
    fn part1(&self, input: &str) -> String {
        let ranges = parse_ranges(input);
        let max_d = max_digits(&ranges);
        // We only need to consider doubles with even digit counts up to max_d
        // half_len goes from 1 to max_d/2 (rounded up to be safe)
        let max_half = (max_d + 1) / 2;

        let mut total: u128 = 0;

        for half_len in 1..=max_half {
            for double in doubles_with_half_len(half_len) {
                for &(start, end) in &ranges {
                    if double >= start && double <= end {
                        total += double;
                        break; // each double counted once even if in multiple ranges
                    }
                }
            }
        }

        total.to_string()
    }

    fn part2(&self, _input: &str) -> String {
        // TODO: Implement part 2
        String::from("not implemented")
    }
}
