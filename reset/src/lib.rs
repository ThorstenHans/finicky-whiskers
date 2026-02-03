use anyhow::{Context, Result};
use spin_sdk::http::{IntoResponse, Request, ResponseBuilder};
use spin_sdk::http_component;
use spin_sdk::key_value::Store;

#[http_component]
fn handle_reset(_req: Request) -> anyhow::Result<impl IntoResponse> {
    if let Err(e) = reset_keyvalue() {
        return Ok(ResponseBuilder::new(500).body(e.to_string()).build());
    }
    if let Err(e) = reset_highscore() {
        return Ok(ResponseBuilder::new(500).body(e.to_string()).build());
    }
    Ok(ResponseBuilder::new(200)
        .body("Finicky Whickers is reset.")
        .build())
}
fn reset_keyvalue() -> Result<()> {
    let store = Store::open_default().with_context(|| "Failed to open default key-value store")?;
    let keys = store
        .get_keys()
        .with_context(|| "Failed to get keys from key-value store")?;

    keys.into_iter()
        .filter(|key| key.starts_with("fw-"))
        .try_for_each(|key| {
            store
                .delete(&key)
                .with_context(|| "Failed to delete {key} from key-value store")
        })?;

    Ok(())
}

const HIGH_SCORE_KEY: &str = "highscore";

fn reset_highscore() -> Result<()> {
    let store = Store::open_default()?;
    store
        .delete(HIGH_SCORE_KEY)
        .with_context(|| "Failed to reset high score")
}
