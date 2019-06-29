// The Nature of Code, by Daniel Shiffman [1]
//
// Example 1.2: Bouncing ball with PVectors
//
// With reference to Nannou example code [2]
//
// [1] https://natureofcode.com/book/
// [2] https://github.com/nannou-org/nannou/blob/master/examples/nature_of_code/chp_01_vectors/1_2_bouncingball_vectors.rs

use nannou::prelude::*;

pub fn execute() {
    nannou::app(model).update(update).run();
}

struct Coordinates {
    x: f32,
    y: f32,
}

struct Model {
    location: Coordinates,
    velocity: Coordinates,
}

fn model(_app: &App) -> Model {
    let location = Coordinates { x: 100.0, y: 100.0 };
    let velocity = Coordinates { x: 2.5, y: 5.0 };

    let _window = _app
        .new_window()
        .with_dimensions(640, 360)
        .view(view)
        .build()
        .unwrap();

    Model { location, velocity }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    _model.location.x = _model.location.x + _model.velocity.x;
    _model.location.y = _model.location.y + _model.velocity.y;

    let window = _app.window_rect();

    if (_model.location.x > window.right()) || (_model.location.x < window.left()) {
        _model.velocity.x = _model.velocity.x * -1.0;
    }

    if (_model.location.y > window.top()) || (_model.location.y < window.bottom()) {
        _model.velocity.y = _model.velocity.y * -1.0;
    }
}

fn view(_app: &App, _model: &Model, frame: Frame) -> Frame {
    let draw = _app.draw();
    draw.background().color(BLUE);

    draw.ellipse()
        .x_y(_model.location.x, _model.location.y)
        .w_h(16.0, 16.0)
        .rgba(0.5, 0.5, 0.5, 1.0);

    draw.to_frame(_app, &frame).unwrap();

    frame
}
