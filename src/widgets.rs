use druid::widget::prelude::*;
use druid::widget::{Button, Flex, Label, Slider};
use druid::{
    AppLauncher, Color, Data, Lens, LocalizedString, MouseButton, Point, Rect, TimerToken,
    WidgetExt, WindowDesc,
};

use crate::data::*;

pub struct PaintCanvas {
    pixel_size: Size,
}

pub struct ViewCanvas {
    pixel_size: Size,
}

impl ViewCanvas {
    pub fn new() -> Self {
        Self {
            pixel_size: (0., 0.).into(),
        }
    }
}

impl PaintCanvas {
    pub fn new() -> Self {
        Self {
            pixel_size: (0., 0.).into(),
        }
    }

    fn grid_pos(&self, p: Point) -> Option<CanvasPos> {
        let w0 = self.pixel_size.width;
        let h0 = self.pixel_size.height;
        if p.x < 0.0 || p.y < 0.0 || w0 == 0.0 || h0 == 0.0 {
            return None;
        }
        let row = (p.x / w0) as usize;
        let col = (p.y / h0) as usize;
        if row >= CANVAS_SIZE || col >= CANVAS_SIZE {
            return None;
        }
        Some(CanvasPos { row, col })
    }
}

impl Widget<Canvas> for PaintCanvas {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut Canvas, _env: &Env) {
        match event {
            Event::WindowConnected => {
                ctx.request_paint();
            }
            Event::MouseDown(e) => {
                if e.button == MouseButton::Left {
                    data.drawing = true;
                    let grid_pos_opt = self.grid_pos(e.pos);
                    grid_pos_opt
                        .iter()
                        .for_each(|pos| data[*pos] = data.current_color.clone());
                }
            }
            Event::MouseUp(e) => {
                if e.button == MouseButton::Left {
                    data.drawing = false;
                }
            }
            Event::MouseMove(e) => {
                if data.drawing {
                    let grid_pos_opt = self.grid_pos(e.pos);
                    grid_pos_opt
                        .iter()
                        .for_each(|pos| data[*pos] = data.current_color.clone());
                }
            }
            _ => {}
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &Canvas,
        _env: &Env,
    ) {
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &Canvas, _data: &Canvas, _env: &Env) {
        ctx.request_paint();
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &Canvas,
        _env: &Env,
    ) -> Size {
        let max_size = bc.max();
        let min_side = max_size.height.min(max_size.width);
        Size {
            width: min_side,
            height: min_side,
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Canvas, _env: &Env) {
        let size: Size = ctx.size();
        let w0 = size.width / CANVAS_SIZE as f64;
        let h0 = size.height / CANVAS_SIZE as f64;
        let cell_size = Size {
            width: w0,
            height: h0,
        };
        self.pixel_size = cell_size;
        for row in 0..CANVAS_SIZE {
            for col in 0..CANVAS_SIZE {
                let pos = CanvasPos { row, col };
                let point = Point {
                    x: w0 * row as f64,
                    y: h0 * col as f64,
                };
                let rect = Rect::from_origin_size(point.floor(), cell_size.ceil());
                ctx.fill(rect, &data[pos]);
            }
        }
    }
}

impl Widget<Canvas> for ViewCanvas {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut Canvas, _env: &Env) {
        match event {
            Event::WindowConnected => {
                ctx.request_paint();
            }
            _ => {}
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &Canvas,
        _env: &Env,
    ) {
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &Canvas, _data: &Canvas, _env: &Env) {
        ctx.request_paint();
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &Canvas,
        _env: &Env,
    ) -> Size {
        let max_size = bc.max();
        let min_side = max_size.height.min(max_size.width);
        Size {
            width: min_side,
            height: min_side,
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Canvas, _env: &Env) {
        let size: Size = ctx.size();
        let w0 = size.width / CANVAS_SIZE as f64;
        let h0 = size.height / CANVAS_SIZE as f64;
        let cell_size = Size {
            width: w0,
            height: h0,
        };
        self.pixel_size = cell_size;
        for row in 0..CANVAS_SIZE {
            for col in 0..CANVAS_SIZE {
                let pos = CanvasPos { row, col };
                let point = Point {
                    x: w0 * row as f64,
                    y: h0 * col as f64,
                };
                let rect = Rect::from_origin_size(point.floor(), cell_size.ceil());
                ctx.fill(rect, &data[pos]);
            }
        }
    }
}
