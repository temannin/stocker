// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::UPCResponse;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: UPCResponse = serde_json::from_str(&json).unwrap();
// }

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct UpcResponse {
    pub(crate) added_time: Option<String>,
    pub(crate) modified_time: Option<String>,
    pub(crate) title: Option<String>,
    pub(crate) alias: Option<serde_json::Value>,
    pub(crate) description: Option<String>,
    pub(crate) brand: Option<String>,
    pub(crate) manufacturer: Option<serde_json::Value>,
    pub(crate) msrp: Option<serde_json::Value>,
    #[serde(rename = "ASIN")]
    pub(crate) asin: Option<serde_json::Value>,
    pub(crate) category: Option<String>,
    pub(crate) categories: Option<String>,
    pub(crate) stores: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) barcode: Option<String>,
    pub(crate) success: Option<bool>,
    pub(crate) timestamp: Option<i64>,
    pub(crate) images: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) metadata: Option<Metadata>,
    pub(crate) metanutrition: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    pub(crate) quantity: Option<String>,
    pub(crate) countries: Option<String>,
    pub(crate) ingredients: Option<String>,
}
