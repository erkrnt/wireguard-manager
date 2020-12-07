use actix_web::{App, HttpServer};
//use rkv::Value;

mod route;
mod schema;
mod store;

use store::State;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = State::new();

    let store = state.get_store();

    let env = state.manager.read().unwrap();

    //let mut writer = env.write().unwrap();

    //store
    //.put(&mut writer, "test", &Value::Str("Hello, world!"))
    //.unwrap();

    //writer.commit().unwrap();

    //let reader = env.read().expect("reader");

    //let value = store.get(&reader, "test").unwrap();

    //println!("Get string {:?}", value);

    HttpServer::new(|| App::new().service(route::create_interface))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
