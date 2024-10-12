use anyhow::Result;
use clap::Parser;
use colored::Color;
use greprs::{input::*, print::*, search::*};
use std::str::FromStr;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    ignore_case: bool,

    #[clap(short = 'v', long)]
    invert_match: bool,

    #[clap(short, long)]
    regex: bool,

    #[clap(short, long)]
    color: Option<String>, // Accept color as a string

    needle: String,
    file: Option<std::path::PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Get the input source based on the file argument
    let mut input_source: Box<dyn InputSource> = if let Some(path) = args.file {
        Box::new(FileInput::new(path)?)
    } else {
        Box::new(StdinInput::new())
    };

    // Choose the appropriate searcher based on the `--regex` flag
    let searcher: Box<dyn Searcher> = if args.regex {
        Box::new(RegexSearcher::new(&args.needle, args.ignore_case)?)
    } else if args.ignore_case {
        Box::new(CaseInsensitiveSearcher::new(&args.needle))
    } else {
        Box::new(ExactSearcher::new(&args.needle))
    };

    // Handle the --color flag (convert color name to lowercase before parsing)
    let mut printer: Box<dyn Printer<std::io::Stdout>> = if let Some(color_name) = args.color {
        let color_name_lower = color_name.to_lowercase();

        // Convert the error from `Color::from_str` to an `anyhow::Error`
        let color = Color::from_str(&color_name_lower)
            .map_err(|_| anyhow::anyhow!("Invalid color option: {}", color_name_lower))?;

        Box::new(ColoredPrinter::new(std::io::stdout(), color))
    } else {
        Box::new(PlainPrinter::new(std::io::stdout()))
    };

    // Search and print results
    for line in input_source.lines() {
        let line = line?;
        if searcher.search(&line) ^ args.invert_match {
            printer.print(&line, &args.needle)?;
        }
    }

    Ok(())
}
