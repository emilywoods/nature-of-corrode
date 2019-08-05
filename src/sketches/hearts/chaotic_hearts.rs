use nannou::prelude::*;

pub fn execute() {
    nannou::app(model).update(update).run();
}

struct Model {
    hearts: Vec<Heart>,
}

struct Heart {
    position: Point2,
    velocity: Vector2,
    acceleration: Vector2,
    life_span: f32,
}

impl Heart {
    fn new(coordinates: Point2) -> Self {
        Heart {
            acceleration: vec2(0.01, 0.01),
            velocity: vec2(random_f32() - 1.0, random_f32()),
            position: coordinates ,
            life_span: 255.0,
        }
    }

    fn update(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
        self.life_span -= 5.0;
    }

    fn display(&self, draw: &app::Draw) {
        let size = 5.0 + (150.0 - self.life_span) * self.position.x;
        let number_of_points = (2.0 * PI * 100.0).round() as i32;

        let vertices = (0..number_of_points)
            .map(|i| {
                let increment = i as f32 * 0.01;
                let x = size * (16.0 * pow(increment.sin(), 3));
                let y = size
                    * (13.0 * increment.cos()
                        - (5.0 * (2.0 * increment).cos())
                        - (2.0 * (3.0 * increment).cos())
                        - (4.0 * increment).cos());
                pt2(x, y)
            })
            .enumerate()
            .map(|(_, p)| {
                let r = (255.0 - self.life_span) / 255.0;
                let g = self.velocity.x;
                let b = 0.5;
                let rgba = nannou::color::Rgba::new(r, g, b, self.life_span / 255.0);
                geom::vertex::Rgba(p, rgba)
            });

        draw.polyline().vertices(1.0, vertices);
    }
}

fn model(_app: &App) -> Model {
    _app.new_window()
        .with_dimensions(640, 360)
        .view(view)
        .build()
        .unwrap();
    Model { hearts: Vec::new() }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    _model.hearts.push(Heart::new(pt2(0.0, 0.0)));
    for i in 0.._model.hearts.len() {
        _model.hearts[i].update();
    }
}

fn view(_app: &App, _model: &Model, frame: Frame) -> Frame {
    let draw = _app.draw();
    draw.background().color(BLACK);

    for heart in _model.hearts.iter() {
        heart.display(&draw);
    }

    draw.to_frame(_app, &frame).unwrap();

    frame
}
