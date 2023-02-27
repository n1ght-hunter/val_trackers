use std::fs;

use regex::Regex;
use reqwest::Client;

#[derive(Debug)]
pub enum GetAuthErrors {
    Reqwest(reqwest::Error),
    Regex(regex::Error),
    Capure(String),
}

pub async fn auth_cookies(client: &Client) -> Result<reqwest::Response, reqwest::Error> {
    let res = client
        .post("https://auth.riotgames.com/api/v1/authorization")
        .body(fs::read("body.json").unwrap())
        .header("Content-Type", "application/json")
        .send()
        .await?;
    if res.status() == 403 {
        let res = client
            .post("https://auth.riotgames.com/api/v1/authorization")
            .body(fs::read("body.json").unwrap())
            .header("Content-Type", "application/json")
            .send()
            .await;
        res
    } else {
        Ok(res)
    }
}

pub struct AuthRequest {
    pub access_token: String,
    pub id_token: String,
    pub expires_in: String,
}

pub async fn auth_request(
    client: &Client,
    (user_name, password): (String, String),
) -> Result<AuthRequest, GetAuthErrors> {
    println!("running auth request");
    let res = client
    .put("https://auth.riotgames.com/api/v1/authorization")
        .body(format!("{{\"type\":\"auth\",\"username\":\"{}\",\"password\":\"{}\",\"remember\":true,\"language\":\"en_US\"}}", user_name, password))
        .header("Content-Type", "application/json")
        .send().await.map_err(GetAuthErrors::Reqwest)?
        ;
    let raw_data = res.text().await.map_err(GetAuthErrors::Reqwest)?;
    let pattern = Regex::new(r"access_token=((?:[a-zA-Z]|\d|\.|-|_)*).*id_token=((?:[a-zA-Z]|\d|\.|-|_)*).*expires_in=(\d*)")
    .map_err(GetAuthErrors::Regex)?;
    if let Some(caps) = pattern.captures(&raw_data) {
        let access_token = caps
            .get(1)
            .map(|x| x.as_str().to_string())
            .ok_or(GetAuthErrors::Capure("no access token".into()))?;
        let id_token = caps
            .get(2)
            .map(|x| x.as_str().to_string())
            .ok_or(GetAuthErrors::Capure("no id_token".into()))?;
        let expires_in = caps
            .get(3)
            .map(|x| x.as_str().to_string())
            .ok_or(GetAuthErrors::Capure("no expires_in".into()))?;
        Ok(AuthRequest {
            access_token,
            id_token,
            expires_in,
        })
    } else {
        Err(GetAuthErrors::Capure("No captures".into()))
    }
}
