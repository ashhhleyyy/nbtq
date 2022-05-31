use std::{fs::File, io, path::PathBuf};

use clap::Parser;
use color_eyre::Result;

/// Convert NBT files into JSON for easy processing with tools like `jq`
#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    input: PathBuf,
    #[clap(short, long)]
    pretty: bool,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    let mut input = File::open(cli.input)?;
    let nbt = nbtq::detect_and_read_nbt(&mut input)?;
    let output = io::stdout();
    if cli.pretty {
        serde_json::to_writer_pretty(output.lock(), &nbt)?;
    } else {
        serde_json::to_writer(output.lock(), &nbt)?;
    }
    Ok(())
}
