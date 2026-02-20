pub mod pdf_view;
pub mod sidebar;
pub mod status_bar;
pub mod titlebar;
pub mod toolbar;

use crate::app::shortcuts;
use crate::theme::ThemeColors;
use gpui::*;

use super::PdfReaderApp;

impl PdfReaderApp {
    pub(super) fn render_ui(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = self.state.get_theme();
        let colors = ThemeColors::for_theme(theme);
        let tabs = self.state.get_all_tabs();
        let active_tab_id = self.state.get_active_tab_id();

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(colors.background)
            .track_focus(&self.focus_handle)
            .child(self.render_combined_titlebar(&tabs, active_tab_id, colors, cx))
            .child({
                let has_doc = active_tab_id.is_some();
                self.render_toolbar(has_doc, colors, cx)
            })
            .child(
                div()
                    .flex_1()
                    .overflow_hidden()
                    .flex()
                    .flex_row()
                    .child(if self.show_sidebar && active_tab_id.is_some() {
                        self.render_sidebar(active_tab_id, colors, cx)
                            .into_any_element()
                    } else {
                        div().into_any_element()
                    })
                    .child(self.render_pdf_view(active_tab_id, colors, cx)),
            )
            .child(self.render_status_bar(active_tab_id, colors, cx))
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, window, cx| {
                shortcuts::handle_key_down_event(this, event, window, cx);
            }))
    }
}
