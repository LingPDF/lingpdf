use gpui::*;

pub fn handle_key_down_event(
    this: &mut super::PdfReaderApp,
    event: &KeyDownEvent,
    window: &mut Window,
    cx: &mut Context<super::PdfReaderApp>,
) {
    let keystroke = &event.keystroke;
    let key = keystroke.key.as_str();
    let modifiers = keystroke.modifiers;

    #[cfg(target_os = "macos")]
    {
        if modifiers.platform {
            match key {
                "o" => this.open_file_dialog(cx),
                "w" => {
                    if let Some(tab_id) = this.state.get_active_tab_id() {
                        this.close_tab(tab_id, cx);
                    }
                }
                "q" => {
                    cx.quit();
                }
                "p" => this.print(cx),
                "+" | "=" => this.zoom_in(cx),
                "-" => this.zoom_out(cx),
                "0" => this.reset_zoom(cx),
                "1" => this.fit_width(cx),
                "2" => this.fit_page(cx),
                "r" => this.rotate_clockwise(cx),
                "b" => {
                    this.show_sidebar = !this.show_sidebar;
                    cx.notify();
                }
                "g" => {}
                "t" => this.toggle_theme(cx),
                _ => {}
            }
        } else if modifiers.platform && modifiers.control && key == "f" {
            window.toggle_fullscreen();
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        if modifiers.control {
            match key {
                "o" => this.open_file_dialog(cx),
                "w" => {
                    if let Some(tab_id) = this.state.get_active_tab_id() {
                        this.close_tab(tab_id, cx);
                    }
                }
                "q" => {
                    cx.quit();
                }
                "p" => this.print(cx),
                "+" | "=" => this.zoom_in(cx),
                "-" => this.zoom_out(cx),
                "0" => this.reset_zoom(cx),
                "1" => this.fit_width(cx),
                "2" => this.fit_page(cx),
                "r" => this.rotate_clockwise(cx),
                "b" => {
                    this.show_sidebar = !this.show_sidebar;
                    cx.notify();
                }
                "g" => {}
                "t" => this.toggle_theme(cx),
                _ => {}
            }
        } else if !modifiers.control && !modifiers.shift && !modifiers.alt && key == "f11" {
            window.toggle_fullscreen();
        }
    }

    match key {
        "left" => this.prev_page(cx),
        "right" => this.next_page(cx),
        "pageup" => this.prev_page(cx),
        "pagedown" => this.next_page(cx),
        "home" => {
            if let Some(tab_id) = this.state.get_active_tab_id() {
                this.state.update_active_tab(|tab| {
                    tab.current_page = 0;
                });
                this.render_current_tab_page(tab_id, cx);
                cx.notify();
            }
        }
        "end" => {
            if let Some(tab_id) = this.state.get_active_tab_id() {
                this.state.update_active_tab(|tab| {
                    tab.current_page = tab.page_count.saturating_sub(1);
                });
                this.render_current_tab_page(tab_id, cx);
                cx.notify();
            }
        }
        _ => {}
    }
}
