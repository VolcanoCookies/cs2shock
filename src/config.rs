use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub shock_intensity: i32,
    pub username: String,
    pub code: String,
    pub apikey: String,
}
