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
pub struct Zombie {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
impl Herpooles {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Herpooles {
        Herpooles { x, y }
    }
}
#[wasm_bindgen]
impl Zombie {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Zombie {
        Zombie { x, y }
    }
}
#[wasm_bindgen]
pub fn draw(ctx: &web_sys::CanvasRenderingContext2d, h: &Herpooles, z: &Zombie) {
    set_panic_hook();
    ctx.clear_rect(1.0, 1.0, 998.0, 798.0);
    draw_herpooles(ctx, &h);
    draw_zombie(ctx, z);
}

fn draw_herpooles(ctx: &web_sys::CanvasRenderingContext2d, h: &Herpooles) {
    ctx.set_fill_style(&JsValue::from_str("black"));
    ctx.fill_rect(h.x, h.y, 20.0, 20.0);
}

fn draw_zombie(ctx: &web_sys::CanvasRenderingContext2d, h: &Zombie) {
    ctx.set_fill_style(&JsValue::from_str("red"));
    ctx.fill_rect(h.x, h.y, 20.0, 20.0);
}
#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    Ok(())
}
