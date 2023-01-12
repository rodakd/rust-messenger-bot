use dotenv::dotenv;

mod args;
mod client;
mod core;
mod env;
mod handlers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let args = args::get_args();
    let env = env::get_env();
    let client = client::get_client(&args.web_driver_port).await?;
    core::login_to_messenger(&client, &env).await?;
    core::listen(&client, &args.recipient_id).await?;

    return Ok(());
}
