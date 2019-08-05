use nannou::prelude::*;

pub fn execute() {
    nannou::sketch(view);
}

fn view(app: &App, frame: Frame) -> Frame {
    let _win = app.window_rect();
    let time = app.time;
    let draw = app.draw();

    draw.background().color(BLACK);

    let number_of_points = (2.0 * PI * 100.0).round() as i32;

    let smallest_size = 5.0 + time;
    let smallest_heart = (0..number_of_points)
        .map(|i| {
            let scaled_increment = i as f32 * 0.01;
            let x = smallest_size * x_vertex(scaled_increment);
            let y = smallest_size * y_vertex(scaled_increment);
            pt2(x, y)
        })
        .enumerate()
        .map(|(i, p)| {
            let i_as_fract = i as f32 / number_of_points as f32;
            let r = (255.0 - (time + i_as_fract) % 1.0) / 255.0;
            let rgba = nannou::color::Rgba::new(r, random_f32() * 2.0 - 1.0, 0.5, 1.0);
            geom::vertex::Rgba(p, rgba)
        });

    let mid_size = 10.0 + time;
    let mid_heart = (0..number_of_points)
        .map(|i| {
            let scaled_increment = i as f32 * 0.01;
            let x = mid_size * x_vertex(scaled_increment);
            let y = mid_size * y_vertex(scaled_increment);
            pt2(x, y)
        })
        .enumerate()
        .map(|(i, p)| {
            let i_as_fract = i as f32 / number_of_points as f32;
            let r = (time + i_as_fract) % 1.0;
            let g = (time + 0.5 - i_as_fract) % 1.0;
            let b = (time + 0.5 + i_as_fract) % 1.0;
            let rgba = nannou::color::Rgba::new(r, g, b, 1.0);
            geom::vertex::Rgba(p, rgba)
        });

    let biggest_size = 20.0 + time;
    let biggest_heart = (0..number_of_points)
        .map(|i| {
            let scaled_increment = i as f32 * 0.01;
            let x = biggest_size * x_vertex(scaled_increment);
            let y = biggest_size * y_vertex(scaled_increment);
            pt2(x, y)
        })
        .enumerate()
        .map(|(_, p)| {
            let rgba = nannou::color::Rgba::new(0.5, random_f32() * 2.0 - 1.0, 0.5, 1.0);
            geom::vertex::Rgba(p, rgba)
        });

    draw.polyline().vertices(4.0, smallest_heart);
    draw.polyline().vertices(4.0, mid_heart);
    draw.polyline().vertices(4.0, biggest_heart);

    draw.to_frame(app, &frame).unwrap();

    frame
}

fn x_vertex(num: f32) -> f32 {
    let x_vertex = 16.0 * pow(num.sin(), 3);
    x_vertex
}

fn y_vertex(num: f32) -> f32 {
    let y_vertex = 13.0 * num.cos()
        - (5.0 * (2.0 * num).cos())
        - (2.0 * (3.0 * num).cos())
        - (4.0 * num).cos();
    y_vertex
}
