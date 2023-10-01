use actix_web::web;

mod graph;
mod directions;

// 檔案路徑
pub const NODES_FILEPATH: [&str; 3] = ["src", "assets", "data.json"];
pub const PARAMS_FILEPATH: [&str; 3] = ["src", "assets", "params.json"];

// 路由
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/navigation")
            .service(graph::graph)
            .service(directions::directions)
    );  
}
