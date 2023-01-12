use fantoccini::{Client, ClientBuilder};

pub async fn get_client(web_driver_port: &str) -> Result<Client, Box<dyn std::error::Error>> {
    let mut caps = serde_json::map::Map::new();
    caps.insert(
        "moz:firefoxOptions".to_string(),
        serde_json::json!({ "args": ["--headless"] }),
    );

    let client = ClientBuilder::native()
        .capabilities(caps)
        .connect(&format!("http://localhost:{}", web_driver_port))
        .await?;

    return Ok(client);
}
