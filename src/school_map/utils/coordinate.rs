use crate::school_map::structs::{Params, GeoCoord, ImgCoord, NodesMap};

/// 平面圖像素座標轉經緯度座標
pub fn img_to_geo(params: &Params, xy: ImgCoord) -> GeoCoord {
    let x = xy[0] as f64;
    let y = xy[1] as f64;
    let [a, b] = &params.base;
    let [c1, c2] = &params.c12;
    let theta = &params.theta_deg * (std::f64::consts::PI / 180.0);
    let lon = c1 * (theta.cos() * x + theta.sin() * y) + a;
    let lat = c2 * (theta.sin() * x - theta.cos() * y) + b;
    [lon, lat]
}

/// 經緯度座標轉平面圖像素座標
pub fn geo_to_img(params: &Params, lonlat: GeoCoord) -> ImgCoord {
    let [a, b] = &params.base;
    let [c3, c4] = &params.c34;
    let theta = &params.theta_deg * (std::f64::consts::PI / 180.0);
    let lon = lonlat[0] - a;
    let lat = lonlat[1] - b;
    let x = c3 * (theta.cos() * lon + theta.sin() * lat);
    let y = c4 * (theta.sin() * lon - theta.cos() * lat);
    let x = x.round() as u64;
    let y = y.round() as u64;
    [x, y]
}

/// 取得已標記點內最近的點
pub fn get_nearby_node(nodes: &NodesMap, xy: ImgCoord) -> String {
    let distance_vec: Vec<(String, u64)> = nodes.keys()
        .map(|key| {
            let point = nodes[key].img_coord;
            let x1 = xy[0] as i64;
            let y1 = xy[1] as i64;
            let x2 = point[0] as i64;
            let y2 = point[1] as i64;
            let [dx, dy] = [x2-x1, y2-y1];
            let distance = ((dx*dx + dy*dy) as f64).sqrt() as u64;
            (key.clone(), distance)
        }).collect();
    match distance_vec.iter().min_by_key(|(_, distance)| distance) {
        Some((key, _)) => key.clone(),
        _ => String::new(),
    }
}

/// 經緯度差算距離
pub fn haversine_formula([lon1, lat1]: GeoCoord, [lon2, lat2]: GeoCoord) -> f64 {
    // 將經緯度轉為弧度
    let lat1_rad = lat1.to_radians();
    let lon1_rad = lon1.to_radians();
    let lat2_rad = lat2.to_radians();
    let lon2_rad = lon2.to_radians();
    // 地球的半徑 (km)
    const EARTH_RADIUS: f64 = 6371.0;
    // 半正矢公式 (Haversine formula)
    let dlon = lon2_rad - lon1_rad;
    let dlat = lat2_rad - lat1_rad;
    let a = (dlat / 2.0).sin().powi(2) + lat1_rad.cos() * lat2_rad.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    let distance = EARTH_RADIUS * c;
    
    distance * 1000.0
}
