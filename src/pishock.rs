use std::sync::Arc;

use log::{debug, error, info};
use serde::Serialize;
use serde_json::{Map, Value};

use crate::{config::Config, NAME};

pub async fn shock(config: Arc<Config>, intensity: i32, duration: i32) {
    debug!( target: "PiShock API", "Sending shock: {}, {}", intensity, duration);

    let res = post(
        config,
        PiShockOp::Shock {
            intensity,
            duration,
        },
    )
    .await;

    match res {
        Ok(_) => {
            info!( target: "PiShock API",
                "Successfully sent shock (intensity: {}, duration: {})", intensity, duration
            );
        }
        Err(e) => {
            error!( target: "PiShock API", "Failed to send shock: {}", e);
        }
    }
}

pub async fn vibrate(config: Arc<Config>, intensity: i32, duration: i32) {
    debug!( target: "PiShock API",
        "Sending vibrate: {}, {}", intensity, duration
    );

    let res = post(
        config,
        PiShockOp::Vibrate {
            intensity,
            duration,
        },
    )
    .await;

    match res {
        Ok(_) => {
            info!(
                target: "PiShock API",
                "Successfully sent vibrate (intensity: {}, duration: {})", intensity, duration
            );
        }
        Err(e) => {
            error!( target: "PiShock API", "Failed to send vibrate: {}", e);
        }
    }
}

pub async fn beep(config: Arc<Config>, duration: i32) {
    debug!( target: "PiShock API", "Sending beep: {}", duration);

    let res = post(config, PiShockOp::Beep { duration }).await;

    match res {
        Ok(_) => {
            info!(
                        target: "PiShock API",
                "Successfully sent beep (duration: {})", duration
            );
        }
        Err(e) => {
            error!( target: "PiShock API", "Failed to send beep: {}", e);
        }
    }
}

pub async fn post(config: Arc<Config>, body: PiShockOp) -> Result<i32, String> {
    let mut raw_body = Value::Object(Map::new());

    if let Value::Object(inner) = &mut raw_body {
        inner.insert(
            "Username".to_owned(),
            Value::String(config.username.clone()),
        );
        inner.insert("Name".to_owned(), Value::String(NAME.to_string()));
        inner.insert("Code".to_owned(), Value::String(config.code.clone()));
        inner.insert("Apikey".to_owned(), Value::String(config.apikey.clone()));

        match body {
            PiShockOp::Beep { duration } => {
                inner.insert("Duration".to_owned(), Value::Number(duration.into()));
                inner.insert("Op".to_owned(), Value::Number(2.into()));
            }
            PiShockOp::Vibrate {
                intensity,
                duration,
            } => {
                inner.insert("Intensity".to_owned(), Value::Number(intensity.into()));
                inner.insert("Duration".to_owned(), Value::Number(duration.into()));
                inner.insert("Op".to_owned(), Value::Number(1.into()));
            }
            PiShockOp::Shock {
                intensity,
                duration,
            } => {
                inner.insert("Intensity".to_owned(), Value::Number(intensity.into()));
                inner.insert("Duration".to_owned(), Value::Number(duration.into()));
                inner.insert("Op".to_owned(), Value::Number(0.into()));
            }
        }
    } else {
        return Err("raw_body is not an object".into());
    }

    let res = reqwest::Client::new()
        .post("https://do.pishock.com/api/apioperate")
        .json(&raw_body)
        .send()
        .await;

    match res {
        Ok(res) => {
            if res.status().is_success() {
                return Ok(res.status().as_u16().clone() as i32);
            } else {
                return Err(format!(
                    "Failed to post to pishock: {}",
                    res.status().as_u16().clone()
                ));
            }
        }
        Err(e) => {
            return Err(e.to_string());
        }
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum PiShockOp {
    Beep { duration: i32 },
    Vibrate { intensity: i32, duration: i32 },
    Shock { intensity: i32, duration: i32 },
}

impl PiShockOp {
    pub fn to_op(self) -> i32 {
        match self {
            PiShockOp::Beep { duration: _ } => 2,
            PiShockOp::Vibrate {
                intensity: _,
                duration: _,
            } => 1,
            PiShockOp::Shock {
                intensity: _,
                duration: _,
            } => 0,
        }
    }
}
