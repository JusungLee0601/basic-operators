mod utils;

use wasm_bindgen::prelude::*;

use std::fmt;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Debug)]
pub struct View {
    data: Vec<String>,
}

impl fmt::Display for View {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for strings in self.data.iter() {
            write!(f, "{} \n", strings);
        }

        Ok(())
    }
}

#[wasm_bindgen]
impl View {
    pub fn new() -> View {
        let mut data = Vec::new();
        let dummy = "dummy";
        data.push(dummy.to_owned());
        View {data}
    }

    pub fn update(&mut self, input: String) {
        self.data.push(input)
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

