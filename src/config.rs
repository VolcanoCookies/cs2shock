use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub max_shock_intensity: i32,
    pub shock_duration: i32,
    pub beep_on_match_start: bool,
    pub beep_on_round_start: bool,
    pub username: String,
    pub code: String,
    pub apikey: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_shock_intensity: 0,
            shock_duration: 0,
            beep_on_match_start: false,
            beep_on_round_start: false,
            username: String::new(),
            code: String::new(),
            apikey: String::new(),
        }
    }
}
