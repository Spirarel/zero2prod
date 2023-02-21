use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

use crate::routes::{health_check, subscribe};

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // wrap in arc pointer
    let connection = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .service(health_check)
            .service(subscribe)
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
