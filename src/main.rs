use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use env_logger::Env;

mod route;
mod schema;
mod store;

use store::State;

pub struct AppState {
    state: State,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .data(AppState {
                state: State::new(),
            })
            .service(route::create_interface)
            .service(route::get_interfaces)
            .service(route::delete_interface)
    })
    .bind("127.0.0.1:23455")?
    .run()
    .await
}
