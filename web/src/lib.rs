use kash::statements::Statement;
use kash_repo::{mem::MemRepo, repo::RepoLike};
use lazy_static::lazy_static;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

lazy_static! {
    pub static ref REPO: Mutex<MemRepo> = Mutex::new(MemRepo::new());
}

#[wasm_bindgen]
pub fn get_all() -> JsValue {
    JsValue::from_serde(&REPO.lock().unwrap().get_all().unwrap()).unwrap()
}

#[wasm_bindgen]
pub fn insert(statement: &JsValue) {
    REPO.lock()
        .unwrap()
        .insert(&statement.into_serde::<Statement>().unwrap())
        .unwrap()
}
