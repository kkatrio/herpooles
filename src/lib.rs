use wasm_bindgen::prelude::*;
#[macro_use]
mod utils;
use utils::set_panic_hook;

#[wasm_bindgen]
pub struct Herpooles {
    xh: f64,
    yh: f64,
}

#[wasm_bindgen]
impl Herpooles {
    #[wasm_bindgen(constructor)]
    pub fn new(xh: f64, yh: f64) -> Herpooles {
        Herpooles { xh, yh }
    }
}
#[wasm_bindgen]
pub fn draw(ctx: &web_sys::CanvasRenderingContext2d, h: &Herpooles) {
    set_panic_hook();
    create_herpooles(ctx, &h);
}

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    Ok(())
}

fn create_herpooles(ctx: &web_sys::CanvasRenderingContext2d, h: &Herpooles) {
    ctx.rect(h.xh, h.yh, 20.0, 20.0);
    ctx.stroke();
}
