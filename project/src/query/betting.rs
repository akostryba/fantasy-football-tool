use serde_json::Value;
use reqwest::Client;

use crate::query::{REQUEST_HOST, BETTING, API_KEY};

pub async fn request_betting (client: &Client, gameID: &str) -> Result<(Value), Box<dyn std::error::Error>> {
    //let betting_param = vec![("gameID", gameID)];
    let betting_request : String = format!("{}{}", *REQUEST_HOST, *BETTING);
    let betting_response = client.get(betting_request)
          .header("X-RapidAPI-Key",
          *API_KEY)
          .query(&betting_param)
          .send()
          .await?;
    let data = betting_response.json::<serde_json::Value>().await?;
    Ok(data)
  }