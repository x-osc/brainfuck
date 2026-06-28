use std::{
    fs,
    io::{self, BufWriter},
    path::PathBuf,
};

use clap::{Parser, Subcommand};

mod interpreter;

/// brain fuck
#[derive(Parser, Debug)]
#[command(version)]
#[command(propagate_version = true)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run a single brainfuck file
    Bf { path: PathBuf },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match &args.command {
        Commands::Bf { path } => {
            let contents = fs::read_to_string(&path)?;

            interpreter::run(contents, "".into(), BufWriter::new(io::stdout()))?;
        }
    }

    Ok(())
}
