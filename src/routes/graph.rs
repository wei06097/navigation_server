use crate::routes;
use crate::school_map::{self, structs::NodesMap};
use actix_web::{get, HttpResponse, Responder};

#[get("/graph")]
pub async fn graph() -> impl Responder {
    let path = routes::NODES_FILEPATH.to_vec();
    let nodes = school_map::read_json::<NodesMap>(&path).unwrap();
    HttpResponse::Ok().json(nodes)
}
