use miette::Result;

mod day_01;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Day 1, Part 1: {}", day_01::part_1().await?);

    Ok(())
}
