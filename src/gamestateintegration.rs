use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Map {
    pub mode: String,
    pub name: String,
    pub phase: MapPhase,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MapPhase {
    Warmup,
    Intermission,
    GameOver,
    Live,
    Unknown,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Round {
    pub phase: RoundPhase,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RoundPhase {
    Freezetime,
    Live,
    Over,
    Unknown,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct PlayerState {
    pub health: i32,
    pub armor: i32,
    pub helmet: bool,
    pub flashed: i32,
    pub smoked: i32,
    pub burning: i32,
    pub money: i32,
    pub round_kills: i32,
    pub round_killhs: i32,
    pub equip_value: i32,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct PlayerMatchStats {
    pub kills: i32,
    pub assists: i32,
    pub deaths: i32,
    pub mvps: i32,
    pub score: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Player {
    pub steamid: String,
    pub name: String,
    pub state: PlayerState,
    pub match_stats: PlayerMatchStats,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Provider {
    pub name: String,
    pub appid: i32,
    pub version: i32,
    pub steamid: String,
    pub timestamp: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Payload {
    pub provider: Option<Provider>,
    pub map: Option<Map>,
    pub round: Option<Round>,
    pub player: Option<Player>,
}
