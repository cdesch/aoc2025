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

fn simulate(input: &str) -> Vec<i64> {
    let rotations = parse_rotations(input);
    let mut pos: i64 = 50;
    let mut positions = Vec::with_capacity(rotations.len());

    for (dir, dist) in &rotations {
        match dir {
            'L' => pos = (pos - dist).rem_euclid(100),
            'R' => pos = (pos + dist).rem_euclid(100),
            _ => panic!("unexpected direction: {}", dir),
        }
        positions.push(pos);
    }

    positions
}

impl Solution for Day01 {
    fn part1(&self, input: &str) -> String {
        let positions = simulate(input);
        let count = positions.iter().filter(|&&p| p == 0).count();
        count.to_string()
    }

    fn part2(&self, _input: &str) -> String {
        // Part 2 not yet available
        String::from("not implemented")
    }
}
