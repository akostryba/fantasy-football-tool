use std::error::Error;
use std::fmt;
use std::fmt::{Formatter, Display};
use rocket::form::Form;
use rocket_dyn_templates::{Template,context};

use crate::Input;

#[derive(Debug)]
pub enum MyError {
  PlayerNotFoundError(String),
  RequestError(String),
  QueryError(String),
}

//Forgot to mention in the video: 
//I implemented the Display and Error traits from std::error and std::fmt for MyErrors by providing definitions for fmt and source functions

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

pub fn detect_query_error(data_vec : Vec<(String, &String)>) {
  for (key, value) in data_vec{
    if *value==String::from("null"){
      let query_error = MyError::QueryError(format!("{}", key));
      println!("{}", query_error);
    }
  }
}

pub fn handle_error(err: &MyError, s : &Form<Input<'_>>) -> Template{
  match err {
      MyError::PlayerNotFoundError(player_name) => {
          println!("{}", err);
          // Handle PlayerNotFoundError
          return Template::render("test2", context!{
            default_player1 : format!("Player not found: {}", player_name),
            default_player2 : format!("Player not found: {}", player_name),
          });
      }
      MyError::RequestError(_request_name) => {
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
            player1_proj : format!("Missing {}", query),
            player2_proj : format!("Missing {}", query),
            week : s.week,
            opponent_1 : String::from("Data Missing"),
            opponent_2 : String::from("Data Missing"),
            team1_ppg : String::from("Data Missing"),
            team2_ppg : String::from("Data Missing"),
            opponent_1_appg : String::from("Data Missing"),
            opponent_2_appg : String::from("Data Missing"),
            game1_spread : String::from("Data Missing"),
            over_under_1 : String::from("Data Missing"),
            game2_spread : String::from("Data Missing"),
            over_under_2 : String::from("Data Missing"),
            team1_points : String::from("Data Missing"),
            team2_points : String::from("Data Missing"),
          });
      }
  }
}