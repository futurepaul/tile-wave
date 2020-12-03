use druid::{AppLauncher, WindowDesc};

mod data;
use data::{AppState, Canvas};

mod view;
use view::build_ui;

mod widgets;

pub fn main() {
    let main_window = WindowDesc::new(build_ui)
        .title("Tial Wave")
        .window_size((438.0, 400.0))
        .resizable(false);

    let initial_state = AppState::new();

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}
