use std::{fs::OpenOptions, io::Write};

use log::error;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ShockMode {
    Random,
    LastHitPercentage,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub shock_mode: ShockMode,
    pub min_duration: i32,
    pub max_duration: i32,
    pub min_intensity: i32,
    pub max_intensity: i32,
    pub beep_on_match_start: bool,
    pub beep_on_round_start: bool,
    pub username: String,
    pub code: String,
    pub apikey: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            shock_mode: ShockMode::Random,
            min_duration: 1,
            max_duration: 1,
            min_intensity: 1,
            max_intensity: 1,
            beep_on_match_start: false,
            beep_on_round_start: false,
            username: String::new(),
            code: String::new(),
            apikey: String::new(),
        }
    }
}
impl Config {
    pub fn validate(&self) -> bool {
        if self.min_duration < 1 || self.min_duration > 15 {
            error!(target: "Config", "min_duration must be between 1 and 15");
            return false;
        }

        if self.max_duration < 1 || self.max_duration > 15 {
            error!(target: "Config", "max_duration must be between 1 and 15");
            return false;
        }

        if self.min_duration > self.max_duration {
            error!(target: "Config", "min_duration must be less than or equal to max_duration");
            return false;
        }

        if self.min_intensity < 0 || self.min_intensity > 100 {
            error!(target: "Config", "min_intensity must be between 0 and 100");
            return false;
        }

        if self.max_intensity < 0 || self.max_intensity > 100 {
            error!(target: "Config", "max_intensity must be between 0 and 100");
            return false;
        }

        if self.min_intensity > self.max_intensity {
            error!(target: "Config", "min_intensity must be less than or equal to max_intensity");
            return false;
        }

        return true;
    }

    pub fn write_to_file(&self, path: &str) {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(path)
            .expect(format!("Failed to open config file, {}", path).as_str());

        let raw = format!(
            "# Shock mode, one of; random, last_hit_percentage
shock_mode = \"random\"
# Minimum duration of the shock
min_duration = {}
# Maximum duration of the shock
max_duration = {}
# Minimum intensity of the shock, 1-100
min_intensity = {}
# Maximum intensity of the shock, 1-100
max_intensity = {}
# Beep when match starts
beep_on_match_start = {}
# Beep when round starts
beep_on_round_start = {}
# PiShock username to access the api
username = \"{}\"
# PiShock share code to access the api
code = \"{}\"
# PiShock api key to access the api
apikey = \"{}\"",
            self.min_duration,
            self.max_duration,
            self.min_intensity,
            self.max_intensity,
            self.beep_on_match_start,
            self.beep_on_round_start,
            self.username,
            self.code,
            self.apikey
        );

        file.write_all(raw.as_bytes())
            .expect("Failed to write config file");
    }
}
