use miette::Result;

mod day_01;
mod day_02;
mod day_03;

#[tokio::main]
async fn main() -> Result<()> {
    // println!("{}", day_01::part_1_and_2().await?);
    // println!("{}", day_02::part_1_and_2());
    println!("{}", day_03::part_1());

    Ok(())
}
