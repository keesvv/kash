use kash::statements::Statement;
use kash_repo::{mem::MemRepo, repo::RepoLike};
use lazy_static::lazy_static;
use std::error::Error;
use std::result;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

lazy_static! {
    pub static ref REPO: Mutex<MemRepo> = Mutex::new(MemRepo::new());
}

pub type Result<T> = result::Result<T, JsValue>;

pub fn throw<E: Error>(err: E) -> JsValue {
    JsValue::from(err.to_string())
}

#[wasm_bindgen]
pub fn get_all() -> Result<JsValue> {
    JsValue::from_serde(&REPO.lock().map_err(throw)?.get_all().map_err(throw)?).map_err(throw)
}

#[wasm_bindgen]
pub fn insert(statement: &JsValue) -> Result<()> {
    REPO.lock()
        .map_err(throw)?
        .insert(&statement.into_serde::<Statement>().map_err(throw)?)
        .map_err(throw)
}

#[wasm_bindgen]
pub fn insert_all(statements: &JsValue) -> Result<()> {
    REPO.lock()
        .map_err(throw)?
        .insert_all(&statements.into_serde::<Vec<Statement>>().map_err(throw)?)
        .map_err(throw)
}
