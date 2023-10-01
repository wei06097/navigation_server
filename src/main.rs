use actix_web::{App, HttpServer};
use navigation_server::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(routes::config)
    })
    .bind(("192.168.244.130", 3000))?
    .run()
    .await
}
