use std::error::Error;

use idment_lib::snowpunk::snowpunk::Snowpunk;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let a = Snowpunk::new().build().await.to_string();
    println!("{:#?}", a);
    Ok(())
}
