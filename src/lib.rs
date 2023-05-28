use wasm_bindgen::prelude::*;
#[macro_use]
mod utils;
use utils::set_panic_hook;

#[wasm_bindgen]
pub struct Herpooles {
    pub x: f64,
    pub y: f64,
    pub alive: bool,
}

#[wasm_bindgen]
pub struct Zombie {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
impl Herpooles {
    #[wasm_bindgen(constructor)] // TODO: why is this needed
    pub fn new() -> Herpooles {
        Herpooles {
            x: 500.0,
            y: 500.0,
            alive: true,
        }
    }
}

#[wasm_bindgen]
impl Zombie {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Zombie {
        Zombie { x: 500.0, y: 400.0 }
    }
}

#[derive(PartialEq)]
enum HState {
    Alive,
    Dead,
}

// TODO: maybe use associated constants or constans in another mod:
// https://stackoverflow.com/questions/36928569/how-can-i-create-enums-with-constant-values-in-rust
impl HState {
    fn color(&self) -> &str {
        match *self {
            HState::Dead => "red",
            HState::Alive => "green",
        }
    }
}

#[wasm_bindgen]
pub fn draw(ctx: &web_sys::CanvasRenderingContext2d, h: &mut Herpooles, z: &Zombie) {
    set_panic_hook();
    ctx.clear_rect(1.0, 1.0, 998.0, 798.0);

    // TODO: use an enum
    let herpooles_state = if is_herpooles_alive(h, z) {
        HState::Alive
    } else {
        HState::Dead
    };

    draw_herpooles(ctx, &h, herpooles_state.color());
    draw_zombie(ctx, z, "orange");

    // set flag to dead, so that js stops frame requests
    if herpooles_state == HState::Dead {
        log!("herpooles state dead!");
        h.alive = false
    }
}

fn draw_herpooles(ctx: &web_sys::CanvasRenderingContext2d, h: &Herpooles, c: &str) {
    ctx.set_fill_style(&JsValue::from_str(c));
    ctx.fill_rect(h.x, h.y, 20.0, 20.0);
}

fn draw_zombie(ctx: &web_sys::CanvasRenderingContext2d, h: &Zombie, c: &str) {
    ctx.set_fill_style(&JsValue::from_str(c));
    ctx.fill_rect(h.x, h.y, 20.0, 20.0);
}

fn is_herpooles_alive(h: &Herpooles, z: &Zombie) -> bool {
    let d = (h.x - z.x) * (h.x - z.x) + (h.y - z.y) * (h.y - z.y);
    log!("d: {}", d);
    d > 400.0 // TODO: calculate based on herpooles and zombie area
}

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    Ok(())
}
