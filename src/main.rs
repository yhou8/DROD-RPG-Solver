#[macro_use]
extern crate bitflags;
extern crate rust_dense_bitset;

mod drod;
use drod::{Level, Player, RouteState, Search};

fn main() {
    let init_player = Player::default();
    loop {
        let level = Level::new();
        let mut search = Search::new(level, init_player);
        let _init_step = RouteState::with_player(init_player);
        search.search();
    }
}
