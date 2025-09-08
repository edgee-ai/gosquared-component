use crate::exports::edgee::components::data_collection::{Data, Dict, EdgeeRequest, Event, HttpMethod};
use crate::helpers::insert_if_nonempty;
use gosquared::{GoSquaredTrackPayload, GoSquaredPageviewPayload};
use exports::edgee::components::data_collection::Guest;
use std::collections::HashMap;
use serde_json::json;
mod helpers;
mod gosquared;


wit_bindgen::generate!({world: "data-collection", path: ".edgee/wit", generate_all});
export!(Component);

struct Component;

impl Guest for Component {
    fn page(event: Event, settings_dict: Dict) -> Result<EdgeeRequest, String> {
        let settings = Settings::new(settings_dict).map_err(|e| e.to_string())?;
        let payload = GoSquaredPageviewPayload::new(&event);
        let json_payload = serde_json::to_string(&payload).map_err(|e| e.to_string())?;

        Ok(EdgeeRequest {
            method: HttpMethod::Post,
            url: "https://data.gosquared.com/event".to_string(),
            headers: vec![("Content-Type".to_string(), "application/json".to_string())],
            body: json_payload,
            forward_client_headers: true,
        })
    }

    fn track(event: Event, settings_dict: Dict) -> Result<EdgeeRequest, String> {
        let settings = Settings::new(settings_dict).map_err(|e| e.to_string())?;

        let mut properties = HashMap::new();
        let mut event_name = "track".to_string();

        if let Data::Track(data) = &event.data {
            event_name = data.name.clone();
            for (k, v) in &data.properties {
                let k = k.replace(" ", "_");
                insert_if_nonempty(&mut properties, &k, v);
            }
        }

        let payload = GoSquaredTrackPayload::new(&event, event_name, properties);
        let json_payload = serde_json::to_string(&payload).map_err(|e| e.to_string())?;

        Ok(EdgeeRequest {
            method: HttpMethod::Post,
            url: "https://data.gosquared.com/event".to_string(),
            headers: vec![("Content-Type".to_string(), "application/json".to_string())],
            body: json_payload,
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