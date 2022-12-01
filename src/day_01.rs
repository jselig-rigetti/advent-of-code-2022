// https://adventofcode.com/2022/day/1

use miette::{miette, IntoDiagnostic, Result};
use reqwest::Response;
use std::{fmt::Debug, ops::Add, str::FromStr};

pub(crate) async fn download_href(href: &str) -> Result<Response> {
    reqwest::get(href).await.into_diagnostic()
}

pub(crate) async fn response_as_text(response: Response) -> Result<String> {
    response.text().await.into_diagnostic()
}

pub(crate) fn parse_line_groups<T: FromStr>(text: &str) -> Result<Vec<Vec<T>>>
where
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

pub(crate) fn sum_groups<T: Add<Output = T> + Copy>(groups: Vec<Vec<T>>) -> Vec<Option<T>> {
    groups
        .into_iter()
        .map(|group| {
            group.into_iter().fold(None, |acc, chunk| {
                Some(acc.map_or(chunk, |sum| sum + chunk))
            })
        })
        .collect()
}

pub(crate) fn index_of_largest_group<T: PartialOrd + Copy>(
    groups: Vec<Option<T>>,
) -> Option<usize> {
    let mut index = 0;
    let mut largest: Option<T> = None;

    for (i, group) in groups.into_iter().enumerate() {
        if let Some(group) = group {
            largest = Some(largest.map_or_else(
                || group,
                |largest| {
                    if group < largest {
                        largest
                    } else {
                        index = i;
                        group
                    }
                },
            ))
        }
    }

    match largest {
        Some(_) => Some(index),
        None => None,
    }
}

pub(crate) async fn part_1() -> Result<String> {
    let elf_index = download_href("https://adventofcode.com/2022/day/1/input")
        .await
        .map(response_as_text)?
        .await
        .map(|text| parse_line_groups::<u16>(&text))?
        .map(sum_groups)
        .map(index_of_largest_group)?
        .unwrap_or(0);

    Ok(format!("Most caloric elf: {}", elf_index + 1))
}
