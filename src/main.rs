use nannou::prelude::*;
use std::process::exit;

struct Model {
    _windowa: WindowId,
    // _windowb: WindowId,
    position: Option<Point2>,
}

fn main() {
    nannou::app(model).event(event).run();
}

fn model(app: &App) -> Model {
    let _windowa = app.new_window().fullscreen().view(view_circles).build().unwrap();
    // change view() arguments to differnet view_ functions
    // let _windowb = app.new_window().view(view_circles).build().unwrap();
    Model {
        _windowa,
        // _windowb,
        position: None,
    }
}

fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent { simple, .. } => match simple {
            Some(nannou::event::WindowEvent::MouseMoved(pos)) => {
                if model.position.is_none() {
                    model.position = Some(pos)
                }
                
                if app.time > 0.1 && model.position.unwrap() != pos {
                    println!("{} - {}", pos, model.position.unwrap());
                    exit(0)
                }
            }
            Some(nannou::event::WindowEvent::MousePressed(..)) => exit(0),
            _ => (),
        },
        _ => (),
    }
}

fn view_circles(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    // draw.to_frame(app, &frame).unwrap();

    let win = app.window_rect();
    let width = win.w();
    let height = win.h();

    let in_a_row = 15;
    let in_a_column = 10;

    let time = app.time/1.5;

    draw.rect().w_h(width, height).color(rgba(0.0, 0.0, 0.0, 0.2)); // background

    for i in 0..in_a_row {
        for j in 0..in_a_column {
            let i = i as f32;
            let j = j as f32;

            let x = width/(in_a_row - 1) as f32 * i - width/2.0;
            let y = height/(in_a_column - 1) as f32 * j - height/2.0;
            let r = width * 0.01 * ((time + i * 0.5 + j * 0.25).sin() + 1.1) * (i + j)/2.0;

            let red = (time * 1.2).sin();
            let green = (time * 1.5 + i * 0.5 + j * 0.5).sin();
            let blue = (time - i * 0.5 - j * 0.5).sin();

            draw.ellipse()
                .x_y(x, y)
                .radius(r)
                .color(rgb((blue + red + green) * 0.2, green * 0.2, blue * 0.9));
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
