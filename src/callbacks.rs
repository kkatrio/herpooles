use crate::PressedKeys;
use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

enum KeyboardCodes {
    Left = 37,
    Up = 38,
    Right = 39,
    Down = 40,
}

//TODO: use RefCell and borrow?
fn handle_keydown_event(event: web_sys::KeyboardEvent, pressed: &Rc<Cell<PressedKeys>>) {
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

pub fn add_key_events(pressed_keys: &Rc<Cell<PressedKeys>>, document: &web_sys::Document) {
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
}

pub fn add_restart_event(document: &web_sys::Document) {
    // try again callback
    let location = document.location().unwrap();
    let restart_closure =
        Closure::wrap(Box::new(move || location.reload().unwrap()) as Box<dyn Fn()>);
    let restart_button = document.get_element_by_id("restart").unwrap();
    restart_button
        .add_event_listener_with_callback("click", restart_closure.as_ref().unchecked_ref())
        .unwrap();
    restart_closure.forget();
}
