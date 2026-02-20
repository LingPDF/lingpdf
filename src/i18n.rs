use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    English,
    Chinese,
    Spanish,
}

impl Default for Language {
    fn default() -> Self {
        if let Ok(lang) = std::env::var("LANG") {
            if lang.starts_with("zh") {
                return Language::Chinese;
            } else if lang.starts_with("es") {
                return Language::Spanish;
            }
        }
        Language::English
    }
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Chinese => "zh",
            Language::Spanish => "es",
        }
    }
}

pub struct I18n;

impl I18n {
    pub fn new(language: Language) -> Self {
        rust_i18n::set_locale(language.as_str());
        Self
    }

    pub fn set_language(language: Language) {
        rust_i18n::set_locale(language.as_str());
    }
}

#[macro_export]
macro_rules! tr {
    ($key:expr) => {
        rust_i18n::t!($key).to_string()
    };
    ($key:expr, $($arg:expr => $value:expr),+) => {
        rust_i18n::t!($key, $($arg => $value),+).to_string()
    };
}
