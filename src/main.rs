fn main() -> anyhow::Result<()> {
    let (old, new) = name_current_branch::Renamer::try_new(".")?.name_auto()?;
    println!("{old} → {new}");

    Ok(())
}
