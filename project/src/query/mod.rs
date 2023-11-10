pub mod fantasy;
pub mod players;
pub mod schedule;
pub mod teams;
pub use fantasy::{get_standard_projection, get_ppr_projection, get_halfppr_projection, request_fantasy};
pub use players::{get_player, request_players};
pub use schedule::{request_schedule, get_game, get_opp_id};
pub use teams::{request_teams, get_team};
pub use lazy_static::lazy_static;

lazy_static! {
    pub static ref REQUEST_HOST : &'static str = "https://tank01-nfl-live-in-game-real-time-statistics-nfl.p.rapidapi.com";
    pub static ref BETTING : &'static str = "/getNFLBettingOdds?";
    pub static ref SCHEDULE : &'static str = "/getNFLTeamSchedule?";
    pub static ref FANTASY : &'static str = "/getNFLProjections?";
    pub static ref PLAYERS : &'static str = "/getNFLPlayerList";
    pub static ref TEAMS : &'static str = "/getNFLTeams";
    pub static ref API_KEY : &'static str = "7edefd3921mshf363ed62e8f497bp1ddf5bjsnd14740483cb3";
}