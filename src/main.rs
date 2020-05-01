#[macro_use]
extern crate bitflags;

extern crate rust_dense_bitset;

mod essplayer;
mod level;
mod search;
mod stat;

fn main() {
    let init_stat = stat::PlayerStat::default();
    loop {
        let level = level::Level::new();
        let mut search = search::Search::new(level, init_stat);
        let _ = essplayer::EssPlayer::with_stat(init_stat);
        search.search();
    }
}
