// https://adventofcode.com/2022/day/2

use std::{cmp::Ordering, env, fs};

pub(crate) fn get_player_game(elf: &char, our: &char) -> Ordering {
    match (elf, our) {
        ('A', 'X') | ('B', 'Y') | ('C', 'Z') => Ordering::Equal,
        ('A', 'Y') | ('B', 'Z') | ('C', 'X') => Ordering::Greater,
        ('A', 'Z') | ('B', 'X') | ('C', 'Y') => Ordering::Less,
        _ => unreachable!("invalid plays: {} vs {}", elf, our),
    }
}

pub(crate) fn get_game_points(elf: &char, our: &char) -> u32 {
    match get_player_game(elf, our) {
        Ordering::Less => 0,
        Ordering::Equal => 3,
        Ordering::Greater => 6,
    }
}

pub(crate) fn get_play_points(our: &char) -> u32 {
    match our {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => unreachable!("invalid play: {}", our),
    }
}

pub(crate) fn score_game(elf: &char, our: &char) -> u32 {
    get_game_points(elf, our) + get_play_points(our)
}

pub(crate) fn parse_file(file: &str) -> Vec<(char, char)> {
    file.lines()
        .map(|line| {
            let mut chars = line.chars();
            let elf = chars.next().unwrap();
            chars.next();
            let our = chars.next().unwrap();
            (elf, our)
        })
        .collect::<Vec<_>>()
}

pub(crate) fn collect_scores(games: Vec<(char, char)>) -> Vec<u32> {
    games
        .iter()
        .map(|(elf, our)| score_game(elf, our))
        .collect()
}

pub(crate) fn part_1() -> String {
    let scores = env::args()
        .nth(1)
        .map(fs::read_to_string)
        .expect("failed to read input file")
        .as_deref()
        .map(parse_file)
        .map(collect_scores)
        .expect("failed to parse scores");

    let player_total_score: u32 = scores.iter().sum();

    format!(
        r#"
Part 1: Total game score: {}
"#,
        player_total_score,
    )
}
