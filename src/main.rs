use actix_web::{App, HttpServer};

mod route;
mod schema;
mod store;

use store::State;

pub struct AppState {
    state: State,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(AppState {
                state: State::new(),
            })
            .service(route::create_interface)
            .service(route::get_interfaces)
    })
    .bind("127.0.0.1:23455")?
    .run()
    .await
}
