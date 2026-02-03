use anyhow::{Error, Result};
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use spin_sdk::http::{IntoResponse, Method, Request, Response};
use spin_sdk::http_component;
use spin_sdk::key_value::Store;
use std::collections::HashMap;

const HIGH_SCORE_KEY: &str = "highscore";

#[http_component]
fn handle_highscore(req: Request) -> Result<impl IntoResponse> {
    let res_body: String = match *req.method() {
        Method::Get => serde_json::to_string_pretty(&get_highscore().unwrap()).unwrap(),
        Method::Post => {
            let Ok(payload) = serde_json::from_slice::<HighScore>(req.body()) else {
                return Ok(Response::new(400, "Bad Request"));
            };
            replace_highscore(payload.clone())?;
            let highscores = match get_highscore() {
                Ok(highscores) => highscores,
                Err(e) => panic!("Tried to get high score: {}", Error::msg(e.to_string())),
            };

            // Check if the incoming score made the high score list
            let incoming_score_pos = highscores
                .iter()
                .position(|s| s.ulid.unwrap() == payload.ulid.unwrap());

            let rank = match incoming_score_pos {
                Some(r) => {
                    println!("It is a high score at {}", r + 1);
                    r + 1
                }
                None => {
                    println!("It is not a high score");
                    delete_highscore(payload.ulid.unwrap())?;
                    0
                }
            };

            // Setting up response
            let response = HighScoreResult {
                is_high_score: rank > 0,
                rank,
                high_score_table: highscores,
            };

            serde_json::to_string_pretty(&response)?
        }
        _ => return Ok(Response::new(405, "Method not supported")),
    };

    Ok(Response::new(200, res_body))
}

fn get_highscore() -> Result<Vec<HighScore>> {
    let store = Store::open_default()?;
    let highscores = match store.get_json::<HashMap<Ulid, HighScore>>(HIGH_SCORE_KEY)? {
        Some(h) => {
            let mut scores: Vec<HighScore> = h.into_values().collect();
            scores.sort_by(|a, b| b.score.cmp(&a.score));
            scores.into_iter().take(10).collect()
        }
        None => Vec::<HighScore>::new(),
    };
    Ok(highscores)
}

fn replace_highscore(highscore: HighScore) -> Result<()> {
    let ulid = highscore.ulid.expect("ulid is required");
    let store = Store::open_default()?;
    let found = store.get_json::<HashMap<Ulid, HighScore>>(HIGH_SCORE_KEY)?;
    let scores = match found {
        Some(mut hs) => {
            hs.insert(ulid, highscore);
            hs
        }
        None => {
            let mut scores = HashMap::<Ulid, HighScore>::new();
            scores.insert(ulid, highscore);
            scores
        }
    };
    store.set_json(HIGH_SCORE_KEY, &scores)?;
    Ok(())
}

fn delete_highscore(ulid: Ulid) -> Result<()> {
    let store = Store::open_default()?;
    let new_scores = match store.get_json::<HashMap<Ulid, HighScore>>(HIGH_SCORE_KEY)? {
        Some(mut hs) => {
            hs.remove(&ulid);
            hs
        }
        None => HashMap::new(),
    };
    store.set_json(HIGH_SCORE_KEY, &new_scores)?;
    Ok(())
}

#[derive(Deserialize, Serialize, Clone)]
struct HighScore {
    score: i32,
    username: String,
    ulid: Option<Ulid>,
}

#[derive(Deserialize, Serialize)]
struct HighScoreResult {
    is_high_score: bool,
    rank: usize,
    high_score_table: Vec<HighScore>,
}
