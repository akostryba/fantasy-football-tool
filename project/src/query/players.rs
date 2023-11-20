use reqwest::Client;
use crate::json::my_structs::{PlayerBody, PlayerRoot};

use crate::query::{REQUEST_HOST, PLAYERS, API_KEY};

pub async fn request_players (client : &Client) -> Result<PlayerRoot, Box<dyn std::error::Error>>{
    let players_request : String = format!("{}{}", *REQUEST_HOST, *PLAYERS);
    let players_response = client.get(players_request)
          .header("X-RapidAPI-Key",
          *API_KEY)
          .send()
          .await?;
    let players = players_response.json::<PlayerRoot>().await?;
    Ok(players)
  }

pub fn get_player<'a>(player_vec : &'a Vec<PlayerBody>, target_name : &'a str) -> Option<&'a PlayerBody> {
  return player_vec.iter().find(|player| player.long_name == target_name);
}

