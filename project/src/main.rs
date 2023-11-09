use reqwest::Client;
use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket_dyn_templates::{Template,context};
use std::path::{Path,PathBuf};

pub mod json;

pub use json::my_structs;
pub use json::my_structs::{PlayerBody, PlayerRoot};
use serde::Serialize;
use serde_json::Value;
use lazy_static::lazy_static;

//static strings
lazy_static! {
    static ref REQUEST_HOST : &'static str = "https://tank01-nfl-live-in-game-real-time-statistics-nfl.p.rapidapi.com";
    static ref BETTING : &'static str = "/getNFLBettingOdds?";
    static ref SCHEDULE : &'static str = "/getNFLGamesForWeek?";
    static ref FANTASY : &'static str = "/getNFLProjections?";
    static ref PLAYERS : &'static str = "/getNFLPlayerList";
    static ref API_KEY : &'static str = "7edefd3921mshf363ed62e8f497bp1ddf5bjsnd14740483cb3";
}

fn get_player<'a>(player_vec : &'a Vec<PlayerBody>, target_name : &'a str) -> Option<&'a PlayerBody> {
  return player_vec.iter().find(|player| player.long_name == target_name);
}

fn get_player_id (player : &PlayerBody) -> &String {
  return &player.player_id;
}

fn get_ppr_projection<'a> (player_id: &'a String, json : &'a Value) -> &'a Value {
  return &json["body"]["playerProjections"][player_id]["fantasyPointsDefault"]["PPR"];
}

fn create_client () -> Client{
  return reqwest::Client::new();
}

async fn request_players (client : &Client) -> Result<(PlayerRoot), Box<dyn std::error::Error>>{
  let players_request : String = format!("{}{}", *REQUEST_HOST, *PLAYERS);
  let players_response = client.get(players_request)
        .header("X-RapidAPI-Key",
        *API_KEY)
        .send()
        .await?;
  let players = players_response.json::<my_structs::PlayerRoot>().await?;
  Ok(players)
}
 
async fn request_fantasy (client: &Client, week: &str) -> Result<(Value), Box<dyn std::error::Error>> {
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

#[macro_use] extern crate rocket;


#[get ("/")]
fn index () -> Template {
  Template::render ("test2", context! { })
}

#[derive (Debug,FromForm)]
struct Input<'r> {
  player1 : &'r str, 
  player2 : &'r str,
  week : &'r str,
}

#[post ("/analyze", data = "<s>")]
async fn analyze (s : Form<Input<'_>>) -> Template {
  println! ("received submission of {s:?}");
  let client = create_client();
  let players = match request_players(&client).await{
    Ok(p) => p,
    _ =>{
      return Template::render ("test2", context! { });
    }
  };
  let player1 = match get_player(&players.body, s.player1){
    Some(p) => p,
        None => {
            return Template::render ("test2", context! { });
        }
    };

    let player2 = match get_player(&players.body, s.player2){
      Some(p) => p,
          None => {
              return Template::render ("test2", context! { });
          }
      };

  let fantasy_data = match request_fantasy(&client, s.week).await{
    Ok(d) => d,
    _ =>{
      return Template::render ("test2", context! { });
    }
  };
  let ppr_proj1 = get_ppr_projection(&player1.player_id, &fantasy_data);
  let ppr_proj2 = get_ppr_projection(&player2.player_id, &fantasy_data);
  //format! ("Player 1 ID: {}. Projected PPR Points: {}\nPlayer 2 ID: {}. Projected PPR Points: {}", player1.player_id, ppr_proj1, player2.player_id, ppr_proj2)
  Template::render("test2", context! {
    player1_name: &player1.long_name,
    player2_name: &player2.long_name,
    player1_proj : ppr_proj1,
    player2_proj : ppr_proj2,
    week : s.week
  })
}


#[launch]
fn rocket() -> _ {
  rocket::build()
    .mount ("/", routes![index, analyze])
    .attach (Template::fairing ())
}