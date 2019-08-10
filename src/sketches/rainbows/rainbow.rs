extern crate rand;

use nannou::prelude::*;
use rand::seq::SliceRandom;

pub fn execute() {
    nannou::app(model).update(update).run();
}

struct Model {
    rainbow: Vec<Arc>,
    current_colours: Vec<(f32, f32, f32)>,
    all_colour_groups: Vec<(Vec<(f32, f32, f32)>)>,
}

struct Arc {
    position: Point2,
    velocity: Vector2,
    acceleration: Vector2,
    life_span: f32,
    colour_choice: (f32, f32, f32),
}

impl Arc {
    fn new(coordinates: Point2, index: usize, colour_scheme: &Vec<(f32, f32, f32)>) -> Self {
        let color_index = index % colour_scheme.len();
        let colour_choice = colour_scheme[color_index];

        Arc {
            position: coordinates,
            velocity: vec2(0.05, 0.05),
            acceleration: vec2(0.05, 0.05),
            colour_choice,
            life_span: 150.0,
        }
    }

    fn update(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
        self.life_span -= 5.0;
    }

    fn display(&self, draw: &app::Draw) {
        let size = 5.0 + (150.0 - self.life_span) * self.position.x;

        let (r, g, b) = self.colour_choice;
        draw.ellipse()
            .xy(pt2(0.0, -200.0))
            .w_h(size, size)
            .rgb(r, g, b);
    }
}

fn model(_app: &App) -> Model {
    _app.new_window()
        .with_dimensions(640, 360)
        .mouse_pressed(mouse_pressed)
        .view(view)
        .build()
        .unwrap();

    let rainbow = vec![
        (1.0, 0.0, 0.0),     // red
        (1.0, 0.498, 0.0),   // orange
        (1.0, 1.0, 0.0),     // yellow
        (0.0, 1.0, 0.0),     // green
        (0.0, 0.0, 1.0),     // blue
        (0.294, 0.0, 0.509), // indigo
        (0.580, 0.0, 0.827), // violet
    ];

    let ace = vec![
        (0.0, 0.0, 0.0),
        (0.639, 0.639, 0.639),
        (1.0, 1.0, 1.0),
        (0.506, 0.0, 0.510),
    ];

    let agender = vec![
        (0.0, 0.0, 0.0),
        (1.0, 1.0, 1.0),
        (0.686, 0.925, 0.725),
        (0.733, 0.733, 0.733),
        (0.0, 0.0, 0.0),
    ];

    let bi = vec![
        (0.851, 0.0, 0.435),
        (0.455, 0.302, 0.596),
        (0.0, 0.2, 0.671),
    ];

    let bear = vec![
        (0.388, 0.220, 0.0),
        (0.843, 0.388, 0.0),
        (1.0, 0.871, 0.345),
        (1.0, 0.906, 0.710),
        (0.333, 0.333, 0.333),
        (0.0, 0.0, 0.0),
    ];

    let genderqueer = vec![
        (0.710, 0.494, 0.863), // purple
        (1.0, 1.0, 1.0),       // white
        (0.286, 0.502, 0.133), // green
    ];

    let genderfluid = vec![
        (1.0, 0.459, 0.635),
        (1.0, 1.0, 1.0),
        (0.745, 0.094, 0.839),
        (0.0, 0.0, 0.0),
        (0.2, 0.243, 0.741),
    ];

    let leather = vec![
        (0.0, 0.0, 0.0),
        (0.0, 0.0, 1.0),
        (1.0, 1.0, 1.0),
        (1.0, 0.0, 0.0),
        (0.0, 0.0, 0.0),
        (0.0, 0.0, 1.0),
        (0.0, 0.0, 0.0),
        (0.0, 0.0, 1.0),
        (0.0, 0.0, 0.0),
        (0.0, 0.0, 1.0),
    ];

    let lesbian = vec![
        (0.643, 0.0, 0.380),
        (0.718, 0.333, 0.573),
        (0.925, 0.925, 0.918),
        (0.769, 0.306, 0.333),
        (0.541, 0.118, 0.016),
    ];

    let pan = vec![(1.0, 0.078, 0.549), (1.0, 0.855, 0.0), (0.020, 0.682, 1.0)];

    let nb = vec![
        (0.0, 0.0, 0.0),
        (1.0, 0.957, 0.2),
        (1.0, 1.0, 1.0),
        (0.608, 0.349, 0.816),
        (0.0, 0.0, 0.0),
    ];

    let trans = vec![
        (0.333, 0.804, 0.988),
        (1.0, 1.0, 1.0),
        (0.969, 0.659, 0.722),
    ];

    Model {
        rainbow: Vec::new(),
        current_colours: vec![
            (1.0, 0.0, 0.0),     // red
            (1.0, 0.498, 0.0),   // orange
            (1.0, 1.0, 0.0),     // yellow
            (0.0, 1.0, 0.0),     // green
            (0.0, 0.0, 1.0),     // blue
            (0.294, 0.0, 0.509), // indigo
            (0.580, 0.0, 0.827), //violet
        ],
        all_colour_groups: vec![
            rainbow,
            ace,
            agender,
            bear,
            bi,
            genderqueer,
            genderfluid,
            lesbian,
            leather,
            pan,
            nb,
            trans,
        ],
    }
}

fn mouse_pressed(_app: &App, _model: &mut Model, _button: MouseButton) {
    _model.current_colours = _model
        .all_colour_groups
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_vec();
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    _model.rainbow.push(Arc::new(
        pt2(0.0, 10.0),
        _model.rainbow.len(),
        &_model.current_colours,
    ));
    for i in 0.._model.rainbow.len() {
        _model.rainbow[i].update();
    }

    if _model.rainbow.len() > 150 {
        _model.rainbow.drain(0..(2 * _model.current_colours.len()));
        println!("> 150")
    }

    // TODO: this is a work-around to prevent to drain older Arcs
    // A nicer approach should be found
    if _model.rainbow.len() > 500 {
        _model.rainbow.drain(0..500);
        println!("> 500")
    }
}

fn view(_app: &App, _model: &Model, frame: Frame) -> Frame {
    let draw = _app.draw();
    draw.background().color(BLACK);

    for particle in _model.rainbow.iter() {
        particle.display(&draw);
    }

    draw.to_frame(_app, &frame).unwrap();

    frame
}
