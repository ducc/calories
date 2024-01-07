use std::sync::RwLock;

use chrono::NaiveDate;
use reqwest::Method;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;
use tracing::info;

use crate::entries::EntriesResponse;

const TOKEN_CACHE_PATH: &str = "nutracheck_token.json";
const APP_API_KEY: &str = "76ef7432-e7d8-437d-b5f6-2adf7aa8d224";
const USER_AGENT: &str = "Nutracheck.iOS.phone";

pub struct Client {
    client: reqwest::Client,
    username: String,
    password: String,
    token: RwLock<NCTokenResponse>,
}

impl Client {
    pub async fn new_from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let username = std::env::var("NUTRACHECK_USERNAME")?;
        let password = std::env::var("NUTRACHECK_PASSWORD")?;

        let token = match get_token_file().await? {
            Some(token_resp) => token_resp,
            None => {
                let token_resp = get_new_nc_token(&username, &password).await?;
                write_token_file(token_resp.clone()).await?;
                token_resp
            }
        };

        Ok(Self {
            client: reqwest::Client::new(),
            username,
            password,
            token: RwLock::new(token),
        })
    }

    async fn refresh_token_if_expired(&self) -> Result<(), Box<dyn std::error::Error + '_>> {
        let expired = self.token.read()?.is_expired();
        if !expired {
            return Ok(());
        }

        let mut guard = self.token.write()?;

        let token_resp = get_new_nc_token(&self.username, &self.password).await?;

        *guard = token_resp;

        Ok(())
    }

    pub async fn entries(
        &self,
        date: NaiveDate,
    ) -> Result<EntriesResponse, Box<dyn std::error::Error + '_>> {
        let entries = self
            .send_request::<EntriesResponse>(
                Method::GET,
                &format!("v1.3/diary/entries?date={}", date.to_string()),
            )
            .await?;
        Ok(entries)
    }

    async fn send_request<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
    ) -> Result<T, Box<dyn std::error::Error + '_>> {
        self.refresh_token_if_expired().await?;

        let token = self.token.read()?;

        let req = self
            .client
            .request(method, format!("https://api.nutracheck.com/api/{}", path))
            .header("Authorization", &token.token)
            // .header("Content-Type", "application/json")
            .header("X-Api-Key", APP_API_KEY)
            .header("User-Agent", USER_AGENT);

        println!("{:?}", req);

        let response = req.send().await?;

        println!("resp code: {}", response.status().as_u16());

        let resp_body = response.text().await?;

        println!("resp: {}", resp_body);

        let de_response: T = serde_json::from_str(&resp_body)?;

        Ok(de_response)
    }
}

async fn get_new_nc_token(
    username: &str,
    password: &str,
) -> Result<NCTokenResponse, Box<dyn std::error::Error>> {
    info!("getting new token");

    let body = json!({
        "username": username,
        "password": password,
    })
    .to_string();

    let client = reqwest::Client::new();
    let resp = client
        .post("https://api.nutracheck.com/api/v1/users/authenticate")
        .header("Content-Type", "application/json")
        .header("X-Api-Key", APP_API_KEY)
        .header("User-Agent", USER_AGENT)
        .body(body)
        .send()
        .await?
        .json::<NCTokenResponse>()
        .await?;

    Ok(resp)
}

async fn get_token_file() -> Result<Option<NCTokenResponse>, Box<dyn std::error::Error>> {
    if !tokio::fs::try_exists(TOKEN_CACHE_PATH).await? {
        return Ok(None);
    }

    let data = tokio::fs::read(TOKEN_CACHE_PATH).await?;

    let token_response: NCTokenResponse = serde_json::from_slice(&data)?;

    if token_response.is_expired() {
        return Ok(None);
    }

    Ok(Some(token_response))
}

async fn write_token_file(resp: NCTokenResponse) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::to_vec(&resp)?;

    tokio::fs::write(TOKEN_CACHE_PATH, &data).await?;

    Ok(())
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NCTokenResponse {
    // #[serde(rename = "userID")]
    // pub user_id: i64,
    pub token: String,
    pub status: NCTokenStatus,
}

impl NCTokenResponse {
    /// is_expired checks if the token has less than 5 minutes left before expiring
    pub fn is_expired(&self) -> bool {
        self.status.date_expiry < chrono::Utc::now() + chrono::Duration::minutes(5)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NCTokenStatus {
    pub account_status: String,
    pub account_status_enum: String,
    pub date_expiry: chrono::DateTime<chrono::Utc>,
    pub date_registration: String,
    pub date_since_last_use: String,
    pub days_since_last_use: i64,
    pub debug_identifier: String,
}
