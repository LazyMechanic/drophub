use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_hooks::prelude::*;

#[hook]
pub fn use_display_mode() -> UseLocalStorageHandle<DisplayMode> {
    use_local_storage("theme".into())
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisplayMode {
    Light,
    Dark,
}

impl DisplayMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            DisplayMode::Light => "light",
            DisplayMode::Dark => "dark",
        }
    }

    pub fn is_dark(&self) -> bool {
        *self == DisplayMode::Dark
    }
}
