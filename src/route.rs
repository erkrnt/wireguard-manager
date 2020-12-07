use crate::schema;
use actix_web::{post, web, Error, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
struct CreateInterfaceParams {
    address: String,
    listen_port: u16,
    post_down: String,
    post_up: String,
    save_config: bool,
}

#[post("/interface")]
async fn create_interface(params: web::Json<CreateInterfaceParams>) -> Result<HttpResponse, Error> {
    let interface = schema::Interface {
        id: String::from(""),
        address: String::from(""),
        listen_port: 27509,
        post_down: String::from(""),
        post_up: String::from(""),
        save_config: false,
    };

    Ok(HttpResponse::Ok().json(interface))
}
