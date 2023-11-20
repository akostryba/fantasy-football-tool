use reqwest::Client;
use rocket::form::Form;
use rocket_dyn_templates::{Template,context};

pub mod json;
pub mod query;
pub use query::{get_standard_projection, get_ppr_projection, get_halfppr_projection, request_fantasy, 
                request_players, get_player,
                request_schedule, get_game, get_opp_id,
                request_teams, get_team, get_allowed_ppg,
                request_betting, get_over_under, get_spread};
pub mod my_errors;
pub use my_errors::{MyError, detect_query_error, handle_error};


pub use json::my_structs;

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
pub struct Input<'r> {
  pub player1 : &'r str, 
  pub player2 : &'r str,
  pub week : &'r str,
  pub scoring : &'r str,
}

#[post ("/analyze", data = "<s>")]
async fn analyze_route (s : Form<Input<'_>>) -> Template {
  match analyze(&s).await{
    Ok(template) => template,
    Err(err) => {
      return handle_error(&*err, &s);
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

  println!("Score1 Found: {}, Score2 Found: {}", proj1, proj2);
  
  let context_vars: Vec<(String, &String)> = vec![(String::from("Projection 1"), &proj1), (String::from("Projection 2"), &proj2)];
  detect_query_error(context_vars);

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

  let opp1_appg = get_allowed_ppg(opp1_team);
  let opp2_appg = get_allowed_ppg(opp2_team);

  let betting_data_1 = match request_betting(&client, &game1.game_id, &game1.game_date).await{
    Ok(d) => d,
    _ =>{
      return Err(Box::new(MyError::RequestError(String::from("Betting Data 1"))));
    }
  };

  let betting_data_2 = match request_betting(&client, &game2.game_id, &game2.game_date).await{
    Ok(d) => d,
    _ =>{
      return Err(Box::new(MyError::RequestError(String::from("Betting Data 2"))));
    }
  };

  let team1_loc = {
    if game1.team_idaway == player1.team_id {
      "away"
    }
    else {
      "home"
    }
  };

  let team2_loc = {
    if game2.team_idaway == player2.team_id {
      "away"
    }
    else {
      "home"
    }
  };

  let game1_spread = get_spread(&betting_data_1, &game1.game_id, team1_loc);
  let ou_1 = get_over_under(&betting_data_1, &game1.game_id);
  let game2_spread = get_spread(&betting_data_2, &game2.game_id, team2_loc);
  let ou_2 = get_over_under(&betting_data_2, &game2.game_id);
  
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
    opponent_1_appg : format!("{:.2}",opp1_appg),
    opponent_2_appg : format!("{:.2}",opp2_appg),
    game1_spread : game1_spread,
    over_under_1 : ou_1,
    game2_spread : game2_spread,
    over_under_2 : ou_2,
  }))
}


#[launch]
fn rocket() -> _ {
  rocket::build()
    .mount ("/", routes![index, analyze_route])
    .attach (Template::fairing ())
}