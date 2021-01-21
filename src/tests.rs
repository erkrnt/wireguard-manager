use super::*;
use actix_http::http::header::CONTENT_TYPE;
use actix_web::test::{call_service, init_service, read_body_json, TestRequest};
use actix_web::App;

use route::{CreateInterfaceParams, DeleteInterfaceParams};
use schema::Interface;

fn create_interface_params_mock() -> CreateInterfaceParams {
    let address = String::from("172.16.0.1/24");

    let post_down = String::from("iptables -D FORWARD -i %i -j ACCEPT; iptables -D FORWARD -o %i -j ACCEPT; iptables -t nat -D POSTROUTING -o eth0 -j MASQUERADE");

    let post_up = String::from("iptables -A FORWARD -i %i -j ACCEPT; iptables -A FORWARD -o %i -j ACCEPT; iptables -t nat -A POSTROUTING -o eth0 -j MASQUERADE");

    CreateInterfaceParams {
        address,
        listen_port: 32774,
        post_down,
        post_up,
        save_config: false,
    }
}

#[actix_rt::test]
async fn get_interfaces_test() {
    let mut app = init_service(
        App::new()
            .data(AppState {
                state: State::new(),
            })
            .service(route::get_interfaces),
    )
    .await;

    let req = TestRequest::get().uri("/interface").to_request();

    let res = call_service(&mut app, req).await;

    assert!(res.status().is_success())
}

#[actix_rt::test]
async fn create_interface_test() {
    let mut app = init_service(
        App::new()
            .data(AppState {
                state: State::new(),
            })
            .service(route::create_interface),
    )
    .await;

    let interface_params = create_interface_params_mock();

    let req = TestRequest::post()
        .set_json(&interface_params)
        .uri("/interface")
        .to_request();

    let res = call_service(&mut app, req).await;

    assert!(res.status().is_success())
}

#[actix_rt::test]
async fn delete_interface_test() {
    let mut app = init_service(
        App::new()
            .data(AppState {
                state: State::new(),
            })
            .service(route::create_interface)
            .service(route::delete_interface),
    )
    .await;

    let create_interface_params = create_interface_params_mock();

    let res_create = TestRequest::post()
        .uri("/interface")
        .header(CONTENT_TYPE, "application/json")
        .set_json(&create_interface_params)
        .send_request(&mut app)
        .await;

    let res_create_body: Interface = read_body_json(res_create).await;

    let delete_interface_params = DeleteInterfaceParams {
        id: res_create_body.id.to_string(),
    };

    let req_delete = TestRequest::delete()
        .uri("/interface")
        .set_json(&delete_interface_params)
        .to_request();

    let res = call_service(&mut app, req_delete).await;

    assert!(res.status().is_success())
}
