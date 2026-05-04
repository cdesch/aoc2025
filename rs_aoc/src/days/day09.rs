use crate::days::Solution;

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
    let mut max_area = 0;
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

impl Solution for Day09 {
    fn part1(&self, input: &str) -> String {
        let points = parse_points(input);
        largest_rectangle(&points).to_string()
    }

    fn part2(&self, _input: &str) -> String {
        // Part 2 not yet revealed
        String::from("not implemented")
    }
}
