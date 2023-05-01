mod handlers;

use actix_files::Files;
use actix_web::{App, HttpServer};
use handlers::*;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(upload)
            .service(get_progress)
            .service(Files::new("/images", "./images"))
            .service(actix_files::Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
