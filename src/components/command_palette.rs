//! Command palette for keyboard-driven navigation and actions.

use crate::types::Theme;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Command {
    pub id: String,
    pub name: String,
    pub description: String,
    pub shortcut: Option<String>,
    pub action: CommandAction,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CommandAction {
    ToggleTheme,
    ToggleExplanations,
    ToggleFileInfo,
    ToggleGps,
    ExportJson,
    ExportCsv,
    ExportTxt,
    CopyJson,
    CopyCsv,
    CopyTxt,
    UploadNew,
    ExpandImage,
    SelectAllMetadata,
    DeselectAllMetadata,
}

struct PaletteColors {
    overlay: &'static str,
    background: &'static str,
    text: &'static str,
    border: &'static str,
    selected_bg: &'static str,
    shortcut_bg: &'static str,
    shortcut_text: &'static str,
    description: &'static str,
}

const LIGHT_PALETTE_COLORS: PaletteColors = PaletteColors {
    overlay: "rgba(0,0,0,0.5)",
    background: "#ffffff",
    text: "#333333",
    border: "#ddd",
    selected_bg: "#f0f8ff",
    shortcut_bg: "#f5f5f5",
    shortcut_text: "#666",
    description: "#888",
};

const DARK_PALETTE_COLORS: PaletteColors = PaletteColors {
    overlay: "rgba(0,0,0,0.7)",
    background: "#1e1e1e",
    text: "#e0e0e0",
    border: "#444",
    selected_bg: "#2a2a2a",
    shortcut_bg: "#333",
    shortcut_text: "#aaa",
    description: "#bbb",
};

/// Properties for [`CommandPalette`].
#[derive(Properties, PartialEq)]
pub struct CommandPaletteProps {
    pub visible: bool,
    pub theme: Theme,
    pub on_close: Callback<()>,
    pub on_command: Callback<CommandAction>,
    pub has_image: bool,
    pub is_expanded: bool,
    pub show_explanations: bool,
    pub include_file_info: bool,
    pub include_gps: bool,
}

/// Keyboard-driven command palette for power users.
#[function_component(CommandPalette)]
pub fn command_palette(props: &CommandPaletteProps) -> Html {
    let search_query = use_state(String::new);
    let selected_index = use_state(|| 0);
    let input_ref = use_node_ref();

    let colors = match props.theme {
        Theme::Light => LIGHT_PALETTE_COLORS,
        Theme::Dark => DARK_PALETTE_COLORS,
    };

    // Build available commands based on current state
    let commands = {
        let mut cmds = vec![
            Command {
                id: "toggle-theme".to_string(),
                name: if props.theme == Theme::Light {
                    "Switch to Dark Mode"
                } else {
                    "Switch to Light Mode"
                }
                .to_string(),
                description: "Toggle between light and dark themes".to_string(),
                shortcut: None,
                action: CommandAction::ToggleTheme,
            },
            Command {
                id: "upload-new".to_string(),
                name: "Upload New File".to_string(),
                description: "Select a new file to analyze".to_string(),
                shortcut: Some("Ctrl+O".to_string()),
                action: CommandAction::UploadNew,
            },
        ];

        if props.has_image {
            cmds.extend(vec![
                Command {
                    id: "toggle-explanations".to_string(),
                    name: if props.show_explanations {
                        "Hide Field Explanations"
                    } else {
                        "Show Field Explanations"
                    }
                    .to_string(),
                    description: "Toggle metadata field descriptions".to_string(),
                    shortcut: Some("?".to_string()),
                    action: CommandAction::ToggleExplanations,
                },
                Command {
                    id: "expand-image".to_string(),
                    name: if props.is_expanded {
                        "Close Image View"
                    } else {
                        "Expand Image View"
                    }
                    .to_string(),
                    description: "Toggle full-screen image view".to_string(),
                    shortcut: Some("Space".to_string()),
                    action: CommandAction::ExpandImage,
                },
                Command {
                    id: "select-all".to_string(),
                    name: "Select All Metadata".to_string(),
                    description: "Select all metadata fields for export".to_string(),
                    shortcut: Some("Ctrl+A".to_string()),
                    action: CommandAction::SelectAllMetadata,
                },
                Command {
                    id: "deselect-all".to_string(),
                    name: "Deselect All Metadata".to_string(),
                    description: "Clear all metadata field selections".to_string(),
                    shortcut: Some("Ctrl+D".to_string()),
                    action: CommandAction::DeselectAllMetadata,
                },
                Command {
                    id: "toggle-file-info".to_string(),
                    name: if props.include_file_info {
                        "Exclude File Info"
                    } else {
                        "Include File Info"
                    }
                    .to_string(),
                    description: "Toggle file info in exports (name, size, dimensions)".to_string(),
                    shortcut: None,
                    action: CommandAction::ToggleFileInfo,
                },
                Command {
                    id: "toggle-gps".to_string(),
                    name: if props.include_gps {
                        "Exclude GPS Data"
                    } else {
                        "Include GPS Data"
                    }
                    .to_string(),
                    description: "Toggle GPS location data in exports".to_string(),
                    shortcut: None,
                    action: CommandAction::ToggleGps,
                },
                Command {
                    id: "export-json".to_string(),
                    name: "Export as JSON".to_string(),
                    description: "Download metadata in JSON format".to_string(),
                    shortcut: Some("Ctrl+1".to_string()),
                    action: CommandAction::ExportJson,
                },
                Command {
                    id: "export-csv".to_string(),
                    name: "Export as CSV".to_string(),
                    description: "Download metadata in CSV format".to_string(),
                    shortcut: Some("Ctrl+2".to_string()),
                    action: CommandAction::ExportCsv,
                },
                Command {
                    id: "export-txt".to_string(),
                    name: "Export as Text".to_string(),
                    description: "Download metadata in text format".to_string(),
                    shortcut: Some("Ctrl+3".to_string()),
                    action: CommandAction::ExportTxt,
                },
                Command {
                    id: "copy-json".to_string(),
                    name: "Copy JSON to Clipboard".to_string(),
                    description: "Copy metadata as JSON to clipboard".to_string(),
                    shortcut: Some("Ctrl+Shift+1".to_string()),
                    action: CommandAction::CopyJson,
                },
                Command {
                    id: "copy-csv".to_string(),
                    name: "Copy CSV to Clipboard".to_string(),
                    description: "Copy metadata as CSV to clipboard".to_string(),
                    shortcut: Some("Ctrl+Shift+2".to_string()),
                    action: CommandAction::CopyCsv,
                },
                Command {
                    id: "copy-txt".to_string(),
                    name: "Copy Text to Clipboard".to_string(),
                    description: "Copy metadata as text to clipboard".to_string(),
                    shortcut: Some("Ctrl+Shift+3".to_string()),
                    action: CommandAction::CopyTxt,
                },
            ]);
        }

        cmds
    };

    // Filter commands based on search query
    let filtered_commands = {
        let query = search_query.to_lowercase();
        if query.is_empty() {
            commands.clone()
        } else {
            commands
                .into_iter()
                .filter(|cmd| {
                    cmd.name.to_lowercase().contains(&query)
                        || cmd.description.to_lowercase().contains(&query)
                })
                .collect::<Vec<_>>()
        }
    };

    // Focus input when palette becomes visible
    use_effect_with(
        (props.visible, input_ref.clone()),
        move |(visible, input_ref)| {
            if *visible && let Some(input) = input_ref.cast::<web_sys::HtmlInputElement>() {
                let _ = input.focus();
            }
        },
    );

    // Reset state when palette becomes visible
    use_effect_with(props.visible, {
        let search_query = search_query.clone();
        let selected_index = selected_index.clone();
        move |visible| {
            if *visible {
                search_query.set(String::new());
                selected_index.set(0);
            }
        }
    });

    // Handle keyboard navigation
    let on_keydown = {
        let selected_index = selected_index.clone();
        let filtered_commands = filtered_commands.clone();
        let on_command = props.on_command.clone();
        let on_close = props.on_close.clone();

        Callback::from(move |e: KeyboardEvent| match e.key().as_str() {
            "Escape" => {
                e.prevent_default();
                on_close.emit(());
            }
            "ArrowDown" => {
                e.prevent_default();
                let max_index = filtered_commands.len().saturating_sub(1);
                selected_index.set((*selected_index + 1).min(max_index));
            }
            "ArrowUp" => {
                e.prevent_default();
                selected_index.set(selected_index.saturating_sub(1));
            }
            "Enter" => {
                e.prevent_default();
                if let Some(cmd) = filtered_commands.get(*selected_index) {
                    on_command.emit(cmd.action.clone());
                    on_close.emit(());
                }
            }
            _ => {}
        })
    };

    let on_search_input = {
        let search_query = search_query.clone();
        let selected_index = selected_index.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                search_query.set(target.value());
                selected_index.set(0); // Reset selection when searching
            }
        })
    };

    let overlay_click = {
        let on_close = props.on_close.clone();
        Callback::from(move |e: MouseEvent| {
            // Only close if clicking the overlay, not the modal content
            if let Some(target) = e.target_dyn_into::<web_sys::HtmlElement>()
                && target.class_name().contains("command-palette-overlay")
            {
                on_close.emit(());
            }
        })
    };

    if !props.visible {
        return html! {};
    }

    html! {
        <div
            class="command-palette-overlay"
            style={format!("position: fixed; top: 0; left: 0; width: 100%; height: 100%; background: {}; z-index: 2000; display: flex; align-items: flex-start; justify-content: center; padding-top: 20vh;", colors.overlay)}
            onclick={overlay_click}
        >
            <div style={format!("background: {}; border-radius: 8px; box-shadow: 0 8px 32px rgba(0,0,0,0.3); border: 1px solid {}; width: 90%; max-width: 600px; max-height: 60vh; display: flex; flex-direction: column;", colors.background, colors.border)}>
                <div style="padding: 16px; border-bottom: 1px solid {colors.border};">
                    <input
                        ref={input_ref}
                        type="text"
                        placeholder="Type a command..."
                        value={(*search_query).clone()}
                        oninput={on_search_input}
                        onkeydown={on_keydown.clone()}
                        style={format!("width: 100%; padding: 12px; border: none; outline: none; background: transparent; color: {}; font-size: 16px;", colors.text)}
                    />
                </div>
                <div style="flex: 1; overflow-y: auto; max-height: 400px;">
                    {
                        if filtered_commands.is_empty() {
                            html! {
                                <div style={format!("padding: 20px; text-align: center; color: {};", colors.description)}>
                                    {"No commands found"}
                                </div>
                            }
                        } else {
                            filtered_commands.iter().enumerate().map(|(index, cmd)| {
                                let is_selected = index == *selected_index;
                                let item_bg = if is_selected { colors.selected_bg } else { "transparent" };

                                let cmd_clone = cmd.clone();
                                let on_command = props.on_command.clone();
                                let on_close = props.on_close.clone();
                                let on_click = Callback::from(move |_: MouseEvent| {
                                    on_command.emit(cmd_clone.action.clone());
                                    on_close.emit(());
                                });

                                html! {
                                    <div
                                        key={cmd.id.clone()}
                                        onclick={on_click}
                                        style={format!("padding: 12px 16px; cursor: pointer; background: {}; border-left: 3px solid {}; display: flex; justify-content: space-between; align-items: center;",
                                            item_bg,
                                            if is_selected { colors.border } else { "transparent" }
                                        )}
                                    >
                                        <div style="flex: 1;">
                                            <div style={format!("color: {}; font-weight: 500; margin-bottom: 2px;", colors.text)}>
                                                {&cmd.name}
                                            </div>
                                            <div style={format!("color: {}; font-size: 12px;", colors.description)}>
                                                {&cmd.description}
                                            </div>
                                        </div>
                                        {
                                            if let Some(shortcut) = &cmd.shortcut {
                                                html! {
                                                    <div style={format!("background: {}; color: {}; padding: 2px 6px; border-radius: 3px; font-size: 11px; font-family: monospace;", colors.shortcut_bg, colors.shortcut_text)}>
                                                        {shortcut}
                                                    </div>
                                                }
                                            } else {
                                                html! {}
                                            }
                                        }
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    }
                </div>
            </div>
        </div>
    }
}
