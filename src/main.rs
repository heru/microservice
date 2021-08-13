mod route;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let host: &str = "127.0.0.1";
    let port: u16 = 8080;
    let server = (host, port);
    println!("Server running at http://{}:{}", host, port);
    HttpServer::new(|| {
        App::new()
            .service(route::home)
            .service(route::echo_post)
            .service(route::echo_get)
            .service(route::qr_generator)
            .service(route::qr_generator_get)

    }).bind(server)?
      .run()
      .await
}
