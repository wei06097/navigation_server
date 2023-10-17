use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Node HashMap
pub type NodesMap = HashMap<String, Node>;
/// NodeInfo HashMap
pub type NodeInfoMap = HashMap<String, NodeInfo>;
/// 經緯度座標
pub type GeoCoord = [f64; 2];
/// 平面圖像素座標
pub type ImgCoord = [u64; 2];
/// Node 相鄰點距離
pub type Distance = f64;

/// 校內地圖 Graph 資訊裡的單個點
/// # 元素
/// - geo_coord: 經緯度座標
/// - img_coord: 平面圖像素座標
/// - edges: 相鄰點編號:距離
#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub geo_coord: GeoCoord,
    pub img_coord: ImgCoord,
    pub edges: HashMap<String, Distance>,
}

/// dijkstra 演算法用
/// # 元素
/// - locked: 是否不再更新損失
/// - loss: 損耗
/// - parent: 父節點
/// # 方法
/// - locke_node(): 停止更新損失
/// - set_loss(): 紀錄損耗
/// - set_parent(): 停止更新損失後紀錄父節點
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
    pub fn locke_node(&mut self) {
        self.locked = true;
    }
    pub fn set_loss(&mut self, loss: Distance) {
        self.loss = loss;
    }
    pub fn set_parent(&mut self, parent: String) {
        self.parent = parent;
    }
}

/// dijkstra 演算法用
pub struct NodeKey {
    pub key: String
}

impl NodeKey {
    pub fn new(key: &str) -> NodeKey {
        NodeKey {
            key: key.to_owned()
        }
    }
    pub fn get(&self) -> &str {
        &self.key
    }
    pub fn set(&mut self, key: &str) {
        self.key = key.to_owned();
    }
}

/// 座標轉換函數的參數
/// # 元素
/// - theta_deg: 經緯度座標與平面圖座標的角度差
/// - base: 平面圖在原點時對應的經緯度
/// - c12: 旋轉矩陣參數 (理論上和 c34 反比)
/// - c34: 旋轉矩陣參數
#[derive(Debug, Serialize, Deserialize)]
pub struct Params {
    pub theta_deg: f64,
    pub base: GeoCoord,
    pub c12: [f64; 2],
    pub c34: [f64; 2],
}
