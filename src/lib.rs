use std::any::Any;

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

#[wasm_bindgen]
pub struct Poo {
    pub x: f32,
    pub y: f32,
    pub direction: Direction,
}

#[wasm_bindgen]
impl Poo {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Poo {
        Poo {
            x: 100.0,
            y: 100.0,
            direction: Direction::South,
        }
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
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
pub fn draw(
    ctx: &web_sys::CanvasRenderingContext2d,
    h: &mut Herpooles,
    z: &mut Zombie,
    p: &mut Poo,
) {
    set_panic_hook();
    ctx.clear_rect(1.0, 1.0, 998.0, 798.0);

    // herpooles
    let herpooles_state = if is_herpooles_alive(h, z) {
        HState::Alive
    } else {
        HState::Dead
    };
    render::draw_herpooles(ctx, &h, herpooles_state.color());

    // poo
    move_poo(p);
    render::draw_poo(ctx, p);

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

fn move_poo(p: &mut Poo) {
    // find unit vector of orbit
    // TODO maybe store the unit vector in the poo instead of the direction.
    let (p_next_x, p_next_y) = match p.direction {
        Direction::North => (p.x, p.y - 1.0),
        Direction::East => (p.x + 1.0, p.y),
        Direction::South => (p.x, p.y + 1.0),
        Direction::West => (p.x - 1.0, p.y),
    };
    let direction_vec = geometry::Vector::new(
        geometry::Point { x: p.x, y: p.y },
        geometry::Point {
            x: p_next_x,
            y: p_next_y,
        },
    );
    let poo_speed = 0.5;
    let mv_vec = direction_vec.unit_vec() * poo_speed;
    p.x = p.x + mv_vec.x;
    p.y = p.y + mv_vec.y;
}

fn handle_event(event: web_sys::EventTarget) {
    let type_id = event.type_id();
    log!("type_id: {:?}", type_id);
}

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    let document = web_sys::window()
        .expect("can't get window")
        .document()
        .expect("can't get document");
    let event_target = document.dyn_into::<web_sys::EventTarget>().unwrap();

    let closure = Closure::wrap(Box::new(handle_event) as Box<dyn FnMut(_)>);
    event_target
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();

    // create data here they will live in JS

    let poo = Poo::new();
    // use them in handles, use them form JS
    Ok(())
}
