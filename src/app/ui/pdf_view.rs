use crate::app::state::ScrollMode;
use crate::theme::ThemeColors;
use crate::tr;
use gpui::*;

use super::super::PdfReaderApp;

impl PdfReaderApp {
    pub(super) fn render_pdf_view(
        &self,
        active_tab_id: Option<usize>,
        colors: ThemeColors,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let scroll_mode = self.state.get_scroll_mode();

        if active_tab_id.is_none() {
            return div()
                .flex_1()
                .h_full()
                .bg(colors.pdf_view)
                .flex()
                .items_center()
                .justify_center()
                .child(
                    div().flex().flex_col().items_center().gap_3().child(
                        div()
                            .text_size(px(14.0))
                            .text_color(colors.text_secondary)
                            .child(tr!("welcome_message")),
                    ),
                )
                .into_any_element();
        }

        if let Some(tab_id) = active_tab_id {
            if let Some(tab) = self.state.tabs.get_tab(tab_id) {
                if let Some(image) = &tab.page_image {
                    let (width, height) = tab.page_dimensions.unwrap_or((800, 600));
                    let render_image = image.clone();

                    match scroll_mode {
                        ScrollMode::Page => {
                            return div()
                                .flex_1()
                                .overflow_hidden()
                                .bg(colors.pdf_view)
                                .flex()
                                .flex_row()
                                .on_scroll_wheel(cx.listener(
                                    |this, event: &ScrollWheelEvent, _window, cx| match event.delta
                                    {
                                        ScrollDelta::Pixels(delta) => {
                                            if delta.y > px(10.0) {
                                                this.next_page(cx);
                                            } else if delta.y < px(-10.0) {
                                                this.prev_page(cx);
                                            }
                                        }
                                        ScrollDelta::Lines(delta) => {
                                            if delta.y > 0.5 {
                                                this.next_page(cx);
                                            } else if delta.y < -0.5 {
                                                this.prev_page(cx);
                                            }
                                        }
                                    },
                                ))
                                .children([
                                    div().flex_1().h_full().cursor_pointer().on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _event, _window, cx| {
                                            this.prev_page(cx);
                                        }),
                                    ),
                                    div()
                                        .flex_1()
                                        .h_full()
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .child(
                                            img(render_image.clone())
                                                .block()
                                                .max_w(px(width as f32))
                                                .max_h(px(height as f32)),
                                        ),
                                    div().flex_1().h_full().cursor_pointer().on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _event, _window, cx| {
                                            this.next_page(cx);
                                        }),
                                    ),
                                ])
                                .into_any_element();
                        }
                        ScrollMode::Smooth => {
                            return div()
                                .flex_1()
                                .overflow_hidden()
                                .bg(colors.pdf_view)
                                .child(
                                    div()
                                        .flex_1()
                                        .flex()
                                        .flex_col()
                                        .items_center()
                                        .p_4()
                                        .gap_4()
                                        .child(
                                            img(render_image.clone())
                                                .block()
                                                .max_w(px(width as f32)),
                                        ),
                                )
                                .into_any_element();
                        }
                    }
                }
            }
        }

        div()
            .flex_1()
            .h_full()
            .bg(colors.pdf_view)
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(colors.text_secondary)
                    .child(tr!("pdf.loading")),
            )
            .into_any_element()
    }
}
