use rkv::backend::{SafeMode, SafeModeDatabase, SafeModeEnvironment};
use rkv::{Manager, Rkv, SingleStore, StoreOptions};
use std::env;
use std::fs::create_dir_all;
use std::sync::{Arc, RwLock};

pub struct State {
    pub manager: Arc<RwLock<Rkv<SafeModeEnvironment>>>,
}

impl State {
    pub fn new() -> Self {
        let current = env::current_dir().unwrap();

        let data = current.join("data");

        create_dir_all(&data).unwrap();

        let mut m = Manager::<SafeModeEnvironment>::singleton().write().unwrap();

        let manager = m
            .get_or_create(data.as_path(), Rkv::new::<SafeMode>)
            .unwrap();

        Self { manager }
    }

    pub fn get_store(&self) -> SingleStore<SafeModeDatabase> {
        let env = self.manager.read().unwrap();

        env.open_single("wireguard_manager", StoreOptions::create())
            .unwrap()
    }
}
