

use reqwest::Client;
use serde_derive::{Serialize, Deserialize};


use crate::subscriptions::lockfile::{LockFile};

#[derive(Clone, Debug)]
pub struct TokenNEntitlement {
    token: Option<Entitlements>,
    client: Client,
    file: LockFile,
}

impl TokenNEntitlement {
    pub fn new(client: Client, file: LockFile) -> Self {
        Self {
            token: None,
            client,
            file,
        }
    }

    pub async fn get_token_n_entitlement(&mut self) -> Result<Entitlements, TokenError> {
        if let Some(token) = &self.token {
            return Ok(token.clone());
        } else {
            let file = self.file.get_file().await;
            if let Some(file) = file {
                let token = self
                    .client
                    .get(format!(
                        "https://127.0.0.1:{}/entitlements/v1/token",
                        file.port
                    ))
                    .basic_auth("Authorization", Some(file.password))
                    .send()
                    .await
                    .map_err(TokenError::RequestError)?
                    .json::<Entitlements>()
                    .await
                    .map_err(TokenError::RequestError)?;
                self.token = Some(token.clone());
                return Ok(token);
            } else {
                return Err(TokenError::LockFile);
            }
        }
    }
}

#[derive(Debug)]
pub enum TokenError {
    RequestError(reqwest::Error),
    LockFile,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entitlements {
    pub access_token: String,
    pub entitlements: Vec<serde_json::Value>,
    pub issuer: String,
    pub subject: String,
    pub token: String,
}

// pub async fn get_token_n_entitlement(client: &Client) -> Result<Match, reqwest::Error> {
//     Ok(client
//         .get(format!("{}/matches/{}", BASE_URL, match_uuid))
//         .query(&[("source", "overwolf")])
//         .header("TRN-API-Key", TRN_API_KEY)
//         .send()
//         .await?
//         .json::<Root>()
//         .await?
//         .data)
// }
