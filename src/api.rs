use std::sync::Arc;

use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use log::info;
use rand::{rngs::StdRng, Rng, SeedableRng};
use tokio::sync::{Mutex, RwLock};

use crate::{
    config::{self, Config},
    gamestateintegration::{MapPhase, Payload, RoundPhase},
    pishock, AppState, GameState, PlayerState,
};

pub async fn run(config: Arc<RwLock<Config>>) {
    info!("Sending test beep");
    pishock::beep(config.clone(), 1).await;

    let state = AppState {
        game_state: Arc::from(Mutex::from(GameState::default())),
        config: config.clone(),
    };

    let app = Router::new()
        .route("/data", post(read_data))
        .with_state(state);

    info!("Starting server on {}", "127.0.0.1:3000");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn read_data(State(state): State<AppState>, Json(payload): Json<Payload>) -> StatusCode {
    let mut game_state = state.game_state.lock().await;
    let config = state.config.read().await;

    if let Some(provider) = payload.provider {
        game_state.steam_id = provider.steamid;
    }

    if let Some(map) = payload.map {
        if game_state.map_phase == MapPhase::Warmup && map.phase == MapPhase::Live {
            info!("Match started");

            if config.beep_on_match_start {
                pishock::beep(state.config.clone(), 2).await;
            }

            // Reset game state to default
            game_state.reset();
        }

        game_state.map_phase = map.phase;
    }

    if let Some(round) = payload.round {
        if game_state.round_phase == RoundPhase::Freezetime && round.phase == RoundPhase::Live {
            if config.beep_on_round_start {
                info!("Round started");
                pishock::beep(state.config.clone(), 1).await;
            }
        }

        game_state.round_phase = round.phase;
    }

    if game_state.map_phase != MapPhase::Live {
        return StatusCode::OK;
    }

    if let Some(player) = payload.player {
        if player.steamid != game_state.steam_id {
            return StatusCode::OK;
        }

        if let Some(player_state) = &mut game_state.player_state {
            if player_state.health > player.state.health && player.state.health > 0 {
                // Took damage and survived

                /*
                println!("Player took damage, vibrating");

                let diff = player_state.health - player.state.health;

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
                 */
            }

            if player.match_stats.deaths > player_state.deaths {
                // Died

                // Github Copilot is based
                // let res = pishock::post(&api_state, pishock::PiShockOp::Shock { intensity: 100, duration: 1 }).await;

                info!("Player died, shocking");

                match config.shock_mode {
                    config::ShockMode::Random => {
                        let mut rng = StdRng::from_entropy();
                        let intensity = rng.gen_range(config.min_intensity..=config.max_intensity);
                        let duration = rng.gen_range(config.min_duration..=config.max_duration);

                        pishock::shock(state.config.clone(), intensity, duration).await;
                    }
                    config::ShockMode::LastHitPercentage => {
                        let intensity = (player_state.health as f32 / 100.0
                            * config.max_intensity as f32)
                            as i32;
                        let duration = (player_state.health as f32 / 100.0
                            * config.max_duration as f32)
                            as i32;

                        pishock::shock(state.config.clone(), intensity, duration).await;
                    }
                };
            }

            player_state.health = player.state.health;
            player_state.armor = player.state.armor;
            player_state.kills = player.match_stats.kills;
            player_state.deaths = player.match_stats.deaths;
        } else {
            println!("Player state initialized");

            game_state.player_state = Some(PlayerState {
                health: player.state.health,
                armor: player.state.armor,
                kills: player.match_stats.kills,
                deaths: player.match_stats.deaths,
            });
        }
    }

    StatusCode::OK
}
