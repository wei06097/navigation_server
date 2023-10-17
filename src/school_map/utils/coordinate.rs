use crate::school_map::structs::{Params, GeoCoord, ImgCoord, NodesMap};

/// 平面圖像素座標 -> 經緯度座標
/// # 說明
/// - 輸入
///     - 座標轉換函數相關參數
///     - 平面圖像素座標
/// - 輸出
///     - 經緯度座標
///
/// # Examples
/// 需要 [school_map::read_json](../fn.read_json.html) 函數讀取參數
/// ```
/// use crate::navigation_server::school_map::read_json;
/// use crate::navigation_server::school_map::structs::Params;
/// use crate::navigation_server::school_map::coordinate::img_to_geo;
///
/// let path = vec!["src", "assets", "params.json"]; //檔案路徑
/// let params = read_json::<Params>(&path).unwrap(); //讀取檔案並轉為結構
/// let lonlat_coord = img_to_geo(&params, [20, 30]);
/// ```
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

/// 經緯度座標 -> 平面圖像素座標
/// # 說明
/// - 輸入
///     - 座標轉換函數相關參數
///     - 經緯度座標
/// - 輸出
///     - 平面圖像素座標
///
/// # Examples
/// 需要 [school_map::read_json](../fn.read_json.html) 函數讀取參數
/// ```
/// use crate::navigation_server::school_map::read_json;
/// use crate::navigation_server::school_map::structs::Params;
/// use crate::navigation_server::school_map::coordinate::geo_to_img;
///
/// let path = vec!["src", "assets", "params.json"]; //檔案路徑
/// let params = read_json::<Params>(&path).unwrap(); //讀取檔案並轉為結構
/// let xy_coord = geo_to_img(&params, [121.54026281, 25.01236468]);
/// ```
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

/// 取得最近的已標記點
/// # 說明
/// - 輸入
///     - 校內地圖 Graph 資訊
///     - 平面圖像素座標
/// - 輸出
///     - 最近的點
///
/// # Examples
/// 需要 [school_map::read_json](../fn.read_json.html) 函數讀取參數
/// ```
/// use crate::navigation_server::school_map::structs::NodesMap;
/// use crate::navigation_server::school_map::read_json;
/// use crate::navigation_server::school_map::coordinate::get_nearby_node;
///
/// let path = vec!["src", "assets", "data.json"]; //檔案路徑
/// let nodes = read_json::<NodesMap>(&path).unwrap(); //讀取檔案並轉為結構
/// let nearby_node = get_nearby_node(&nodes, [20, 30]);
/// ```
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

/// 算出兩個經緯度座標間的距離
/// # 說明
/// - 輸入
///     - 經緯度座標 A
///     - 經緯度座標 B
/// - 輸出
///     - 距離 (公尺)
///
/// # Examples
/// ```
/// use crate::navigation_server::school_map::coordinate::haversine_formula;
///
/// let distance = haversine_formula(
///     [121.54026281917675, 25.012364685321245],
///     [121.5404747956079, 25.012529348340735]
/// );
/// ```
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
