use wasm_bindgen::prelude::*;

use crate::Herpooles;
use crate::Zombie;

pub fn draw_herpooles(ctx: &web_sys::CanvasRenderingContext2d, h: &Herpooles, c: &str) {
    ctx.set_stroke_style(&JsValue::from_str(c));
    let hx: f64 = h.x.into();
    let hy: f64 = h.y.into();

    // rectangle
    //ctx.fill_rect(h.x.into(), h.y.into(), 20.0, 20.0); //TODO: pameterize size

    // triangle
    //ctx.begin_path();
    //ctx.move_to(hx, hy);
    //ctx.line_to(hx + 10.0, hy - 20.0);
    //ctx.line_to(hx + 20.0, hy);
    //ctx.close_path();
    //ctx.fill();

    // Define the dimensions of the rectangle
    //let rect_width = 40.0;
    //let rect_height = 60.0;

    // Set the starting position for drawing the human figure
    let start_x = hx; //rect_width / 2.0;
    let start_y = hy; //rect_height / 6.0;

    // Set the sizes for different body parts
    const SCALE: f64 = 0.2;
    let head_radius = 20.0 * SCALE;
    let body_height = 100.0 * SCALE;
    let leg_height = 100.0 * SCALE;
    let arm_width = 80.0 * SCALE;
    let elbow_relative_point = 0.5; // ratio
    let neck_height = body_height * 0.2;

    // Set the color and line width for the path
    //ctx.line_width = 2;

    // Begin drawing the path
    ctx.begin_path();

    // Draw the head
    ctx.arc(
        start_x,
        start_y,
        head_radius,
        0.0,
        std::f64::consts::PI * 2.0,
    )
    .unwrap();

    // Draw the body
    ctx.move_to(start_x, start_y + head_radius);
    ctx.line_to(start_x, start_y + head_radius + body_height);

    // Draw the legs
    ctx.move_to(start_x, start_y + head_radius + body_height);
    ctx.line_to(
        start_x - arm_width / 2.0,
        start_y + head_radius + body_height + leg_height,
    );
    ctx.move_to(start_x, start_y + head_radius + body_height);
    ctx.line_to(
        start_x + arm_width / 2.0,
        start_y + head_radius + body_height + leg_height,
    );

    // Draw the arms until the elbow
    ctx.move_to(start_x, start_y + head_radius + neck_height);
    ctx.line_to(
        start_x - arm_width / 2.0,           // elbow
        start_y + head_radius + neck_height, // this is the height of the shoulders
    );
    ctx.move_to(start_x, start_y + head_radius + neck_height);
    ctx.line_to(
        start_x + arm_width / 2.0,
        start_y + head_radius + neck_height,
    );

    // Draw the hands -- elbow is at start_x - arm_width * elbow_relative_point
    ctx.move_to(
        start_x - arm_width * elbow_relative_point,
        start_y + head_radius + neck_height,
    );
    ctx.line_to(
        start_x - arm_width,                       // until the end of the hand
        start_y + head_radius + body_height * 0.1, // TODO: parameterize the tilt of the hands
    );
    ctx.move_to(
        start_x + arm_width * elbow_relative_point,
        start_y + head_radius + neck_height,
    );
    ctx.line_to(
        start_x + arm_width, // until the end of the hand
        start_y + head_radius + body_height * 0.1,
    );
    ctx.close_path();
    ctx.stroke();

    // Draw the crown
    ctx.begin_path();
    ctx.set_fill_style(&JsValue::from_str("brown"));
    let crown_height = 5.0 * SCALE;
    let crown_width = 50.0 * SCALE;
    let half_base = crown_width / 6.0;
    let crown_bottom = start_y - head_radius - crown_height;
    let crown_top = crown_bottom - crown_width / 2.0;
    let extra_height = crown_width * 0.1;
    let crown_start_x = start_x - crown_width / 2.0;
    // Left triangle of the crown
    ctx.move_to(crown_start_x, crown_bottom);
    ctx.line_to(crown_start_x + half_base, crown_top);
    ctx.line_to(crown_start_x + 2.0 * half_base, crown_bottom);
    ctx.line_to(crown_start_x, crown_bottom);
    // Middle triangle
    ctx.move_to(crown_start_x + 2.0 * half_base, crown_bottom);
    ctx.line_to(crown_start_x + 3.0 * half_base, crown_top - extra_height);
    ctx.line_to(crown_start_x + 4.0 * half_base, crown_bottom);
    ctx.line_to(crown_start_x + 2.0 * half_base, crown_bottom);
    // Right triangle
    ctx.move_to(crown_start_x + 4.0 * half_base, crown_bottom);
    ctx.line_to(crown_start_x + 5.0 * half_base, crown_top);
    ctx.line_to(crown_start_x + 6.0 * half_base, crown_bottom);
    ctx.line_to(crown_start_x + 4.0 * half_base, crown_bottom);

    ctx.close_path();
    ctx.fill();

    // Draw the cape
    ctx.begin_path();
    ctx.set_fill_style(&JsValue::from_str("black"));
    // elbow is at start_x + arm_width * elbow_relative_point
    let cape_width = arm_width; // elbow to albow
    let cape_height = leg_height + body_height;
    let cape_start_x = start_x - cape_width / 2.0;
    let cape_end_x = cape_start_x + cape_width;
    let cape_top_y = start_y + head_radius + neck_height; // start at the shoulders
    let cape_bottom_y = cape_top_y + body_height;
    let left_tilt = cape_width * 0.1;

    ctx.move_to(cape_start_x, cape_top_y);
    ctx.line_to(cape_start_x - left_tilt, cape_bottom_y);
    ctx.line_to(cape_end_x - left_tilt, cape_bottom_y);
    ctx.line_to(cape_end_x, cape_top_y);
    ctx.line_to(cape_start_x, cape_top_y);

    let cape_control_x1 = cape_start_x + cape_width * 0.2;
    let cape_control_y1 = cape_bottom_y + cape_height * 0.2;
    let cape_control_x2 = cape_start_x + cape_width * 0.65;
    let cape_control_y2 = cape_bottom_y;

    ctx.move_to(cape_start_x - left_tilt, cape_bottom_y);
    ctx.bezier_curve_to(
        cape_control_x1,
        cape_control_y1,
        cape_control_x2,
        cape_control_y2,
        cape_end_x - left_tilt,
        cape_bottom_y,
    );

    ctx.fill();

    /*
    ctx.begin_path();
    ctx.set_fill_style(&JsValue::from_str("blue"));
    ctx.arc(
        cape_start_x - left_tilt,
        cape_bottom_y,
        5.0,
        0.0,
        2.0 * std::f64::consts::PI * 2.0,
    )
    .unwrap();
    ctx.arc(
        cape_end_x - left_tilt,
        cape_bottom_y,
        5.0,
        0.0,
        2.0 * std::f64::consts::PI * 2.0,
    )
    .unwrap();
    ctx.close_path();
    ctx.fill();

    ctx.begin_path();
    ctx.set_fill_style(&JsValue::from_str("red"));
    ctx.arc(
        cape_control_x1,
        cape_control_y1,
        5.0,
        0.0,
        2.0 * std::f64::consts::PI * 2.0,
    )
    .unwrap(); // Start point
    ctx.arc(
        cape_control_x2,
        cape_control_y2,
        5.0,
        0.0,
        2.0 * std::f64::consts::PI * 2.0,
    )
    .unwrap();
    ctx.close_path();
    ctx.fill();
    */
}

pub fn draw_zombie(ctx: &web_sys::CanvasRenderingContext2d, h: &Zombie, c: &str) {
    ctx.set_stroke_style(&JsValue::from_str(c));
    //ctx.fill_rect(h.x.into(), h.y.into(), 20.0, 20.0);

    let zx: f64 = h.x.into();
    let zy: f64 = h.y.into();
    // Set the starting position for drawing the human figure
    let start_x = zx; //rect_width / 2.0;
    let start_y = zy; //rect_height / 6.0;

    // Set the sizes for different body parts
    let head_radius = 4.0;
    let body_height = 12.0;
    let leg_height = 12.0;
    let arm_width = 8.0;

    // Set the color and line width for the path
    //ctx.line_width = 2;

    // Begin drawing the path
    ctx.begin_path();

    // Draw the head
    ctx.arc(
        start_x,
        start_y,
        head_radius,
        0.0,
        std::f64::consts::PI * 2.0,
    )
    .unwrap();

    // Draw the body
    ctx.move_to(start_x, start_y + head_radius);
    ctx.line_to(start_x, start_y + head_radius + body_height);

    // Draw the legs
    ctx.move_to(start_x, start_y + head_radius + body_height);
    ctx.line_to(
        start_x - arm_width / 2.0,
        start_y + head_radius + body_height + leg_height,
    );
    ctx.move_to(start_x, start_y + head_radius + body_height);
    ctx.line_to(
        start_x + arm_width / 2.0,
        start_y + head_radius + body_height + leg_height,
    );

    // Draw the arms
    ctx.move_to(
        start_x - arm_width / 2.0,
        start_y + head_radius + body_height / 2.0,
    );
    ctx.line_to(
        start_x - arm_width,
        start_y + head_radius + body_height / 2.0,
    );
    ctx.move_to(
        start_x + arm_width / 2.0,
        start_y + head_radius + body_height / 2.0,
    );
    ctx.line_to(
        start_x + arm_width,
        start_y + head_radius + body_height / 2.0,
    );

    // End drawing the path
    ctx.close_path();

    // Stroke the path to display it on the canvas
    ctx.stroke();
}
