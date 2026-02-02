# API & Routes Reference

This document outlines the HTTP endpoints available in the EXA ROBOT control panel and the CLI commands.

## 🌐 Public Routes
Routes accessible without authentication (or handling their own auth).

| Method | Path | Handler | Description |
|--------|------|---------|-------------|
| `GET` | `/` | `(Closure)` | Redirects to Dashboard |
| `GET` | `/{admin_path}/login` | `handlers::admin::get_login` | Admin login page |
| `POST` | `/{admin_path}/login` | `handlers::admin::login` | Process login attempt |
| `POST` | `/api/payments/cryptobot` | `handlers::admin::handle_payment` | CryptoBot Webhook |

## 🔗 Agent API V2
Routes used by the Node Agent to communicate with the Panel.

| Method | Path | Handler | Description |
|--------|------|---------|-------------|
| `POST` | `/api/v2/node/heartbeat` | `api::v2::node::heartbeat` | Node status reporting |
| `GET` | `/api/v2/node/config` | `api::v2::node::get_config` | Fetch Sing-box configuration |

## 🧠 Client API V2 (AI)
Routes used by Client App/Panel for intelligent features.

| Method | Path | Handler | Description |
|--------|------|---------|-------------|
| `GET` | `/api/v2/client/recommended` | `api::v2::client::get_recommended_nodes` | Get optimal servers based on latency/load |

## 🛡️ Admin Routes (Authenticated)
All routes below are prefixed with `/{admin_path}` (Default: `/admin`) and require an active session.

### Dashboard & Settings
| Method | Path | Handler | Description |
|--------|------|---------|-------------|
| `GET` | `/dashboard` | `get_dashboard` | Main overview |
| `GET` | `/settings` | `get_settings` | Settings editor |
| `POST` | `/settings/save` | `save_settings` | Save global settings |
| `POST` | `/settings/bot/toggle` | `toggle_bot` | Start/Stop Telegram bot |
| `POST` | `/logout` | `logout` | Destroy session |

### Node Management
| Method | Path | Handler | Description |
|--------|------|---------|-------------|
| `GET` | `/nodes` | `get_nodes` | List all nodes |
| `POST` | `/nodes/install` | `install_node` | Register new node |
| `GET` | `/nodes/:id/edit` | `get_node_edit` | Edit node details |
| `POST` | `/nodes/:id/update` | `update_node` | Save node changes |
| `POST` | `/nodes/:id/activate` | `activate_node` | Enable/Disable node |
| `GET` | `/nodes/:id/script` | `get_node_install_script` | Get installation script |
| `GET` | `/nodes/:id/raw-install` | `get_node_raw_install_script` | Raw script content |
| `POST` | `/nodes/:id/sync` | `sync_node` | Force config sync |
| `GET` | `/nodes/:id/logs` | `node_control::pull_node_logs` | View agent logs |
| `POST` | `/nodes/:id/restart` | `node_control::restart_node_service` | Restart agent/VPN |
| `DELETE` | `/nodes/:id/delete` | `delete_node` | Remove node |
| `POST` | `/api/admin/frontend-servers` | `handlers::frontend::register` | Register new frontend |
    

### Network Configuration (Inbounds)
| Method | Path | Handler | Description |
|--------|------|---------|-------------|
| `GET` | `/nodes/:id/inbounds` | `get_node_inbounds` | List inbounds for node |
| `POST` | `/nodes/:id/inbounds` | `add_inbound` | Create new inbound |
| `GET` | `/nodes/:id/inbounds/:id` | `get_edit_inbound` | Edit inbound form |
| `POST` | `/nodes/:id/inbounds/:id` | `update_inbound` | Save inbound changes |
| `DELETE` | `/nodes/:id/inbounds/:id` | `delete_inbound` | Delete inbound |

### Plans & Users
| Method | Path | Handler | Description |
|--------|------|---------|-------------|
| `GET` | `/plans` | `get_plans` | List service plans |
| `POST` | `/plans/add` | `add_plan` | Create new plan |
| `POST` | `/plans/:id` | `update_plan` | Update plan details |
| `DELETE` | `/plans/:id` | `delete_plan` | Delete plan & **refund active users** |
| `GET` | `/plans/:id/bindings` | `get_plan_bindings` | Manage plan nodes |
| `GET` | `/users` | `get_users` | List users |
| `GET` | `/users/:id` | `get_user_details` | User detail view |
| `POST` | `/users/:id/balance` | `update_user_balance` | Manually adjust balance |
| `POST` | `/users/:id/gift` | `admin_gift_subscription` | Gift a sub to user |

### Subscription Management
| Method | Path | Handler | Description |
|--------|------|---------|-------------|
| `DELETE` | `/users/subs/:id` | `delete_user_subscription` | Terminate subscription |
| `POST` | `/users/subs/:id/refund` | `refund_user_subscription` | Refund to balance |
| `POST` | `/users/subs/:id/extend` | `extend_user_subscription` | Add time/traffic |
| `GET` | `/subs/:id/devices` | `get_subscription_devices` | View connected IPs |

### Store Management
| Method | Path | Handler | Description |
|--------|------|---------|-------------|
| `GET` | `/store/categories` | `categories_page` | Manage categories |
| `POST` | `/store/categories` | `add_category` | Create category |
| `DELETE` | `/store/categories/:id` | `delete_category` | Delete category |
| `GET` | `/store/products` | `products_page` | Manage digital goods |
| `POST` | `/store/products` | `add_product` | Add product |
| `DELETE` | `/store/products/:id` | `delete_product` | Delete product |
| `GET` | `/store/orders` | `orders_page` | View purchase history |

## 💻 CLI Commands

Run via `./exarobot <COMMAND>` or `cargo run -- <COMMAND>`

| Command | Subcommand | Arguments | Description |
|---------|------------|-----------|-------------|
| `serve` | - | - | Start the Web Server and Bot |
| `install` | - | - | Install as systemd service |
| `admin` | `reset-password` | `<username> <new_pass>` | Reset admin credentials |
| `admin` | `info` | - | Show connection info |

## 📄 Inbound Configuration Schema

When adding or updating inbounds via API, the `settings` and `stream_settings` fields expect specific JSON structures.

### Hysteria 2 (Sing-box 1.11+)

**Settings JSON:**
```json
{
  "protocol": "hysteria2",
  "up_mbps": 100,          // Upload limit (Mbps), Integer
  "down_mbps": 100,        // Download limit (Mbps), Integer
  "obfs": {                // Optional Obfuscation
    "type": "salamander",
    "password": "your-obfs-password"
  },
  "masquerade": "/opt/exarobot/apps/panel/assets/masquerade", // Or URL
  "users": []              // Managed automatically by orchestration
}
```

**Stream Settings JSON:**
```json
{
  "network": "udp",
  "security": "tls",
  "tls_settings": {
    "server_name": "drive.google.com",
    "certificates": [
      {
         "certificate_path": "/etc/sing-box/certs/cert.pem",
         "key_path": "/etc/sing-box/certs/key.pem"
      }
    ],
    "alpn": ["h3"]
  }
}
```

### VLESS Reality

**Settings JSON:**
```json
{
  "protocol": "vless",
  "clients": [],           // Managed automatically
  "decryption": "none",
  "fallbacks": []
}
```

**Stream Settings JSON:**
```json
{
  "network": "tcp",
  "security": "reality",
  "reality_settings": {
    "show": false,
    "dest": "drive.google.com:443",
    "server_names": [
      "drive.google.com",
      "www.google.com"
    ],
    "private_key": "YOUR_PRIVATE_KEY",
    "short_ids": ["f6c54d03071d32a3"]
  },
  "tls_settings": null
}
```
