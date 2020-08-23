extern crate bitflags;
extern crate lazy_static;
extern crate rust_dense_bitset;
extern crate serde_json;
extern crate structopt;

mod drod;

use drod::{LevelInfo, Search, SearchConfig};

use structopt::StructOpt;

use std::fs;
use std::fs::File;
use std::io;
use std::path::PathBuf;

#[derive(StructOpt)]
#[structopt(no_version, about)]
struct Config {
    #[structopt(flatten)]
    search_config: SearchConfig,

    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// Output file
    #[structopt(parse(from_os_str))]
    output: PathBuf,
}

fn main() -> io::Result<()> {
    let config = Config::from_args();
    let input_data = fs::read(config.input)?;
    let json_value = serde_json::from_slice(&input_data)?;
    let level_info = LevelInfo::new(json_value)?;

    let mut output_file = File::create(config.output)?;
    let mut stdout = io::stdout();
    let mut search = Search::new(
        config.search_config,
        level_info,
        &mut output_file,
        &mut stdout,
    );
    search.search()
}
