use dotenv;
use webhook::client::{WebhookClient, WebhookResult};

#[tokio::main]
async fn main() -> WebhookResult<()> {
    dotenv::dotenv()?;

    let website_url = dotenv::var("WEBSITE_URL")?;
    let response = reqwest::get(&website_url).await?;

    if !response.status().is_success() {
        let webhook_url = dotenv::var("WEBHOOK_URL")?;
        let client = WebhookClient::new(&webhook_url);

        client.send(|message| message.
            content("Website is down!")
            .username("Website status")).await?;
    }

    Ok(())
}
