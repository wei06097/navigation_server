use super::types::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// 地圖的每個節點
#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub geo_coord: [f64; 2],
    pub img_coord: [u64; 2],
    pub edges: HashMap<String, Distance>,
}

/// dijkstra 演算法用
#[derive(Debug, Serialize, Deserialize)]
pub struct NodeInfo {
    pub locked: bool,
    pub loss: Distance,
    pub parent: String,
}

impl NodeInfo {
    pub fn default() -> NodeInfo {
        NodeInfo {
            locked: false,
            loss: Distance::MAX,
            parent: String::new(),
        }
    }
}
