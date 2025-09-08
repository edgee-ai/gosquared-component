use crate::exports::edgee::components::data_collection::{Data, Dict, EdgeeRequest, Event, HttpMethod};
use exports::edgee::components::data_collection::Guest;
use std::collections::HashMap;
use serde_json::json;
mod helpers;
use crate::helpers::insert_if_nonempty;

wit_bindgen::generate!({world: "data-collection", path: ".edgee/wit", generate_all});
export!(Component);

struct Component;

impl Guest for Component {
    fn page(event: Event, settings_dict: Dict) -> Result<EdgeeRequest, String> {
        let settings = Settings::new(settings_dict).map_err(|e| e.to_string())?;
        let mut properties = HashMap::new();

        if let Data::Page(data) = &event.data {
            for (k, v) in &data.properties {
                insert_if_nonempty(&mut properties, k, v);
            }
        }

        let payload = json!({
            "site_token": settings.site_token,
            "event": "pageview",
            "person_id": event.context.user.user_id,
            "timestamp": event.timestamp,
            "properties": properties
        });

        Ok(EdgeeRequest {
            method: HttpMethod::Post,
            url: "https://data.gosquared.com/event".to_string(),
            headers: vec![("Content-Type".to_string(), "application/json".to_string())],
            body: payload.to_string(),
            forward_client_headers: true,
        })
    }

    fn track(event: Event, settings_dict: Dict) -> Result<EdgeeRequest, String> {
        let settings = Settings::new(settings_dict).map_err(|e| e.to_string())?;
        let mut properties = HashMap::new();
        let mut event_name = "track".to_string();
        let language = event
            .context
            .client
            .locale
            .split('-')
            .next()
            .unwrap_or("")
            .to_string();


        if let Data::Track(data) = &event.data {
            event_name = data.name.clone();

            for (k, v) in &data.properties {
                insert_if_nonempty(&mut properties, k, v);
            }
        }

        let mut payload = json!({
            "site_token": settings.site_token,
            "event": {
                "name": event_name,
                "data": properties
            },
            "timestamp": event.timestamp,
            "person_id": event.context.user.user_id,
            "visitor_id": event.context.user.anonymous_id,
            "page": {
                "url": event.context.page.url,
                "title": event.context.page.title,
            },
            "referrer": event.context.page.referrer,
            "ip": event.context.client.ip,
            "user_agent": event.context.client.user_agent,
            "screen": {
                "height": event.context.client.screen_height,
                "width": event.context.client.screen_width,
                "pixel_ratio": event.context.client.screen_density,
            },
            "campain": {
                "name": event.context.campaign.name,
                "source": event.context.campaign.source,
                "medium": event.context.campaign.medium,
                "content": event.context.campaign.content,
                "term": event.context.campaign.term,
            },
        });

        if let Some(map) = payload.as_object_mut() {
            map.insert("language".to_string(), json!(language));
        }

        Ok(EdgeeRequest {
            method: HttpMethod::Post,
            url: "https://data.gosquared.com/event".to_string(),
            headers: vec![("Content-Type".to_string(), "application/json".to_string())],
            body: payload.to_string(),
            forward_client_headers: true,
        })
    }

    fn user(event: Event, settings_dict: Dict) -> Result<EdgeeRequest, String> {
        let settings = Settings::new(settings_dict).map_err(|e| e.to_string())?;
        let mut properties = HashMap::new();

        for (k, v) in &event.context.user.properties {
            insert_if_nonempty(&mut properties, k, v);
        }

        let payload = json!({
            "site_token": settings.site_token,
            "person_id": event.context.user.user_id,
            "properties": properties
        });

        Ok(EdgeeRequest {
            method: HttpMethod::Post,
            url: "https://data.gosquared.com/identify".to_string(),
            headers: vec![("Content-Type".to_string(), "application/json".to_string())],
            body: payload.to_string(),
            forward_client_headers: true,
        })
    }
}

pub struct Settings {
    pub api_key: String,
    pub site_token: String,
}

impl Settings {
    pub fn new(settings_dict: Dict) -> anyhow::Result<Self> {
        let settings_map: HashMap<String, String> = settings_dict
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

        let api_key = settings_map
            .get("api_key")
            .filter(|t| !t.trim().is_empty())
            .ok_or_else(|| anyhow::anyhow!("Missing or empty 'api_key' setting"))?
            .to_string();

        let site_token = settings_map
            .get("site_token")
            .filter(|s| !s.is_empty())
            .ok_or_else(|| anyhow::anyhow!("Missing 'site_token'"))?
            .to_string();

        Ok(Self { api_key, site_token })
    }
}