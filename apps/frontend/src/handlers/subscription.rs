use axum::{
    extract::{Path, Query, State},
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use tracing::{info, error};
use crate::AppState;

#[derive(Deserialize)]
pub struct SubParams {
    pub client: Option<String>, // "clash" | "v2ray" | "singbox"
}

pub async fn subscription_handler(
    Path(uuid): Path<String>,
    Query(params): Query<SubParams>,
    State(state): State<AppState>,
) -> Response {
    info!("Subscription request: UUID={}, client={:?}", uuid, params.client);
    
    // 1. Get subscription from main panel
    let sub = match state.panel_client.get_subscription(&uuid).await {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to fetch subscription: {}", e);
            return (StatusCode::NOT_FOUND, "Subscription not found").into_response();
        }
    };
    
    // 2. Check if active
    if sub.status != "active" {
        info!("Subscription {} is inactive (status: {})", uuid, sub.status);
        return (StatusCode::FORBIDDEN, "Subscription inactive or expired").into_response();
    }
    
    // 3. Get active nodes from panel
    let nodes = match state.panel_client.get_active_nodes().await {
        Ok(nodes) if !nodes.is_empty() => nodes,
        _ => {
            error!("No active nodes available");
            return (StatusCode::SERVICE_UNAVAILABLE, "No servers available").into_response();
        }
    };
    
    // 4. Get user keys from panel
    let user_keys = match state.panel_client.get_user_keys(sub.user_id).await {
        Ok(keys) => keys,
        Err(e) => {
            error!("Failed to get user keys: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Internal error").into_response();
        }
    };
    
    // 5. Generate config based on client type
    let client_type = params.client.as_deref().unwrap_or("singbox");
    let (content, content_type, filename) = match client_type {
        "clash" => {
            match generate_clash_config(&nodes, &user_keys) {
                Ok(yaml) => (yaml, "application/yaml", "config.yaml"),
                Err(e) => {
                    error!("Failed to generate Clash config: {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Config generation failed").into_response();
                }
            }
        }
        "v2ray" => {
            match generate_v2ray_config(&nodes, &user_keys) {
                Ok(b64) => (b64, "text/plain", "config.txt"),
                Err(e) => {
                    error!("Failed to generate V2Ray config: {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Config generation failed").into_response();
                }
            }
        }
        _ => {
            match generate_singbox_config(&nodes, &user_keys) {
                Ok(json) => (json, "application/json", "config.json"),
                Err(e) => {
                    error!("Failed to generate Sing-box config: {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Config generation failed").into_response();
                }
            }
        }
    };
    
    info!("Generated {} config for subscription {} ({} bytes)", client_type, uuid, content.len());
    
    // 6. Return with proper headers
    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, content_type),
            (header::CONTENT_DISPOSITION, format!("inline; filename={}", filename).as_str()),
        ],
        content
    ).into_response()
}

// Config generators (simplified versions from main panel)
fn generate_clash_config(nodes: &[crate::panel_client::Node], keys: &crate::panel_client::UserKeys) -> anyhow::Result<String> {
    use serde_json::json;
    
    let mut proxies = Vec::new();
    
    for node in nodes {
        proxies.push(json!({
            "name": format!("{} VLESS", node.name),
            "type": "vless",
            "server": node.ip,
            "port": node.vpn_port,
            "uuid": keys.user_uuid,
            "network": "tcp",
            "tls": true,
            "servername": node.domain.as_ref().unwrap_or(&"www.google.com".to_string()),
            "reality-opts": {
                "public-key": node.reality_pub.as_ref().unwrap_or(&"".to_string()),
                "short-id": node.short_id.as_ref().unwrap_or(&"".to_string())
            },
            "client-fingerprint": "chrome"
        }));
    }
    
    let proxy_names: Vec<String> = proxies.iter()
        .map(|p| p["name"].as_str().unwrap().to_string())
        .collect();
    
    let config = json!({
        "proxies": proxies,
        "proxy-groups": [{
            "name": "EXA-ROBOT",
            "type": "select",
            "proxies": proxy_names
        }],
        "rules": ["MATCH,EXA-ROBOT"]
    });
    
    Ok(serde_yaml::to_string(&config)?)
}

fn generate_v2ray_config(nodes: &[crate::panel_client::Node], keys: &crate::panel_client::UserKeys) -> anyhow::Result<String> {
    let mut links = Vec::new();
    
    for node in nodes {
        let vless_link = format!(
            "vless://{}@{}:{}?encryption=none&flow=xtls-rprx-vision&security=reality&sni={}&fp=chrome&pbk={}&sid={}&type=tcp#{}",
            keys.user_uuid,
            node.ip,
            node.vpn_port,
            node.domain.as_ref().unwrap_or(&"www.google.com".to_string()),
            node.reality_pub.as_ref().unwrap_or(&"".to_string()),
            node.short_id.as_ref().unwrap_or(&"".to_string()),
            urlencoding::encode(&format!("{} VLESS", node.name))
        );
        links.push(vless_link);
    }
    
    use base64::Engine;
    Ok(base64::engine::general_purpose::STANDARD.encode(links.join("\n")))
}

fn generate_singbox_config(nodes: &[crate::panel_client::Node], keys: &crate::panel_client::UserKeys) -> anyhow::Result<String> {
    use serde_json::json;
    
    let mut outbounds = vec![];
    
    for node in nodes {
        outbounds.push(json!({
            "type": "vless",
            "tag": format!("{}_vless", node.name),
            "server": node.ip,
            "server_port": node.vpn_port,
            "uuid": keys.user_uuid,
            "flow": "xtls-rprx-vision",
            "tls": {
                "enabled": true,
                "server_name": node.domain.as_ref().unwrap_or(&"www.google.com".to_string()),
                "reality": {
                    "enabled": true,
                    "public_key": node.reality_pub.as_ref().unwrap_or(&"".to_string()),
                    "short_id": node.short_id.as_ref().unwrap_or(&"".to_string())
                }
            }
        }));
    }
    
    let config = json!({
        "log": { "level": "info" },
        "inbounds": [{
            "type": "mixed",
            "tag": "mixed-in",
            "listen": "127.0.0.1",
            "listen_port": 2080
        }],
        "outbounds": outbounds,
        "route": {
            "rules": [],
            "final": outbounds.get(0).and_then(|o| o["tag"].as_str()).unwrap_or("direct")
        }
    });
    
    Ok(serde_json::to_string_pretty(&config)?)
}
