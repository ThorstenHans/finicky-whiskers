use std::str::FromStr;

use anyhow::{Context, Result};
use rand::Rng;
use serde::Serialize;
use spin_sdk::http::conversions::IntoBody;
use spin_sdk::http::{send, IntoResponse, Request, RequestBuilder, Response, ResponseBuilder};
use spin_sdk::{http_component, variables};
use ulid::Ulid;
const MAX_INDEX: usize = 30_000;
const FLAVOURS: &[&str] = &["chicken", "fish", "beef", "veg"];

async fn track() {
    let should_track = FromStr::from_str(
        variables::get("track_game_start")
            .unwrap_or("false".to_string())
            .as_str(),
    )
    .unwrap_or_default();

    if should_track {
        let tracking_origin = variables::get("track_game_start_url").unwrap_or_default();
        let tracking_path = variables::get("track_game_start_path").unwrap_or_default();
        let tracking_url = format!("{}{}", tracking_origin, tracking_path);
        if String::is_empty(&tracking_url) {
            println!("Should track game start but cant as tacking url is not set");
            return;
        }
        let req = RequestBuilder::new(spin_sdk::http::Method::Post, tracking_url).build();
        let res: Result<Response> = send(req).await.with_context(|| "Error tracking game start");
        match res {
            Ok(v) => println!("Received {} while tracking game start", v.status()),
            Err(e) => println!("Could not track game start: {}", e),
        }
    } else {
        println!("Won't track game start")
    }
}
#[http_component]
async fn handle_session(_req: Request) -> anyhow::Result<impl IntoResponse> {
    track().await;
    let mut rng = rand::rng();
    let mut index = 0_usize;
    let mut menu = Vec::new();

    while index < MAX_INDEX {
        menu.push(MenuItem {
            demand: random_flavour(&mut rng).to_string(),
            offset: index,
        });
        index += rng.random_range(1000..=3000);
    }
    let pl = Payload {
        id: Ulid::new().to_string(),
        menu,
    };

    Ok(ResponseBuilder::new(200)
        .header("content-type", "application/json")
        .body(pl)
        .build())
}

fn random_flavour(rng: &mut impl Rng) -> &'static str {
    FLAVOURS[rng.random_range(0..FLAVOURS.len())]
}
#[derive(Serialize)]
pub struct MenuItem {
    pub demand: String,
    pub offset: usize,
}

#[derive(Serialize)]
pub struct Payload {
    pub id: String,
    pub menu: Vec<MenuItem>,
}

impl IntoBody for Payload {
    fn into_body(self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap()
    }
}
