//! User preferences management with localStorage persistence.

use serde::{Deserialize, Serialize};
use web_sys::window;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UserPreferences {
    pub show_explanations: bool,
    pub include_basic_info: bool,
    pub include_gps: bool,
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            show_explanations: false,
            include_basic_info: true,
            include_gps: true,
        }
    }
}

const PREFERENCES_KEY: &str = "file_metadata_extractor_preferences";

impl UserPreferences {
    /// Load preferences from localStorage, falling back to defaults
    pub fn load() -> Self {
        if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten())
            && let Ok(Some(json)) = storage.get_item(PREFERENCES_KEY)
            && let Ok(prefs) = serde_json::from_str(&json)
        {
            return prefs;
        }
        Self::default()
    }

    /// Save preferences to localStorage
    pub fn save(&self) {
        if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten())
            && let Ok(json) = serde_json::to_string(self)
        {
            let _ = storage.set_item(PREFERENCES_KEY, &json);
        }
    }

    /// Update and save a single preference field
    pub fn update_and_save(&mut self, update_fn: impl FnOnce(&mut Self)) {
        update_fn(self);
        self.save();
    }
}
