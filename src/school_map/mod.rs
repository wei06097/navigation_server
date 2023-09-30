pub mod structs;
mod utils {
    pub mod dijkstra;
    pub mod read_json;
}

pub use utils::dijkstra::dijkstra;
pub use utils::read_json::read_json;
