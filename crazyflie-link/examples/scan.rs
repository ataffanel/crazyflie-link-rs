use anyhow::Result;
use crazyflie_link::LinkContext;

#[async_std::main]
async fn main() -> Result<()> {
    let context = crate::LinkContext::new(async_executors::AsyncStd);

    let found = context.scan([0xe7; 5]).await?;

    println!("Found {} Crazyflies.", found.len());
    for uri in found {
        println!(" - {}", uri)
    }

    Ok(())
}
