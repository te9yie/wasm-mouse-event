use std::{cell::Cell, rc::Rc};
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::mouse::MouseState;

fn get_context(canvas: &HtmlCanvasElement) -> CanvasRenderingContext2d {
    let context = canvas.get_context("2d").unwrap();
    let context = context.unwrap();
    let context = context.dyn_into::<CanvasRenderingContext2d>().unwrap();
    context
}

struct Rect {
    cx: f64,
    cy: f64,
    radius: f64,
}

impl Rect {
    fn new(cx: f64, cy: f64, radius: f64) -> Self {
        Self { cx, cy, radius }
    }
}

pub struct App {
    canvas: HtmlCanvasElement,
    mouse_state: Rc<Cell<MouseState>>,
    rects: [Rect; 2],
    down: bool,
}

impl App {
    pub fn new(canvas: HtmlCanvasElement, mouse_state: Rc<Cell<MouseState>>) -> Self {
        let rects = [Rect::new(0.0, 0.0, 0.0), Rect::new(0.0, 0.0, 0.0)];
        Self {
            canvas,
            mouse_state,
            rects,
            down: false,
        }
    }

    pub fn update(&mut self, _delta: f64) {
        let width = self.canvas.width() as f64;
        let height = self.canvas.height() as f64;

        let mouse = self.mouse_state.get();
        let x = mouse.x as f64;
        let y = mouse.y as f64;
        let inv_x = width - x;
        let inv_y = height - y;

        self.rects[0].cx = x;
        self.rects[0].cy = height / 2.0;
        self.rects[0].radius = y / 2.0 + 10.0;

        self.rects[1].cx = inv_x;
        self.rects[1].cy = height / 2.0;
        self.rects[1].radius = inv_y / 2.0 + 10.0;

        self.down = (mouse.buttons & 0x1) != 0;
    }

    pub fn draw(&self) {
        let width = self.canvas.width() as f64;
        let height = self.canvas.height() as f64;
        let ctx = get_context(&self.canvas);

        ctx.clear_rect(0.0, 0.0, width, height);
        ctx.set_fill_style(&"#595857".into());
        ctx.stroke_rect(0.0, 0.0, width, height);

        if self.down {
            ctx.set_fill_style(&"#d7003a".into());
        } else {
            ctx.set_fill_style(&"#595857".into());
        }

        for rect in self.rects.iter() {
            ctx.fill_rect(
                rect.cx - rect.radius,
                rect.cy - rect.radius,
                rect.radius * 2.0,
                rect.radius * 2.0,
            );
        }
    }
}
