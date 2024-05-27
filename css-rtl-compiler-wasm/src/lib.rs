use wasm_bindgen::prelude::*;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen]
pub fn convert(css: &str) -> Result<String, String> {
    Ok(String::from(css))
}
