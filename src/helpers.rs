use crate::exports::edgee::components::data_collection::Event;
use crate::gosquared::{CampaignInfo, ScreenInfo};
use chrono::offset::Offset;
use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::Tz;
use std::collections::HashMap;

pub fn timezone_offset_from_string(tz_string: &str) -> Option<i32> {
    if let Ok(tz) = tz_string.parse::<Tz>() {
        let now = Utc::now();
        let offset = tz
            .offset_from_utc_datetime(&now.naive_utc())
            .fix()
            .local_minus_utc();
        return Some(offset / 60); // Offset in minutes
    }

    if let Some(offset_str) = tz_string.strip_prefix("UTC") {
        if let Ok(offset_hours) = offset_str.parse::<i32>() {
            return Some(offset_hours * 60);
        }
        if offset_str.starts_with('+') || offset_str.starts_with('-') {
            return offset_str.parse::<i32>().ok().map(|h| h * 60);
        }
    }

    None
}

pub fn format_last_seen_as_iso(last_seen: i64) -> Option<String> {
    if last_seen <= 0 {
        return None;
    }

    DateTime::from_timestamp(last_seen, 0).map(|dt| dt.to_rfc3339())
}

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
