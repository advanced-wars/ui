mod utils;

use wasm_bindgen::prelude::*;
use crate::utils::{
    pairing_heap::PairingHeap,
    log::console_log,
};

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
    let mut heap: PairingHeap<i32> = PairingHeap::min();
    let numbers = vec![4,3,1,2,5,7,6];
    numbers.iter().for_each(| n | {
        let _ = &mut heap.push(n);
    });
    console_log(&format!("first: {}", heap.pop().unwrap()));
    console_log(&format!("second: {}", heap.pop().unwrap()));
    console_log(&format!("third: {}", heap.pop().unwrap()));

    console_log(&format!("heap -> {:#?}", heap));
}
