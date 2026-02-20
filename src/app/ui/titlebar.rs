use crate::app::tabs::Tab;
use crate::theme::ThemeColors;
use gpui::prelude::FluentBuilder;
use gpui::*;

use super::super::PdfReaderApp;

impl PdfReaderApp {
    pub(super) fn render_combined_titlebar(
        &self,
        tabs: &[Tab],
        active_tab_id: Option<usize>,
        colors: ThemeColors,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        #[cfg(target_os = "macos")]
        let left_padding = px(70.0);
        #[cfg(not(target_os = "macos"))]
        let left_padding = px(8.0);

        let mut titlebar = div()
            .h(px(32.0))
            .w_full()
            .flex()
            .flex_row()
            .items_center()
            .bg(colors.toolbar)
            .border_b_1()
            .border_color(colors.border)
            .px_2()
            .when(cfg!(not(target_os = "macos")), |this| {
                this.on_mouse_move(cx.listener(|_this, _event, window, _cx| {
                    window.start_window_move();
                }))
            })
            .child(div().w(left_padding));

        for tab in tabs.iter() {
            let is_active = Some(tab.id) == active_tab_id;

            titlebar = titlebar.child(
                div()
                    .h(px(28.0))
                    .min_w(px(100.0))
                    .max_w(px(180.0))
                    .px_2()
                    .mx(px(1.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_1()
                    .cursor_pointer()
                    .rounded_sm()
                    .when(is_active, |this| this.bg(colors.background))
                    .when(!is_active, |this| {
                        this.bg(colors.background_secondary)
                            .hover(|hover| hover.bg(colors.background_tertiary))
                    })
                    .child(
                        div()
                            .flex_1()
                            .text_size(px(11.0))
                            .text_color(if is_active {
                                colors.text
                            } else {
                                colors.text_secondary
                            })
                            .text_ellipsis()
                            .child(tab.file_name()),
                    )
                    .when(tabs.len() > 1, |this| {
                        this.child(
                            div()
                                .p(px(2.0))
                                .text_size(px(10.0))
                                .text_color(colors.text_secondary)
                                .cursor_pointer()
                                .hover(|hover| {
                                    hover
                                        .text_color(colors.text)
                                        .bg(colors.background_tertiary)
                                        .rounded_sm()
                                })
                                .child("×")
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener({
                                        let tab_id = tab.id;
                                        move |this, _event, _window, cx| {
                                            this.close_tab(tab_id, cx);
                                        }
                                    }),
                                ),
                        )
                    })
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener({
                            let tab_id = tab.id;
                            move |this, _event, _window, cx| {
                                this.switch_tab(tab_id, cx);
                            }
                        }),
                    ),
            );
        }

        titlebar = titlebar
            .child(
                div()
                    .h(px(28.0))
                    .w(px(28.0))
                    .ml(px(2.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .cursor_pointer()
                    .hover(|hover| hover.bg(colors.background_secondary).rounded_sm())
                    .text_color(colors.text_secondary)
                    .child("+")
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _event, _window, cx| {
                            this.open_file_dialog(cx);
                        }),
                    ),
            )
            .child(div().flex_1());

        #[cfg(not(target_os = "macos"))]
        {
            titlebar = titlebar.child(self.render_window_controls(colors, cx));
        }

        #[cfg(target_os = "macos")]
        {
            titlebar = titlebar.child(
                div()
                    .h(px(28.0))
                    .px_3()
                    .flex()
                    .items_center()
                    .text_size(px(11.0))
                    .text_color(colors.text_secondary)
                    .child("LingPDF"),
            );
        }

        titlebar
    }

    #[cfg(not(target_os = "macos"))]
    fn render_window_controls(
        &self,
        colors: ThemeColors,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(2.0))
            .child(
                div()
                    .h(px(20.0))
                    .w(px(20.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .cursor_pointer()
                    .hover(|hover| hover.bg(colors.background_secondary).rounded_sm())
                    .text_color(colors.text_secondary)
                    .text_size(px(12.0))
                    .child("−")
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|_this, _event, window, _cx| {
                            window.minimize_window();
                        }),
                    ),
            )
            .child(
                div()
                    .h(px(20.0))
                    .w(px(20.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .cursor_pointer()
                    .hover(|hover| hover.bg(colors.background_secondary).rounded_sm())
                    .text_color(colors.text_secondary)
                    .text_size(px(12.0))
                    .child("□")
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|_this, _event, window, _cx| {
                            window.toggle_fullscreen();
                        }),
                    ),
            )
            .child(
                div()
                    .h(px(20.0))
                    .w(px(20.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .cursor_pointer()
                    .hover(|hover| hover.bg(gpui::rgb(0xe8_11_23)).rounded_sm())
                    .text_color(colors.text_secondary)
                    .text_size(px(12.0))
                    .child("×")
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|_this, _event, _window, cx| {
                            cx.quit();
                        }),
                    ),
            )
    }
}
