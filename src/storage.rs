//! Allows use of the Web Storage API / local storage.
//! MDN docs:
//! https://developer.mozilla.org/en-US/docs/Web/API/Storage
//! web-sys docs:
//! https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Storage.html
//! Example syntax:
//! https://github.com/rustwasm/wasm-bindgen/blob/master/examples/todomvc/src/store.rs

extern crate serde;
extern crate serde_json;

pub fn get_storage() -> Option<web_sys::Storage> {
    let window = web_sys::window().unwrap();

    match window.local_storage() {
        Ok(Some(local_storage)) => Some(local_storage),
        Err(_) => None,
        Ok(None) => None,
    }
}

/// Create a new store, from a serializable data structure.
pub fn store_data<T>(storage: &web_sys::Storage, name: &str, data: &T)
where
    T: serde::Serialize,
{
    let serialized = serde_json::to_string(&data).unwrap();
    storage.set_item(name, &serialized).unwrap();
}
