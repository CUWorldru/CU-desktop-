use tauri::{
    image::Image,
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    webview::{PermissionKind, PermissionResponse},
    Manager, WindowEvent,
};

#[cfg(windows)]
mod companion_identity;
#[cfg(windows)]
mod companion_http;

fn tray_icon() -> Image<'static> {
    // Draw a tiny monochrome CU mark in memory so the first shell does not
    // depend on a generated binary icon asset. Replace with approved branding
    // assets before packaging a public release.
    const SIZE: u32 = 32;
    let mut rgba = vec![0_u8; (SIZE * SIZE * 4) as usize];
    for y in 0..SIZE {
        for x in 0..SIZE {
            let dx = x as i32 - 15;
            let dy = y as i32 - 15;
            let d2 = dx * dx + dy * dy;
            let i = ((y * SIZE + x) * 4) as usize;
            if (85..=145).contains(&d2) {
                rgba[i] = 150;
                rgba[i + 1] = 195;
                rgba[i + 2] = 255;
                rgba[i + 3] = 255;
            }
        }
    }
    Image::new_owned(rgba, SIZE, SIZE)
}

pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_http::init())
        .on_permission_request(|webview, kind| {
            // Only the bundled CU app or local Vite dev server may request
            // media permissions. External pages never receive mic/camera access.
            let trusted_app_origin = webview.url().map(|url| {
                let host = url.host_str().unwrap_or_default();
                let bundled_app = host == "tauri.localhost"
                    || (url.scheme() == "tauri" && host == "localhost");
                let local_dev = cfg!(debug_assertions)
                    && matches!(host, "localhost" | "127.0.0.1" | "::1");
                bundled_app || local_dev
            }).unwrap_or(false);

            if !trusted_app_origin {
                return match kind {
                    PermissionKind::Microphone | PermissionKind::Camera => PermissionResponse::Deny,
                    _ => PermissionResponse::Default,
                };
            }

            match kind {
                // WebView2 handles the actual Windows device/OS permission flow.
                PermissionKind::Microphone | PermissionKind::Camera => PermissionResponse::Allow,
                _ => PermissionResponse::Default,
            }
        });

    #[cfg(windows)]
    let builder = builder.invoke_handler(tauri::generate_handler![
        companion_identity::companion_identity_get,
        companion_identity::companion_identity_sign,
        companion_identity::companion_session_info,
        companion_identity::companion_session_clear,
        companion_identity::companion_session_is_present,
        companion_http::companion_pairing_start,
        companion_http::companion_pairing_poll,
        companion_http::companion_pairing_submit_proof,
        companion_http::companion_pairing_exchange,
        companion_http::companion_session_refresh,
        companion_http::companion_session_revoke,
        companion_http::companion_account_me,
        companion_http::companion_account_session
    ]);

    builder
        .setup(|app| {
            let open = MenuItem::with_id(app, "open", "Open CU Desktop", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Quit CU Desktop", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&open, &quit])?;

            TrayIconBuilder::new()
                .icon(tray_icon())
                .tooltip("CU Desktop")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_tray_icon_event(|tray, event| {
                    let should_open = matches!(
                        event,
                        TrayIconEvent::Click {
                            button: MouseButton::Left,
                            button_state: MouseButtonState::Up,
                            ..
                        } | TrayIconEvent::DoubleClick {
                            button: MouseButton::Left,
                            ..
                        }
                    );
                    if should_open {
                        if let Some(window) = tray.app_handle().get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "open" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => app.exit(0),
                    _ => {}
                })
                .build(app)?;
            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                // Closing the window hides CU; the tray Quit command exits it.
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .run(tauri::generate_context!())
        .expect("failed to run CU Desktop");
}
