use crate::exports::edgee::components::data_collection::Event;
use crate::gosquared::{CampaignInfo, ScreenInfo};
use std::collections::HashMap;

pub fn insert_if_nonempty(map: &mut HashMap<String, String>, key: &str, value: &str) {
    if !value.trim().is_empty() {
        map.insert(key.to_string(), value.to_string());
    }
}

pub fn parse_language(locale: &str) -> Option<String> {
    locale.split('-').next().map(|s| s.to_string())
}

pub fn screen_from_event(event: &Event) -> ScreenInfo {
    ScreenInfo {
        height: event.context.client.screen_height,
        width: event.context.client.screen_width,
        pixel_ratio: Some(event.context.client.screen_density),
    }
}

pub fn campaign_from_event(event: &Event) -> CampaignInfo {
    CampaignInfo {
        name: event.context.campaign.name.clone(),
        source: event.context.campaign.source.clone(),
        medium: event.context.campaign.medium.clone(),
        content: event.context.campaign.content.clone(),
        term: event.context.campaign.term.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_if_nonempty_works() {
        let mut map = HashMap::new();
        insert_if_nonempty(&mut map, "key", "value");
        assert_eq!(map.get("key"), Some(&"value".to_string()));

        insert_if_nonempty(&mut map, "empty", "");
        assert!(!map.contains_key("empty"));
    }

    #[test]
    fn parse_browser_info_detects_chrome() {
        let ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";
        let (name, version) = parse_browser_info(ua);
        assert_eq!(name, Some("Chrome".to_string()));
        assert_eq!(version, Some("120.0.0.0".to_string()));
    }

    #[test]
    fn mixpanel_endpoint_works() {
        let region = "api-eu";
        let endpoint = mixpanel_endpoint(region);
        assert_eq!(endpoint, "https://api-eu.mixpanel.com");
    }
}
