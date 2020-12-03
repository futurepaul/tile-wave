use std::ops::{Index, IndexMut};

use druid::{
    im::{vector, Vector},
    Env, EventCtx,
};
use druid::{Color, Data, Lens};

pub const CANVAS_SIZE: usize = 16;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub active_canvas: Canvas,
    pub modules: Vector<Canvas>,
    pub selected_color: Color,
    pub palette: Vector<Color>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            active_canvas: Canvas::new(),
            modules: vector![],
            selected_color: Color::WHITE,
            palette: vector![Color::BLACK, Color::WHITE, Color::rgb8(10, 127, 127)],
        }
    }

    pub fn click_color(ctx: &mut EventCtx, (data, color): &mut (Self, Color), _env: &Env) {
        dbg!(color.clone());
        data.selected_color = color.clone();
        data.active_canvas.current_color = color.clone();
    }
}

// struct CanvasLens;

// impl Lens<AppState, Canvas> for CanvasLens {
//     fn with<V, F: FnOnce(&Canvas) -> V>(&self, data: &AppState, f: F) -> V {
//         let canvas =
//     }

//     fn with_mut<V, F: FnOnce(&mut Canvas) -> V>(&self, data: &mut AppState, f: F) -> V {
//         todo!()
//     }
// }

#[derive(Clone, Data, PartialEq)]
pub struct Canvas {
    pub drawing: bool,
    pub current_color: Color,
    pub storage: Vector<Color>,
}

impl Canvas {
    pub fn new() -> Self {
        let storage = (0..CANVAS_SIZE * CANVAS_SIZE)
            .into_iter()
            .map(|_| Color::BLACK)
            .collect();
        Self {
            drawing: false,
            current_color: Color::WHITE,
            storage,
        }
    }
}

#[derive(Clone, Copy, Data)]
pub struct CanvasPos {
    pub row: usize,
    pub col: usize,
}

impl Index<CanvasPos> for Canvas {
    type Output = Color;
    fn index(&self, pos: CanvasPos) -> &Self::Output {
        let idx = pos.row * CANVAS_SIZE + pos.col;
        self.storage.index(idx)
    }
}

impl IndexMut<CanvasPos> for Canvas {
    fn index_mut(&mut self, pos: CanvasPos) -> &mut Self::Output {
        let idx = pos.row * CANVAS_SIZE + pos.col;
        self.storage.index_mut(idx)
    }
}
