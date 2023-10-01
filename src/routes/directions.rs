use crate::routes;
use crate::school_map::{
    self, coordinate,
    structs::{NodesMap, Params, GeoCoord, ImgCoord},
};
use actix_web::{post, web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

/// 定義 Request Body 結構
#[derive(Debug, Deserialize)]
pub struct RequestBody {
    source: GeoCoord,
    destination: GeoCoord,
}
/// 定義 Response Body 結構
#[derive(Debug, Serialize)]
pub struct ResponseBody {
    source_xy: ImgCoord,
    destination_xy: ImgCoord,
    best_path: Vec<String>,
    total_distance: f64,
}

#[post("/directions")]
pub async fn directions(data: web::Json<RequestBody>) -> impl Responder {
    let [source_lonlat, destination_lonlat] = [data.source, data.destination];
    // 從參數檔取得 nodes 跟 params 並轉成結構
    let path = routes::NODES_FILEPATH.to_vec();
    let nodes = school_map::read_json::<NodesMap>(&path).unwrap();
    let path = routes::PARAMS_FILEPATH.to_vec();
    let params = school_map::read_json::<Params>(&path).unwrap();
    // 經緯度座標轉換為平面圖座標
    let source_xy = coordinate::geo_to_img(&params, source_lonlat);
    let destination_xy = coordinate::geo_to_img(&params, destination_lonlat);
    // 找出最近的 nodes
    let node_a = coordinate::get_nearby_node(&nodes, source_xy);
    let node_b = coordinate::get_nearby_node(&nodes, destination_xy);
    let node_a = &node_a[..];
    let node_b = &node_b[..];
    // 最佳路徑演算法
    let (best_path, distance) = school_map::dijkstra(&nodes, node_a, node_b);
    // 算實際位置跟 node 的距離
    let distance_s = coordinate::haversine_formula(nodes[node_a].geo_coord, source_lonlat);
    let distance_d = coordinate::haversine_formula(nodes[node_a].geo_coord, source_lonlat);
    let total_distance = distance + distance_s + distance_d;
    HttpResponse::Ok().json(ResponseBody {
        source_xy, destination_xy, best_path, total_distance
    })
}
