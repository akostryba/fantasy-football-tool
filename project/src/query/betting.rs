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

pub fn get_implied_points <'a> (over_under : &String, spread : &String) -> f32 {
      let first = &spread.as_str()[1..2];
      match first{
            "+" => over_under.as_str()[1..over_under.len()-1].parse::<f32>().unwrap()/2.0 - spread.as_str()[2..spread.len()-1].parse::<f32>().unwrap()/2.0, 
            "-" => over_under.as_str()[1..over_under.len()-1].parse::<f32>().unwrap()/2.0 + spread.as_str()[2..spread.len()-1].parse::<f32>().unwrap()/2.0, 
            _ => -1.0
      } 
}