use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
mod geometry;
mod render;
#[macro_use]
mod utils;
use utils::set_panic_hook;

use wasm_bindgen::prelude::*;
#[macro_use]
mod callbacks;
mod game;

#[derive(Default, Copy, Clone)]
pub struct PressedKeys {
    right: bool,
    left: bool,
    up: bool,
    down: bool,
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}
// window.request_anination_frame -> Result<i32, JsValue>
pub fn request_animation_frame(f: &Closure<dyn FnMut()>) -> i32 {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK")
}
pub fn cancel_animation_frame(handle: i32) {
    window()
        .cancel_animation_frame(handle)
        .expect("should register `cancelAnimationFrame` OK");
}

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    set_panic_hook();
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

    // canvas boarder
    ctx.stroke_rect(
        0.0,
        0.0,
        htmlcanvas.width().into(),
        htmlcanvas.height().into(),
    );
    // use the hardcoded values, no need for this
    //let eastwall: f32 = htmlcanvas.width() as f32;
    //let westwall: f32 = 0.0;
    //let southwall: f32 = htmlcanvas.height() as f32;
    //let northwall: f32 = 0.0;

    // game-over sound
    let audio =
        web_sys::HtmlAudioElement::new_with_src("zombie-hit.wav").expect("Could not load wav");

    // keyboard events
    let pressed_keys = PressedKeys {
        left: false,
        right: false,
        up: false,
        down: false,
    };
    // pressed_keys need shared ownership, but do they need interior mutability? the main loop does
    // not set a value -- no clone here
    let pressed_keys = Rc::new(Cell::new(pressed_keys));
    callbacks::add_key_events(&pressed_keys, &document); // pressed_keys rc not moved in here?

    // actors
    let mut zombie = vec![game::Zombie::new()];
    // needs interior mutability because it may move (change coordinates) and also fire poo (push
    // poo in its vec). Also it probably needs to be thread safe.
    let herpooles = game::Herpooles::new();
    let herpooles = Rc::new(RefCell::new(herpooles));
    // moved in the main_loop_closure
    let closed_herpooles = herpooles.clone();

    // animation_id is used in the first frame request.
    let animation_id = Rc::new(Cell::new(0));
    // moved in the main_loop_closure
    let closed_animation_id = animation_id.clone();

    // main game loop
    // create two Rc -- one is moved in the closure
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let main_loop_closure = Closure::wrap(Box::new(move || {
        game::step(&ctx, &closed_herpooles, &mut zombie, &pressed_keys);

        if closed_herpooles.borrow().is_alive() {
            let id = request_animation_frame(g.borrow().as_ref().unwrap());
            closed_animation_id.set(id);
        } else {
            let _promise = audio.play().unwrap();
        }
    }) as Box<dyn FnMut()>);
    // store the closure in the Rc
    *f.borrow_mut() = Some(main_loop_closure);
    // request the first frame
    animation_id.set(request_animation_frame(f.borrow().as_ref().unwrap()));

    // we want to call the main_loop closure from the play_pause closure, so we create another clone of the main_loop_closure Rc.
    // Same for the animation_id.
    let p = f.clone();
    let pp_animation_id = animation_id.clone();
    callbacks::add_play_pause_control(pp_animation_id, p, &document);
    callbacks::add_restart_event(&document);
    // fist herpooles Rc moved in here
    callbacks::add_shoot(herpooles, &document);

    Ok(())
}
