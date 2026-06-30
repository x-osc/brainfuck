use std::{
    fs,
    io::{self, BufWriter},
    path::PathBuf,
};

use clap::{Args, Parser, Subcommand};

mod interpreter;

/// brain fuck
#[derive(Parser, Debug)]
#[command(version)]
#[command(propagate_version = true)]
struct CliArgs {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run a single brainfuck file
    Bf(BfArgs),
}

#[derive(Args, Debug)]
struct BfArgs {
    path: PathBuf,
    #[arg(short, long)]
    input: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();

    match &args.command {
        Commands::Bf(bf_args) => {
            let contents = fs::read_to_string(&bf_args.path)?;

            interpreter::run(
                &contents,
                &bf_args.input.as_deref().unwrap_or(""),
                BufWriter::new(io::stdout()),
            )?;
        }
    }

    Ok(())
}
