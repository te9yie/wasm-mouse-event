use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement, MouseEvent};

use app::App;
use mouse::MouseState;

mod app;
mod mouse;

fn create_canvas(width: u32, height: u32) -> HtmlCanvasElement {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    let canvas = document.create_element("canvas").unwrap();
    let canvas = canvas.dyn_into::<HtmlCanvasElement>().unwrap();
    canvas.set_width(width);
    canvas.set_height(height);
    body.append_child(&canvas).unwrap();
    canvas
}

fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) {
    let window = window().unwrap();
    window
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}

#[wasm_bindgen(start)]
pub fn main() {
    const WIDTH: u32 = 400;
    const HEIGHT: u32 = 200;

    let canvas = create_canvas(WIDTH, HEIGHT);

    let mouse_state = Rc::new(Cell::new(MouseState::default()));
    {
        let mouse_state = Rc::clone(&mouse_state);
        let f = Closure::wrap(Box::new(move |e: MouseEvent| {
            let current = MouseState {
                x: e.client_x(),
                y: e.client_y(),
                buttons: e.buttons(),
            };
            mouse_state.set(current);
        }) as Box<dyn FnMut(_)>);

        let window = window().unwrap();
        let document = window.document().unwrap();
        // not canvas.
        document
            .add_event_listener_with_callback("mousemove", f.as_ref().unchecked_ref())
            .unwrap();
        document
            .add_event_listener_with_callback("mousedown", f.as_ref().unchecked_ref())
            .unwrap();
        document
            .add_event_listener_with_callback("mouseup", f.as_ref().unchecked_ref())
            .unwrap();
        f.forget();
    }

    let mut app = App::new(canvas, mouse_state);

    {
        const DELTA_LIMIT: f64 = 1000.0 / 30.0;
        let mut last_update_time = 0.0;
        let f = Rc::new(RefCell::new(None));
        let ff = Rc::clone(&f);
        *ff.borrow_mut() = Some(Closure::wrap(Box::new(move |now: f64| {
            let delta: f64 /* why need? */ = now - last_update_time;
            let delta = delta.clamp(0.0, DELTA_LIMIT);
            last_update_time = now;

            app.update(delta);
            app.draw();

            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut(_)>));
        request_animation_frame(ff.borrow().as_ref().unwrap());
    }
}
