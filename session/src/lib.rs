use rand::Rng;
use serde::Serialize;
use spin_sdk::http::conversions::IntoBody;
use spin_sdk::http::{IntoResponse, Request, ResponseBuilder};
use spin_sdk::http_component;
use ulid::Ulid;
const MAX_INDEX: usize = 30_000;
const FLAVOURS: &[&str] = &["chicken", "fish", "beef", "veg"];

#[http_component]
fn handle_session(_req: Request) -> anyhow::Result<impl IntoResponse> {
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
