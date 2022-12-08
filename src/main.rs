#![feature(iter_array_chunks)]

use std::{env, fs};

use miette::Result;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;

#[tokio::main]
async fn main() -> Result<()> {
    let input = env::args().nth(1).map(fs::read_to_string).unwrap().unwrap();

    // println!("{}", day_01::part_1_and_2().await?);
    // println!("{}", day_02::part_1_and_2());
    // println!("{}", day_03::part_1());
    // println!("{}", day_04::solve());
    // println!("{}", day_05::solve());
    // println!("{}", day_06::solve(input));
    println!("{}", day_07::solve(input));

    Ok(())
}
