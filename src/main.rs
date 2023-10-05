use dotenv::dotenv;
use actix_cors::Cors;
use actix_web::{App, HttpServer};
use navigation_server::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let host = match std::env::var("HOST") {
        Ok(host) => host,
        Err(_) => String::from("0.0.0.0"),
    };
    let port: u16 = match std::env::var("PORT") {
        Ok(port) => port.parse().unwrap(),
        Err(_) => 80,
    };
    println!("==> Starting at {}:{}", host, port);
    
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
