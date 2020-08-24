mod model;
mod search;

pub use model::LevelInfo;
pub use search::Search;
pub use search::SearchConfig;

type VertexIDType = u8;

trait Ge<Rhs = Self> {
    fn ge(&self, other: &Rhs) -> bool;
}
