use crate::days::Solution;
use std::collections::BTreeSet;

pub struct Day09;

fn parse_points(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut parts = line.trim().split(',');
            let x: i64 = parts.next().unwrap().parse().unwrap();
            let y: i64 = parts.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect()
}

fn largest_rectangle(points: &[(i64, i64)]) -> i64 {
    let mut max_area: i64 = 0;
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];
            let area = ((x2 - x1).abs() + 1) * ((y2 - y1).abs() + 1);
            if area > max_area {
                max_area = area;
            }
        }
    }
    max_area
}

/// For Part 2, we need to find the largest rectangle with red tile opposite corners
/// where every tile in the rectangle is red or green (inside/on the polygon).
///
/// The polygon is formed by connecting consecutive red tiles with axis-aligned segments.
fn largest_rectangle_constrained(points: &[(i64, i64)]) -> i64 {
    let n = points.len();
    if n < 2 {
        return 0;
    }

    // Collect all unique x and y coordinates for compression.
    // We need the coordinates of the red points, plus the boundary segments.
    // For the polygon fill, we need all x and y values that appear on edges.
    // Since edges are axis-aligned between consecutive points, the only
    // distinct x/y values that matter are those of the red points themselves.
    let mut xs_set = BTreeSet::new();
    let mut ys_set = BTreeSet::new();
    for &(x, y) in points {
        xs_set.insert(x);
        ys_set.insert(y);
    }
    let xs: Vec<i64> = xs_set.into_iter().collect();
    let ys: Vec<i64> = ys_set.into_iter().collect();

    let xi = |v: i64| xs.binary_search(&v).unwrap();
    let yi = |v: i64| ys.binary_search(&v).unwrap();

    let cx = xs.len();
    let cy = ys.len();

    // Build a grid in compressed coordinates.
    // Between consecutive compressed coordinates, there may be gaps.
    // We represent the grid with "expanded" coordinates:
    //   - Even indices represent the actual compressed coordinate values.
    //   - Odd indices represent the gaps between consecutive values.
    // This gives us a grid of size (2*cx - 1) x (2*cy - 1).
    let gw = 2 * cx - 1;
    let gh = 2 * cy - 1;

    // Mark boundary cells on the expanded grid.
    // An edge between consecutive red points is axis-aligned.
    let mut boundary = vec![vec![false; gh]; gw];

    for i in 0..n {
        let (x1, y1) = points[i];
        let (x2, y2) = points[(i + 1) % n];

        let ei1 = 2 * xi(x1);
        let ej1 = 2 * yi(y1);
        let ei2 = 2 * xi(x2);
        let ej2 = 2 * yi(y2);

        if ei1 == ei2 {
            // Vertical edge (same x)
            let min_j = ej1.min(ej2);
            let max_j = ej1.max(ej2);
            for j in min_j..=max_j {
                boundary[ei1][j] = true;
            }
        } else {
            // Horizontal edge (same y)
            let min_i = ei1.min(ei2);
            let max_i = ei1.max(ei2);
            for i in min_i..=max_i {
                boundary[i][ej1] = true;
            }
        }
    }

    // Flood-fill from outside to find exterior cells.
    // Any cell not reached by the flood fill and not on the boundary is interior.
    let mut exterior = vec![vec![false; gh]; gw];
    let mut stack: Vec<(usize, usize)> = Vec::new();

    // Start flood fill from all border cells of the expanded grid that aren't boundary.
    for i in 0..gw {
        for &j in &[0, gh - 1] {
            if !boundary[i][j] && !exterior[i][j] {
                exterior[i][j] = true;
                stack.push((i, j));
            }
        }
    }
    for j in 0..gh {
        for &i in &[0, gw - 1] {
            if !boundary[i][j] && !exterior[i][j] {
                exterior[i][j] = true;
                stack.push((i, j));
            }
        }
    }

    while let Some((ci, cj)) = stack.pop() {
        for (di, dj) in [(-1i64, 0), (1, 0), (0, -1), (0, 1)] {
            let ni = ci as i64 + di;
            let nj = cj as i64 + dj;
            if ni >= 0 && ni < gw as i64 && nj >= 0 && nj < gh as i64 {
                let ni = ni as usize;
                let nj = nj as usize;
                if !boundary[ni][nj] && !exterior[ni][nj] {
                    exterior[ni][nj] = true;
                    stack.push((ni, nj));
                }
            }
        }
    }

    // A cell in the expanded grid is "allowed" if it's not exterior.
    // (It's either boundary or interior.)
    // allowed[i][j] = !exterior[i][j]
    // But we only care about cells at even coordinates (actual compressed values)
    // and odd coordinates (gaps between values) for the rectangle check.

    // Build a 2D prefix sum of "blocked" cells over the expanded grid.
    // blocked[i][j] = 1 if exterior[i][j], else 0
    // But for the rectangle query, we need to check: for a rectangle defined by
    // two red points (in original coords), are all expanded-grid cells in that
    // range allowed?

    // For a rectangle from red point (xa, ya) to (xb, yb) (with xa <= xb, ya <= yb),
    // the expanded grid range is [2*xi(xa), 2*xi(xb)] x [2*yi(ya), 2*yi(yb)].
    // We need all cells in this range to be allowed (not exterior).

    // Build prefix sum of blocked (exterior) cells.
    let mut prefix = vec![vec![0i64; gh + 1]; gw + 1];
    for i in 0..gw {
        for j in 0..gh {
            let val = if exterior[i][j] { 1 } else { 0 };
            prefix[i + 1][j + 1] = val + prefix[i][j + 1] + prefix[i + 1][j] - prefix[i][j];
        }
    }

    let rect_blocked = |r1: usize, c1: usize, r2: usize, c2: usize| -> i64 {
        // Sum of blocked cells in [r1..=r2] x [c1..=c2]
        prefix[r2 + 1][c2 + 1] - prefix[r1][c2 + 1] - prefix[r2 + 1][c1] + prefix[r1][c1]
    };

    // Now check all pairs of red points.
    let mut max_area: i64 = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];

            // Skip degenerate rectangles (same row or column → area would just be a line)
            if x1 == x2 || y1 == y2 {
                continue;
            }

            let xa = x1.min(x2);
            let xb = x1.max(x2);
            let ya = y1.min(y2);
            let yb = y1.max(y2);

            let ei_a = 2 * xi(xa);
            let ei_b = 2 * xi(xb);
            let ej_a = 2 * yi(ya);
            let ej_b = 2 * yi(yb);

            let blocked = rect_blocked(ei_a, ej_a, ei_b, ej_b);
            if blocked == 0 {
                let area = (xb - xa + 1) * (yb - ya + 1);
                if area > max_area {
                    max_area = area;
                }
            }
        }
    }

    max_area
}

impl Solution for Day09 {
    fn part1(&self, input: &str) -> String {
        let points = parse_points(input);
        largest_rectangle(&points).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let points = parse_points(input);
        largest_rectangle_constrained(&points).to_string()
    }
}
