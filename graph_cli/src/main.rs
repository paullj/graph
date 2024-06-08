use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use graph_core;
use std::fs;
use tempfile::Builder;

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
    /// Open the output file in the default browser
    #[arg(short = 'b', long, default_value_t = false)]
    open: bool,
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

    let content = fs::read_to_string(&args.input_path)
        .with_context(|| format!("could not read file `{}`", args.input_path.display()))?;

    let test = graph_core::generate_graph(&content);
    match test {
        Ok(graph) => {
            let clone = graph.clone();
            if let Some(output_path) = &args.output_path {
                fs::write(output_path, clone).with_context(|| {
                    format!(
                        "Could not write to output file at `{}`",
                        output_path.display()
                    )
                })?;
            } else {
                // TODO: Print the graph to the console (with formatting?)
                // println!("{}", clone);
            }
            // Open the output file in the browser
            if args.open {
                // Create a temporary file
                let temp_file = Builder::new()
                    .suffix(".svg")
                    .tempfile()
                    .with_context(|| "Could not create temporary file")?;
                // Write the graph to the temporary file
                fs::write(temp_file.path(), graph)?;
                // Open the temporary file in the browser
                let output_path = temp_file
                    .path()
                    .to_str()
                    .with_context(|| "Could not convert path to string")?;
                // Open the file in the browser
                let _ = webbrowser::open(output_path)
                    .with_context(|| "Could not open graph with browser")?;
                // wait for 1s to allow the browser to open
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
        Err(error) => eprintln!("{error}"),
    }

    Ok(())
}
