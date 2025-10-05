//! Identifies duplicate files in batch uploads by SHA-256 hash.

use crate::types::{ImageData, Theme};
use std::collections::HashMap;
use std::rc::Rc;
use yew::prelude::*;

struct DuplicateColors {
    background: &'static str,
    text: &'static str,
    border: &'static str,
    duplicate_item: &'static str,
}

const LIGHT_DUPLICATE_COLORS: DuplicateColors = DuplicateColors {
    background: "#fff3cd",
    text: "#856404",
    border: "#ffeaa7",
    duplicate_item: "#fff9e6",
};

const DARK_DUPLICATE_COLORS: DuplicateColors = DuplicateColors {
    background: "#3d3520",
    text: "#ffc107",
    border: "#6b5a2a",
    duplicate_item: "#4a3d1a",
};

#[derive(Properties, PartialEq)]
pub struct DuplicateDetectorProps {
    pub batch_items: Vec<Rc<ImageData>>,
    pub theme: Theme,
}

/// Analyzes batch items and displays duplicate file warnings.
#[function_component(DuplicateDetector)]
pub fn duplicate_detector(props: &DuplicateDetectorProps) -> Html {
    let colors = match props.theme {
        Theme::Light => LIGHT_DUPLICATE_COLORS,
        Theme::Dark => DARK_DUPLICATE_COLORS,
    };

    // Group items by hash - only include items with hashes
    let mut hash_groups: HashMap<String, Vec<&ImageData>> = HashMap::new();
    for item in &props.batch_items {
        if let Some(hash) = &item.sha256_hash {
            hash_groups.entry(hash.clone()).or_default().push(item);
        }
    }

    // Find duplicates (hashes with more than one file)
    let duplicates: Vec<_> = hash_groups
        .iter()
        .filter(|(_, items)| items.len() > 1)
        .collect();

    if duplicates.is_empty() {
        return html! {};
    }

    // Count total duplicate files
    let total_duplicates: usize = duplicates.iter().map(|(_, items)| items.len()).sum();
    let unique_hashes = duplicates.len();

    html! {
        <div style={format!(
            "background: {}; padding: 15px; border-radius: 4px; margin-bottom: 20px; border: 2px solid {}; color: {};",
            colors.background, colors.border, colors.text
        )}>
            <h3 style="margin: 0 0 10px 0; display: flex; align-items: center; gap: 8px;">
                <span>{"⚠️"}</span>
                <span>{"Duplicate Files Detected"}</span>
            </h3>
            <p style="margin: 0 0 15px 0; font-size: 14px;">
                {format!("Found {} duplicate file{} ({} unique file{} with duplicates)",
                    total_duplicates,
                    if total_duplicates == 1 { "" } else { "s" },
                    unique_hashes,
                    if unique_hashes == 1 { "" } else { "s" }
                )}
            </p>

            {for duplicates.iter().map(|(hash, items)| {
                html! {
                    <div style={format!(
                        "background: {}; padding: 10px; border-radius: 4px; margin-bottom: 10px; border: 1px solid {};",
                        colors.duplicate_item, colors.border
                    )}>
                        <div style="margin-bottom: 8px;">
                            <strong>{"Hash: "}</strong>
                            <code style="font-size: 11px; word-break: break-all;">
                                {&hash[..16]}{"..."}
                            </code>
                        </div>
                        <div style="margin-left: 15px;">
                            <strong>{format!("{} identical files:", items.len())}</strong>
                            <ul style="margin: 5px 0 0 0; padding-left: 20px;">
                                {for items.iter().map(|item| {
                                    html! {
                                        <li style="font-size: 13px; margin: 2px 0;">
                                            {&item.name}
                                            <span style="color: #888; font-size: 11px; margin-left: 8px;">
                                                {format!("({} KB)", item.size / 1024)}
                                            </span>
                                        </li>
                                    }
                                })}
                            </ul>
                        </div>
                    </div>
                }
            })}

            <div style="margin-top: 10px; padding: 8px; background: rgba(0,0,0,0.05); border-radius: 3px; font-size: 12px;">
                <strong>{"💡 Tip: "}</strong>
                {"These files are byte-for-byte identical. Consider removing duplicates to save space."}
            </div>
        </div>
    }
}
