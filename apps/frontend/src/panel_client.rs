use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Clone)]
pub struct PanelClient {
    client: Client,
    base_url: String,
    auth_token: String,
}

impl PanelClient {
    pub fn new(base_url: String, auth_token: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            auth_token,
        }
    }
    
    pub async fn get_subscription(&self, uuid: &str) -> Result<Subscription> {
        let url = format!("{}/api/internal/subscriptions/{}", self.base_url, uuid);
        
        let response = self.client
            .get(&url)
            .bearer_auth(&self.auth_token)
            .send()
            .await?;
        
        Ok(response.json().await?)
    }
    
    pub async fn get_active_nodes(&self) -> Result<Vec<Node>> {
        let url = format!("{}/api/internal/nodes/active", self.base_url);
        
        let response = self.client
            .get(&url)
            .bearer_auth(&self.auth_token)
            .send()
            .await?;
        
        Ok(response.json().await?)
    }
    
    pub async fn get_user_keys(&self, user_id: i64) -> Result<UserKeys> {
        let url = format!("{}/api/internal/users/{}/keys", self.base_url, user_id);
        
        let response = self.client
            .get(&url)
            .bearer_auth(&self.auth_token)
            .send()
            .await?;
        
        Ok(response.json().await?)
    }
    
    pub async fn send_heartbeat(&self, stats: FrontendStats) -> Result<()> {
        let url = format!("{}/api/internal/frontend/heartbeat", self.base_url);
        
        self.client
            .post(&url)
            .bearer_auth(&self.auth_token)
            .json(&stats)
            .send()
            .await?;
        
        Ok(())
    }
}

// Data structures (simplified - will be shared with main panel)
#[derive(Debug, Serialize, Deserialize)]
pub struct Subscription {
    pub id: i64,
    pub user_id: i64,
    pub status: String,
    pub used_traffic: i64,
    pub subscription_uuid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: i64,
    pub name: String,
    pub ip: String,
    pub vpn_port: i64,
    pub reality_pub: Option<String>,
    pub short_id: Option<String>,
    pub domain: Option<String>,
    pub country_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserKeys {
    pub user_uuid: String,
    pub hy2_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FrontendStats {
    pub requests_count: u64,
    pub bandwidth_used: u64,
}
