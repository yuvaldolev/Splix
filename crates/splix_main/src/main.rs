use splix::Splix;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut splix = Splix::new()?;
    splix.run().await?;

    Ok(())
}
