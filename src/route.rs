use crate::{schema, AppState};
use actix_web::{delete, get, post, web, Error, HttpResponse};
use rkv::Value;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use schema::Interface;

#[derive(Deserialize, Serialize)]
pub struct CreateInterfaceParams {
    pub address: String,
    pub listen_port: u16,
    pub post_down: String,
    pub post_up: String,
    pub save_config: bool,
}

#[derive(Deserialize, Serialize)]
pub struct DeleteInterfaceParams {
    pub id: String,
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

    let serialized = serde_json::to_string(&interface).unwrap();

    let state = &data.state;

    let store = state.interface;

    let env = state.manager.read().unwrap();

    let mut writer = env.write().unwrap();

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

#[delete("/interface")]
async fn delete_interface(
    data: web::Data<AppState>,
    params: web::Json<DeleteInterfaceParams>,
) -> Result<HttpResponse, Error> {
    let id = &params.id;

    let state = &data.state;

    let store = state.interface;

    let env = state.manager.read().unwrap();

    let mut writer = env.write().unwrap();

    store.delete(&mut writer, id.to_string()).unwrap();

    writer.commit().unwrap();

    Ok(HttpResponse::Ok().finish())
}
