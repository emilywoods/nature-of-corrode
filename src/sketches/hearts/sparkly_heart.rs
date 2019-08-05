use nannou::prelude::*;

pub fn execute() {
    nannou::sketch(view);
}

fn view(app: &App, frame: Frame) -> Frame {
    app.window_rect();
    let draw = app.draw();

    draw.background().color(BLACK);

    let n_points = (2.0 * PI * 100.0).round() as i32;
    let half_thickness = 1.0;

    let vertices = (0..n_points)
        .map(|i| {
            let num = i  as f32 * 0.01;
            let x = 2.0 *16.0 * pow(num.sin(), 3);
            let y = -2.0 * (-1.0 * (13.0 * num.cos() - (5.0 * (2.0 * num).cos()) - (2.0 * (3.0 * num).cos()) - (4.0*num).cos()));

            pt2(x, y)
        })
        .enumerate()
        .map(|(_, p)| {
            let r =random_f32() * 2.0 - 1.0;
            let g = random_f32() * 2.0 - 1.0;
            let rgba = nannou::color::Rgba::new(r, g, 0.5, 1.0);
            geom::vertex::Rgba(p, rgba)
        });

    draw.polyline().vertices(half_thickness, vertices);

    draw.to_frame(app, &frame).unwrap();

    frame
}

