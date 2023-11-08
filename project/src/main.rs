pub mod json;

pub use json::my_structs;
pub use json::my_structs::PlayerBody;
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

//testing json query
fn ex(json_str : &str) -> serde_json::Result<()>{ 
    let data : Value = serde_json::from_str(json_str)?;
    println!("{}", data["body"]["20231102_TEN@PIT"]["fanduel"]["totalUnder"]);
    Ok(())
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{


    //let query_param = vec![("gameDate", "20231102")];
    //let schedule_param = vec![("week", "all"), ("seasonType", "reg"), ("season", "2022")];
    //let schedule_request : String = format!("{}{}", *REQUEST_HOST, *SCHEDULE);
    let fantasy_param = vec![("week", "1")];
    let fantasy_request : String = format!("{}{}", *REQUEST_HOST, *FANTASY);
    let players_request : String = format!("{}{}", *REQUEST_HOST, *PLAYERS);

    let client = reqwest::Client::new();
    //let response = client.get("https://tank01-nfl-live-in-game-real-time-statistics-nfl.p.rapidapi.com/getNFLBettingOdds?")
    let response_fantasy = client.get(fantasy_request)
        .header("X-RapidAPI-Key",
        *API_KEY)
        .query(&fantasy_param)
        .send()
        .await?;

    let response_player = client.get(players_request)
        .header("X-RapidAPI-Key",
        *API_KEY)
        .send()
        .await?;

    let players = response_player.json::<my_structs::PlayerRoot>().await?;
    let player = match get_player(&players.body, "Justin Fields"){
        Some(p) => p,
        None => {
            eprintln!("Player not found");
            return Ok(());
        }
    };

    //println!("{:?}", player);
    let id = &player.player_id;
    let data = &response_fantasy.json::<serde_json::Value>().await?;
    let projection = get_ppr_projection(id, data);
    println!("Player Name: {}", player.long_name);
    println!("PPR Projected Points: {}", projection);

    Ok(())
}
