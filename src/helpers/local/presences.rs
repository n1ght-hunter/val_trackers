// https://127.0.0.1:{% lockfileport  %}/chat/v4/presences

use reqwest::Client;

use crate::subscriptions::lockfile::File;

pub async fn get_presences(file: File, client: Client) -> Result<String, reqwest::Error> {
    Ok(client
        .get(format!("https://127.0.0.1:{}/chat/v4/presences", file.port))
        .basic_auth("Authorization", Some(file.password))
        .send()
        .await?
        .text()
        .await?)
}
