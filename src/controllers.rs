use druid::{
    widget::Controller, ContextMenu, Data, Env, Event, EventCtx, LocalizedString, MenuDesc,
    MenuItem, Widget,
};

use crate::data::{AppState, CLEAR_CANVAS, SAVE_CANVAS, SHOW_MAP_WINDOW};

pub struct ContextMenuController;

impl<T, W: Widget<T>> Controller<T, W> for ContextMenuController {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            Event::MouseDown(ref mouse) if mouse.button.is_right() => {
                let menu = ContextMenu::new(make_context_menu::<AppState>(), mouse.pos);
                ctx.show_context_menu(menu);
            }
            _ => child.event(ctx, event, data, env),
        }
    }
}

fn make_context_menu<T: Data>() -> MenuDesc<T> {
    MenuDesc::empty()
        .append(MenuItem::new(LocalizedString::new("Save"), SAVE_CANVAS))
        .append(MenuItem::new(LocalizedString::new("Clear"), CLEAR_CANVAS))
        .append(MenuItem::new(LocalizedString::new("Show Map"), SHOW_MAP_WINDOW))
}
