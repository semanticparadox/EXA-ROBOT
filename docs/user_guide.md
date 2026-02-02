# 📖 EXA-ROBOT User Guide

## 🤔 What is this project?
EXA-ROBOT is a smart **VPN Management System**.
Think of it like a remote control for your own private internet service provider. It allows you to create VPN servers, sell subscriptions to users, and manage everything via a Telegram Bot.

## 🧱 Key Modules (How it works)

The system consists of two main parts:

### 1. The Panel (The Brain) 🧠
*   **Location**: `apps/panel`
*   **What it does**: This is the central command center. It hosts the Database, the Website, and the Telegram Bot.
*   **Function**: It decides who can connect, how much they pay, and which server they should use.
*   **Tech**: Built with Rust (Axum), it's very fast and secure.

### 2. The Agent (The Muscle) 💪
*   **Location**: `apps/agent`
*   **What it does**: This runs on your remote VPS servers (the nodes).
*   **Function**: It actually handles the VPN traffic (using Sing-box). It connects back to the **Panel** to ask "Is this user allowed?" and reports health stats (CPU, RAM).
*   **Smart Features**: It generates "Decoy Traffic" to look like a normal web server to censors.

---

## ✨ Key Features (Simple English)

### 🌍 AI Route Optimization
*   **Problem**: Users don't know which server is fastest.
*   **Solution**: The system checks your location and recommends the best server based on distance and how busy it is. (Like Google Maps for VPNs).

### 🕵️ Decoy Traffic
*   **Problem**: Censors (like RKN) can see you are running a VPN because the server is too quiet or only sends encrypted data.
*   **Solution**: The Agent pretends to visit Google/Netflix in the background, making it look like a regular home computer.

### 🛑 Emergency Kill Switch
*   **Problem**: If the Panel goes down, you lose control of who is connecting.
*   **Solution**: If an Agent can't talk to the Panel for 5 minutes, it automatically turns off the VPN to prevent unauthorized access.

### 🛡️ Bandwidth Shaping
*   **Problem**: One user downloading torrents slows down everyone else.
*   **Solution**: The system identifies torrent traffic and slows it down, keeping the internet fast for web browsing and YouTube.

---

## 🚀 How to Install

We have a single script that does everything.

### 1. Requirements
*   A server (VPS) with Ubuntu 20.04 or higher.
*   A domain name (e.g., `vpn.example.com`).

### 2. Installation Command
Access your server via SSH and run:

```bash
# To install the Panel (The Brain)
./scripts/install.sh --role panel --domain vpn.example.com --email you@example.com

# To install an Agent (The Muscle)
# (get the TOKEN from your Panel after installing it)
./scripts/install.sh --role agent --panel-url https://vpn.example.com --token YOUR_TOKEN
```

## 📚 About the Docs Folder
*   `c4-context.md`: A high-level diagram of how everything connects. (New & Accurate)
*   `advanced_features.md`: Guide for Distributed Frontend, Subscriptions, and AdBlocking. (Power User)
*   `api_reference.md`: Technical details for programmers. (Reference)
*   `project_map.md`: A map of source code files. (Reference)
*   `readiness_report.md`: An older status report about V0.8.0. (Outdated)
