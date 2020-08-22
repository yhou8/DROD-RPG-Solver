mod model;
mod solver;

pub use model::LevelInfo;
pub use solver::Player;
pub use solver::Search;
pub use solver::SearchConfig;

type VertexIDType = u8;

trait Ge<Rhs = Self> {
    fn ge(&self, other: &Rhs) -> bool;
}
