use fantoccini::Client;
use reqwest;
use serde::Deserialize;

use crate::core::send_message;

pub async fn handle_response(
    client: &Client,
    last_message: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let m = last_message.to_lowercase().trim().to_string();

    if m == "meme" {
        return handle_meme(&client).await;
    }

    return Ok(());
}

#[derive(Deserialize, Debug)]
struct MemeResponse {
    url: String,
}

pub async fn handle_meme(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let resp: MemeResponse = reqwest::get("https://meme-api.com/gimme")
        .await?
        .json()
        .await?;

    return send_message(&client, &resp.url).await;
}
