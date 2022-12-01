use miette::Result;

mod day_01;

#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", day_01::part_1_and_2().await?);

    Ok(())
}
