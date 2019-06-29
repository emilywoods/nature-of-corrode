// The Nature of Code, by Daniel Shiffman [1]
//
// Example 1.4: Multiplying a Vector
//
// With reference to Nannou example code [2]
//
// [1] https://natureofcode.com/book/
// [2] https://github.com/nannou-org/nannou/blob/master/examples/nature_of_code/chp_01_vectors/1_4_vector_multiplication.rs

use nannou::prelude::*;

pub fn execute() {
    nannou::sketch(run);
}

fn run(_app: &App, frame: Frame) -> Frame {

    let origin = vec2(0.0, 0.0);
    let mut mouse_position = _app.mouse.position();
    mouse_position *= 0.5;

    let _window = _app.main_window().set_inner_size_points(640.0, 360.0);


    let draw = _app.draw();
    draw.background().color(BLUE);

    draw.line()
        .start(origin)
        .end(mouse_position);

    draw.to_frame(_app, &frame).unwrap();

    frame
}
