# Function Map

This document maps the core business logic functions within the `services` module and critical helpers in the `bot`.

## 📦 Store Service (`services/store_service.rs`)
Handles database interactions for users, plans, and subscriptions.

| Function | Parameters | Returns | Description |
|----------|------------|---------|-------------|
| `upsert_user` | `tg_id`, `username`, `full_name`, `referrer_id` | `Result<User>` | Creates or updates a Telegram user. |
| `get_active_plans` | - | `Result<Vec<Plan>>` | Fetches plans enabled for sale. |
| `create_subscription` | `user_id`, `plan_id`, `duration_days` | `Result<Subscription>` | Creates a new subscription record. |
| `transfer_subscription` | `sub_id`, `from_user_id`, `to_username` | `Result<()>` | Moves a subscription to another user. |
| `extend_subscription` | `sub_id`, `days` | `Result<()>` | Adds days to an existing subscription. |
| `redeem_gift_code` | `user_id`, `code` | `Result<Subscription>` | Activates a gift code for a user. |

## 💰 Pay Service (`services/pay_service.rs`)
Manages payment providers and order processing.

| Function | Parameters | Returns | Description |
|----------|------------|---------|-------------|
| `create_cryptobot_invoice` | `user_id`, `amount`, `type` | `Result<String>` | Generates a payment link via CryptoBot. |
| `create_nowpayments_invoice` | `user_id`, `plan_id` | `Result<String>` | Generates a payment link via NOWPayments. |
| `process_webhook` | `provider`, `payload` | `Result<()>` | Handles incoming payment notifications. |
| `fulfill_order` | `order_id` | `Result<()>` | Activates services after successful payment. |

## 🌐 Orchestration Service (`services/orchestration_service.rs`)
Syncs logic between the DB and the Network.

| Function | Parameters | Returns | Description |
|----------|------------|---------|-------------|
| `assign_node_to_user` | `user_id` | `Result<Node>` | Selects the best available node for a user. |
| `sync_node_config` | `node_id` | `Result<()>` | Triggers a config push (or pull flag) for a node. |
| `generate_vless_uuid` | - | `String` | Creates a robust UUID for VLESS. |

## 🤖 Bot Handlers (`bot/mod.rs`)
Event-driven handlers for Telegram updates.

| Handler/Match | Trigger | Action |
|---------------|---------|--------|
| `/start <args>` | Command | Registers user, handles referrals, shows main menu. |
| `📦 Digital Store` | Text | Displays product categories. |
| `🛍 Buy Subscription` | Text | Shows active VPN plans. |
| `🔐 My Services` | Text | Lists user subscriptions with status and stats. |
| `👤 My Profile` | Text | Shows balance and ID. |
| `callback: buy_dur_X` | Button | Initiates purchase flow for a specific duration. |
| `callback: pay_cryptobot` | Button | Generates CryptoBot invoice. |

## 🖥️ Node Manager (`node_manager.rs`)
| Function | Parameters | Returns | Description |
|----------|------------|---------|-------------|
| `heartbeat` | `node_id`, `stats` | `Result<()>` | Updates node last_seen and resource usage. |
| `get_node_config` | `node_token` | `Result<Config>` | Returns the full Sing-box JSON for the agent. |
