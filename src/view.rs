use druid::{
    im::Vector,
    lens,
    widget::Button,
    widget::Label,
    widget::List,
    widget::Painter,
    widget::{CrossAxisAlignment, Flex, MainAxisAlignment, SizedBox},
    Color, LensExt, RenderContext, Widget, WidgetExt,
};

use crate::{
    controllers::ContextMenuController,
    data::*,
    widgets::{PaintCanvas, ViewCanvas},
};

fn single_color() -> impl Widget<(AppState, Color)> {
    let my_painter = Painter::new(|ctx, (data, color): &(AppState, Color), env| {
        let bounds = ctx.size().to_rect();
        ctx.fill(bounds, color);
        if &data.selected_color == color {
            ctx.stroke(bounds.inset(-4.).floor(), &Color::BLACK, 2.);
            ctx.stroke(bounds.inset(-2.).floor(), &Color::WHITE, 2.);
        }
    });

    let single_color = SizedBox::empty()
        .fix_height(32.)
        .fix_width(32.)
        .background(my_painter);

    single_color
        .border(Color::BLACK, 2.)
        .padding((0., 0., 0., -2.))
        .on_click(AppState::click_color)
}

fn single_module() -> impl Widget<(AppState, Canvas)> {
    let my_painter = Painter::new(|ctx, (data, module): &(AppState, Canvas), env| {
        let bounds = ctx.size().to_rect();
        if data.active_canvas_id == module.id {
            ctx.stroke(bounds.inset(-4.).floor(), &Color::BLACK, 2.);
            ctx.stroke(bounds.inset(-2.).floor(), &Color::WHITE, 2.);
        }
    });

    let too_many_lenses = lens::Identity.map(
        |(data, module): &(AppState, Canvas)| module.clone(),
        |(data, module): &mut (AppState, Canvas), new_data| {
            *module = new_data;
        },
    );

    let single_module = SizedBox::new(ViewCanvas::new().lens(too_many_lenses))
        .fix_height(32.)
        .fix_width(32.);

    single_module
        .padding((0., 0., 2., 0.))
        .on_click(AppState::click_module)
        .background(my_painter)
}

pub fn modules() -> impl Widget<AppState> {
    let module_lens = lens::Identity.map(
        |data: &AppState| (data.clone(), data.modules.clone()),
        |data: &mut AppState, (new_data, _): (AppState, Vector<Canvas>)| {
            *data = new_data;
        },
    );

    let modules = List::new(single_module).horizontal().lens(module_lens);

    let add = Button::new("+").on_click(AppState::click_add_module);

    let row = Flex::row()
        .with_child(modules)
        .with_flex_spacer(1.)
        .with_child(add);

    SizedBox::new(row).fix_height(32.).background(Color::WHITE)
}

fn single_map_module() -> impl Widget<Canvas> {
    let single_module = SizedBox::new(ViewCanvas::new())
        .fix_height(32.)
        .fix_width(32.);

    single_module
}

fn map_row() -> impl Widget<Vector<Canvas>> {
    List::new(single_map_module).horizontal()
}

fn map_grid() -> impl Widget<Vector<Vector<Canvas>>> {
    List::new(map_row)
}

pub fn map_window() -> impl Widget<AppState> {
    map_grid().lens(AppState::map)
}

pub fn build_ui() -> impl Widget<AppState> {
    let canvas = PaintCanvas::new();

    let palette_lens = lens::Identity.map(
        |data: &AppState| (data.clone(), data.palette.clone()),
        |data: &mut AppState, (new_data, _): (AppState, Vector<Color>)| {
            *data = new_data;
        },
    );

    let row = Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_flex_child(
            canvas.controller(ContextMenuController).lens(CanvasLens),
            1.,
        )
        .with_spacer(2.)
        .with_child(List::new(single_color).lens(palette_lens));

    Flex::column()
        .with_child(row)
        .with_spacer(2.)
        .with_child(modules())
        .background(Color::WHITE)
        .border(Color::WHITE, 2.)
}
