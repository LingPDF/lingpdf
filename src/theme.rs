use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Theme {
    Light,
    #[default]
    Dark,
}

#[derive(Debug, Clone, Copy)]
pub struct ThemeColors {
    pub background: Rgba,
    pub background_secondary: Rgba,
    pub background_tertiary: Rgba,
    pub text: Rgba,
    pub text_secondary: Rgba,
    pub border: Rgba,
    pub toolbar: Rgba,
    pub status_bar: Rgba,
    pub pdf_view: Rgba,
    pub moon_color: Rgba,
    pub sun_color: Rgba,
}

impl ThemeColors {
    pub fn for_theme(theme: Theme) -> Self {
        match theme {
            Theme::Light => Self::light(),
            Theme::Dark => Self::dark(),
        }
    }

    fn light() -> Self {
        Self {
            background: rgb(0xffffff),
            background_secondary: rgb(0xf5f5f5),
            background_tertiary: rgb(0xe8e8e8),
            text: rgb(0x1a1a1a),
            text_secondary: rgb(0x666666),
            border: rgb(0xd0d0d0),
            toolbar: rgb(0xe0e0e0),
            status_bar: rgb(0xe0e0e0),
            pdf_view: rgb(0xf0f0f0),
            moon_color: rgb(0x1a1a1a),
            sun_color: rgb(0xffcc00),
        }
    }

    fn dark() -> Self {
        Self {
            background: rgb(0x333333),
            background_secondary: rgb(0x2b2b2b),
            background_tertiary: rgb(0x404040),
            text: rgb(0xcccccc),
            text_secondary: rgb(0x888888),
            border: rgb(0x1a1a1a),
            toolbar: rgb(0x2b2b2b),
            status_bar: rgb(0x2b2b2b),
            pdf_view: rgb(0x404040),
            moon_color: rgb(0xcccccc),
            sun_color: rgb(0xffdd44),
        }
    }
}
