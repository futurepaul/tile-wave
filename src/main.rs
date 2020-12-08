use druid::{AppLauncher, WindowDesc};

mod data;
use data::{AppState, Canvas};

mod view;
use view::build_ui;

mod widgets;

mod controllers;
mod delegate;

pub fn main() {
    let main_window = WindowDesc::new(build_ui)
        .title("Tial Wave")
        .window_size((438.0, 434.0))
        .resizable(false);

    let initial_state = AppState::new();

    AppLauncher::with_window(main_window)
        .delegate(delegate::Delegate)
        .launch(initial_state)
        .expect("Failed to launch application");
}
