use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Parser)]
#[command(version)]
struct Args {
    /// Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// Number lines
    #[arg(
        short = 'n',
        long = "number",
        visible_aliases = ["number-all", "lines"],
        conflicts_with = "number_nonblank_lines",
        help = "Number all output lines",
        long_help = "Number all output lines, including blank lines. Use -b/--number-nonblank to number only non-blank lines."
    )]
    number_lines: bool,

    /// Number non-blank lines
    #[arg(
        short = 'b',
        long = "number-nonblank",
        visible_aliases = ["number-non-blank", "nonblank"],
        help = "Number non-blank output lines",
        long_help = "Number only non-blank output lines. Blank lines are printed but not numbered."
    )]
    number_nonblank_lines: bool,
}

fn open(filename: &str) -> Result<Box<dyn BufRead>, Box<dyn std::error::Error>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn output_lines(buf: Box<dyn BufRead>, number_all: bool, number_nonblank: bool) -> io::Result<()> {
    let mut line_no_all: usize = 1;
    let mut line_no_nonblank: usize = 1;

    for line in buf.lines() {
        let line = line?;
        if number_all {
            println!("{line_no_all:>6}\t{line}");
            line_no_all += 1;
        } else if number_nonblank {
            if line.is_empty() {
                println!();
            } else {
                println!("{line_no_nonblank:>6}\t{line}");
                line_no_nonblank += 1;
            }
        } else {
            println!("{line}");
        }
    }
    Ok(())
}
fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let number_all = args.number_lines;
    let number_nonblank = args.number_nonblank_lines;

    for filename in args.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {filename}: {err}"),
            Ok(buf) => {
                if let Err(err) = output_lines(buf, number_all, number_nonblank) {
                    eprintln!("Failed to read {filename}: {err}");
                }
            }
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
