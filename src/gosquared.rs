use crate::exports::edgee::components::data_collection::Event;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Default)]
pub struct GoSquaredTrackPayload {
    event: GoSquaredEvent,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    person_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    visitor_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<PageInfo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    referrer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    ip: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    character_set: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    user_agent: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    screen: Option<ScreenInfo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    returning: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    campaign: Option<CampaignInfo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<LocationInfo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    last_pageview: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total: Option<TotalInfo>,
}

#[derive(Serialize, Default)]
pub struct GoSquaredEvent {
    name: String,
    data: HashMap<String, String>,
}

#[derive(Serialize, Default)]
pub struct PageInfo {
    url: String,
    title: String,
    previous: i32,
    index: i32,
}

#[derive(Serialize, Default)]
pub struct ScreenInfo {
    height: i32,
    width: i32,
    pixel_ratio: Option<f32>,
    depth: Option<f64>,
}

#[derive(Serialize, Default)]
pub struct CampaignInfo {
    name: String,
    source: String,
    medium: String,
    content: String,
    term: String,
}

#[derive(Serialize, Default)]
pub struct LocationInfo {
    timezone_offset: i32,
}

#[derive(Serialize, Default)]
pub struct TotalInfo {
    visits: i32,
    pageviews: i32,
}

impl GoSquaredTrackPayload {
    pub fn new(event: &Event, event_name: String, properties: HashMap<String, String>) -> Self {
        GoSquaredTrackPayload {
            event: GoSquaredEvent {
                name: event_name,
                data: properties,
            },
            timestamp: Some(event.timestamp.to_string()),
            person_id: Some(event.context.user.user_id.clone()),
            visitor_id: Some(event.context.user.anonymous_id.clone()),
            referrer: Some(event.context.page.referrer.clone()),
            ip: Some(event.context.client.ip.clone()),
            user_agent: Some(event.context.client.user_agent.clone()),
            language: Some(event.context.client.locale.split('-').next().unwrap_or("").to_string()),
            page: Some(PageInfo {
                url: event.context.page.url.clone(),
                title: event.context.page.title.clone(),
                ..Default::default()
            }),
            screen: Some(ScreenInfo {
                height: event.context.client.screen_height,
                width: event.context.client.screen_width,
                pixel_ratio: Some(event.context.client.screen_density),
                depth: None,
            }),
            campaign: Some(CampaignInfo {
                name: event.context.campaign.name.clone(),
                source: event.context.campaign.source.clone(),
                medium: event.context.campaign.medium.clone(),
                content: event.context.campaign.content.clone(),
                term: event.context.campaign.term.clone(),
            }),
            ..Default::default()
        }
    }
}
