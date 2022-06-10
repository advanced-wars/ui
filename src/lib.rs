mod utils;

use wasm_bindgen::prelude::*;
use crate::utils::pairing_heap::PairingHeap;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    let heap: PairingHeap<i32> = PairingHeap::min();
    let num = 3;
    heap.add(&num);
    alert("Hello, ui!");
}
