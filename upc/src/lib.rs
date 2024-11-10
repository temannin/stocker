mod response;

use anylist::Item;
use isahc::prelude::*;
use response::UpcResponse;
use std::error::Error;

pub fn get_item(code: &str, api_key: &str) -> Result<Item, Box<dyn Error>> {
    // Send the request to the API
    let mut response = isahc::Request::get(format!(
        "https://api.upcdatabase.org/product/{code}?apikey={api_key}"
    ))
    .header("Content-Type", "application/json")
    .body(())?
    .send()?;

    let body = response.text()?;

    // Strip out the warnings and other HTML parts, leaving only the JSON data
    let start_index = body.find('{').unwrap_or(0); // Find the first '{' to locate the JSON part
    let json_body = &body[start_index..]; // Extract JSON from the first '{'

    // Attempt to parse the cleaned-up JSON string
    let upc_result: Result<UpcResponse, _> = serde_json::from_str(json_body);

    let upc: UpcResponse = match upc_result {
        Ok(parsed_upc) => parsed_upc,
        Err(_) => {
            // If JSON parsing fails, return an error
            return Err("Failed to parse the API response as JSON".into());
        }
    };

    // Create an Item struct with safe default values for potentially null fields
    let item: Item = Item {
        name: upc.title.unwrap_or_default(),
        notes: upc.description.unwrap_or_default(),
        list: "Kroger List".to_string(),
    };

    Ok(item)
}
