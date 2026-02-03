use std::collections::HashMap;

use anyhow::Result;
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use spin_sdk::http::{IntoResponse, Request, ResponseBuilder};
use spin_sdk::http_component;
use spin_sdk::key_value::Store;

#[http_component]
fn handle_scoreboard(req: Request) -> Result<impl IntoResponse> {
    let ulid = get_ulid(req.query())?;

    let score = match get_scores(ulid) {
        Ok(scores) => scores,
        Err(e) => {
            eprintln!("Error fetching scorecard: {}", e);
            // Return a blank scorecard.
            Scorecard::new(ulid)
        }
    };

    let msg = serde_json::to_string(&score)?;
    Ok(ResponseBuilder::new(200).body(msg).build())
}

#[derive(Deserialize, Serialize)]
pub struct Scorecard {
    pub ulid: Ulid,
    pub beef: i32,
    pub fish: i32,
    pub chicken: i32,
    pub veg: i32,
    pub total: i32,
}

impl Scorecard {
    fn new(ulid: Ulid) -> Self {
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

fn get_ulid(query: &str) -> Result<Ulid> {
    let params = simple_query_parser(query);
    match params.get("ulid") {
        Some(raw_ulid) => {
            let ulid = raw_ulid.parse()?;
            Ok(ulid)
        }
        None => anyhow::bail!("ULID is required in query parameters"),
    }
}

fn get_scores(ulid: Ulid) -> Result<Scorecard> {
    let store = Store::open_default()?;

    let maybe_scorecard = store
        .get_json::<Scorecard>(format!("fw-{}", ulid.to_string()).as_str())
        .map_err(|e| anyhow::anyhow!("Error fetching from key/value: {e}"))?;
    Ok(maybe_scorecard.unwrap_or(Scorecard::new(ulid)))
}

fn simple_query_parser(query: &str) -> HashMap<String, String> {
    let mut dict = HashMap::new();
    query.split('&').for_each(|s| {
        if let Some((k, v)) = s.split_once('=') {
            dict.insert(k.to_string(), v.to_string());
        }
    });
    dict
}
