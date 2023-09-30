use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// 型態定義
pub type NodesMap = HashMap<String, Node>;
pub type NodeInfoMap = HashMap<String, NodeInfo>;
pub type Distance = f64;

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

/// 座標轉換函數的參數
#[derive(Debug, Serialize, Deserialize)]
pub struct Params {
    theta_deg: f64,
    base: Vec<f64>,
    c12: Vec<f64>,
    c34: Vec<f64>,
}
