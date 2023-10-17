use actix_web::web;

mod graph;
mod directions;
mod transform;

/// 檔案路徑 : 校內地圖 Graph 資訊
pub const NODES_FILEPATH: [&str; 3] = ["src", "assets", "data.json"];
/// 檔案路徑 : 座標轉換函數相關參數
pub const PARAMS_FILEPATH: [&str; 3] = ["src", "assets", "params.json"];

/// 路由設定
/// # API 網址 (詳細請看 README.md)
/// `/navigation/graph`<br>
/// 取得校內地圖 Graph 資訊<br><br>
/// `/navigation/directions`<br>
/// 取得兩點間的最佳路徑<br><br>
/// `/navigation/transform`<br>
/// 平面圖像素座標與經緯度座標的相互轉換
///
/// # Examples
/// ```
/// use actix_web::{App, HttpServer};
/// use crate::navigation_server::routes;
///
/// HttpServer::new(|| {
///     App::new()
///         .configure(routes::config)
/// });
/// ```
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/navigation")
            .service(graph::graph)
            .service(directions::directions)
            .service(transform::transform)
    );  
}
