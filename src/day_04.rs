// https://adventofcode.com/2022/day/2

use std::{collections::HashSet, env, fs, num::ParseIntError, str::FromStr};

pub(crate) struct Assignment {
    lo: u32,
    hi: u32,
}

impl FromStr for Assignment {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ints = s
            .split('-')
            .map(|x| x.parse::<u32>().expect("expected int"));
        Ok(Self {
            lo: ints.next().expect("expected first low int"),
            hi: ints.next().expect("expected second high int"),
        })
    }
}

impl Assignment {
    fn includes(&self, other: &Self) -> bool {
        self.lo <= other.lo && self.hi >= other.hi
    }
}

pub(crate) fn parse_file(file: &str) -> Vec<(Assignment, Assignment)> {
    file.lines()
        .map(|line| {
            let mut assignments = line
                .split(',')
                .map(|part| part.parse::<Assignment>().expect("expected assignment"));
            (
                assignments.next().expect("expected first assignment"),
                assignments.next().expect("expected second assignment"),
            )
        })
        .collect::<Vec<_>>()
}

pub(crate) fn score_group(assignment: (Assignment, Assignment)) -> u32 {
    let (l, r) = assignment;
    if l.includes(&r) || r.includes(&l) {
        1
    } else {
        0
    }
}

pub(crate) fn score_all_groups(assignments: Vec<(Assignment, Assignment)>) -> Vec<u32> {
    assignments.into_iter().map(score_group).collect()
}

pub(crate) fn solve() -> String {
    let input = env::args()
        .nth(1)
        .map(fs::read_to_string)
        .expect("failed to read input file");

    let data_part_1 = input.as_deref().map(parse_file);

    let part_1_score: u32 = data_part_1
        .map(score_all_groups)
        .expect("failed to parse scores")
        .into_iter()
        .sum();

    format!(
        r#"
Part 1: Total game score: {}
"#,
        part_1_score,
    )
}
