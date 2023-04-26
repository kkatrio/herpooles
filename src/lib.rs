use wasm_bindgen::prelude::*;
#[macro_use]
mod utils;
use utils::set_panic_hook;

#[wasm_bindgen]
pub struct Herpooles {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
impl Herpooles {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Herpooles {
        Herpooles { x, y }
    }
    // use set method, if we keep the data in js
}
#[wasm_bindgen]
pub fn draw(ctx: &web_sys::CanvasRenderingContext2d, h: &Herpooles) {
    set_panic_hook();
    ctx.clear_rect(0.0, 0.0, 1000.0, 800.0);
    draw_herpooles(ctx, &h);
}

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    Ok(())
}

fn draw_herpooles(ctx: &web_sys::CanvasRenderingContext2d, h: &Herpooles) {
    ctx.fill_rect(h.x, h.y, 20.0, 20.0);
}
