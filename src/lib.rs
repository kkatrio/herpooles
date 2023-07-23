use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
mod geometry;
mod render;
#[macro_use]
mod utils;
use game::Herpooles;
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
    callbacks::add_key_events(&pressed_keys, &document);

    // actors
    let mut poo = game::Poo::new();
    let mut zombie = game::Zombie::new();
    // needs interior mutability because it may move (change coordinates) and also fire poo (push
    // poo in its vec). Also it probably needs to be thread safe.
    let mut herpooles = game::Herpooles::new();

    //let herpooles = Rc::new(Cell::new(herpooles));
    //let closed_herpooles = herpooles.clone();

    // game over sound
    let audio =
        web_sys::HtmlAudioElement::new_with_src("zombie-hit.wav").expect("Could not load wav");

    // main game loop
    // animation_id is used in the first frame request only.
    let animation_id = Rc::new(Cell::new(0));
    // used in the main_loop_closure
    let closed_animation_id = animation_id.clone();
    // create two Rc
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let main_loop_closure = Closure::wrap(Box::new(move || {
        game::draw(&ctx, &mut herpooles, &mut zombie, &mut poo);

        //game::move_herpooles(&herpooles, pressed_keys);
        if pressed_keys.get().right && herpooles.x < 1000.0 {
            herpooles.x += 10.0;
        }
        if pressed_keys.get().left && herpooles.x > 0.0 {
            herpooles.x -= 10.0;
        }
        if pressed_keys.get().up && herpooles.y > 0.0 {
            herpooles.y -= 10.0;
        }
        if pressed_keys.get().down && herpooles.y < 800.0 {
            herpooles.y += 10.0;
        }

        if herpooles.alive {
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

    // play-pause callback -- keep it here for now
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

    callbacks::add_restart_event(&document);
    //callbacks::add_shoot(&herpooles, &document);

    Ok(())
}
