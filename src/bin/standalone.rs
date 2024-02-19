use anyhow::Result;
use hpa_demo::run_server;

#[tokio::main]
async fn main() -> Result<()> {
    run_server().await?;
    Ok(())
}
