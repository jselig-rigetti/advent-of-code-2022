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

pub(crate) fn get_player_play(outcome: &char, elf: &char) -> char {
    match (outcome, elf) {
        ('X', 'A') | ('Y', 'C') | ('Z', 'B') => 'Z',
        ('X', 'B') | ('Y', 'A') | ('Z', 'C') => 'X',
        ('X', 'C') | ('Y', 'B') | ('Z', 'A') => 'Y',
        _ => unreachable!("invalid plays: {} vs {}", outcome, elf),
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

pub(crate) fn collect_scores(games: &Vec<(char, char)>) -> Vec<u32> {
    games
        .iter()
        .map(|(elf, our)| score_game(elf, our))
        .collect()
}

pub(crate) fn part_1_and_2() -> String {
    let games = env::args()
        .nth(1)
        .map(fs::read_to_string)
        .expect("failed to read input file")
        .as_deref()
        .map(parse_file)
        .expect("failed to parse scores");

    let part_1_scores: u32 = collect_scores(&games).iter().sum();

    let games = games
        .into_iter()
        .map(|(elf, outcome)| (elf, get_player_play(&outcome, &elf)))
        .collect::<Vec<_>>();

    let part_2_scores: u32 = collect_scores(&games).iter().sum();

    format!(
        r#"
Part 1: Total game score: {}
Part 2: Total game score: {}
"#,
        part_1_scores, part_2_scores,
    )
}
