use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use structopt::StructOpt;
use std::fs;
use std::fs::File;

/// Like Unix Cat but written in Rust
#[derive(StructOpt, Debug)]
#[structopt(name = "rat")]
struct Opt {

    /// File(s) to process
    #[structopt(name = "FILE", parse(from_os_str))]
    file: Vec<PathBuf>,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();

    if opt.file.is_empty() {
        eprintln!("No input files specified\n");
    }

    for path in &opt.file {
        let meta = fs::metadata(&path).expect("Unable to Open");
        if meta.is_file() {
            let f = File::open(&path).expect("[rat Error] File not found.");
            let reader = BufReader::new(&f);
            for line in reader.lines() {
                let line = line?;
                println!("{}", line);
            }
        }
    }
    Ok(())
}