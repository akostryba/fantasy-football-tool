use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleRoot {
    pub status_code: i64,
    pub body: ScheduleBody,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleBody {
    pub team: String,
    pub schedule: Vec<Game>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    #[serde(rename = "gameID")]
    pub game_id: String,
    pub season_type: String,
    pub away: String,
    pub game_time: String,
    pub game_date: String,
    #[serde(rename = "teamIDHome")]
    pub team_idhome: String,
    pub game_week: String,
    #[serde(rename = "teamIDAway")]
    pub team_idaway: String,
    pub home: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerRoot {
    pub status_code: i64,
    pub body: Vec<PlayerBody>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerBody {
    #[serde(rename = "espnID")]
    pub espn_id: String,
    pub espn_name: String,
    #[serde(rename = "espnIDFull")]
    pub espn_idfull: String,
    pub weight: String,
    pub jersey_num: String,
    pub cbs_short_name: Option<String>,
    pub team: String,
    #[serde(rename = "yahooPlayerID")]
    pub yahoo_player_id: Option<String>,
    pub age: String,
    pub espn_link: String,
    pub yahoo_link: Option<String>,
    pub b_day: String,
    pub espn_headshot: Option<String>,
    pub is_free_agent: String,
    #[serde(rename = "rotoWirePlayerIDFull")]
    pub roto_wire_player_idfull: Option<String>,
    pub cbs_long_name: Option<String>,
    pub injury: PlayerInjury,
    #[serde(rename = "teamID")]
    pub team_id: String,
    pub pos: String,
    pub school: String,
    #[serde(rename = "cbsPlayerID")]
    pub cbs_player_id: Option<String>,
    pub long_name: String,
    #[serde(rename = "rotoWirePlayerID")]
    pub roto_wire_player_id: Option<String>,
    pub height: String,
    #[serde(rename = "cbsPlayerIDFull")]
    pub cbs_player_idfull: Option<String>,
    pub last_game_played: Option<String>,
    #[serde(rename = "playerID")]
    pub player_id: String,
    pub exp: String,
    #[serde(rename = "cbsBDay")]
    pub cbs_bday: Option<String>,
    pub cbs_age: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerInjury {
    pub description: String,
    pub inj_date: String,
    pub designation: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamsRoot {
    pub status_code: i64,
    pub body: Vec<TeamsBody>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamsBody {
    pub team_abv: String,
    pub team_city: String,
    pub current_streak: CurrentStreak,
    pub loss: String,
    pub team_name: String,
    pub nfl_com_logo1: String,
    #[serde(rename = "teamID")]
    pub team_id: String,
    pub tie: String,
    pub pa: String,
    pub pf: String,
    pub espn_logo1: String,
    pub wins: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentStreak {
    pub result: String,
    pub length: String,
}