pub mod config;
pub mod gamestateintegration;
pub mod pishock;

use std::sync::Arc;

use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use config::Config;
use config_file::FromConfigFile;
use gamestateintegration::{MapPhase, Payload, RoundPhase};
use serde_json::Value;
use tokio::sync::Mutex;

pub const NAME: &str = "CS2 Shocker";

#[derive(Debug, Clone)]
struct AppState {
    game_state: Arc<Mutex<GameState>>,
    config: Config,
}

#[derive(Debug, Clone)]
struct GameState {
    round_phase: RoundPhase,
    map_phase: MapPhase,
    health: i32,
    armor: i32,
    kills: i32,
    deaths: i32,
    steam_id: String,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            round_phase: RoundPhase::Unknown,
            map_phase: MapPhase::Unknown,
            health: 0,
            armor: 0,
            kills: 0,
            deaths: 0,
            steam_id: String::new(),
        }
    }
}

impl GameState {
    fn reset(&mut self) {
        self.round_phase = RoundPhase::Unknown;
        self.map_phase = MapPhase::Unknown;
        self.health = 0;
        self.armor = 0;
        self.kills = 0;
        self.deaths = 0;
    }
}

#[tokio::main]
async fn main() {
    let config =
        config::Config::from_config_file("config.toml").expect("Failed to load config file");

    let state = AppState {
        game_state: Arc::from(Mutex::from(GameState::default())),
        config: config.clone(),
    };

    let app = Router::new()
        .route("/data", post(read_data))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn read_data(State(state): State<AppState>, Json(payload): Json<Payload>) -> StatusCode {
    let mut game_state = state.game_state.lock().await;
    let config = state.config;

    if let Some(provider) = payload.provider {
        game_state.steam_id = provider.steamid;
    }

    if let Some(map) = payload.map {
        if game_state.map_phase == MapPhase::Warmup && map.phase == MapPhase::Live {
            println!("Match started");

            let res = pishock::post(&config, pishock::PiShockOp::Beep { duration: 2 }).await;
            match res {
                Ok(code) => println!("Beeped with code {}", code),
                Err(e) => println!("Error while beeping: {}", e),
            }

            // Reset game state to default
            game_state.reset();
        }

        game_state.map_phase = map.phase;
    }

    if let Some(round) = payload.round {
        if game_state.round_phase == RoundPhase::Freezetime && round.phase == RoundPhase::Live {
            println!("Round started");

            let res = pishock::post(&config, pishock::PiShockOp::Beep { duration: 1 }).await;
            match res {
                Ok(code) => println!("Beeped with code {}", code),
                Err(e) => println!("Error while beeping: {}", e),
            }
        }

        game_state.round_phase = round.phase;
    }

    if let Some(player) = payload.player {
        if player.steamid != game_state.steam_id {
            return StatusCode::OK;
        }

        if game_state.health > player.state.health && player.state.health > 0 {
            // Took damage and survived

            println!("Player took damage, vibrating");

            let diff = game_state.health - player.state.health;

            let res = pishock::post(
                &config,
                pishock::PiShockOp::Vibrate {
                    intensity: diff,
                    duration: 1,
                },
            )
            .await;

            match res {
                Ok(code) => println!("Vibrated with code {}", code),
                Err(e) => println!("Error while vibrating: {}", e),
            };
        }

        if player.match_stats.deaths > game_state.deaths {
            // Died

            // Github Copilot is based
            // let res = pishock::post(&api_state, pishock::PiShockOp::Shock { intensity: 100, duration: 1 }).await;

            println!("Player died, shocking");

            let res = pishock::post(
                &config,
                pishock::PiShockOp::Shock {
                    intensity: config.shock_intensity,
                    duration: 1,
                },
            )
            .await;

            match res {
                Ok(code) => println!("Shock with code {}", code),
                Err(e) => println!("Error while shocking: {}", e),
            }
        }

        game_state.health = player.state.health;
        game_state.armor = player.state.armor;
        game_state.kills = player.match_stats.kills;
        game_state.deaths = player.match_stats.deaths;
    }

    StatusCode::OK
}
