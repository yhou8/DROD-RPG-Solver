#[macro_use]
extern crate bitflags;
extern crate rust_dense_bitset;

mod drod;
use drod::{LevelInfo, Player, Search, SearchConfig};

use std::fs::File;
use std::io;

fn main() -> io::Result<()> {
    // TODO read output path from args
    let output_path = "output.txt";
    let mut output_file = File::create(output_path)?;
    let mut stdout = io::stdout();

    // TODO read config from args
    let mut search_config = SearchConfig::new(&mut output_file, &mut stdout);
    search_config.print_new_highscore = false;
    search_config.calculate_optimal_player_by_stat = false;
    search_config.print_local_optimal_player_by_score = false;
    search_config.print_local_optimal_player_by_stat = false;
    search_config.print_global_optimal_player_by_stat = false;
    let search_config = search_config;

    // TODO read level layout from file
    let level_info = LevelInfo::new();

    // TODO read initial stats from file
    let init_player = Player::new(500, 10, 10);

    let mut search = Search::new(search_config, level_info, init_player);
    search.search()?;
    Ok(())
}
