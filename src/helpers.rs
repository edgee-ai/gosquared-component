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
    fn parse_language_works() {
        assert_eq!(parse_language("en-US"), Some("en".to_string()));
        assert_eq!(parse_language("fr-FR"), Some("fr".to_string()));
        assert_eq!(parse_language("de"), Some("de".to_string()));
        assert_eq!(parse_language(""), Some("".to_string()));
    }
}
