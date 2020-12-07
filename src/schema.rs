use serde::Serialize;

#[derive(Serialize)]
pub struct Interface {
    pub id: String,
    pub address: String,
    pub listen_port: u16,
    pub post_down: String,
    pub post_up: String,
    pub save_config: bool,
}

#[derive(Serialize)]
pub struct Peer {
    pub id: String,
    pub address: String,
    pub interface_id: String,
    pub private_key: String,
    pub public_key: String,
    pub user_id: String,
}

#[derive(Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
}
