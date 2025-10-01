//! Privacy risk warning component that displays privacy assessment and warnings.

use crate::types::{ImageData, PrivacyRiskLevel, Theme};
use yew::prelude::*;

struct RiskColors {
    background: &'static str,
    border: &'static str,
    text: &'static str,
    icon: &'static str,
}

const LIGHT_RISK_COLORS_LOW: RiskColors = RiskColors {
    background: "#d1ecf1",
    border: "#bee5eb",
    text: "#0c5460",
    icon: "ℹ️",
};

const LIGHT_RISK_COLORS_MEDIUM: RiskColors = RiskColors {
    background: "#fff3cd",
    border: "#ffeaa7",
    text: "#856404",
    icon: "⚠️",
};

const LIGHT_RISK_COLORS_HIGH: RiskColors = RiskColors {
    background: "#f8d7da",
    border: "#f5c6cb",
    text: "#721c24",
    icon: "⚠️",
};

const LIGHT_RISK_COLORS_CRITICAL: RiskColors = RiskColors {
    background: "#f8d7da",
    border: "#dc3545",
    text: "#721c24",
    icon: "🚨",
};

const DARK_RISK_COLORS_LOW: RiskColors = RiskColors {
    background: "#0d3d47",
    border: "#17a2b8",
    text: "#d1ecf1",
    icon: "ℹ️",
};

const DARK_RISK_COLORS_MEDIUM: RiskColors = RiskColors {
    background: "#3d3d0a",
    border: "#ffc107",
    text: "#fff3cd",
    icon: "⚠️",
};

const DARK_RISK_COLORS_HIGH: RiskColors = RiskColors {
    background: "#3d0a0a",
    border: "#dc3545",
    text: "#f8d7da",
    icon: "⚠️",
};

const DARK_RISK_COLORS_CRITICAL: RiskColors = RiskColors {
    background: "#3d0a0a",
    border: "#dc3545",
    text: "#f8d7da",
    icon: "🚨",
};

/// Properties for [`PrivacyRiskWarning`].
#[derive(Properties, PartialEq)]
pub struct PrivacyRiskWarningProps {
    pub image_data: ImageData,
    pub theme: Theme,
}

/// Component that displays privacy risk assessment and warnings
#[function_component(PrivacyRiskWarning)]
pub fn privacy_risk_warning(props: &PrivacyRiskWarningProps) -> Html {
    let risk = props.image_data.calculate_privacy_risk();

    // Skip rendering if no privacy risks detected
    if risk.score == 0 {
        return html! {};
    }

    let colors = match (&risk.level, props.theme) {
        (PrivacyRiskLevel::Low, Theme::Light) => LIGHT_RISK_COLORS_LOW,
        (PrivacyRiskLevel::Medium, Theme::Light) => LIGHT_RISK_COLORS_MEDIUM,
        (PrivacyRiskLevel::High, Theme::Light) => LIGHT_RISK_COLORS_HIGH,
        (PrivacyRiskLevel::Critical, Theme::Light) => LIGHT_RISK_COLORS_CRITICAL,
        (PrivacyRiskLevel::Low, Theme::Dark) => DARK_RISK_COLORS_LOW,
        (PrivacyRiskLevel::Medium, Theme::Dark) => DARK_RISK_COLORS_MEDIUM,
        (PrivacyRiskLevel::High, Theme::Dark) => DARK_RISK_COLORS_HIGH,
        (PrivacyRiskLevel::Critical, Theme::Dark) => DARK_RISK_COLORS_CRITICAL,
    };

    let level_text = match risk.level {
        PrivacyRiskLevel::Low => "Low Risk",
        PrivacyRiskLevel::Medium => "Medium Risk",
        PrivacyRiskLevel::High => "High Risk",
        PrivacyRiskLevel::Critical => "Critical Risk",
    };

    html! {
        <div style={format!(
            "background: {}; padding: 15px; border-radius: 4px; margin-top: 20px; border: 2px solid {}; color: {};",
            colors.background, colors.border, colors.text
        )}>
            <h3 style="margin: 0 0 10px 0;">
                {colors.icon}{" Privacy Risk Assessment"}
            </h3>

            <div style="display: flex; align-items: center; gap: 10px; margin-bottom: 15px;">
                <span style="font-weight: bold; font-size: 14px;">
                    {"Risk Level: "}{level_text}
                </span>
                <span style={format!(
                    "background: {}; border: 1px solid {}; padding: 2px 8px; border-radius: 4px; font-size: 12px; font-weight: bold;",
                    colors.border, colors.border
                )}>
                    {format!("Score: {}/100", risk.score)}
                </span>
            </div>

            {
                if !risk.sensitive_fields.is_empty() {
                    html! {
                        <div style="margin-bottom: 15px;">
                            <h4 style="margin: 0 0 8px 0; font-size: 13px;">{"Sensitive Information Found:"}</h4>
                            <ul style="margin: 0; padding-left: 20px;">
                                {
                                    risk.sensitive_fields.iter().map(|field| {
                                        html! {
                                            <li style="margin-bottom: 4px; font-size: 13px;">{field}</li>
                                        }
                                    }).collect::<Html>()
                                }
                            </ul>
                        </div>
                    }
                } else {
                    html! {}
                }
            }

            {
                if !risk.warnings.is_empty() {
                    html! {
                        <div style="margin-bottom: 15px;">
                            <h4 style="margin: 0 0 8px 0; font-size: 13px;">{"Privacy Warnings:"}</h4>
                            <ul style="margin: 0; padding-left: 20px;">
                                {
                                    risk.warnings.iter().map(|warning| {
                                        html! {
                                            <li style="margin-bottom: 4px; font-size: 13px;">{warning}</li>
                                        }
                                    }).collect::<Html>()
                                }
                            </ul>
                        </div>
                    }
                } else {
                    html! {}
                }
            }

            {
                if !risk.consistency_issues.is_empty() {
                    html! {
                        <div style="margin-bottom: 15px; padding: 10px; background: rgba(255, 193, 7, 0.15); border-left: 3px solid #ffc107; border-radius: 4px;">
                            <h4 style="margin: 0 0 8px 0; font-size: 13px;">{"⚙️ Metadata Consistency Issues:"}</h4>
                            <ul style="margin: 0; padding-left: 20px;">
                                {
                                    risk.consistency_issues.iter().map(|issue| {
                                        html! {
                                            <li style="margin-bottom: 4px; font-size: 13px;">{issue}</li>
                                        }
                                    }).collect::<Html>()
                                }
                            </ul>
                            <p style="margin: 8px 0 0 0; font-size: 12px; font-style: italic;">
                                {"Note: Consistency issues may indicate image modification or incomplete metadata."}
                            </p>
                        </div>
                    }
                } else {
                    html! {}
                }
            }

            <div style="margin-top: 15px; padding: 10px; background: rgba(0,0,0,0.1); border-radius: 4px; font-size: 12px;">
                <strong>{"💡 Recommendation:"}</strong>
                {" Use the Image Cleaner below to remove all sensitive metadata before sharing this photo."}
            </div>
        </div>
    }
}
