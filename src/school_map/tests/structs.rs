use std::collections::HashMap;
use crate::school_map::structs::{Node, NodeInfo, NodeKey, Params};

#[test]
fn create_node() {
    let mut edge_hash = HashMap::new();
    edge_hash.insert("edge1".to_owned(), 15.5);
    let node = Node {
        geo_coord: [0.0, 0.0],
        img_coord: [100, 100],
        edges: edge_hash,
    };
    assert_eq!(node.geo_coord, [0.0, 0.0]);
    assert_eq!(node.img_coord, [100, 100]);
    assert!(!node.edges.is_empty());
}

#[test]
fn create_node_info() {
    let node_info = NodeInfo::default();
    assert!(!node_info.locked);
    assert_eq!(node_info.loss, f64::MAX);
    assert_eq!(node_info.parent, "");
}

#[test]
fn write_node_info() {
    let mut node_info = NodeInfo::default();
    node_info.locke_node();
    node_info.set_loss(0.5);
    node_info.set_parent("parent".to_owned());
    assert!(node_info.locked);
    assert_eq!(node_info.loss, 0.5);
    assert_eq!(node_info.parent, "parent");
}

#[test]
fn node_key() {
    let mut node_key = NodeKey::new("");
    node_key.set("key");
    assert_eq!(node_key.get(), "key");
}

#[test]
fn create_params() {
    let params = Params {
        theta_deg: 0.0,
        base: [121.548763, 25.012345],
        c12: [1.1, 1.2],
        c34: [2.1, 2.2],
    };
    let theta = params.theta_deg * (std::f64::consts::PI / 180.0);
    assert_eq!(theta.cos(), 1.0);
    assert_eq!(params.base, [121.548763, 25.012345]);
    assert_eq!(params.c12, [1.1, 1.2]);
    assert_eq!(params.c34, [2.1, 2.2]);
}
