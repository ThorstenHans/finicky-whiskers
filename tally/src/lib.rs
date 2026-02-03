use std::collections::HashMap;

use anyhow::Result;
use chrono::{Duration, Utc};
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use spin_sdk::http::{IntoResponse, Request, ResponseBuilder};
use spin_sdk::http_component;
use spin_sdk::key_value::Store;

use crate::tally::Tally;
mod tally;

/// Game duration in seconds
const GAME_DURATION_SECONDS: i64 = 30;

/// A simple Spin HTTP component.
#[http_component]
fn handle_tally(req: Request) -> Result<impl IntoResponse> {
    match parse_query_params(req.query()) {
        Ok(tally) => {
            // Should store something in key/value.
            match serde_json::to_string(&tally) {
                Ok(payload) => {
                    if let Err(e) = tally_score(&payload) {
                        eprintln!("Error tallying score: {}", e)
                    } else {
                        println!("Tallied score: {}", payload)
                    }
                }
                Err(e) => eprintln!("Error serializing JSON: {}", e),
            }

            // Send a response
            let msg = format!("ULID: {:?}", tally.ulid);
            Ok(ResponseBuilder::new(200).body(msg).build())
        }
        Err(e) => Err(e),
    }
}

fn simple_query_parser(q: &str) -> HashMap<String, String> {
    let mut dict = HashMap::new();
    q.split('&').for_each(|s| {
        if let Some((k, v)) = s.split_once('=') {
            dict.insert(k.to_string(), v.to_string());
        }
    });
    dict
}

fn parse_query_params(query: &str) -> Result<Tally> {
    // Get the necessary stuff out of the request:
    let params = simple_query_parser(query);
    let ulid = params.get("ulid");
    let food = params.get("food");
    let correct = params.get("correct");

    if ulid.is_none() || food.is_none() || correct.is_none() {
        anyhow::bail!("ULID, food, and correct are required: {}", query);
    }

    validate_ulid(ulid.unwrap().as_str())?;

    Ok(Tally {
        ulid: ulid.unwrap().clone(),
        food: food.unwrap().clone(),
        correct: correct.unwrap().to_lowercase().starts_with("t"),
    })
}

fn validate_ulid(ulid: &str) -> anyhow::Result<Ulid> {
    let id: Ulid = ulid.parse()?;

    // Check expiration
    let now = Utc::now();
    if id.datetime() + Duration::seconds(GAME_DURATION_SECONDS) < now {
        anyhow::bail!("Session is expired")
    }

    Ok(id)
}

fn tally_score(msg: &str) -> anyhow::Result<()> {
    let tally_mon: Tally = serde_json::from_str(msg)?;

    if !tally_mon.correct {
        return Ok(());
    }

    let id: rusty_ulid::Ulid = tally_mon.ulid.parse()?;

    let store = Store::open_default()?;
    let mut scorecard = match store.get(format!("fw-{}", &id.to_string()).as_str()) {
        Err(_) => Scorecard::new(id),
        Ok(data) => match data {
            Some(d) => serde_json::from_slice(&d).unwrap_or_else(|_| Scorecard::new(id)),
            None => Scorecard::new(id),
        },
    };

    match tally_mon.food.as_str() {
        "chicken" => scorecard.chicken += 1,
        "fish" => scorecard.fish += 1,
        "beef" => scorecard.beef += 1,
        "veg" => scorecard.veg += 1,
        _ => {}
    };

    scorecard.total += 1;

    if let Ok(talled_mon) = serde_json::to_vec(&scorecard) {
        store
            .set(format!("fw-{}", &id.to_string()).as_str(), &talled_mon)
            .map_err(|_| anyhow::anyhow!("Error saving to key/value store"))?;
    }

    Ok(())
}

#[derive(Deserialize, Serialize)]
struct Scorecard {
    pub ulid: rusty_ulid::Ulid,
    pub beef: i32,
    pub fish: i32,
    pub chicken: i32,
    pub veg: i32,
    pub total: i32,
}

impl Scorecard {
    fn new(ulid: rusty_ulid::Ulid) -> Self {
        Scorecard {
            ulid,
            beef: 0,
            fish: 0,
            chicken: 0,
            veg: 0,
            total: 0,
        }
    }
}
