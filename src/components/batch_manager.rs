//! Batch management UI: shows basic progress when multiple files are selected.
//! Phase 1, slice 1: minimal skeleton to support sequential processing and progress.

use crate::types::ImageData;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct BatchManagerProps {
    #[prop_or_default]
    pub in_progress: bool,
    #[prop_or(0)]
    pub processed: usize,
    #[prop_or(0)]
    pub total: usize,
    #[prop_or_default]
    pub last_file: Option<ImageData>,
}

#[function_component(BatchManager)]
pub fn batch_manager(props: &BatchManagerProps) -> Html {
    if !props.in_progress || props.total == 0 {
        return html! {};
    }

    let pct = if props.total > 0 {
        (props.processed * 100) / props.total
    } else {
        0
    };

    html! {
        <div style="background:#eef5ff;border:1px solid #cfe2ff;border-radius:6px;padding:12px;margin:16px 0;" data-testid="batch-progress">
            <div style="font-weight:bold;color:#084298;">{"Batch Processing"}</div>
            <div style="margin:8px 0;color:#084298;" data-testid="batch-status">{ format!("Processed {} of {} ({}%)", props.processed, props.total, pct) }</div>
            <div style="height:8px;background:#dbeafe;border-radius:4px;overflow:hidden;">
                <div style={format!("height:8px;background:#0d6efd;width:{}%", pct)} data-testid="batch-progress-bar"></div>
            </div>
        </div>
    }
}
