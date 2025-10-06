use clap::Parser;

/// Automatically generate meaningful git branch names using Claude Code
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// Target branch to compare against (defaults to repository default branch)
    target_branch: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let (old, new) =
        name_current_branch::Renamer::try_new(".")?.name_auto(cli.target_branch.as_deref())?;
    println!("{old} → {new}");

    Ok(())
}
