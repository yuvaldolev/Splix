use splix::Splix;

fn main() -> anyhow::Result<()> {
    let splix = Splix::new();
    splix.run();

    Ok(())
}
