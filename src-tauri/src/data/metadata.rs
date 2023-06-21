use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Map;
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LCUSchema {
    pub components: Components,
    pub info: Info,
    pub openapi: String,
    pub paths: HashMap<String, HashMap<String, Operation>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub title: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Components {
    pub schemas: HashMap<String, Schema>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    pub description: Option<String>,
    pub properties: Option<Map<String, Value>>,
    #[serde(rename = "type")]
    pub _type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    pub description: Option<String>,
    pub operation_id: String,
    pub parameters: Vec<Parameter>,
    pub response: Option<Map<String, Value>>,
    pub summary: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    #[serde(rename = "in")]
    pub _in: String,
    pub name: String,
    pub required: bool,
    #[serde(rename = "type")]
    pub _type: String,
}
