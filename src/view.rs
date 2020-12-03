use druid::{
    im::Vector,
    lens,
    widget::Label,
    widget::List,
    widget::Painter,
    widget::{CrossAxisAlignment, Flex, MainAxisAlignment, SizedBox},
    Color, LensExt, RenderContext, Widget, WidgetExt,
};

use crate::{data::*, widgets::PaintCanvas};

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

pub fn build_ui() -> impl Widget<AppState> {
    let canvas = PaintCanvas {
        pixel_size: (0., 0.).into(),
    };

    Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_flex_child(canvas.lens(AppState::active_canvas), 1.)
        .with_spacer(2.)
        .with_child(List::new(single_color).lens(lens::Identity.map(
            |data: &AppState| (data.clone(), data.palette.clone()),
            |data: &mut AppState, (new_data, _): (AppState, Vector<Color>)| {
                *data = new_data;
            },
        )))
        .background(Color::WHITE)
        .border(Color::WHITE, 2.)
}
