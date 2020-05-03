#[macro_use]
extern crate bitflags;
extern crate rust_dense_bitset;

mod drod;
use drod::{Level, PlayerStat, RouteState, Search};

fn main() {
    let init_stat = PlayerStat::default();
    loop {
        let level = Level::new();
        let mut search = Search::new(level, init_stat);
        let _init_step = RouteState::with_stat(init_stat);
        search.search();
    }
}
