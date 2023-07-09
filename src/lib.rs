use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

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

enum KeyboardCodes {
    Left = 37,
    Up = 38,
    Right = 39,
    Down = 40,
}

#[derive(Default, Copy, Clone)]
pub struct PressedKeys {
    right: bool,
    left: bool,
    up: bool,
    down: bool,
}

//TODO: use RefCell and borrow?
pub fn handle_keydown_event(event: web_sys::KeyboardEvent, pressed: &Rc<Cell<PressedKeys>>) {
    if event.key_code() == KeyboardCodes::Left as u32 {
        let mut keys = pressed.take();
        keys.left = true;
        pressed.set(keys);
    } else if event.key_code() == KeyboardCodes::Right as u32 {
        let mut keys = pressed.take();
        keys.right = true;
        pressed.set(keys);
    } else if event.key_code() == KeyboardCodes::Up as u32 {
        let mut keys = pressed.take();
        keys.up = true;
        pressed.set(keys);
    } else if event.key_code() == KeyboardCodes::Down as u32 {
        let mut keys = pressed.take();
        keys.down = true;
        pressed.set(keys);
    }
}

fn handle_keyup_event(event: web_sys::KeyboardEvent, pressed: &Rc<Cell<PressedKeys>>) {
    if event.key_code() == KeyboardCodes::Left as u32 {
        let mut keys = pressed.take();
        keys.left = false;
        pressed.set(keys);
    } else if event.key_code() == KeyboardCodes::Right as u32 {
        let mut keys = pressed.take();
        keys.right = false;
        pressed.set(keys);
    } else if event.key_code() == KeyboardCodes::Up as u32 {
        let mut keys = pressed.take();
        keys.up = false;
        pressed.set(keys);
    } else if event.key_code() == KeyboardCodes::Down as u32 {
        let mut keys = pressed.take();
        keys.down = false;
        pressed.set(keys);
    }
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

// window.request_anination_frame -> Result<i32, JsValue>
fn request_animation_frame(f: &Closure<dyn FnMut()>) -> i32 {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK")
}

fn cancel_animation_frame(handle: i32) {
    window()
        .cancel_animation_frame(handle)
        .expect("should register `cancelAnimationFrame` OK");
}

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    let document = window().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let htmlcanvas = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("Failed to convert to HtmlCanvasElement");
    let ctx = htmlcanvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    //draw canvas boarder
    ctx.stroke_rect(
        0.0,
        0.0,
        htmlcanvas.width().into(),
        htmlcanvas.height().into(),
    );
    let eastwall: f32 = htmlcanvas.width() as f32;
    let westwall: f32 = 0.0;
    let southwall: f32 = htmlcanvas.height() as f32;
    let northwall: f32 = 0.0;

    // keyboard events
    let pressed_keys = PressedKeys {
        left: false,
        right: false,
        up: false,
        down: false,
    };
    let pressed_keys = Rc::new(Cell::new(pressed_keys));
    // keydown
    let pressed_down_keys = pressed_keys.clone();
    let keydown_closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        handle_keydown_event(event, &pressed_down_keys)
    }) as Box<dyn FnMut(_)>);
    document
        .add_event_listener_with_callback("keydown", keydown_closure.as_ref().unchecked_ref())
        .unwrap();
    keydown_closure.forget();
    // keyup
    let pressed_up_keys = pressed_keys.clone();
    let keyup_closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        handle_keyup_event(event, &pressed_up_keys)
    }) as Box<dyn FnMut(_)>);
    document
        .add_event_listener_with_callback("keyup", keyup_closure.as_ref().unchecked_ref())
        .unwrap();
    keyup_closure.forget();

    // data
    let mut poo = Poo::new();
    let mut zombie = Zombie::new();
    let mut herpooles = Herpooles::new();

    // game over sound
    let audio =
        web_sys::HtmlAudioElement::new_with_src("zombie-hit.wav").expect("Could not load wav");

    // animation_id is used in the first frame request only.
    let animation_id = Rc::new(Cell::new(0));
    // used in the main_loop_closure
    let closed_animation_id = animation_id.clone();

    // create two Rc
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let main_loop_closure = Closure::wrap(Box::new(move || {
        draw(&ctx, &mut herpooles, &mut zombie, &mut poo);

        // TODO: move this to a seperate function
        if pressed_keys.get().right && herpooles.x < eastwall {
            herpooles.x += 10.0;
        }
        if pressed_keys.get().left && herpooles.x > westwall {
            herpooles.x -= 10.0;
        }
        if pressed_keys.get().up && herpooles.y > northwall {
            herpooles.y -= 10.0;
        }
        if pressed_keys.get().down && herpooles.y < southwall {
            herpooles.y += 10.0;
        }

        if herpooles.alive {
            let id = request_animation_frame(g.borrow_mut().as_ref().unwrap());
            closed_animation_id.set(id);
        } else {
            let _promise = audio.play().unwrap();
        }
    }) as Box<dyn FnMut()>);
    // store the closure in the Rc
    *f.borrow_mut() = Some(main_loop_closure);
    // request the first frame
    animation_id.set(request_animation_frame(f.borrow().as_ref().unwrap()));

    // play-pause callback
    // TODO: move to a seperate function
    // we want to call the main_loop_closure callback from the play_pause_closure, so we create another Rc
    let p = f.clone();
    // we use the animation_id in this closure, so create another clone for play-pause event
    let pp_animation_id = animation_id.clone();
    // get_element_by_id returns an Element, not a &Element
    // we clone it once before the closure, but we clone it again in the closure?
    let play_pause_button = document.get_element_by_id("play-pause").unwrap();
    // wee need to move a clone in the closure
    let pp_button = play_pause_button.clone();
    let play_pause_closure = Closure::wrap(Box::new(move || {
        // needed to set the value of the input button
        let html_input_button = pp_button
            .clone() // why ownership is not tranfered in the closure? It is not used again outside.
            .dyn_into::<web_sys::HtmlInputElement>()
            .unwrap();
        if pp_animation_id.get() == 0 {
            pp_animation_id.set(request_animation_frame(p.borrow().as_ref().unwrap()));
            html_input_button.set_value("Pause");
        } else {
            cancel_animation_frame(animation_id.get());
            pp_animation_id.set(0);
            html_input_button.set_value("Start"); // fix this
        }
    }) as Box<dyn Fn()>); // no FnMut needed
    play_pause_button
        .add_event_listener_with_callback("click", play_pause_closure.as_ref().unchecked_ref())
        .unwrap();
    play_pause_closure.forget();

    // try again callback
    let location = document.location().unwrap();
    let restart_closure =
        Closure::wrap(Box::new(move || location.reload().unwrap()) as Box<dyn Fn()>);
    let restart_button = document.get_element_by_id("restart").unwrap();
    restart_button
        .add_event_listener_with_callback("click", restart_closure.as_ref().unchecked_ref())
        .unwrap();
    restart_closure.forget();

    Ok(())
}
