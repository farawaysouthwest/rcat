use std::io::{self, Write, Error, BufRead};
use std::fs::File;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf
}

fn main() -> Result<(), Error>{

    // standard output buffer.
    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = io::BufWriter::new(stdout);

    // parse cli args.
    let args = Cli::parse();

    // load file to reader.
    let file = File::open(args.path).expect("could not open file");
    let buffer = io::BufReader::new(file);

    // iterate over lines in buffer, write to stdout.
    for line in buffer.lines() {
        writeln!(handle, "{}", line.unwrap_or_default())?;
    }
    Ok(())
}
