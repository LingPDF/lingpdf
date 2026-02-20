#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]
#![allow(unexpected_cfgs)]

use std::sync::Arc;

mod app;
mod i18n;
mod pdf;
mod print;
mod theme;
mod utils;

rust_i18n::i18n!("locales", fallback = "en");

use app::actions::register_actions;
use app::menu::Quit;
use app::{PdfReaderApp, WINDOW_DEFAULT_HEIGHT, WINDOW_DEFAULT_WIDTH};
use gpui::{prelude::*, App, Application, Menu, MenuItem, SystemMenuType, WindowHandle};

/// Update application menus based on current language
fn update_menus(cx: &mut App) {
    let menus = app::menu::create_menus();

    let mut full_menus = vec![Menu {
        name: "LingPDF".into(),
        items: vec![
            MenuItem::os_submenu("Services", SystemMenuType::Services),
            MenuItem::separator(),
            MenuItem::action("Quit", Quit),
        ],
    }];
    full_menus.extend(menus);

    cx.set_menus(full_menus);
}

fn main() {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    let file_path = args.get(1).cloned();

    Application::new().run(move |cx: &mut App| {
        cx.activate(true);

        cx.on_action(|_: &Quit, cx: &mut App| {
            cx.quit();
        });

        let app_state = Arc::new(app::state::AppState::new());
        let language = app_state.get_language();
        i18n::I18n::new(language);

        // Set initial menus
        update_menus(cx);

        #[cfg(target_os = "macos")]
        let titlebar_options = gpui::TitlebarOptions {
            title: Some("LingPDF".into()),
            appears_transparent: true,
            ..Default::default()
        };

        let file_path_clone = file_path.clone();

        let window_handle: WindowHandle<PdfReaderApp> = cx
            .open_window(
                gpui::WindowOptions {
                    #[cfg(target_os = "macos")]
                    titlebar: Some(titlebar_options),
                    #[cfg(not(target_os = "macos"))]
                    titlebar: None,
                    window_bounds: Some(gpui::WindowBounds::Windowed(gpui::Bounds::centered(
                        None,
                        gpui::Size::new(
                            gpui::px(WINDOW_DEFAULT_WIDTH),
                            gpui::px(WINDOW_DEFAULT_HEIGHT),
                        ),
                        cx,
                    ))),
                    ..Default::default()
                },
                move |window, cx| {
                    cx.new(move |cx| {
                        let mut app = PdfReaderApp::new(app_state.clone(), window, cx);

                        if let Some(path_str) = &file_path_clone {
                            let path = std::path::PathBuf::from(path_str);
                            if path.exists() {
                                app.open_file_in_new_tab(path, cx);
                            } else {
                                log::error!("File not found: {}", path_str);
                            }
                        }

                        app
                    })
                },
            )
            .unwrap();

        // Register all actions
        register_actions(cx, window_handle);
    });
}
