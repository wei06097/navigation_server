use dotenv::dotenv;
use actix_cors::Cors;
use actix_web::{App, HttpServer};
use navigation_server::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let host = std::env::var("HOST").unwrap();
    let port = std::env::var("PORT").unwrap();
    let port:u16 = port.parse().unwrap();

    HttpServer::new(|| {
        let cors = Cors::permissive()
            .allow_any_origin();
        App::new()
            .configure(routes::config)
            .wrap(cors)
    })
    .bind((host, port))?
        .run()
        .await
}
