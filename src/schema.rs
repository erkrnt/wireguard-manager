use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct Interface {
    pub id: Uuid,
    pub address: String,
    pub listen_port: u16,
    pub post_down: String,
    pub post_up: String,
    pub save_config: bool,
}

#[derive(Deserialize, Serialize)]
pub struct Peer {
    pub id: String,
    pub address: String,
    pub interface_id: String,
    pub private_key: String,
    pub public_key: String,
    pub user_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
}
