use crate::{schema, AppState};
use actix_web::{get, post, web, Error, HttpResponse};
use rkv::Value;
use serde::Deserialize;
use uuid::Uuid;

use schema::Interface;

#[derive(Deserialize)]
struct CreateInterfaceParams {
    address: String,
    listen_port: u16,
    post_down: String,
    post_up: String,
    save_config: bool,
}

#[get("/interface")]
async fn get_interfaces(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let state = &data.state;

    let store = state.interface;

    let env = state.manager.read().unwrap();

    let reader = env.read().expect("reader");

    let values: Vec<Interface> = store
        .iter_start(&reader)
        .unwrap()
        .map(|value| match value {
            Ok((_, Value::Str(serialized))) => {
                let data: Interface = serde_json::from_str(serialized).unwrap();

                data
            }
            _ => panic!(),
        })
        .collect();

    Ok(HttpResponse::Ok().json(values))
}

#[post("/interface")]
async fn create_interface(
    data: web::Data<AppState>,
    params: web::Json<CreateInterfaceParams>,
) -> Result<HttpResponse, Error> {
    let address = &params.address;

    let post_down = &params.post_down;

    let post_up = &params.post_up;

    let interface_id = Uuid::new_v4();

    let interface = Interface {
        id: interface_id,
        address: address.to_string(),
        listen_port: params.listen_port,
        post_down: post_down.to_string(),
        post_up: post_up.to_string(),
        save_config: params.save_config,
    };

    let state = &data.state;

    let store = state.interface;

    let env = state.manager.read().unwrap();

    let mut writer = env.write().unwrap();

    let serialized = serde_json::to_string(&interface).unwrap();

    store
        .put(
            &mut writer,
            interface_id.to_string(),
            &Value::Str(&serialized),
        )
        .unwrap();

    writer.commit().unwrap();

    Ok(HttpResponse::Ok().json(interface))
}
