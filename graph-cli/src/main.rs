use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use graph_core;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(next_line_help = true)]
struct Arguments {
    /// The path to the .graph file to read
    #[arg(short, long)]
    input_path: std::path::PathBuf,
    /// The path to output to
    #[arg(short, long)]
    output_path: Option<std::path::PathBuf>,
    /// The file format of the output file
    #[arg(value_enum, short, long, default_value_t = OutputFormat::Svg)]
    format: OutputFormat,
}

// TODO: Move this into core?
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum OutputFormat {
    Svg,
    Png,
    Jpg,
}

fn main() -> Result<()> {
    let args = Arguments::parse();

    let content = std::fs::read_to_string(&args.input_path)
        .with_context(|| format!("could not read file `{}`", args.input_path.display()))?;

    let test = graph_core::generate_graph(&content);
    match test {
        Ok(graph) => println!("{graph}"),
        Err(_) => (),
    }

    Ok(())
}
