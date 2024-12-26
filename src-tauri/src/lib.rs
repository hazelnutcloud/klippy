use tauri::{Emitter, Listener, WebviewUrl, WebviewWindowBuilder};

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
                    .with_shortcut("alt+k")?
                    .with_handler(|app, _shortcut, event| match event.state() {
                        tauri_plugin_global_shortcut::ShortcutState::Pressed => {
                            app.emit("hide", ()).unwrap();
                        }
                        _ => return,
                    })
                    .build(),
            )?;

            let cursor = app.cursor_position()?;
            let monitor = app.monitor_from_point(cursor.x, cursor.y)?.unwrap();
            let width = 320.;
            let window_builder = WebviewWindowBuilder::new(app, "klippy", WebviewUrl::default())
                .decorations(false)
                .skip_taskbar(true)
                .always_on_top(true)
                .inner_size(width, monitor.size().height as f64 - 88.0)
                .position(monitor.size().width as f64 - (width + 18.), 0.)
                .resizable(false);
            let window = window_builder.build()?;

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
