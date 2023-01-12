use std::time::Duration;

use fantoccini::{key::Key, Client, Locator};
use tokio::time::sleep;

use crate::{env::Env, handlers::handle_response};

pub async fn login_to_messenger(
    client: &Client,
    env: &Env,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Going to messenger.com ...");
    client.goto("https://www.messenger.com/").await?;
    println!("Done");

    match client
        .wait()
        .at_most(Duration::from_millis(3000))
        .for_element(Locator::XPath(
            r#"//button[@data-testid="cookie-policy-manage-dialog-accept-button"]"#,
        ))
        .await
    {
        Ok(button) => {
            println!("Got cookie button");
            button.click().await?;
        }
        Err(_) => {
            println!("No cookie button");
        }
    }

    let email_field = client
        .wait()
        .for_element(Locator::XPath(r#"//input[@name="email"]"#))
        .await?;

    println!("Got email field");

    let password_field = client
        .wait()
        .for_element(Locator::XPath(r#"//input[@name="pass"]"#))
        .await?;

    println!("Got password field");

    email_field.send_keys(&env.email).await?;
    password_field.send_keys(&env.password).await?;

    let login_button = client
        .wait()
        .for_element(Locator::XPath(r#"//button[@name="login"]"#))
        .await?;

    println!("Got login button");

    login_button.click().await?;

    return Ok(());
}

pub async fn listen(client: &Client, recipient_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://www.messenger.com/t/{}", &recipient_id);
    println!("Going to {}...", &url);
    client.goto(&url).await?;
    println!("Done");

    sleep(Duration::from_millis(3000)).await;

    println!("Listening for messages in chat {}", &recipient_id);
    loop {
        sleep(Duration::from_millis(1000)).await;

        let messages_list = client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@data-pagelet="MWV2MessageList"]/div"#,
            ))
            .await?;

        let messages = messages_list
            .find_all(Locator::XPath(r#"//div[@dir="auto"]"#))
            .await?;

        let last_message = match messages.last() {
            Some(message) => message.text().await?,
            None => continue,
        };

        handle_response(&client, &last_message).await?;
    }
}

pub async fn send_message(
    client: &Client,
    message: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let message_field = client
        .wait()
        .for_element(Locator::XPath(r#"//div[@data-lexical-editor="true"]"#))
        .await?;

    for c in message.chars() {
        message_field.send_keys(&c.to_string()).await?;
    }
    message_field.send_keys(&Key::Enter.to_string()).await?;

    Ok(())
}
