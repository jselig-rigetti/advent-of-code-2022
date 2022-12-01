// https://adventofcode.com/2022/day/1

use miette::{miette, IntoDiagnostic, Result};
use reqwest::{Client, Response};
use std::{env, fmt::Debug, ops::Add, str::FromStr};

pub(crate) async fn download_href(href: &str) -> Result<Response> {
    let cookie = env::var("COOKIE").into_diagnostic()?;

    Client::new()
        .get(href)
        .header("COOKIE", cookie)
        .send()
        .await
        .into_diagnostic()
}

pub(crate) async fn response_as_text(response: Response) -> Result<String> {
    response.text().await.into_diagnostic()
}

pub(crate) fn parse_line_groups<T>(text: &str) -> Result<Vec<Vec<T>>>
where
    T: FromStr,
    T::Err: Debug,
{
    let mut all_groups = vec![];
    let mut curr_group = vec![];

    for (i, line) in text.split('\n').enumerate() {
        if line == "" {
            all_groups.push(curr_group);
            curr_group = vec![];
        } else {
            let value = line
                .parse::<T>()
                .map_err(|e| miette!("{:?} - line {}: {:?}", e, i, line))?;
            curr_group.push(value);
        }
    }

    Ok(all_groups)
}

pub(crate) fn sum_groups<T>(groups: Vec<Vec<T>>) -> Vec<Option<T>>
where
    T: Add<Output = T> + Copy,
{
    groups
        .into_iter()
        .map(|group| {
            group.into_iter().fold(None, |acc, chunk| {
                Some(acc.map_or(chunk, |sum| sum + chunk))
            })
        })
        .collect()
}

pub(crate) fn sort_grouped_values<T>(text: &str) -> Result<Vec<(usize, Option<T>)>>
where
    T: Add<Output = T> + Copy + FromStr + Ord,
    <T as FromStr>::Err: Debug,
{
    let mut groups = parse_line_groups::<T>(text)
        .map(sum_groups)?
        .into_iter()
        .enumerate()
        .collect::<Vec<_>>();

    groups.sort_by(|(_, a), (_, b)| b.cmp(a));

    Ok(groups)
}

pub(crate) async fn part_1_and_2() -> Result<String> {
    let data = download_href("https://adventofcode.com/2022/day/1/input")
        .await
        .map(response_as_text)?
        .await
        .map(|v| sort_grouped_values(&v))??;

    let top_elf = data[0].1.unwrap_or(0);

    let top_three_elves = data[0..3]
        .iter()
        .fold(0, |acc, (_, v)| acc + v.unwrap_or(0));

    Ok(format!(
        r#"
Part 1: Top elf's calories: {}
Part 2: Top three elve's calories: {}
"#,
        top_elf, top_three_elves
    ))
}
