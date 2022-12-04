// https://adventofcode.com/2022/day/2

use std::{collections::HashSet, env, fs};

pub(crate) fn parse_file(file: &str) -> Vec<(&str, &str)> {
    file.lines()
        .map(|line| line.split_at(line.len() / 2))
        .collect::<Vec<_>>()
}

pub(crate) fn find_repeat_chars(row: (&str, &str)) -> HashSet<char> {
    let l = row.0.chars().collect::<HashSet<_>>();
    let r = row.1.chars().collect::<HashSet<_>>();
    l.intersection(&r).copied().collect()
}

pub(crate) fn score_char(c: char) -> u32 {
    let v = c.to_digit(36).expect("must be a valid digit");
    v - 9 + (if c.is_uppercase() { 26 } else { 0 })
}

pub(crate) fn score_first_repeat(repeats: HashSet<char>) -> u32 {
    repeats
        .into_iter()
        .next()
        .map(score_char)
        .unwrap_or_default()
}

pub(crate) fn part_1() -> String {
    let input = env::args()
        .nth(1)
        .map(fs::read_to_string)
        .expect("failed to read input file");

    let data = input
        .as_deref()
        .map(parse_file)
        .expect("failed to parse scores");

    let part_1_score: u32 = data
        .into_iter()
        .map(find_repeat_chars)
        .map(score_first_repeat)
        .sum();

    format!(
        r#"
Part 1: Total game score: {}
"#,
        part_1_score,
    )
}
