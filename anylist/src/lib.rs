use isahc::prelude::*;
use serde::Serialize;
use std::error::Error;

// Define the struct for your JSON body
#[derive(Serialize)]
pub struct Item {
    pub name: String,
    pub notes: String,
    pub list: String,
}

pub fn add_item(item: Item) -> Result<(), Box<dyn Error>> {
    // Serialize the struct to JSON
    let json_body = serde_json::to_string(&item)?;

    // Make the POST request with JSON body
    let response = isahc::Request::post("http://homeassistant:8080/add")
        .header("Content-Type", "application/json")
        .body(json_body)?
        .send()?;

    // Check if the response status is successful
    if !response.status().is_success() {
        return Err(format!("Unable to add item to list: HTTP {}", response.status()).into());
    }

    Ok(())
}
