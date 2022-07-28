mod utils;

use once_cell::sync::OnceCell;
use wasm_bindgen::prelude::*;
use todos::redux_rs::{Store, StoreApi};
use todos::{todo_reducer, TodoAction, TodoState};

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
    store: Box<dyn StoreApi<TodoState, TodoAction> + Send + Sync + 'static>
}

impl ReduxStore {
    pub fn new() -> Self {
        Self { store: Box::new(Store::new(todo_reducer)) }
    }

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



