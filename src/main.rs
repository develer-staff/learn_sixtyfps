use std::time::Duration;
sixtyfps::include_modules!();

fn format_date_time(format: &str) -> sixtyfps::SharedString {
    chrono::Local::now().format(format).to_string().into()
}

fn current_time() -> sixtyfps::SharedString {
    format_date_time("%H:%M")
}

fn date_as_string() -> sixtyfps::SharedString {
    format_date_time("%A %d %B")
}

fn main() {
    let main_window = MainWindow::new();
    let main_window_weak = main_window.as_weak();
    let date_progress: sixtyfps::Timer = Default::default();
    date_progress.start(
        sixtyfps::TimerMode::Repeated,
        Duration::from_secs(1),
        move || {
            if let Some(main_window) = main_window_weak.upgrade() {
                main_window
                    .global::<Utils>()
                    .set_current_time(current_time());
                main_window
                    .global::<Utils>()
                    .set_date_as_string(date_as_string());
                // TODO: update all other strings as well
            }
        },
    );
    main_window.run();
}
