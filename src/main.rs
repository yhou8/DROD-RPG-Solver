#[macro_use]
extern crate bitflags;
extern crate rust_dense_bitset;

mod drod;
use drod::{Level, PlayerStat, Solver};

fn main() {
    let init_player = PlayerStat::default();
    let level = Level::new();
    let solver = Solver::new(level, init_player);

    println!("--------------------------------------------------------------------------------");
    match solver.find_solution() {
        None => println!("Cannot find route through level"),
        Some(solution) => println!("{}", solution),
    }
    println!("--------------------------------------------------------------------------------");
}
