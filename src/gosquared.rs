use crate::exports::edgee::components::data_collection::Event;
use serde::Serialize;
use std::collections::HashMap;
use crate::helpers::{parse_language, screen_from_event, campaign_from_event};


#[derive(Serialize, Default)]
pub struct GoSquaredIdentifyPayload {
    pub person_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub visitor_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<GoSquaredUserProperties>,
}

#[derive(Serialize, Default)]
pub struct GoSquaredPageviewPayload {
    visitor_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<String>,
    page: PageInfo,
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
    returning: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    screen: Option<ScreenInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    campaign: Option<CampaignInfo>,
}

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
pub struct GoSquaredUserProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<GoSquaredCompany>,

    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub custom: HashMap<String, String>,
}

#[derive(Serialize, Default, PartialEq)]
pub struct GoSquaredCompany {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub industry: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
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
    pub height: i32,
    pub width: i32,
    pub pixel_ratio: Option<f32>,
    pub depth: Option<f64>,
}

#[derive(Serialize, Default)]
pub struct CampaignInfo {
    pub name: String,
    pub source: String,
    pub medium: String,
    pub content: String,
    pub term: String,
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
            language: parse_language(&event.context.client.locale),
            page: Some(PageInfo {
                url: event.context.page.url.clone(),
                title: event.context.page.title.clone(),
                ..Default::default()
            }),
            screen: Some(screen_from_event(event)),
            campaign: Some(campaign_from_event(event)),
            ..Default::default()
        }
    }
}

impl GoSquaredPageviewPayload {
    pub fn new(event: &Event) -> Self {
        GoSquaredPageviewPayload {
            visitor_id: event.context.user.anonymous_id.clone(),
            timestamp: Some(event.timestamp.to_string()),
            page: PageInfo {
                url: event.context.page.url.clone(),
                title: event.context.page.title.clone(),
                ..Default::default()
            },
            referrer: Some(event.context.page.referrer.clone()),
            ip: Some(event.context.client.ip.clone()),
            language: parse_language(&event.context.client.locale),
            user_agent: Some(event.context.client.user_agent.clone()),
            returning: None,
            character_set: None,
            screen: Some(screen_from_event(event)),
            campaign: Some(campaign_from_event(event)),
        }
    }
}

impl GoSquaredIdentifyPayload {
    pub fn new(event: &Event) -> Self {
        let mut custom = HashMap::new();
        let mut company = GoSquaredCompany::default();
        let mut props = GoSquaredUserProperties::default();

        for (key, value) in &event.context.user.properties {
            match key.as_str() {
                "email" => props.email = Some(value.clone()),
                "status" => props.status = Some(value.clone()),
                "name" => props.name = Some(value.clone()),
                "first_name" => props.first_name = Some(value.clone()),
                "last_name" => props.last_name = Some(value.clone()),
                "username" => props.username = Some(value.clone()),
                "avatar" => props.avatar = Some(value.clone()),
                "description" => props.description = Some(value.clone()),
                "phone" => props.phone = Some(value.clone()),
                "created_at" => props.created_at = Some(value.clone()),

                "company.name" => company.name = Some(value.clone()),
                "company.industry" => company.industry = Some(value.clone()),
                "company.position" => company.position = Some(value.clone()),
                "company.size" => {
                    if let Ok(parsed) = value.parse::<u32>() {
                        company.size = Some(parsed);
                    }
                }

                _ => {
                    custom.insert(key.clone(), value.clone());
                }
            }
        }

        if company != GoSquaredCompany::default() {
            props.company = Some(company);
        }

        props.custom = custom;

        GoSquaredIdentifyPayload {
            person_id: event.context.user.user_id.clone(),
            visitor_id: Some(event.context.user.anonymous_id.clone()),
            properties: Some(props),
        }
    }
}