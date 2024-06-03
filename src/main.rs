use clap::Parser;
use clap_stdin::FileOrStdin;

/// Format either the stdin or a file
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Either stdin or a file
    input: FileOrStdin,
}

fn main() -> miette::Result<()> {
    let args = Args::parse();
    let input = args.input.contents().unwrap();
    let mut res = input.parse::<kdl::KdlDocument>()?;

    res.fmt();

    println!("{}", res);

    Ok(())
}
