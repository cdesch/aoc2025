use crate::days::Solution;

pub struct Day10;

/// Represents a single machine parsed from the input.
struct Machine {
    /// Target bitmask: bit i is set if light i should be ON.
    target: u64,
    /// Each button is a bitmask of which lights it toggles.
    buttons: Vec<u64>,
}

/// Parse a single line into a Machine.
/// Format: [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
fn parse_machine(line: &str) -> Machine {
    let line = line.trim();

    // Extract indicator light diagram between [ and ]
    let bracket_start = line.find('[').unwrap();
    let bracket_end = line.find(']').unwrap();
    let diagram = &line[bracket_start + 1..bracket_end];

    let mut target: u64 = 0;
    for (i, ch) in diagram.chars().enumerate() {
        if ch == '#' {
            target |= 1u64 << i;
        }
    }

    // Extract button wiring schematics: all (...) groups
    // We need to be careful not to grab the {...} group.
    let mut buttons = Vec::new();
    let after_bracket = &line[bracket_end + 1..];

    // Find the start of the joltage section to exclude it
    let joltage_start = after_bracket.find('{').unwrap_or(after_bracket.len());
    let button_section = &after_bracket[..joltage_start];

    let mut i = 0;
    let bytes = button_section.as_bytes();
    while i < bytes.len() {
        if bytes[i] == b'(' {
            let close = button_section[i..].find(')').unwrap() + i;
            let inner = &button_section[i + 1..close];
            let mut mask: u64 = 0;
            for num_str in inner.split(',') {
                let num_str = num_str.trim();
                if !num_str.is_empty() {
                    let idx: u32 = num_str.parse().unwrap();
                    mask |= 1u64 << idx;
                }
            }
            buttons.push(mask);
            i = close + 1;
        } else {
            i += 1;
        }
    }

    Machine { target, buttons }
}

/// Find the minimum number of button presses to reach the target state.
/// Since each button is a toggle (XOR), pressing it 0 or 1 times is sufficient.
/// We need the minimum-size subset of buttons whose XOR equals the target.
///
/// Strategy: meet-in-the-middle. Split buttons into two halves, enumerate all
/// subsets of each half, then find pairs that XOR to the target with minimum
/// total size.
fn min_presses(machine: &Machine) -> u64 {
    let n = machine.buttons.len();
    let target = machine.target;

    if target == 0 {
        return 0;
    }

    if n <= 20 {
        // For small n, we can do meet-in-the-middle.
        let half1 = n / 2;
        let half2 = n - half1;

        // Enumerate all subsets of the first half.
        // Store: xor_value -> minimum popcount
        let size1 = 1usize << half1;
        let mut map = std::collections::HashMap::with_capacity(size1);

        for mask in 0..size1 {
            let mut xor_val: u64 = 0;
            for bit in 0..half1 {
                if mask & (1 << bit) != 0 {
                    xor_val ^= machine.buttons[bit];
                }
            }
            let count = mask.count_ones();
            let entry = map.entry(xor_val).or_insert(count);
            if count < *entry {
                *entry = count;
            }
        }

        // Enumerate all subsets of the second half.
        let size2 = 1usize << half2;
        let mut best = u64::MAX;

        for mask in 0..size2 {
            let mut xor_val: u64 = 0;
            for bit in 0..half2 {
                if mask & (1 << bit) != 0 {
                    xor_val ^= machine.buttons[half1 + bit];
                }
            }
            let count2 = mask.count_ones();
            // We need xor_val ^ first_half_xor = target
            // So first_half_xor = target ^ xor_val
            let needed = target ^ xor_val;
            if let Some(&count1) = map.get(&needed) {
                let total = (count1 + count2) as u64;
                if total < best {
                    best = total;
                }
            }
        }

        best
    } else {
        // For larger n, use Gaussian elimination over GF(2) to find minimum weight.
        // This shouldn't be needed for typical AoC inputs, but as a fallback:
        // Use BFS/greedy with Gaussian elimination.
        min_presses_gauss(machine)
    }
}

/// Gaussian elimination approach for larger button counts.
/// Finds minimum number of buttons to press using greedy GF(2) elimination.
fn min_presses_gauss(machine: &Machine) -> u64 {
    let n = machine.buttons.len();
    let target = machine.target;

    // Augmented matrix: each row is (button_mask, index_bitmask)
    // We'll track which original buttons are used via a separate vector.
    let mut rows: Vec<(u64, Vec<bool>)> = machine
        .buttons
        .iter()
        .enumerate()
        .map(|(i, &mask)| {
            let mut usage = vec![false; n];
            usage[i] = true;
            (mask, usage)
        })
        .collect();

    // Gaussian elimination
    let mut pivot_row = 0;
    for col in 0..64 {
        // Find a row with this bit set
        let mut found = None;
        for r in pivot_row..rows.len() {
            if rows[r].0 & (1u64 << col) != 0 {
                found = Some(r);
                break;
            }
        }
        if let Some(r) = found {
            rows.swap(pivot_row, r);
            for r in 0..rows.len() {
                if r != pivot_row && rows[r].0 & (1u64 << col) != 0 {
                    let pivot_mask = rows[pivot_row].0;
                    let pivot_usage = rows[pivot_row].1.clone();
                    rows[r].0 ^= pivot_mask;
                    for i in 0..n {
                        rows[r].1[i] ^= pivot_usage[i];
                    }
                }
            }
            pivot_row += 1;
        }
    }

    // Now solve: for each bit set in target, find the corresponding pivot row
    let mut solution = vec![false; n];
    let mut remaining = target;

    for row in &rows {
        if row.0 == 0 {
            continue;
        }
        let leading_bit = 63 - row.0.leading_zeros() as u64;
        if remaining & (1u64 << leading_bit) != 0 {
            remaining ^= row.0;
            for i in 0..n {
                solution[i] ^= row.1[i];
            }
        }
    }

    if remaining != 0 {
        // No solution exists (shouldn't happen per problem statement)
        return u64::MAX;
    }

    solution.iter().filter(|&&b| b).count() as u64
}

fn solve_part1(input: &str) -> u64 {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let machine = parse_machine(line);
            min_presses(&machine)
        })
        .sum()
}

impl Solution for Day10 {
    fn part1(&self, input: &str) -> String {
        solve_part1(input).to_string()
    }

    fn part2(&self, _input: &str) -> String {
        // Part 2 not yet revealed
        String::from("not implemented")
    }
}
