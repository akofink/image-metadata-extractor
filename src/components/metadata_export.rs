use crate::export::{generate_csv, generate_txt};
use crate::types::ImageData;
use crate::utils::download_file;
use std::collections::HashSet;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MetadataExportProps {
    pub image_data: ImageData,
    pub selected_metadata: HashSet<String>,
}

#[function_component(MetadataExport)]
pub fn metadata_export(props: &MetadataExportProps) -> Html {
    let include_basic_info = use_state(|| true);
    let include_gps = use_state(|| true);

    let data = &props.image_data;
    let selected_metadata = &props.selected_metadata;

    let export_json = {
        let data = data.clone();
        let selected_metadata = selected_metadata.clone();
        let include_basic_info = include_basic_info.clone();
        let include_gps = include_gps.clone();

        Callback::from(move |_| {
            let filtered_data =
                data.filter_metadata(&selected_metadata, *include_basic_info, *include_gps);
            if let Ok(json) = serde_json::to_string_pretty(&filtered_data) {
                download_file(
                    &json,
                    &format!("{}_filtered_metadata.json", data.name),
                    "application/json",
                );
            }
        })
    };

    let export_csv = {
        let data = data.clone();
        let selected_metadata = selected_metadata.clone();
        let include_basic_info = include_basic_info.clone();
        let include_gps = include_gps.clone();

        Callback::from(move |_| {
            let filtered_data =
                data.filter_metadata(&selected_metadata, *include_basic_info, *include_gps);
            let csv = generate_csv(&filtered_data);
            download_file(
                &csv,
                &format!("{}_filtered_metadata.csv", data.name),
                "text/csv",
            );
        })
    };

    let export_txt = {
        let data = data.clone();
        let selected_metadata = selected_metadata.clone();
        let include_basic_info = include_basic_info.clone();
        let include_gps = include_gps.clone();

        Callback::from(move |_| {
            let filtered_data =
                data.filter_metadata(&selected_metadata, *include_basic_info, *include_gps);
            let txt = generate_txt(&filtered_data);
            download_file(
                &txt,
                &format!("{}_filtered_metadata.txt", data.name),
                "text/plain",
            );
        })
    };

    html! {
        <div style="background: #fff3cd; padding: 15px; border-radius: 4px; margin-top: 20px; border: 1px solid #ffeaa7;">
            <h3>{"📊 Export Metadata"}</h3>
            <p style="margin-bottom: 15px; color: #856404;">
                {"Download selected metadata in your preferred format:"}
            </p>

            <div style="margin-bottom: 15px; padding: 10px; background: rgba(255,255,255,0.7); border-radius: 4px;">
                <h4 style="margin: 0 0 10px 0; font-size: 14px;">{"Include in Export:"}</h4>
                <div style="display: flex; gap: 15px; flex-wrap: wrap;">
                    <label style="display: flex; align-items: center; gap: 5px; cursor: pointer;">
                        <input
                            type="checkbox"
                            checked={*include_basic_info}
                            onchange={{
                                let include_basic_info = include_basic_info.clone();
                                Callback::from(move |_| include_basic_info.set(!*include_basic_info))
                            }}
                        />
                        {"File Info (name, size, dimensions)"}
                    </label>
                    <label style="display: flex; align-items: center; gap: 5px; cursor: pointer;">
                        <input
                            type="checkbox"
                            checked={*include_gps}
                            onchange={{
                                let include_gps = include_gps.clone();
                                Callback::from(move |_| include_gps.set(!*include_gps))
                            }}
                        />
                        {"GPS Location"}
                    </label>
                </div>
                <div style="margin-top: 10px; font-size: 12px; color: #666;">
                    {format!("{} EXIF fields selected", selected_metadata.len())}
                    {" • Use checkboxes above to select specific metadata"}
                </div>
            </div>

            <div style="display: flex; gap: 10px; flex-wrap: wrap;">
                <button
                    onclick={export_json}
                    style="background: #007bff; color: white; border: none; padding: 8px 16px; border-radius: 4px; cursor: pointer; font-weight: bold;"
                >
                    {"📄 JSON"}
                </button>
                <button
                    onclick={export_csv}
                    style="background: #28a745; color: white; border: none; padding: 8px 16px; border-radius: 4px; cursor: pointer; font-weight: bold;"
                >
                    {"📊 CSV"}
                </button>
                <button
                    onclick={export_txt}
                    style="background: #6c757d; color: white; border: none; padding: 8px 16px; border-radius: 4px; cursor: pointer; font-weight: bold;"
                >
                    {"📝 Text"}
                </button>
            </div>
        </div>
    }
}
