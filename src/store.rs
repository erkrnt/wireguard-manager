use rkv::backend::{SafeMode, SafeModeDatabase, SafeModeEnvironment};
use rkv::{Manager, Rkv, SingleStore, StoreOptions};
use std::env;
use std::fs::create_dir_all;
use std::sync::{Arc, RwLock};

pub struct State {
    pub interface: SingleStore<SafeModeDatabase>,
    pub manager: Arc<RwLock<Rkv<SafeModeEnvironment>>>,
    pub peer: SingleStore<SafeModeDatabase>,
    pub user: SingleStore<SafeModeDatabase>,
}

impl State {
    pub fn new() -> Self {
        let current = env::current_dir().unwrap();

        let data = current.join("data");

        create_dir_all(&data).unwrap();

        let mut m = Manager::<SafeModeEnvironment>::singleton().write().unwrap();

        let created_arc = m
            .get_or_create(data.as_path(), Rkv::new::<SafeMode>)
            .unwrap();

        let interface = {
            let env = created_arc.read().unwrap();
            env.open_single("interface", StoreOptions::create())
                .unwrap()
        };

        let peer = {
            let env = created_arc.read().unwrap();
            env.open_single("peer", StoreOptions::create()).unwrap()
        };

        let user = {
            let env = created_arc.read().unwrap();
            env.open_single("user", StoreOptions::create()).unwrap()
        };

        Self {
            interface,
            manager: created_arc,
            peer,
            user,
        }
    }
}
