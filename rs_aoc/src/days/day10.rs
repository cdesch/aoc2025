use crate::days::Solution;
use std::collections::HashMap;

pub struct Day10;

struct Machine {
    target: u64,
    buttons: Vec<u64>,
}

struct Machine2 {
    joltages: Vec<i64>,
    buttons: Vec<Vec<usize>>,
    num_counters: usize,
}

// ─── Parsing ─────────────────────────────────────────────────────────────────

fn parse_machine(line: &str) -> Machine {
    let line = line.trim();
    let bracket_start = line.find('[').unwrap();
    let bracket_end = line.find(']').unwrap();
    let diagram = &line[bracket_start + 1..bracket_end];
    let mut target: u64 = 0;
    for (i, ch) in diagram.chars().enumerate() {
        if ch == '#' {
            target |= 1u64 << i;
        }
    }
    let buttons = parse_button_masks(&line[bracket_end + 1..]);
    Machine { target, buttons }
}

fn parse_button_masks(after_bracket: &str) -> Vec<u64> {
    let mut buttons = Vec::new();
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
    buttons
}

fn parse_machine2(line: &str) -> Machine2 {
    let line = line.trim();
    let bracket_end = line.find(']').unwrap();
    let after_bracket = &line[bracket_end + 1..];
    let joltage_start = after_bracket.find('{').unwrap();
    let button_section = &after_bracket[..joltage_start];
    let mut buttons: Vec<Vec<usize>> = Vec::new();
    let mut i = 0;
    let bytes = button_section.as_bytes();
    while i < bytes.len() {
        if bytes[i] == b'(' {
            let close = button_section[i..].find(')').unwrap() + i;
            let inner = &button_section[i + 1..close];
            let mut indices = Vec::new();
            for num_str in inner.split(',') {
                let num_str = num_str.trim();
                if !num_str.is_empty() {
                    indices.push(num_str.parse::<usize>().unwrap());
                }
            }
            buttons.push(indices);
            i = close + 1;
        } else {
            i += 1;
        }
    }
    let curly_start = after_bracket.find('{').unwrap();
    let curly_end = after_bracket.find('}').unwrap();
    let joltage_str = &after_bracket[curly_start + 1..curly_end];
    let joltages: Vec<i64> = joltage_str
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();
    let num_counters = joltages.len();
    Machine2 {
        joltages,
        buttons,
        num_counters,
    }
}

// ─── Part 1: XOR / meet-in-the-middle ───────────────────────────────────────

fn min_presses_p1(machine: &Machine) -> u64 {
    let n = machine.buttons.len();
    let target = machine.target;
    if target == 0 {
        return 0;
    }
    if n <= 20 {
        let half1 = n / 2;
        let half2 = n - half1;
        let size1 = 1usize << half1;
        let mut map = HashMap::with_capacity(size1);
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
        min_presses_gauss(machine)
    }
}

fn min_presses_gauss(machine: &Machine) -> u64 {
    let n = machine.buttons.len();
    let target = machine.target;
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
    let mut pivot_row = 0;
    for col in 0..64 {
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
        return u64::MAX;
    }
    solution.iter().filter(|&&b| b).count() as u64
}

// ─── Part 2: ILP via direct branch and bound ────────────────────────────────

/// Solve LP relaxation: minimize sum(x_j) s.t. A*x = b, x >= 0
fn solve_lp(a: &[Vec<f64>], b: &[f64], m: usize, n: usize) -> Option<f64> {
    let big_m_val = 1e12;
    let total_vars = n + m;
    let cols = total_vars + 1;
    let nrows = m + 1;
    let mut tableau = vec![vec![0.0f64; cols]; nrows];
    for i in 0..m {
        for j in 0..n {
            tableau[i][j] = a[i][j];
        }
        tableau[i][n + i] = 1.0;
        tableau[i][total_vars] = b[i];
    }
    for j in 0..n {
        tableau[m][j] = 1.0;
    }
    for j in 0..m {
        tableau[m][n + j] = big_m_val;
    }
    let mut basis: Vec<usize> = (n..n + m).collect();
    for i in 0..m {
        for j in 0..cols {
            tableau[m][j] -= big_m_val * tableau[i][j];
        }
    }
    for _ in 0..100_000 {
        let mut pivot_col = 0;
        let mut min_val = -1e-9;
        for j in 0..total_vars {
            if tableau[m][j] < min_val {
                min_val = tableau[m][j];
                pivot_col = j;
            }
        }
        if min_val >= -1e-9 {
            break;
        }
        let mut pr = None;
        let mut min_ratio = f64::INFINITY;
        for i in 0..m {
            if tableau[i][pivot_col] > 1e-9 {
                let ratio = tableau[i][total_vars] / tableau[i][pivot_col];
                if ratio < min_ratio - 1e-9 {
                    min_ratio = ratio;
                    pr = Some(i);
                }
            }
        }
        let pr = match pr {
            Some(r) => r,
            None => break,
        };
        let pv = tableau[pr][pivot_col];
        for j in 0..cols {
            tableau[pr][j] /= pv;
        }
        for i in 0..nrows {
            if i != pr {
                let f = tableau[i][pivot_col];
                if f.abs() > 1e-15 {
                    for j in 0..cols {
                        tableau[i][j] -= f * tableau[pr][j];
                    }
                }
            }
        }
        basis[pr] = pivot_col;
    }
    for i in 0..m {
        if basis[i] >= n && tableau[i][total_vars].abs() > 1e-6 {
            return None;
        }
    }
    let obj = -tableau[m][total_vars];
    Some(obj)
}

fn min_presses_p2(machine: &Machine2) -> i64 {
    let m = machine.num_counters;
    let n = machine.buttons.len();

    // Build constraint matrix A (m x n): A[i][j] = 1 if button j affects counter i
    let mut a_int = vec![vec![0i64; n]; m];
    let mut a_f64 = vec![vec![0.0f64; n]; m];
    for (j, btn) in machine.buttons.iter().enumerate() {
        for &counter in btn {
            if counter < m {
                a_int[counter][j] = 1;
                a_f64[counter][j] = 1.0;
            }
        }
    }

    let target = &machine.joltages;

    // Upper bound for each button: min over all counters it affects of target[counter]
    let mut upper_bounds = vec![0i64; n];
    for j in 0..n {
        let mut ub = i64::MAX;
        for i in 0..m {
            if a_int[i][j] == 1 && target[i] < ub {
                ub = target[i];
            }
        }
        upper_bounds[j] = ub;
    }

    let mut best = i64::MAX;
    let mut remaining = target.clone();

    // Order buttons by number of counters affected (descending) for better pruning
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| machine.buttons[b].len().cmp(&machine.buttons[a].len()));

    bb_direct(
        &a_int,
        &a_f64,
        target,
        m,
        n,
        &order,
        0,
        0,
        &mut remaining,
        &upper_bounds,
        &mut best,
    );

    best
}

fn bb_direct(
    a_int: &[Vec<i64>],
    a_f64: &[Vec<f64>],
    target: &[i64],
    m: usize,
    n: usize,
    order: &[usize],
    depth: usize,
    current_sum: i64,
    remaining: &mut Vec<i64>,
    upper_bounds: &[i64],
    best: &mut i64,
) {
    if current_sum >= *best {
        return;
    }

    // Check if remaining is all zero
    if remaining.iter().all(|&v| v == 0) {
        if current_sum < *best {
            *best = current_sum;
        }
        return;
    }

    if depth == n {
        return;
    }

    // LP relaxation bound on remaining variables
    let remaining_indices: Vec<usize> = order[depth..].to_vec();
    let nf = remaining_indices.len();
    let mut a_sub = vec![vec![0.0f64; nf]; m];
    for (jj, &j) in remaining_indices.iter().enumerate() {
        for i in 0..m {
            a_sub[i][jj] = a_f64[i][j];
        }
    }
    let b_sub: Vec<f64> = remaining.iter().map(|&v| v as f64).collect();

    if let Some(lp_val) = solve_lp(&a_sub, &b_sub, m, nf) {
        let lb = current_sum + (lp_val - 1e-6).ceil() as i64;
        if lb >= *best {
            return;
        }
    } else {
        return; // infeasible
    }

    let j = order[depth];

    // Upper bound for this button
    let mut ub = upper_bounds[j];
    for i in 0..m {
        if a_int[i][j] == 1 && remaining[i] < ub {
            ub = remaining[i];
        }
    }

    // Try values from 0 to ub
    for val in 0..=ub {
        if current_sum + val >= *best {
            break;
        }

        // Apply
        for i in 0..m {
            remaining[i] -= a_int[i][j] * val;
        }

        // Check no remaining value is negative
        let feasible = remaining.iter().all(|&v| v >= 0);

        if feasible {
            bb_direct(
                a_int,
                a_f64,
                target,
                m,
                n,
                order,
                depth + 1,
                current_sum + val,
                remaining,
                upper_bounds,
                best,
            );
        }

        // Undo
        for i in 0..m {
            remaining[i] += a_int[i][j] * val;
        }
    }
}

// ─── Solve ───────────────────────────────────────────────────────────────────

fn solve_part1(input: &str) -> u64 {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let machine = parse_machine(line);
            min_presses_p1(&machine)
        })
        .sum()
}

fn solve_part2(input: &str) -> i64 {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let machine = parse_machine2(line);
            min_presses_p2(&machine)
        })
        .sum()
}

impl Solution for Day10 {
    fn part1(&self, input: &str) -> String {
        solve_part1(input).to_string()
    }

    fn part2(&self, input: &str) -> String {
        solve_part2(input).to_string()
    }
}
