use serde_json::Value;
use reqwest::Client;

use crate::query::{REQUEST_HOST, FANTASY, API_KEY};

fn get_standard_projection<'a> (player_id: &'a String, json : &'a Value) -> &'a Value {
    return &json["body"]["playerProjections"][player_id]["fantasyPointsDefault"]["standard"];
  }
  
fn get_ppr_projection<'a> (player_id: &'a String, json : &'a Value) -> &'a Value {
    return &json["body"]["playerProjections"][player_id]["fantasyPointsDefault"]["PPR"];
  }
  
fn get_halfppr_projection<'a> (player_id: &'a String, json : &'a Value) -> &'a Value {
    return &json["body"]["playerProjections"][player_id]["fantasyPointsDefault"]["halfPPR"];
  }

pub fn get_projection (player_id : &String, json : &Value, format : &str) -> String {
  let mut result = 
    match format{
      "Standard" => get_standard_projection(player_id, json).to_string(),
      "PPR" => get_ppr_projection(player_id, json).to_string(),
      "Half PPR" => get_halfppr_projection(player_id, json).to_string(),
      _ => String::from("Error"),
    };
  if result != String::from("null"){
    result = result.as_str()[1..result.len()-1].to_string();
  };
  return result;
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