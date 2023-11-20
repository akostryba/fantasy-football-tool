use serde_json::Value;
use reqwest::Client;

use crate::query::{REQUEST_HOST, BETTING, API_KEY};

pub async fn request_betting (client: &Client, game_id: &str, game_date: &str) -> Result<(Value), Box<dyn std::error::Error>> {
    let betting_param = vec![("gameID", game_id), ("gameDate", game_date)];
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

pub fn get_over_under<'a> (json : &'a Value, game_id: &str) -> &'a Value {
      return &json["body"][game_id]["fanduel"]["totalUnder"];
}

pub fn get_spread<'a> (json : &'a Value, game_id: &str, location : &str) -> &'a Value {
      match location {
            "away" => return &json["body"][game_id]["fanduel"]["awayTeamSpread"],
            "home" => return &json["body"][game_id]["fanduel"]["homeTeamSpread"],
            _ => return &Value::Null,
      };
}