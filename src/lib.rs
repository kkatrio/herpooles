use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
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
    let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
        xh = event.offset_x() as f64;
        yh = event.offset_y() as f64;
        update_context(&context, &xh, &yh);
    });
    canvas.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;

    fn update_context(ctx: &web_sys::CanvasRenderingContext2d, xh: &f64, yh: &f64) {
        ctx.clear_rect(0.0, 0.0, 500.0, 500.0);
        ctx.rect(*xh, *yh, 20.0, 20.0);
        ctx.stroke();
    }

    closure.forget();

    Ok(())
}

fn create_herpooles(ctx: &web_sys::CanvasRenderingContext2d, xh: &f64, yh: &f64) {
    ctx.rect(*xh, *yh, 20.0, 20.0);
    ctx.stroke();
}
