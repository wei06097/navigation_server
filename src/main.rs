// use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use navigation_server::school_map;

fn main() {
    let path = vec!["src", "assets", "data.json"];
    let nodes = school_map::read_nodes(&path).unwrap();
    let result = school_map::dijkstra(&nodes, "5", "62");
    println!("{:#?}", result);
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