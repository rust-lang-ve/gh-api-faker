#[macro_use]
extern crate log;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use std::env;

mod error;
mod routes;
mod util;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "warn,info,error,debug");
    env_logger::init();

    let target = format!("{}:{}", "0.0.0.0", "7878");

    println!("Server ready at {}", format!("http://{}", &target));

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_origin("*")
                    .allowed_methods(vec!["GET"])
                    .max_age(3600)
                    .finish(),
            )
            .wrap(Logger::default())
            .wrap(Logger::new("%a %r %s %b %{Referer}i %{User-Agent}i %T"))
            .service(routes::router())
    })
    .bind(target)?
    .run()
    .await
}
