// use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use navigation_server::school_map::{
    self, coordinate,
    structs::{Params, NodesMap},
};

fn main() {
    let path = vec!["src", "assets", "data.json"];
    let nodes = school_map::read_json::<NodesMap>(&path).unwrap();
    let _result = school_map::dijkstra(&nodes, "5", "62");
    // println!("{:#?}", result);

    let path = vec!["src", "assets", "params.json"];
    let params = school_map::read_json::<Params>(&path).unwrap();
    let [lon, lat] = coordinate::img_to_geo(&params, [1047, 1117]);
    let [_x, _y] = coordinate::geo_to_img(&params, [lon, lat]);
    // println!("{}, {}", lat, lon);
    // println!("{}, {}", x, y);
    
}

// #[get("/nodes")]
// async fn hello() -> impl Responder {
//     let filepath = format!("{}/src/data.json", system::dirname());
//     let content = school_map::get_nodes(&filepath).unwrap();
//     HttpResponse::Ok().json(content)
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(hello)
//     })
//     .bind(("192.168.244.130", 3000))?
//     .run()
//     .await
// }