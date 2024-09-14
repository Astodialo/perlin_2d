use chrono::prelude::*;
use nannou::color::GetHue;
use nannou::color::{Hue, LinSrgba, RgbHue};
use nannou::noise::NoiseFn;
use nannou::noise::{BasicMulti, Seedable};
use nannou::prelude::*;
use std::ops::Add;
use std::process;

fn main() {
    nannou::app(model).update(update).run();
}

struct Polarity {
    r: bool,
    g: bool,
    b: bool,
}

struct Model {
    noise: BasicMulti,
    points: Vec<Point2>,
    clrs: Vec<LinSrgba>,
    polarity: Polarity,
    paused: bool,
    ctrl_key_pressed: bool,
}

fn check_clr(model: &mut Model, pol: bool, clr: &str) {
    if pol {
        if clr == "r" {
            model.polarity.r = false;
        }
        if clr == "g" {
            model.polarity.g = false;
        }
        if clr == "b" {
            model.polarity.b = false;
        }
    } else {
        if clr == "r" {
            model.polarity.r = true;
        }
        if clr == "g" {
            model.polarity.g = true;
        }
        if clr == "b" {
            model.polarity.b = true;
        }
    }
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(600, 400)
        .view(view)
        .key_pressed(key_pressed)
        .key_released(key_released)
        .build()
        .unwrap();

    app.set_loop_mode(LoopMode::rate_fps(0.00001));

    Model {
        noise: BasicMulti::new().set_seed(random()),
        points: Vec::new(),
        clrs: Vec::new(),
        polarity: Polarity {
            r: true,
            g: true,
            b: true,
        },
        paused: false,
        ctrl_key_pressed: false,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    if model.paused {
        return;
    }

    if app.elapsed_frames() == 0 {
        model
            .clrs
            .push(lin_srgba(random_f32(), random_f32(), random_f32(), 0.8));
    }

    let rect = app.window_rect();
    let t = app.elapsed_frames() as f64 / 111.;

    if model.points.len() < (rect.w()) as usize {
        let step = (model.points.len() * 3) as f32;
        let amplitude = model.noise.get([step as f64 / 222., 0., t]) as f32;
        let clr_amp = model.noise.get([step as f64 / 111., 0., t / 11.]) as f32;
        let mut new_r = model.clrs[model.clrs.len() - 1].red as f32;
        let mut new_g = model.clrs[model.clrs.len() - 1].green as f32;
        let mut new_b = model.clrs[model.clrs.len() - 1].blue as f32;

        if model.polarity.r {
            new_r += clr_amp / 3.;
            check_clr(model, model.polarity.r, "r")
        } else {
            new_r -= clr_amp / 3.;
            check_clr(model, model.polarity.r, "r")
        }
        if model.polarity.g {
            new_g += clr_amp / 3.;
            check_clr(model, model.polarity.r, "g")
        } else {
            new_g -= clr_amp / 3.;
            check_clr(model, model.polarity.r, "g")
        }
        if model.polarity.b {
            new_b += clr_amp / 3.;
            check_clr(model, model.polarity.r, "b")
        } else {
            new_b -= clr_amp / 3.;
            check_clr(model, model.polarity.r, "b")
        }

        println!("{}--{}--{}", new_r, new_g, new_b);
        let clr = lin_srgba(new_r, new_g, new_b, 0.8);

        model.clrs.push(clr);
        model.points.push(pt2(step, amplitude));
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    if model.paused {
        return;
    }

    let draw = app.draw();
    let rect = app.window_rect();

    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }
    for i in 1..=model.points.len() {
        let p = &model.points[i - 1];
        let clr = model.clrs[i - 1];
        let y = map_range(p[1], 0., 1., rect.top() - 10., rect.bottom() + 10.) - rect.top();
        draw.ellipse()
            .x_y(p[0] + rect.left(), y)
            .w_h(99., 99.)
            .color(clr);
    }

    draw.to_frame(app, &frame).unwrap();
}

/// React to key-presses
fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::C => {
            if model.ctrl_key_pressed {
                process::exit(0);
            }
        }
        Key::S => {
            let file_path = saved_image_path(app);
            app.main_window().capture_frame(file_path);
        }
        Key::Space => {
            model.paused = !model.paused;
        }
        Key::LControl => {
            model.ctrl_key_pressed = true;
        }
        _other_key => {}
    }
}

/// React to key releases
fn key_released(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::LControl => {
            model.ctrl_key_pressed = false;
        }
        _other_key => {}
    }
}

/// Get the path to the next captured frame
fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    app.project_path()
        .expect("failed to locate `project_path`")
        .join("frames")
        .join(format!("frame{:05}", frame.nth()))
        .with_extension("png")
}

/// Get the path to the next saved image
fn saved_image_path(app: &App) -> std::path::PathBuf {
    app.project_path()
        .expect("failed to locate `project_path`")
        .join("saved")
        .join(format!("image{:05}", chrono::offset::Local::now()))
        .with_extension("png")
}
