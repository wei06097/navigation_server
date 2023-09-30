use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// 型態定義
pub type NodesMap = HashMap<String, Node>;
pub type NodeInfoMap = HashMap<String, NodeInfo>;
pub type GeoCoord = [f64; 2];
pub type ImgCoord = [u64; 2];
pub type Distance = f64;

/// 地圖的每個節點
#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub geo_coord: GeoCoord,
    pub img_coord: ImgCoord,
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
    pub theta_deg: f64,
    pub base: GeoCoord,
    pub c12: [f64; 2],
    pub c34: [f64; 2],
}
