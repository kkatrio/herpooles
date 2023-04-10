use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

macro_rules! log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (web_sys::console::log_1(&format!($($t)*).into()))
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    // create canvas
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    document.body().unwrap().append_child(&canvas)?;
    canvas.set_width(800);
    canvas.set_height(600);
    canvas.style().set_property("border", "solid")?;
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    let mut xh = 400.0;
    let mut yh = 500.0;
    //create_herpooles(&context, &xh, &yh);

    // to closure zei gia panta gia kathe click. to rect kai to clear ginotnai gia panta gia kathe
    // click
    //
    // Theloume na paroume mutable reference sto context. Alla exoume provlima giati iparxoun polla
    // closures, ena gia kathe click. Den ginontai drop!
    // Ara mallon den thelw closures. Giati einai mono me static lifetime.

    /*
    let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
        xh = event.offset_x() as f64;
        yh = event.offset_y() as f64;
        context.clear_rect(0.0, 0.0, 500.0, 500.0);
        context.rect(xh, yh, 20.0, 20.0);
        context.stroke();
    });
    canvas.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
    closure.forget();
    */

    // Here we want to call `requestAnimationFrame` in a loop, but only a fixed
    // number of times. After it's done we want all our resources cleaned up. To
    // achieve this we're using an `Rc`. The `Rc` will eventually store the
    // closure we want to execute on each frame, but to start out it contains
    // `None`.
    //
    // After the `Rc` is made we'll actually create the closure, and the closure
    // will reference one of the `Rc` instances. The other `Rc` reference is
    // used to store the closure, request the first frame, and then is dropped
    // by this function.
    //
    // Inside the closure we've got a persistent `Rc` reference, which we use
    // for all future iterations of the loop
    context.rect(xh, yh, 20.0, 20.0);
    // a stroke renders all previous paths (rect) on the context!
    context.stroke();

    //context.set_fill_style(&JsValue::from_str("white"));
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut i = 0;
    *g.borrow_mut() = Some(Closure::new(move || {
        if i > 300 {
            //body().set_text_content(Some("All done!"));

            // Drop our handle to this closure so that it will get cleaned
            // up once we return.
            let _ = f.borrow_mut().take();
            return;
        }

        // Set the body's text content to how many times this
        // requestAnimationFrame callback has fired.
        i += 1;
        let text = format!("requestAnimationFrame has been called {} times.", i);
        //body().set_text_content(Some(&text));

        xh += 1.0;
        yh -= 1.0;
        if i % 10 == 0 {
            context.stroke_rect(xh, yh, 20.0, 20.0);
            context.clear_rect(xh - 2.0, yh - 2.0, 22.0, 22.0);
        }

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap()); //first frame here! g is Some Closure
                                                           //already

    Ok(())
}

fn create_herpooles(ctx: &web_sys::CanvasRenderingContext2d, xh: &f64, yh: &f64) {
    ctx.rect(*xh, *yh, 20.0, 20.0);
    ctx.stroke();
}
