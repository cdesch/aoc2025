use crate::days::Solution;
use std::collections::HashMap;

pub struct Day10;

/// Represents a single machine parsed from the input (Part 1 view).
struct Machine {
    target: u64,
    buttons: Vec<u64>,
}

/// Represents a single machine for Part 2.
struct Machine2 {
    joltages: Vec<i64>,
    /// Each button: list of counter indices it increments.
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

// ─── Part 2: Exact ILP via rational Gaussian elimination + search ────────────

/// Rational number as (numerator, denominator), always kept in lowest terms
/// with positive denominator.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Frac {
    n: i64,
    d: i64,
}

impl Frac {
    fn new(n: i64, d: i64) -> Self {
        if d == 0 {
            panic!("zero denominator");
        }
        if n == 0 {
            return Frac { n: 0, d: 1 };
        }
        let g = gcd(n.abs(), d.abs());
        let sign = if d < 0 { -1 } else { 1 };
        Frac {
            n: sign * n / g,
            d: sign * d / g,
        }
    }

    fn zero() -> Self {
        Frac { n: 0, d: 1 }
    }

    fn is_zero(self) -> bool {
        self.n == 0
    }

    fn add(self, other: Frac) -> Frac {
        Frac::new(self.n * other.d + other.n * self.d, self.d * other.d)
    }

    fn sub(self, other: Frac) -> Frac {
        Frac::new(self.n * other.d - other.n * self.d, self.d * other.d)
    }

    fn mul(self, other: Frac) -> Frac {
        Frac::new(self.n * other.n, self.d * other.d)
    }

    fn div(self, other: Frac) -> Frac {
        Frac::new(self.n * other.d, self.d * other.n)
    }

    fn floor(self) -> i64 {
        if self.d == 1 {
            return self.n;
        }
        // floor division
        self.n.div_euclid(self.d)
    }

    fn ceil(self) -> i64 {
        if self.d == 1 {
            return self.n;
        }
        // ceil = floor + 1 if not exact
        let f = self.n.div_euclid(self.d);
        if f * self.d == self.n {
            f
        } else {
            f + 1
        }
    }

    fn to_f64(self) -> f64 {
        self.n as f64 / self.d as f64
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Solve: minimize sum(x_i) subject to A*x = b, x >= 0, x integer.
///
/// Uses rational Gaussian elimination to find the general solution,
/// then searches over free variables to find the minimum.
fn min_presses_p2(machine: &Machine2) -> i64 {
    let m = machine.num_counters;
    let n = machine.buttons.len();

    // Build augmented matrix [A | b] using rationals.
    // A is m x n, augmented is m x (n+1).
    let mut mat: Vec<Vec<Frac>> = vec![vec![Frac::zero(); n + 1]; m];
    for (j, btn) in machine.buttons.iter().enumerate() {
        for &counter in btn {
            if counter < m {
                mat[counter][j] = Frac::new(1, 1);
            }
        }
    }
    for i in 0..m {
        mat[i][n] = Frac::new(machine.joltages[i], 1);
    }

    // Gaussian elimination with partial pivoting (rational, exact).
    let mut pivot_cols: Vec<usize> = Vec::new(); // which column is the pivot for each pivot row
    let mut pivot_row = 0;

    for col in 0..n {
        // Find a row with non-zero entry in this column
        let mut found = None;
        for r in pivot_row..m {
            if !mat[r][col].is_zero() {
                found = Some(r);
                break;
            }
        }
        let r = match found {
            Some(r) => r,
            None => continue, // no pivot in this column
        };

        // Swap rows
        mat.swap(pivot_row, r);

        // Scale pivot row so pivot element = 1
        let pivot_val = mat[pivot_row][col];
        for j in 0..=n {
            mat[pivot_row][j] = mat[pivot_row][j].div(pivot_val);
        }

        // Eliminate this column from all other rows
        for r in 0..m {
            if r != pivot_row && !mat[r][col].is_zero() {
                let factor = mat[r][col];
                for j in 0..=n {
                    let sub = factor.mul(mat[pivot_row][j]);
                    mat[r][j] = mat[r][j].sub(sub);
                }
            }
        }

        pivot_cols.push(col);
        pivot_row += 1;
    }

    let rank = pivot_cols.len();

    // Check consistency: any row with all-zero LHS but non-zero RHS means infeasible
    for r in rank..m {
        if !mat[r][n].is_zero() {
            return i64::MAX; // infeasible
        }
    }

    // Identify free variables (columns not in pivot_cols)
    let pivot_set: std::collections::HashSet<usize> = pivot_cols.iter().copied().collect();
    let free_vars: Vec<usize> = (0..n).filter(|c| !pivot_set.contains(c)).collect();
    let num_free = free_vars.len();

    // The general solution is:
    //   x[pivot_cols[i]] = mat[i][n] - sum_over_free_j(mat[i][free_vars[j]] * t[j])
    //   x[free_vars[j]] = t[j]
    // where t[j] >= 0 are free integer parameters.
    //
    // We need all x >= 0 and minimize sum(x).
    //
    // For each pivot variable i:
    //   x[pivot_cols[i]] = rhs[i] - sum(coeff[i][j] * t[j])
    //   where rhs[i] = mat[i][n], coeff[i][j] = mat[i][free_vars[j]]
    //
    // Objective: minimize sum(rhs[i]) + sum(t[j]) - sum_i(sum_j(coeff[i][j] * t[j]))
    //          = sum(rhs[i]) + sum_j(t[j] * (1 - sum_i(coeff[i][j])))

    // Extract coefficients for the parametric solution
    let rhs: Vec<Frac> = (0..rank).map(|i| mat[i][n]).collect();
    let coeffs: Vec<Vec<Frac>> = (0..rank)
        .map(|i| free_vars.iter().map(|&fj| mat[i][fj]).collect())
        .collect();

    if num_free == 0 {
        // No free variables: unique solution, just check feasibility
        let mut total: i64 = 0;
        for i in 0..rank {
            let val = rhs[i];
            if val.d != 1 || val.n < 0 {
                return i64::MAX; // not a non-negative integer
            }
            total += val.n;
        }
        return total;
    }

    // For small number of free variables, we can search.
    // We need bounds on each t[j].
    // For each pivot row i: rhs[i] - sum(coeff[i][j] * t[j]) >= 0
    // Also each t[j] >= 0.

    // Compute upper bounds for each free variable independently.
    // For each free var j, for each pivot row i where coeff[i][j] > 0:
    //   t[j] <= rhs[i] / coeff[i][j]  (ignoring other free vars)
    let mut upper_bounds: Vec<i64> = vec![i64::MAX; num_free];
    for j in 0..num_free {
        for i in 0..rank {
            if coeffs[i][j].n > 0 {
                // rhs[i] / coeffs[i][j] is the max t[j] can be (if all others are 0)
                let bound = rhs[i].div(coeffs[i][j]);
                let ub = bound.floor();
                if ub < upper_bounds[j] {
                    upper_bounds[j] = ub;
                }
            }
        }
        if upper_bounds[j] < 0 {
            upper_bounds[j] = 0;
        }
    }

    // Search over free variable combinations.
    // With potentially many free variables and large bounds, we need to be smart.
    // For AoC, num_free is typically small (0-4).
    // If bounds are large, we use the LP relaxation to guide the search.

    let total_combinations: i64 = upper_bounds
        .iter()
        .map(|&ub| ub + 1)
        .try_fold(1i64, |acc, v| acc.checked_mul(v))
        .unwrap_or(i64::MAX);

    if num_free <= 4 && total_combinations <= 10_000_000 {
        // Exhaustive search
        search_free_vars(&rhs, &coeffs, &upper_bounds, num_free, rank)
    } else {
        // Use branch and bound with LP relaxation
        branch_and_bound(&rhs, &coeffs, &upper_bounds, num_free, rank)
    }
}

/// Exhaustive search over free variable values.
fn search_free_vars(
    rhs: &[Frac],
    coeffs: &[Vec<Frac>],
    upper_bounds: &[i64],
    num_free: usize,
    rank: usize,
) -> i64 {
    let mut best = i64::MAX;
    let mut t = vec![0i64; num_free];

    fn recurse(
        depth: usize,
        t: &mut Vec<i64>,
        rhs: &[Frac],
        coeffs: &[Vec<Frac>],
        upper_bounds: &[i64],
        num_free: usize,
        rank: usize,
        best: &mut i64,
        // Current partial values of pivot variables (rhs[i] - sum of assigned coeff*t so far)
        current_rhs: &mut Vec<Frac>,
    ) {
        if depth == num_free {
            // Check all pivot variables are non-negative integers
            let mut total: i64 = 0;
            for i in 0..rank {
                let val = current_rhs[i];
                if val.n < 0 || val.d != 1 {
                    return; // not a valid non-negative integer
                }
                total += val.n;
            }
            // Add free variable values
            for j in 0..num_free {
                total += t[j];
            }
            if total < *best {
                *best = total;
            }
            return;
        }

        // Compute tighter upper bound for t[depth] given current_rhs
        let mut ub = upper_bounds[depth];
        for i in 0..rank {
            if coeffs[i][depth].n > 0 {
                let bound = current_rhs[i].div(coeffs[i][depth]).floor();
                if bound < ub {
                    ub = bound;
                }
            }
        }
        if ub < 0 {
            return;
        }

        // Also compute lower bound for t[depth] from rows where coeff is negative
        let mut lb: i64 = 0;
        for i in 0..rank {
            if coeffs[i][depth].n < 0 {
                // current_rhs[i] - coeff[i][depth] * t[depth] >= 0
                // Since coeff < 0, -coeff > 0, so: current_rhs[i] + |coeff| * t >= 0
                // This is always satisfied for t >= 0 if current_rhs >= 0.
                // But we also need the result to be achievable with remaining free vars.
                // For now, lb stays 0.
            }
        }

        for val in lb..=ub {
            t[depth] = val;
            let fval = Frac::new(val, 1);
            // Update current_rhs
            for i in 0..rank {
                current_rhs[i] = current_rhs[i].sub(coeffs[i][depth].mul(fval));
            }

            recurse(
                depth + 1,
                t,
                rhs,
                coeffs,
                upper_bounds,
                num_free,
                rank,
                best,
                current_rhs,
            );

            // Restore current_rhs
            for i in 0..rank {
                current_rhs[i] = current_rhs[i].add(coeffs[i][depth].mul(fval));
            }
        }
    }

    let mut current_rhs: Vec<Frac> = rhs.to_vec();
    recurse(
        0,
        &mut t,
        rhs,
        coeffs,
        upper_bounds,
        num_free,
        rank,
        &mut best,
        &mut current_rhs,
    );

    best
}

/// Branch and bound for larger search spaces.
fn branch_and_bound(
    rhs: &[Frac],
    coeffs: &[Vec<Frac>],
    upper_bounds: &[i64],
    num_free: usize,
    rank: usize,
) -> i64 {
    // Compute the objective coefficient for each free variable:
    // obj_coeff[j] = 1 - sum_i(coeffs[i][j])
    // The objective is: sum(rhs[i]) + sum_j(obj_coeff[j] * t[j])
    let base_obj: Frac = rhs.iter().fold(Frac::zero(), |acc, &v| acc.add(v));

    let obj_coeffs: Vec<Frac> = (0..num_free)
        .map(|j| {
            let col_sum: Frac = (0..rank).fold(Frac::zero(), |acc, i| acc.add(coeffs[i][j]));
            Frac::new(1, 1).sub(col_sum)
        })
        .collect();

    // Use DFS with pruning
    let mut best = i64::MAX;
    let mut t = vec![0i64; num_free];

    fn bb_recurse(
        depth: usize,
        t: &mut Vec<i64>,
        rhs: &[Frac],
        coeffs: &[Vec<Frac>],
        upper_bounds: &[i64],
        obj_coeffs: &[Frac],
        base_obj: Frac,
        num_free: usize,
        rank: usize,
        best: &mut i64,
        current_rhs: &mut Vec<Frac>,
        current_obj: Frac, // base_obj + sum of obj_coeff[j]*t[j] so far + sum of t[j] already counted
    ) {
        if depth == num_free {
            // Check all pivot variables are non-negative integers
            for i in 0..rank {
                let val = current_rhs[i];
                if val.n < 0 || val.d != 1 {
                    return;
                }
            }
            let total = current_obj.ceil(); // should be exact integer
            if total < *best {
                *best = total;
            }
            return;
        }

        let mut ub = upper_bounds[depth];
        for i in 0..rank {
            if coeffs[i][depth].n > 0 {
                let bound = current_rhs[i].div(coeffs[i][depth]).floor();
                if bound < ub {
                    ub = bound;
                }
            }
        }
        if ub < 0 {
            return;
        }

        // If obj_coeff for this variable is positive, lower values are better.
        // If negative, higher values are better.
        // Order search accordingly for better pruning.
        let range: Box<dyn Iterator<Item = i64>> = if obj_coeffs[depth].n >= 0 {
            Box::new(0..=ub)
        } else {
            Box::new((0..=ub).rev())
        };

        for val in range {
            t[depth] = val;
            let fval = Frac::new(val, 1);
            let obj_delta = obj_coeffs[depth].mul(fval);
            let new_obj = current_obj.add(obj_delta);

            // Prune: if current objective already >= best, skip
            if new_obj.ceil() >= *best {
                if obj_coeffs[depth].n >= 0 {
                    break; // increasing, all further will be worse
                } else {
                    continue; // decreasing, try smaller values
                }
            }

            for i in 0..rank {
                current_rhs[i] = current_rhs[i].sub(coeffs[i][depth].mul(fval));
            }

            bb_recurse(
                depth + 1,
                t,
                rhs,
                coeffs,
                upper_bounds,
                obj_coeffs,
                base_obj,
                num_free,
                rank,
                best,
                current_rhs,
                new_obj,
            );

            for i in 0..rank {
                current_rhs[i] = current_rhs[i].add(coeffs[i][depth].mul(fval));
            }
        }
    }

    let mut current_rhs: Vec<Frac> = rhs.to_vec();
    bb_recurse(
        0,
        &mut t,
        rhs,
        coeffs,
        upper_bounds,
        &obj_coeffs,
        base_obj,
        num_free,
        rank,
        &mut best,
        &mut current_rhs,
        base_obj,
    );

    best
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
