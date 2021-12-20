use nannou::prelude::*;

mod pendulum;
use crate::pendulum::Pendulum;

fn main() {
    nannou::app(model)
        .size(1280, 720)
        .update(update)
        .run();
}

struct Model {
    _window: window::Id,
    pendulum: Pendulum
}

fn model(app: &App) -> Model {
    let _window = app
                            .new_window()
                            .title("Pendulum")
                            .view(view)
                            .build()
                            .unwrap();
    let win = app.window_rect().pad(10.0);

    //Custom Code
    let pendulum = Pendulum::new(win.x(), win.y(), 100.0, 1.0, 150.0, PI, 1.0, 150.0, 3.0);

    Model { _window, pendulum }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    //Custom Code
    let delta_time = _update.since_last.as_secs_f32();
    _app.main_window().set_title(&format!("{}", delta_time));

     _model.pendulum.update(delta_time);
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    //Custom Code
    let radius = 25.0;

    draw
        .ellipse()
        .x_y(_model.pendulum.origin_x, _model.pendulum.origin_y)
        .w_h(radius / 2.0, radius / 2.0)
        .color(BLACK);

    draw
        .ellipse()
        .x(_model.pendulum.top.x)
        .y(_model.pendulum.top.y)
        .w_h(radius, radius)
        .color(BLACK);

    draw
        .line()
        .start(pt2(_model.pendulum.origin_x, _model.pendulum.origin_y))
        .end(pt2(_model.pendulum.top.x, _model.pendulum.top.y))
        .weight(5.0)
        .color(BLACK);

    draw
        .ellipse()
        .x(_model.pendulum.bottom.x)
        .y(_model.pendulum.bottom.y)
        .w_h(radius, radius)
        .color(BLACK);

    draw
        .line()
        .start(pt2(_model.pendulum.top.x, _model.pendulum.top.y))
        .end(pt2(_model.pendulum.bottom.x, _model.pendulum.bottom.y))
        .weight(5.0)
        .color(BLACK);

    draw.to_frame(app, &frame).unwrap();
}