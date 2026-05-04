use crate::days::Solution;

pub struct Day01;

fn parse_rotations(input: &str) -> Vec<(char, i64)> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let dir = line.chars().next().unwrap();
            let dist: i64 = line[1..].parse().unwrap();
            (dir, dist)
        })
        .collect()
}

/// Count how many of the `dist` clicks pass through or land on 0.
/// `pos` is the starting position (0..100), we move `dist` clicks in the given direction.
fn count_zero_visits(pos: i64, dist: i64, dir: char) -> i64 {
    if dist == 0 {
        return 0;
    }
    // The first zero we'd hit when moving from `pos`:
    // For L (subtract): first zero is at click number `pos` (if pos > 0), or `100` (if pos == 0)
    // For R (add): first zero is at click number `100 - pos` (if pos > 0), or `100` (if pos == 0)
    let first = match dir {
        'L' => {
            if pos > 0 {
                pos
            } else {
                100
            }
        }
        'R' => {
            if pos > 0 {
                100 - pos
            } else {
                100
            }
        }
        _ => panic!("unexpected direction: {}", dir),
    };

    if first > dist {
        0
    } else {
        // first, first+100, first+200, ... all <= dist
        (dist - first) / 100 + 1
    }
}

impl Solution for Day01 {
    fn part1(&self, input: &str) -> String {
        let rotations = parse_rotations(input);
        let mut pos: i64 = 50;
        let mut count = 0;

        for (dir, dist) in &rotations {
            match dir {
                'L' => pos = (pos - dist).rem_euclid(100),
                'R' => pos = (pos + dist).rem_euclid(100),
                _ => panic!("unexpected direction: {}", dir),
            }
            if pos == 0 {
                count += 1;
            }
        }

        count.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let rotations = parse_rotations(input);
        let mut pos: i64 = 50;
        let mut count: i64 = 0;

        for (dir, dist) in &rotations {
            count += count_zero_visits(pos, *dist, *dir);
            match dir {
                'L' => pos = (pos - dist).rem_euclid(100),
                'R' => pos = (pos + dist).rem_euclid(100),
                _ => panic!("unexpected direction: {}", dir),
            }
        }

        count.to_string()
    }
}
