use wasm_bindgen::prelude::*;
#[macro_use]
mod utils;
use utils::set_panic_hook;
pub mod geometry;
mod render;

#[wasm_bindgen]
pub struct Herpooles {
    pub x: f32,
    pub y: f32,
    pub alive: bool,
}

#[wasm_bindgen]
pub struct Zombie {
    pub x: f32,
    pub y: f32,
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
pub fn draw(ctx: &web_sys::CanvasRenderingContext2d, h: &mut Herpooles, z: &mut Zombie) {
    set_panic_hook();
    ctx.clear_rect(1.0, 1.0, 998.0, 798.0);

    // herpooles
    let herpooles_state = if is_herpooles_alive(h, z) {
        HState::Alive
    } else {
        HState::Dead
    };
    render::draw_herpooles(ctx, &h, herpooles_state.color());
    render::draw_poo(ctx, &h);

    // zombies
    move_zombies(z, h);
    render::draw_zombie(ctx, &z, "grey");

    // set flag to dead, so that js stops frame requests
    if herpooles_state == HState::Dead {
        log!("herpooles state dead!");
        h.alive = false
    }
}

fn is_herpooles_alive(h: &Herpooles, z: &Zombie) -> bool {
    let d = (h.x - z.x) * (h.x - z.x) + (h.y - z.y) * (h.y - z.y);
    //log!("d: {}", d);
    d > 400.0 // TODO: calculate based on herpooles and zombie area
}

fn move_zombies(z: &mut Zombie, h: &Herpooles) {
    // find vector z -> h
    let zp = geometry::Point { x: z.x, y: z.y };
    let hp = geometry::Point { x: h.x, y: h.y };
    let zh_vec = geometry::Vector::new(zp, hp);

    // apply A + d n
    let zombie_speed = 0.5;
    let mv_vec: geometry::Vector = zh_vec.unit_vec() * zombie_speed;
    let pos: geometry::Point = zp + mv_vec;

    z.x = pos.x;
    z.y = pos.y;
}

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    Ok(())
}
