use crate::theme::ThemeColors;
use gpui::*;

pub fn toolbar_btn<F>(label: &str, colors: ThemeColors, on_click: F) -> impl IntoElement
where
    F: Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
{
    toolbar_btn_with_color(label, colors, colors.text, on_click)
}

pub fn toolbar_btn_with_color<F>(
    label: &str,
    colors: ThemeColors,
    text_color: gpui::Rgba,
    on_click: F,
) -> impl IntoElement
where
    F: Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
{
    div()
        .px_2()
        .py(px(2.0))
        .bg(colors.background_tertiary)
        .rounded_sm()
        .cursor_pointer()
        .child(
            div()
                .text_size(px(12.0))
                .text_color(text_color)
                .child(label.to_string()),
        )
        .on_mouse_down(MouseButton::Left, on_click)
}

pub fn toolbar_btn_enabled<F>(
    label: &str,
    enabled: bool,
    colors: ThemeColors,
    on_click: F,
) -> AnyElement
where
    F: Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
{
    if enabled {
        toolbar_btn(label, colors, on_click).into_any_element()
    } else {
        div()
            .px_2()
            .py(px(2.0))
            .rounded_sm()
            .child(div().text_size(px(12.0)).child(label.to_string()))
            .bg(colors.background_secondary)
            .text_color(colors.text_secondary)
            .into_any_element()
    }
}
