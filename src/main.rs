#![feature(iter_array_chunks)]

use miette::Result;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;

#[tokio::main]
async fn main() -> Result<()> {
    // println!("{}", day_01::part_1_and_2().await?);
    // println!("{}", day_02::part_1_and_2());
    // println!("{}", day_03::part_1());
    // println!("{}", day_04::solve());
    println!("{}", day_05::solve());

    Ok(())
}
