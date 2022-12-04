// https://adventofcode.com/2022/day/2

use std::{collections::HashSet, env, fs};

pub(crate) fn parse_file(file: &str) -> Vec<(&str, &str)> {
    file.lines()
        .map(|line| line.split_at(line.len() / 2))
        .collect::<Vec<_>>()
}

pub(crate) fn parse_file_part_2(file: &str) -> Vec<Vec<&str>> {
    file.lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| chunk.to_vec())
        .collect()
}

pub(crate) fn find_repeat_chars(row: (&str, &str)) -> HashSet<char> {
    let l = row.0.chars().collect::<HashSet<_>>();
    let r = row.1.chars().collect::<HashSet<_>>();
    l.intersection(&r).copied().collect()
}

pub(crate) fn find_repeat_chars_part_2(group: Vec<&str>) -> HashSet<char> {
    group
        .into_iter()
        .map(|line| line.chars().collect::<HashSet<_>>())
        .fold(None, |acc: Option<HashSet<char>>, next| match acc {
            None => Some(next),
            Some(curr) => Some(curr.intersection(&next).copied().collect()),
        })
        .expect("expected at least one group member")
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

    let data_part_1 = input
        .as_deref()
        .map(parse_file)
        .expect("failed to parse scores");

    let part_1_score: u32 = data_part_1
        .into_iter()
        .map(find_repeat_chars)
        .map(score_first_repeat)
        .sum();

    let data_part_2 = input
        .as_deref()
        .map(parse_file_part_2)
        .expect("failed to parse scores");

    let part_2_score: u32 = data_part_2
        .into_iter()
        .map(find_repeat_chars_part_2)
        .map(score_first_repeat)
        .sum();

    format!(
        r#"
Part 1: Total game score: {}
Part 2: Total game score: {}
"#,
        part_1_score, part_2_score,
    )
}
