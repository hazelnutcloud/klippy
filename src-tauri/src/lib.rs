use tauri::{Emitter, Listener, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_positioner::{Position, WindowExt};

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            app.handle().plugin(
                tauri_plugin_global_shortcut::Builder::new()
                    .with_shortcut("alt+.")?
                    .with_handler(|app, _shortcut, event| match event.state() {
                        tauri_plugin_global_shortcut::ShortcutState::Pressed => {
                            app.emit("hide", ()).unwrap();
                        }
                        _ => return,
                    })
                    .build(),
            )?;
            app.handle().plugin(tauri_plugin_positioner::init())?;

            let cursor = app.cursor_position()?;
            let monitor = app.monitor_from_point(cursor.x, cursor.y)?.unwrap_or(
                app.primary_monitor()?.unwrap_or(
                    app.available_monitors()?
                        .first()
                        .expect("no monitors found")
                        .clone(),
                ),
            );

            let window_builder = WebviewWindowBuilder::new(app, "klippy", WebviewUrl::default())
                .skip_taskbar(true)
                .always_on_top(true)
                .inner_size(320., monitor.size().height as f64 - 88.);
            let window = window_builder.build()?;

            window.set_decorations(false)?;
            window.set_resizable(false)?;
            window.move_window(Position::TopRight)?;

            app.listen("hide", move |_| {
                if window.is_visible().unwrap() {
                    window.hide().unwrap();
                } else {
                    window.show().unwrap();
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
