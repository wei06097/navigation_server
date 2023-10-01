use std::collections::HashMap;
use crate::school_map::structs::{Node, NodeInfo, Params};

#[test]
fn node_creation() {
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
fn node_info_default() {
    let node_info = NodeInfo::default();
    assert!(!node_info.locked);
    assert_eq!(node_info.loss, f64::MAX);
    assert_eq!(node_info.parent, "");
}

#[test]
fn params_creation() {
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
