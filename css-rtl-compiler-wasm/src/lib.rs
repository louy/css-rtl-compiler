use wasm_bindgen::prelude::*;

use css_rtl_compiler_core::convert_css;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen]
pub fn convert(css: &str) -> Result<String, String> {
    convert_css(css)
}
