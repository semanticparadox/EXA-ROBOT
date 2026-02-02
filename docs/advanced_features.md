# 🚀 EXA-ROBOT Advanced Features Guide

This guide covers advanced deployment scenarios and power-user features.

## 1. 🛡️ Distributed Frontend Deployment (Anti-Censorship)
To protect your main Panel from being blocked, you can deploy a lightweight "Frontend" on a separate server.
*   **Concept**: Users visit `frontend.com` (Frontend). It forwards requests to `panel.com` (Backend) behind the scenes.
*   **Benefit**: If `frontend.com` is blocked, you just spin up a new cheap VPS in 2 minutes. The main Panel (user database) stays safe.

### Installation
On a **fresh** server (separate from your Panel), run:

```bash
./scripts/install.sh --role frontend \
  --domain frontend.your-domain.com \
  --panel-url https://panel.your-main-domain.com \
  --token YOUR_ADMIN_TOKEN \
  --region "us-east"
```

*Note: You get the `auth_token` from the Admin Panel -> Settings -> Frontend Nodes.*

---

## 2. 🔗 Subscription Links (Clash, Hiddify, V2Ray)
EXA-ROBOT supports universal subscription formats. Users don't need to manually configure IPs.

### How to use
1.  **Telegram Bot**:
    *   User sends `/start`.
    *   Bot replies with a "Subscribe" button.
    *   User clicks it to get their personal Subscription URL.

2.  **Supported Clients**:
    *   **iOS**: Stash, Shadowrocket, FoXray.
    *   **Android**: Hiddify Next, v2rayNG, Sing-box.
    *   **Windows/Mac**: Hiddify, Clash Verge.

### Formats Provided
*   **Clash Meta (YAML)**: Fully configured with rule sets (AdBlock, Regional Routing).
*   **Sing-box (JSON)**: Native format for highest performance.
*   **V2Ray (Base64)**: Universal fallback for older clients.

---

## 3. 🚫 DNS Ad-Blocking & Privacy Protection
You can enforce ad-blocking on the server side, so users consume less data and see fewer ads.

### How it works
The `sing-box` agent intercepts all DNS requests.
*   **Ads/Trackers**: Blocked immediately (return 0.0.0.0).
*   **Adult Content**: Optional Family Mode.

### Configuration
Go to **Admin Panel** -> **Nodes** -> **Edit Node** -> **DNS Policies**.
*   ✅ **Block Ads**: Enabled by default.
*   ✅ **Block Trackers**: Enabled by default.

### Impact
*   Faster browsing for users (less junk loaded).
*   Privacy protection (trackers can't "phone home").
