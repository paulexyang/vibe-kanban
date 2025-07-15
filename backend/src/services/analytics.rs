// Analytics service disabled - no data collection

use serde_json::Value;

#[derive(Debug, Clone)]
pub struct AnalyticsConfig {
    pub posthog_api_key: String,
    pub posthog_api_endpoint: String,
    pub enabled: bool,
}

impl AnalyticsConfig {
    pub fn new(_user_enabled: bool) -> Self {
        Self {
            posthog_api_key: String::new(),
            posthog_api_endpoint: String::new(),
            enabled: false, // Always disabled
        }
    }
}

#[derive(Debug)]
pub struct AnalyticsService {
    config: AnalyticsConfig,
}

impl AnalyticsService {
    pub fn new(config: AnalyticsConfig) -> Self {
        Self { config }
    }

    pub fn is_enabled(&self) -> bool {
        false // Always disabled
    }

    pub fn track_event(&self, _user_id: &str, _event_name: &str, _properties: Option<Value>) {
        // No-op - analytics disabled
    }
}

/// Returns a dummy user ID
pub fn generate_user_id() -> String {
    "disabled".to_string()
}