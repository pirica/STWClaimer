use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct LoginModel {
    pub access_token: String,
    pub expires_in: i64,
    pub expires_at: String,
    pub token_type: String,
    pub refresh_token: String,
    pub refresh_expires: i64,
    pub refresh_expires_at: String,
    pub account_id: String,
    pub client_id: String,
    pub internal_client: bool,
    pub client_service: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub app: String,
    pub in_app_id: String,
    pub device_id: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeModel {
    pub expires_in_seconds: i32,
    pub code: String,
    pub creating_client_id: String
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all= "camelCase")]
pub struct Device {
    pub device_id: String,
    pub account_id: String,
    pub secret: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub notifications: Vec<ProfileNotification>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProfileNotification {
    #[serde(rename = "type")]
    pub notification_type: String,
    pub primary: bool,
    pub days_logged_in: i32,
    pub items: Vec<NotificationItem>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NotificationItem {
    pub item_type: String,
    pub item_guid: String,
    pub item_profile: String,
    pub quantity: i32
}