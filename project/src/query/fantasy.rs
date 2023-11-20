use serde_json::Value;
use reqwest::Client;

use crate::query::{REQUEST_HOST, FANTASY, API_KEY};

pub fn get_standard_projection<'a> (player_id: &'a String, json : &'a Value) -> &'a Value {
    return &json["body"]["playerProjections"][player_id]["fantasyPointsDefault"]["standard"];
  }
  
pub fn get_ppr_projection<'a> (player_id: &'a String, json : &'a Value) -> &'a Value {
    return &json["body"]["playerProjections"][player_id]["fantasyPointsDefault"]["PPR"];
  }
  
pub fn get_halfppr_projection<'a> (player_id: &'a String, json : &'a Value) -> &'a Value {
    return &json["body"]["playerProjections"][player_id]["fantasyPointsDefault"]["halfPPR"];
  }

pub async fn request_fantasy (client: &Client, week: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let fantasy_param = vec![("week", week)];
    let fantasy_request : String = format!("{}{}", *REQUEST_HOST, *FANTASY);
    let fantasy_response = client.get(fantasy_request)
          .header("X-RapidAPI-Key",
          *API_KEY)
          .query(&fantasy_param)
          .send()
          .await?;
    let data = fantasy_response.json::<serde_json::Value>().await?;
    Ok(data)
  }