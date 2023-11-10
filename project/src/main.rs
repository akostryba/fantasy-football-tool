use reqwest::Client;
use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket_dyn_templates::{Template,context};
use std::path::{Path,PathBuf};

pub mod json;
pub mod query;
pub use query::{get_standard_projection, get_ppr_projection, get_halfppr_projection, request_fantasy, 
                request_players, get_player,
                request_schedule, get_game, get_opp_id,
                request_teams, get_team};


pub use json::my_structs;
use serde::Serialize;
use serde_json::Value;

fn create_client () -> Client{
  return reqwest::Client::new();
}

#[macro_use] extern crate rocket;


#[get ("/")]
fn index () -> Template {
  Template::render ("test2", context! { 
    default_player1 : String::from("Stefon Diggs"),
    default_player2 : String::from("Michael Thomas"),
  })
}

#[derive (Debug,FromForm)]
struct Input<'r> {
  player1 : &'r str, 
  player2 : &'r str,
  week : &'r str,
  scoring : &'r str,
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
      return Template::render ("test2", context! { 
        default_player1 : String::from("Player not found"),
        default_player2 : String::from("Michael Thomas"),
      });
      }
    };
  println!("Player 1 Found: {}", player1.player_id);

  let player2 = match get_player(&players.body, s.player2){
    Some(p) => p,
    None => {
      return Template::render ("test2", context! { 
        default_player1 : String::from("Stefon Diggs"),
        default_player2 : String::from("Player not found"),
      });
      }
    };
  
    println!("Player 2 Found: {}", player2.player_id);

  let fantasy_data = match request_fantasy(&client, s.week).await{
    Ok(d) => d,
    _ =>{
      return Template::render ("test2", context! { });
    }
  };

  let proj1 = match s.scoring{
    "Standard" => get_standard_projection(&player1.player_id, &fantasy_data),
    "PPR" => get_ppr_projection(&player1.player_id, &fantasy_data),
    "Half PPR" => get_halfppr_projection(&player1.player_id, &fantasy_data),
    _ => &Value::Null
  };

  let proj2 = match s.scoring{
    "Standard" => get_standard_projection(&player2.player_id, &fantasy_data),
    "PPR" => get_ppr_projection(&player2.player_id, &fantasy_data),
    "Half PPR" => get_halfppr_projection(&player2.player_id, &fantasy_data),
    _ => &Value::Null
  };

  let schedule1 = match request_schedule(&client, &player1.team_id).await {
    Ok(s) => s,
    _ => return Template::render ("test2", context! { }),
  };

  let schedule2 = match request_schedule(&client, &player2.team_id).await {
    Ok(s) => s,
    _ => return Template::render ("test2", context! { }),
  };

  let game1 = match get_game(&s.week, &schedule1.body.schedule){
    Some(g) => g,
    None => return Template::render ("test2", context! { }),
  };

  let game2 = match get_game(&s.week, &schedule2.body.schedule){
    Some(g) => g,
    None => return Template::render ("test2", context! { }),
  };

  let opp1 = get_opp_id(&player1.team_id, game1);
  let opp2 = get_opp_id(&player2.team_id, game2);

  let teams = match request_teams(&client).await{
    Ok(t) => t,
    _ => return Template::render ("test2", context! { }),
  };

  let opp1_team = match get_team(&teams.body, opp1){
    Some(t) => t,
    None => return Template::render ("test2", context! { }),
  };

  let opp2_team = match get_team(&teams.body, opp2){
    Some(t) => t,
    None => return Template::render ("test2", context! { }),
  };

  println!("Score1 Found: {}, Score2 Found: {}", proj1, proj2);
  //format! ("Player 1 ID: {}. Projected PPR Points: {}\nPlayer 2 ID: {}. Projected PPR Points: {}", player1.player_id, ppr_proj1, player2.player_id, ppr_proj2)
  Template::render("test2", context! {
    default_player1 : &player1.long_name,
    default_player2 :&player2.long_name,
    player1_name: &player1.long_name,
    player2_name: &player2.long_name,
    player1_proj : proj1,
    player2_proj : proj2,
    week : s.week,
    opponent_1 : &opp1_team.team_name,
    opponent_2 : &opp2_team.team_name,
  })
}


#[launch]
fn rocket() -> _ {
  rocket::build()
    .mount ("/", routes![index, analyze])
    .attach (Template::fairing ())
}