use crate::school_map::structs::*;

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
    node_infos.insert(source.to_string(), NodeInfo {
        locked: true,
        loss: 0.0,
        parent: String::new(),
    });
    
    let mut last_locked_node_origin = source.to_string();
    loop {
        let last_locked_node = {
            let reference = &last_locked_node_origin;
            &reference[..]
        };
        // 2. 找出與上鎖格相鄰且未上鎖的格子
        let neighbors: Vec<_> = nodes[last_locked_node].edges.keys()
            .filter(|id| {
                let id = &id[..];
                !node_infos[id].locked
            }).collect();
        for key in neighbors {
            // 3. 計算相鄰格的損耗
            let old_loss = node_infos[key].loss;
            let new_loss = node_infos[last_locked_node].loss + nodes[last_locked_node].edges[key];
            if new_loss < old_loss {
                node_infos.insert(key.to_string(), NodeInfo {
                    loss: new_loss,
                    parent: last_locked_node.to_string(),
                    ..node_infos[key]
                });
            }
        }
        // 1. 鎖住損耗最低的格子
        // ---- 找出消耗最小的
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
                let mut_reference = &mut last_locked_node_origin;
                *mut_reference = key.clone();
            }
        }
        // ---- 鎖住他
        let last_locked_node = {
            let reference = &last_locked_node_origin;
            &reference[..]
        };
        node_infos.insert(last_locked_node.to_string(), NodeInfo {
            locked: true,
            loss: node_infos[last_locked_node].loss,
            parent: node_infos[last_locked_node].parent.clone(),
        });
        // 終點鎖住 => 結束
        if last_locked_node == destination {
            break;
        }
    }
    // 從終點往回找路徑
    let mut base_path: Vec<String> = vec![];
    let mut current_origin = destination.to_string();
    loop {
        let current = {
            let current = &current_origin;
            &current[..]
        };
        base_path.push(current.to_string());
        match node_infos[current].parent.as_str() {
            "" => break,
            other => {
                let mut_current = &mut current_origin;
                *mut_current = other.to_string();
            }
        }
    }
    base_path.reverse();
    // 算總距離
    let mut totaal_distance: f64 = 0.0;
    for window in base_path.windows(2) {
        let current = &window[0];
        let next = &window[1];
        totaal_distance = totaal_distance + nodes[current].edges[next];
    }
    (base_path, totaal_distance)
}
