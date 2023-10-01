pub mod structs;
mod utils {
    pub mod dijkstra;
    pub mod read_json;
    pub mod coordinate;
}

pub use utils::dijkstra::dijkstra;
pub use utils::read_json::read_json;
pub use utils::coordinate;

// 測試
#[cfg(test)]
mod tests {
    mod structs;
    mod utils;
}
