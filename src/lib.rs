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
    let window = window();
    // using the window here and then moving it into the main closure
    let document = window.document().unwrap();
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

    // gets moved in the main closure
    let score_element = document
        .get_element_by_id("score")
        .expect("should have #score on the page");
    let score_element = Rc::new(score_element);

    // canvas boarder
    let width = htmlcanvas.width() as f64;
    let height = htmlcanvas.height() as f64;
    ctx.stroke_rect(1.0, 1.0, width - 1.0, height - 1.0);

    // keyboard events
    let pressed_keys = PressedKeys {
        left: false,
        right: false,
        up: false,
        down: false,
    };
    let pressed_keys = Rc::new(Cell::new(pressed_keys));
    callbacks::add_key_events(&pressed_keys, &document);

    let herpooles = Rc::new(RefCell::new(game::Herpooles::new()));
    callbacks::add_shoot(&herpooles, &document);

    // Not sure what is the value of keeping this in the heap,
    // I just do not want the Controller to own the Zombies.
    let zombies = Box::new((0..10).map(|_| game::Zombie::new()).collect());
    let mut controller = game::Controller::new(zombies);

    // animation_id is used in the first frame request.
    let animation_id = Rc::new(Cell::new(0));
    // moved in the main_loop_closure
    let closed_animation_id = animation_id.clone();

    // main game loop
    // create two Rc -- one is moved in the closure
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let main_loop_closure = Closure::new(move || {
        controller.check();
        game::step(
            &ctx,
            // Need a reference because cannot move it out of its environment (closure is FnMut),
            &herpooles,
            &pressed_keys,
            &mut controller,
        );

        callbacks::update_score(&controller.score, &score_element, &window);

        if herpooles.borrow().is_alive() {
            let id = request_animation_frame(g.borrow().as_ref().unwrap());
            closed_animation_id.set(id);
        } else {
            let audio = web_sys::HtmlAudioElement::new_with_src("resources/zombie-hit.wav")
                .expect("Could not load wav");
            let _promise = audio.play().unwrap();
        }
    });
    // store the closure in the Rc
    *f.borrow_mut() = Some(main_loop_closure);
    // request the first frame
    animation_id.set(request_animation_frame(f.borrow().as_ref().unwrap()));

    callbacks::add_play_pause_control(animation_id, f, &document);
    callbacks::add_restart_event(&document);
    Ok(())
}
