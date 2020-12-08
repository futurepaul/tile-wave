pub struct Delegate;

use druid::{AppDelegate, Command, DelegateCtx, Env, Handled, Target};

use crate::data::{AppState, CLEAR_CANVAS, SAVE_CANVAS};

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        match cmd {
            // _ if cmd.is(sys_cmds::NEW_FILE) => {
            //     let new_win = WindowDesc::new(ui_builder)
            //         .menu(make_menu(data))
            //         .window_size((data.selected as f64 * 100.0 + 300.0, 500.0));
            //     ctx.new_window(new_win);
            //     Handled::Yes
            // }
            _ if cmd.is(SAVE_CANVAS) => {
                data.save_active_canvas_as_image();
                Handled::Yes
            }
            _ if cmd.is(CLEAR_CANVAS) => {
                data.clear_active_canvas();
                Handled::Yes
            }
            _ => Handled::No,
        }
    }
}
