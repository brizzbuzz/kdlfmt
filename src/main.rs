use clap::Parser;
use clap_stdin::FileOrStdin;

/// Format either the stdin or a file
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Either stdin or a file
    input: FileOrStdin,
}

fn main() {
    let args = Args::parse();
    let input = args.input.contents().unwrap();
    let mut doc = kdl::KdlDocument::new();
    doc.nodes_mut().push(input.parse().unwrap());
    // TODO: Should write to file instead of stdout if a file is provided
    println!("{}", &doc.to_string());
}
