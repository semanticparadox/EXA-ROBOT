# Project File Map

**Project Root:** `/Users/smtcprdx/Documents/exarobot`

## 📁 Root Directory
- **`DESCRIPTION.md`**: Project overview, key features, and marketing copy.
- **`README.md`**: Technical documentation, installation guide, and architecture details.
- **`Cargo.toml`**: Workspace configuration definition.
- **`Cargo.lock`**: Exact dependency versions lockfile.

## 📁 docs/
- **`user_guide.md`**: Simple explanation and installation guide.
- **`advanced_features.md`**: Guide for Distributed Frontend, Subscriptions, and AdBlocking.
- **`c4-context.md`**: Architectural diagrams.
- **`api_reference.md`**: Technical API reference.
- **`project_map.md`**: This file.

## 📁 apps/
Contains the main applications of the workspace.

### 📁 apps/panel/
The central control plane (Web App + Telegram Bot).

#### `apps/panel/src/`
- **`main.rs`**: Application entry point. Initializes DB, settings, bot, and HTTP server. Defines API routes.
- **`settings.rs`**: Management of dynamic runtime settings (stored in DB).
- **`ssh.rs`**: SSH helper functions for server management.

#### `apps/panel/src/handlers/`
HTTP Request Handlers for the Admin Panel.
- **`admin.rs`**: General admin routes (Dashboard, Users, Plans, Auth).
- **`admin_network.rs`**: Network-specific routes (Node Inbounds, Plan Bindings).
- **`admin_store.rs`**: Store management routes (Categories, Products, Orders).
- **`node_control.rs`**: Node diagnostics and control (Logs, Restart, Health).
- **`client.rs`**: Client-facing API (AI Routing, Auth).
- **`frontend.rs`**: Distributed Frontend management.

#### `apps/panel/src/services/`
Core Business Logic.
- **`store_service.rs`**: Massive service handling Users, Plans, Subscriptions, and Store interactions.
- **`pay_service.rs`**: Payment processing (CryptoBot, NOWPayments) and Order lifecycle.
- **`orchestration_service.rs`**: Coordinates logic between Database and Network nodes.
- **`connection_service.rs`**: Enforces device limits and connection tracking.
- **`traffic_service.rs`**: Monitors and syncs traffic usage.
- **`monitoring.rs`**: Background tasks for system health.

#### `apps/panel/src/bot/`
Telegram Bot Logic.
- **`mod.rs`**: Main bot dispatcher, message handlers, callback queries, and UI/Keyboards.
- **`bot_manager.rs`**: Manages the bot lifecycle (start/stop) from the web panel.

#### `apps/panel/src/models/`
Database Structs (SQLx).
- **`(Various files)`**: Type definitions mapping to SQLite tables.

#### `apps/panel/src/singbox/`
Sing-box Configuration Generation Module.
- **`mod.rs`**: Module exports.
- **`config.rs`**: Rust structs representing the Sing-box JSON schema (Inbounds, Outbounds, Route, Experimental).
- **`generator.rs`**: Logic to convert Database Inbounds -> Sing-box Config. Handles bandwidth limits, auth formatting, and JSON generation.

#### `apps/panel/migrations/`
- **`001_complete_schema.sql`**: The single source of truth for the database schema.

### 📁 apps/agent/
The lightweight node daemon.

#### `apps/agent/src/`
- **`main.rs`**: Pulls config from Panel, manages Sing-box process, reports stats.

## 📁 libs/
Shared libraries.

### 📁 libs/shared/
- **`src/lib.rs`**: Shared types and DTOs used by both Panel and Agent (ensures type safety across the network).

## 📁 scripts/
Utilities for deployment and maintenance.
- **`install.sh`**: The "One-Line" installer script.
- **`uninstall.sh`**: Cleanup script.
- **`systemd/`**: Service unit files for systemd (`exarobot-panel.service`, `exarobot-agent.service`).

---
**Note:** This map covers the primary source files. Generated files (target/) and git metadata are excluded.
