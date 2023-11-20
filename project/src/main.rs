use reqwest::Client;
use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket_dyn_templates::{Template,context};
use std::path::{Path,PathBuf};
use std::error::Error;
use std::fmt;
use std::fmt::{Formatter, Display};

pub mod json;
pub mod query;
pub use query::{get_standard_projection, get_ppr_projection, get_halfppr_projection, request_fantasy, 
                request_players, get_player,
                request_schedule, get_game, get_opp_id,
                request_teams, get_team};


pub use json::my_structs;
use serde::Serialize;
use serde_json::{Value, json};

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

#[derive(Debug)]
struct PlayerNotFoundError {
  player_name: String,
}

impl Display for PlayerNotFoundError {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write! (f, "Player not found: {}", self.player_name)
  }
}

impl Error for PlayerNotFoundError {
  fn source (&self) -> Option<&(dyn Error + 'static)>{
    None
  }
}

#[derive(Debug)]
struct RequestError{
  request_name: String,
}

impl Display for RequestError {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write! (f, "Error with: {}", self.request_name)
  }
}

impl Error for RequestError {
  fn source (&self) -> Option<&(dyn Error + 'static)>{
    None
  }
}

#[derive(Debug)]
struct QueryError{
  query: String,
}

impl Display for QueryError {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write! (f, "Couldn't find: {}", self.query)
  }
}

impl Error for QueryError {
  fn source (&self) -> Option<&(dyn Error + 'static)>{
    None
  }
}

#[derive(Debug)]
enum MyError {
  PlayerNotFoundError(String),
  RequestError(String),
  QueryError(String),
}

impl Display for MyError {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      MyError::PlayerNotFoundError(msg) => write! (f, "Player not found: {}", msg),
      MyError::RequestError(msg) => write! (f, "Request Error: {}", msg),
      MyError::QueryError(msg) => write! (f, "Query Error: {}", msg),
    }
  }
}

impl Error for MyError{
  fn source (&self) -> Option<&(dyn Error + 'static)>{
    None
  }
}

fn detect_query_error(data_vec : Vec<(String, &String)>) {
  for (key, value) in data_vec{
    if *value==String::from("null"){
      let query_error = MyError::QueryError(format!("{}", key));
      println!("{}", query_error);
    }
  }
}

fn handle_error(err: &MyError, s : &Form<Input<'_>>) -> Template{
  match err {
      MyError::PlayerNotFoundError(player_name) => {
          println!("{}", err);
          // Handle PlayerNotFoundError
          return Template::render("test2", context!{
            default_player1 : format!("Player not found: {}", player_name),
            default_player2 : format!("Player not found: {}", player_name),
          });
      }
      MyError::RequestError(request_name) => {
          println!("{}", err);
          // Handle RequestError
          return Template::render("test2", context!{
            default_player1 : String::from("Try again later"),
            default_player2 : String::from("Try again later"),
          });
      }
      MyError::QueryError(query) =>{
          println!("{}", err);
          // Handle QueryError
          return Template::render("test2", context!{
            default_player1 : s.player1,
            default_player2 : s.player2,
            player1_name: String::from(s.player1),
            player2_name: String::from(s.player2),
            player1_proj : String::from("Missing"),
            player2_proj : String::from("Missing"),
            week : s.week,
            opponent_1 : String::from("Data Missing"),
            opponent_2 : String::from("Data Missing"),
          });
      }
  }
}

#[post ("/analyze", data = "<s>")]
async fn analyze_route (s : Form<Input<'_>>) -> Template {
  match analyze(&s).await{
    Ok(template) => template,
    Err(err) => {
      return handle_error(&*err, &s);
      // match &err{
      // MyError::RequestError(s) => {
      //   println!("{}", err);
      //   return Template::render("test2", context! {
      //     default_player1 : String::from("Try again later"),
      //     default_player2 : String::from("Try again later"),
      //     })
      // }
      // QueryError => {
      //   println!("{}", err);
      //   return Template::render("test2", context! {
      //     default_player1 : String::from("Try again later"),
      //     default_player2 : String::from("Try again later"),
      //     })
      // }
      // }
    }
  }
}

async fn analyze (s : &Form<Input<'_>>) -> Result<Template, Box<MyError>> {
  println! ("received submission of {s:?}");
  let client = create_client();
  let players = match request_players(&client).await{
    Ok(p) => p,
    _ =>{
      //return Ok(Template::render ("test2", context! { }));
      return Err(Box::new(MyError::RequestError(String::from("Player Request"))));
    }
  };
  let player1 = match get_player(&players.body, s.player1){
    Some(p) => p,
    None => {
      return Err(Box::new(MyError::PlayerNotFoundError(s.player1.to_string())));
      }
    };
  println!("Player 1 Found: {}", player1.player_id);

  let player2 = match get_player(&players.body, s.player2){
    Some(p) => p,
    None => {
      return Err(Box::new(MyError::PlayerNotFoundError(s.player2.to_string())));
      }
    };
  
    println!("Player 2 Found: {}", player2.player_id);

  let fantasy_data = match request_fantasy(&client, s.week).await{
    Ok(d) => d,
    _ =>{
      return Err(Box::new(MyError::RequestError(String::from("Fantasy Projections"))));
    }
  };

  let mut proj1 = 
    match s.scoring{
    "Standard" => get_standard_projection(&player1.player_id, &fantasy_data).to_string(),
    "PPR" => get_ppr_projection(&player1.player_id, &fantasy_data).to_string(),
    "Half PPR" => get_halfppr_projection(&player1.player_id, &fantasy_data).to_string(),
    _ => String::from("Error"),
  };

  if proj1 != String::from("null"){
    proj1 = proj1.as_str()[1..proj1.len()-1].to_string();
  }

  let mut proj2 = 
    match s.scoring{
    "Standard" => get_standard_projection(&player2.player_id, &fantasy_data).to_string(),
    "PPR" => get_ppr_projection(&player2.player_id, &fantasy_data).to_string(),
    "Half PPR" => get_halfppr_projection(&player2.player_id, &fantasy_data).to_string(),
    _ => String::from("Error"),
  };

  if proj2 != String::from("null"){
    proj2 = proj2.as_str()[1..proj2.len()-1].to_string();
  }

  let schedule1 = match request_schedule(&client, &player1.team_id).await {
    Ok(s) => s,
    _ => return Err(Box::new(MyError::RequestError(String::from("Schedule 1 Request")))),
  };

  let schedule2 = match request_schedule(&client, &player2.team_id).await {
    Ok(s) => s,
    _ => return Err(Box::new(MyError::RequestError(String::from("Schedule 2 Request")))),
  };

  let game1 = match get_game(&s.week, &schedule1.body.schedule){
    Some(g) => g,
    None => return Err(Box::new(MyError::QueryError(String::from("Game 1")))),
  };

  let game2 = match get_game(&s.week, &schedule2.body.schedule){
    Some(g) => g,
    None => return Err(Box::new(MyError::QueryError(String::from("Game 2")))),
  };

  let opp1 = get_opp_id(&player1.team_id, game1);
  let opp2 = get_opp_id(&player2.team_id, game2);

  let teams = match request_teams(&client).await{
    Ok(t) => t,
    _ => return Err(Box::new(MyError::RequestError(String::from("Teams Request")))),
  };

  let opp1_team = match get_team(&teams.body, opp1){
    Some(t) => t,
    None => return Err(Box::new(MyError::QueryError(String::from("Opp Team 1")))),
  };

  let opp2_team = match get_team(&teams.body, opp2){
    Some(t) => t,
    None => return Err(Box::new(MyError::QueryError(String::from("Opp Team 2")))),
  };

  println!("Score1 Found: {}, Score2 Found: {}", proj1, proj2);
  
  let context_vars: Vec<(String, &String)> = vec![(String::from("Projection 1"), &proj1), (String::from("Projection 2"), &proj2)];
  detect_query_error(context_vars);
  
  //format! ("Player 1 ID: {}. Projected PPR Points: {}\nPlayer 2 ID: {}. Projected PPR Points: {}", player1.player_id, ppr_proj1, player2.player_id, ppr_proj2);
  Ok(Template::render("test2", context! {
    default_player1 : &player1.long_name,
    default_player2 :&player2.long_name,
    player1_name: &player1.long_name,
    player2_name: &player2.long_name,
    player1_proj : proj1,
    player2_proj : proj2,
    week : s.week,
    opponent_1 : &opp1_team.team_name,
    opponent_2 : &opp2_team.team_name,
  }))
}


#[launch]
fn rocket() -> _ {
  rocket::build()
    .mount ("/", routes![index, analyze_route])
    .attach (Template::fairing ())
}