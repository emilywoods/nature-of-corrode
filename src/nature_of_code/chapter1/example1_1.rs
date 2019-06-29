// The Nature of Code, by Daniel Shiffman [1]
//
// Example 1.1: Bouncing ball with no vectors
//
// With help from Nannou example code [2]
//
// [1] https://natureofcode.com/
// [2] https://github.com/nannou-org/nannou/blob/master/examples/nature_of_code/chp_01_vectors/1_1_bouncingball_novectors.rs

use nannou::prelude::*;

pub fn execute() {
    nannou::app(model).update(update).run();
}

struct Model {
    x: f32,
    y: f32,
    xspeed: f32,
    yspeed: f32,
}

fn model(_app: &App) -> Model {
    let x = 100.0;
    let y = 100.0;
    let xspeed = 1.0;
    let yspeed = 3.3;

    let _window = _app
        .new_window()
        .with_dimensions(640, 360)
        .view(view)
        .build()
        .unwrap();

    Model {
        x,
        y,
        xspeed,
        yspeed,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    _model.x = _model.x + _model.xspeed;
    _model.y = _model.y + _model.yspeed;

    let window = _app.window_rect();

    if (_model.x > window.right()) || (_model.x < window.left()) {
        _model.xspeed = _model.xspeed * -1.0;
    }

    if (_model.y > window.top()) || (_model.y < window.bottom()) {
        _model.yspeed = _model.yspeed * -1.0;
    }
}

fn view(_app: &App, _model: &Model, frame: Frame) -> Frame {
    let draw = _app.draw();
    draw.background().color(WHITE);

    draw.ellipse()
        .x_y(_model.x, _model.y)
        .w_h(16.0, 16.0)
        .rgba(0.5, 0.5, 0.5, 1.0);

    draw.to_frame(_app, &frame).unwrap();

    frame
}
