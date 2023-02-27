use reqwest_impersonate::Client;

static BODY: &'static str = "{
    \"client_id\": \"play-valorant-web-prod\",
    \"nonce\": \"1\",
    \"redirect_uri\": \"https://playvalorant.com/opt_in\",
    \"response_type\": \"token id_token\"
}";

pub async fn auth_cookies(
    client: &Client,
) -> Result<reqwest_impersonate::Response, reqwest_impersonate::Error> {
    let res = client
        .post("https://auth.riotgames.com/api/v1/authorization")
        .body(BODY)
        .header("Content-Type", "application/json")
        .send()
        .await?;
    if res.status() == 403 {
        let res = client
            .post("https://auth.riotgames.com/api/v1/authorization")
            .body(BODY)
            .header("Content-Type", "application/json")
            .send()
            .await;
        res
    } else {
        Ok(res)
    }
}
