use std::error::Error;

use idment_lib::snowpunk::snowpunk::snowpunk;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let a = snowpunk().await?;
    println!("{:#?}", a);
    Ok(())
}
