use crate::exports::edgee::components::data_collection::{
    Data, Dict, EdgeeRequest, Event, HttpMethod,
};
use crate::gosquared::GoSquaredIdentifyPayload;
use crate::helpers::insert_if_nonempty;
use exports::edgee::components::data_collection::Guest;
use gosquared::{GoSquaredPageviewPayload, GoSquaredTrackPayload};
use serde::Serialize;
use std::collections::HashMap;
mod gosquared;
mod helpers;

wit_bindgen::generate!({world: "data-collection", path: ".edgee/wit", generate_all});
export!(Component);

struct Component;

impl Guest for Component {
    fn page(event: Event, settings_dict: Dict) -> Result<EdgeeRequest, String> {
        let payload = GoSquaredPageviewPayload::new(&event);
        common(
            "https://api.gosquared.com/tracking/v1/pageview",
            &payload,
            settings_dict,
        )
    }

    fn track(event: Event, settings_dict: Dict) -> Result<EdgeeRequest, String> {
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
        common(
            "https://api.gosquared.com/tracking/v1/event",
            &payload,
            settings_dict,
        )
    }

    fn user(event: Event, settings_dict: Dict) -> Result<EdgeeRequest, String> {
        let payload = GoSquaredIdentifyPayload::new(&event);

        common(
            "https://api.gosquared.com/tracking/v1/identify",
            &payload,
            settings_dict,
        )
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

        Ok(Self {
            api_key,
            site_token,
        })
    }
}

pub fn common<T: Serialize>(
    endpoint: &str,
    payload: &T,
    settings_dict: Dict,
) -> Result<EdgeeRequest, String> {
    let settings = Settings::new(settings_dict).map_err(|e| e.to_string())?;
    let json_payload = serde_json::to_string(payload).map_err(|e| e.to_string())?;

    let url = format!(
        "{}?api_key={}&site_token={}",
        endpoint, settings.api_key, settings.site_token
    );

    Ok(EdgeeRequest {
        method: HttpMethod::Post,
        url,
        headers: vec![("Content-Type".into(), "application/json".into())],
        body: json_payload,
        forward_client_headers: true,
    })
}
