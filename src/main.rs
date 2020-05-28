mod parser;

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use anyhow::{Context, Result};
use structopt::StructOpt;

use parser::{LogCount, LogEntry};

// The usual command line arguments declaration
#[derive(StructOpt)]
#[structopt(
    name = "octolog",
    about = "Count bytes from a JSON log file",
    author = "Aitor Ruano <codearm@pm.me>"
)]
struct Opt {
    #[structopt(parse(from_os_str), help = "input log file to parse")]
    input: PathBuf,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let f = File::open(opt.input).context("Failed to open input log file")?;
    let reader = BufReader::new(f);
    let stream = serde_json::Deserializer::from_reader(reader).into_iter::<LogEntry>();

    let mut count = LogCount::new();
    for log in stream {
        count.insert(log.context("Failed parsing JSON text from input log file")?);
    }

    print!("{}", count);
    Ok(())
}
