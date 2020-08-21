mod model;
mod solver;

// pub use model::Level;
// pub use model::PlayerStat;
// pub use solver::Solver;

type VertexIDType = u8;

trait Ge<Rhs = Self> {
    fn ge(&self, other: &Rhs) -> bool;
}
