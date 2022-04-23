use std::io::{self, Write, Read};
use std::fs::File;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf
}

fn main() {

    // parse cli args
    let args = Cli::parse();

    // open file
    let file = File::open(args.path).expect("could not open file");

    // load file to reader
    let mut reader = io::BufReader::new(file);

    
    // read from buffer into string
    let mut content = String::new();
    reader.read_to_string(&mut content).expect("could not read file");

    // output to standard output buffer
    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = io::BufWriter::new(stdout);
    writeln!(handle, "{}", content).expect("could not output data");
}
