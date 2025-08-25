use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `head`
struct Args {
    /// Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// Number of lines to print
    #[arg(
        short = 'n',
        long = "lines",
        default_value = "10",
        conflicts_with = "bytes",
        help = "Number of lines",
        long_help = "Number of lines to print",
    )]
    lines: u64,

    /// Number of bytes to print
    #[arg(
        short = 'c',
        long = "bytes",
        help = "Number of bytes",
        long_help = "Number of bytes to print",
        value_parser = clap::value_parser!(u64).range(1..),
    )]
    bytes: Option<u64>,
}

fn open(filename: &str) -> Result<Box<dyn BufRead>, Box<dyn std::error::Error>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn output_lines(buf: Box<dyn BufRead>, lines: u64, bytes: Option<u64>) -> io::Result<()> {
    //     let mut line_no_all: usize = 1;
    //     let mut line_no_nonblank: usize = 1;
    //
    for line in buf.lines() {
        let line = line?;
        //         if number_all {
        //             println!("{line_no_all:>6}\t{line}");
        //             line_no_all += 1;
        //         } else if number_nonblank {
        //             if line.is_empty() {
        //                 println!();
        //             } else {
        //                 println!("{line_no_nonblank:>6}\t{line}");
        //                 line_no_nonblank += 1;
        //             }
        //         } else {
        println!("{line}");
        //         }
    }
    Ok(())
}
fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    println!("{:#?}", args);
    let num_files = args.files.len();

    for (file_num, filename) in args.files.iter().enumerate() {
        match open(filename) {
            Err(err) => eprintln!("Failed to open {filename}: {err}"),
            Ok(mut file) => {
                if num_files > 1 {
                    println!(
                        "{}==> {filename} <==",
                        if file_num > 0 { "\n" } else { "" },
                    );
                }
            }
        }
    }
    //
    // let lines = args.lines;
    // let bytes = args.bytes;
    //
    // for filename in args.files {
    //     match open(&filename) {
    //         Err(err) => eprintln!("Failed to open {filename}: {err}"),
    //         Ok(buf) => {
    //             if let Err(err) = output_lines(buf, lines, bytes) {
    //                 eprintln!("Failed to read {filename}: {err}");
    //             }
    //         }
    //     }
    // }

    Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}