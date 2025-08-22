use clap::Parser;

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

fn main() {
    let args = Args::parse();
    println!("{args:#?}");
}
