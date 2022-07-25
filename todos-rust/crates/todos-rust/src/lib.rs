mod utils;

use once_cell::sync::OnceCell;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}


static INSTANCE: OnceCell<ReduxStore> = OnceCell::new();

fn get_store() -> &'static ReduxStore {
    unsafe {
        INSTANCE.get_unchecked()
    }
}

struct ReduxStore {

}

impl ReduxStore {
    fn do_greet(&self) {
        alert("Hello from redux store!!");
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    let _ = INSTANCE.set(ReduxStore {});
}

#[wasm_bindgen]
pub fn greet() {
    get_store().do_greet();
}



