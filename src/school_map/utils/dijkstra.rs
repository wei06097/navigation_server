use crate::school_map::structs::*;

/// 最佳路徑演算法
/// # 說明
/// - 輸入
///     - 校內地圖 Graph 資訊
///     - 起點編號
///     - 終點編號
/// - 輸出
///     - 最佳路徑(編號)
///     - 總距離
///
/// # Examples
/// 需要 [school_map::read_json](./fn.read_json.html) 函數讀取 Graph 資訊
/// ```
/// use crate::navigation_server::school_map::structs::NodesMap;
/// use crate::navigation_server::school_map::read_json;
/// use crate::navigation_server::school_map::dijkstra;
///
/// let path = vec!["src", "assets", "data.json"]; //檔案路徑
/// let nodes = read_json::<NodesMap>(&path).unwrap(); //讀取檔案並轉為結構
/// let (best_path, distance) = dijkstra(&nodes, "1", "10");
/// ```
pub fn dijkstra(nodes: &NodesMap, source: &str, destination: &str) -> (Vec<String>, f64) {
    if source == destination {
        return (vec![], 0.0);
    }
    // 產生 NodeInfo 的 HashMap，並初始化
    let mut node_infos: NodeInfoMap = Default::default();
    for key in nodes.keys() {
        node_infos.insert(key.clone(), NodeInfo::default());
    }
    // 1. 鎖住損耗最低的格子(起點)
    let object_mut = node_infos.get_mut(source).unwrap();
    object_mut.locke_node();
    object_mut.set_loss(0.0);

    let mut last_locked = NodeKey::new(source);
    loop {
        let last_locked_node = last_locked.get();
        // 2. 找出與上鎖格相鄰且未上鎖的格子
        let neighbors: Vec<_> = nodes[last_locked_node].edges.keys()
            .filter(|id| {
                let id = &id[..];
                !node_infos[id].locked
            }).collect();
        for key in neighbors {
            // 3. 計算相鄰格的損耗
            let old_loss = node_infos[key].loss;
            let new_loss = node_infos[last_locked_node].loss 
                + nodes[last_locked_node].edges[key];
            if new_loss < old_loss {
                let object_mut = node_infos.get_mut(key).unwrap();
                object_mut.set_loss(new_loss);
                object_mut.set_parent(last_locked_node.to_owned());
            }
        }
        // 1. 鎖住損耗最低的格子
        let mut min: Distance = Distance::MAX;
        let unlocked_vec: Vec<_> = node_infos.keys()
            .filter(|id| {
                let id = &id[..];
                node_infos[id].parent != "" && !node_infos[id].locked
            }).collect();
        for key in unlocked_vec {
            let loss = node_infos[key].loss;
            if loss < min {
                min = loss;
                last_locked.set(key);
            }
        }
        let last_locked_node = last_locked.get();
        let parent = node_infos[last_locked_node].parent.clone();
        let object_mut = node_infos.get_mut(last_locked_node).unwrap();
        object_mut.set_parent(parent);
        object_mut.locke_node();

        // 終點鎖住 => 結束
        if last_locked_node == destination {
            break;
        }
    }
    // 從終點往回找路徑
    let mut best_path: Vec<String> = vec![];
    let mut current = NodeKey::new(destination);
    loop {
        best_path.push(current.get().to_owned());
        match node_infos[current.get()].parent.as_str() {
            "" => break,
            other => current.set(other)
        }
    }
    best_path.reverse();
    // 算總距離
    let mut total_distance: f64 = 0.0;
    for window in best_path.windows(2) {
        let current = &window[0];
        let next = &window[1];
        total_distance = total_distance + nodes[current].edges[next];
    }
    (best_path, total_distance)
}
