use reqwest::Client;

use crate::query::{REQUEST_HOST, SCHEDULE, API_KEY};
use crate::my_structs::{ScheduleRoot, Game};

pub async fn request_schedule (client : &Client, team_id : &str) -> Result<ScheduleRoot, Box<dyn std::error::Error>>{
    let schedule_param = vec![("teamID", team_id)];
    let schedule_request : String = format!("{}{}", *REQUEST_HOST, *SCHEDULE);
    let schedule_response = client.get(schedule_request)
          .header("X-RapidAPI-Key",
          *API_KEY)
          .query(&schedule_param)
          .send()
          .await?;
    let schedule = schedule_response.json::<ScheduleRoot>().await?;
    Ok(schedule)
}

pub fn get_game<'a> (week : &'a str, schedule: &'a Vec<Game>) -> Option<&'a Game>{
    let target_week = format!("Week {}", week);
    return schedule.iter().find(|game| game.game_week == target_week);
}

pub fn get_opp_id<'a> (team_id : &'a str, game : &'a Game) -> &'a String{
    if game.team_idaway == team_id {
        return &game.team_idhome;
    }
    else{
        return &game.team_idaway;
    }
}