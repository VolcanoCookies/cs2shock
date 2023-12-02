use serde::Serialize;
use serde_json::{json, Value};

use crate::{config::Config, NAME};

pub async fn post(config: &Config, body: PiShockOp) -> Result<i32, String> {
    let mut raw_body = json!(body);
    if let Value::Object(inner) = &mut raw_body {
        inner.insert(
            "Username".to_owned(),
            Value::String(config.username.clone()),
        );
        inner.insert("Name".to_owned(), Value::String(NAME.to_string()));
        inner.insert("Code".to_owned(), Value::String(config.code.clone()));
        inner.insert("Apikey".to_owned(), Value::String(config.apikey.clone()));
        inner.insert("Op".to_owned(), Value::Number(body.to_op().into()));
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
                return Ok(res.status().as_u16() as i32);
            } else {
                return Err(format!(
                    "Failed to post to pishock: {}",
                    res.status().as_u16()
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
