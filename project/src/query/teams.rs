use reqwest::Client;
use crate::json::my_structs::{TeamsRoot, TeamsBody};

use crate::query::{REQUEST_HOST, TEAMS, API_KEY};

pub async fn request_teams (client : &Client) -> Result<TeamsRoot, Box<dyn std::error::Error>>{
    let teams_request : String = format!("{}{}", *REQUEST_HOST, *TEAMS);
    let teams_response = client.get(teams_request)
          .header("X-RapidAPI-Key",
          *API_KEY)
          .send()
          .await?;
    let teams = teams_response.json::<TeamsRoot>().await?;
    Ok(teams)
  }

pub fn get_team<'a> (teams : &'a Vec<TeamsBody>, target_team_id : &'a str) -> Option<&'a TeamsBody>{
    return teams.iter().find(|team | team.team_id == target_team_id);
}