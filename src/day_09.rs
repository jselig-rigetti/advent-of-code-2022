use std::{collections::HashSet, fmt::Display};

fn get_direction_transform(direction: &str) -> (i32, i32) {
    match direction {
        "U" => (0, 1),
        "D" => (0, -1),
        "L" => (-1, 0),
        "R" => (1, 0),
        _ => panic!("unsupported direction: {}", direction),
    }
}

struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn move_direction(&mut self, direction: &str) {
        let (x, y) = get_direction_transform(direction);
        self.x += x;
        self.y += y;
    }

    fn follow(&mut self, lead: &Self) {
        let (ax, ay) = (lead.x, lead.y);
        let (bx, by) = (self.x, self.y);
        let (dx, dy) = (ax - bx, ay - by);
        let (sx, sy) = (dx.abs().clamp(1, 2) - 1, dy.abs().clamp(1, 2) - 1);
        let mul = sx.max(sy);

        self.x += dx.clamp(-1, 1) * mul;
        self.y += dy.clamp(-1, 1) * mul;
    }
}

impl Default for Position {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

type Move<'a> = (&'a str, i32);

fn parse_moves(input: &str) -> Vec<Move> {
    input
        .trim()
        .lines()
        .map(|l| l.trim().split_whitespace().array_chunks().next().unwrap())
        .map(|[direction, count]| (direction, count.parse::<i32>().unwrap()))
        .collect()
}

fn solve_n_tails(moves: &Vec<Move>, size: usize) -> usize {
    let mut visited = HashSet::new();
    let mut head = Position::default();
    let mut tails: Vec<_> = (0..size).map(|_| Position::default()).collect();

    for (direction, count) in moves {
        for _ in 0..*count {
            head.move_direction(direction);
            let tail = tails.iter_mut().fold(&mut head, |lead, curr| {
                curr.follow(lead);
                curr
            });
            visited.insert(tail.to_string());
        }
    }

    visited.len()
}

fn solve_part_1(moves: &Vec<Move>) -> usize {
    solve_n_tails(moves, 1)
}

fn solve_part_2(moves: &Vec<Move>) -> usize {
    solve_n_tails(moves, 9)
}

pub(crate) fn solve(input: String) -> String {
    let moves = parse_moves(&input);

    let part_1_answer = solve_part_1(&moves);
    let part_2_answer = solve_part_2(&moves);

    format!(
        r#"
Part 1: {}
Part 2: {}
"#,
        part_1_answer, part_2_answer,
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    static INPUT: &str = "
    R 4
    U 4
    L 3
    D 1
    R 4
    D 1
    L 5
    R 2";

    #[rstest(input, expected, case(INPUT, 13))]
    fn test_part_1(input: &str, expected: usize) {
        let moves = parse_moves(input);
        assert_eq!(solve_part_1(&moves), expected);
    }

    static INPUT_2: &str = "
    R 5
    U 8
    L 8
    D 3
    R 17
    D 10
    L 25
    U 20";

    #[rstest(input, expected, case(INPUT_2, 36))]
    fn test_part_2(input: &str, expected: usize) {
        let moves = parse_moves(input);
        assert_eq!(solve_part_2(&moves), expected);
    }
}
