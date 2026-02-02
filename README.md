# VPN Control Panel

## Secure Installation (Recommended)

For maximum security, verify the installer's GPG signature before running:

```bash
# Download installer and signature
curl -sSLO https://raw.githubusercontent.com/semanticparadox/EXA-ROBOT/main/scripts/install.sh
curl -sSLO https://raw.githubusercontent.com/semanticparadox/EXA-ROBOT/main/scripts/install.sh.asc

# Import official GPG key
curl -sSL https://raw.githubusercontent.com/semanticparadox/EXA-ROBOT/main/gpg-key.asc | gpg --import

# Verify signature
gpg --verify install.sh.asc install.sh

# If "Good signature" shown, proceed with installation
sudo bash install.sh
```

**Why verify?**
- ✅ Protects against man-in-the-middle (MITM) attacks
- ✅ Ensures the script hasn't been tampered with
- ✅ Verifies authenticity from official source

---

## Quick Installation

**One-line install** (GPG verification will be skipped, less secure):

```bash
curl -sSL https://raw.githubusercontent.com/semanticparadox/EXA-ROBOT/main/scripts/install.sh | sudo bash
```

### Installation Options

**Standard Install**: Default. Clones source code to `/opt/exarobot/source` for easier debugging/modding.

**Clean Install**: Adds `--clean`. Builds binaries in a temp folder and installs ONLY the executables. Keeps server clean.
```bash
curl -sSL https://raw.githubusercontent.com/semanticparadox/EXA-ROBOT/main/scripts/install.sh | sudo bash -s -- --clean
```

### Manual Arguments

Skip the interactive menu by passing arguments:

```bash
# Install Panel Only (Clean)
sudo bash install.sh --clean --role panel --domain panel.example.com

# Install Agent Only  
sudo bash install.sh --role agent --panel https://panel.example.com --token YOUR_TOKEN

# Install Both (Panel + Agent)
sudo bash install.sh --role both --domain panel.example.com
```

> **Tip:** For one-line piped install, replace `sudo bash install.sh` with the curl command from above.

---

## Updates

To update your installation to the latest version (preserves your config/DB):

```bash
/opt/exarobot/source/scripts/update.sh
# OR if using clean install, download it manually:
curl -sSL https://raw.githubusercontent.com/semanticparadox/EXA-ROBOT/main/scripts/update.sh | sudo bash
```

---

## Configuration Paths
- **Binaries**: `/opt/exarobot/`
- **Panel Config**: `/opt/exarobot/.env`
- **Agent Config**: `/opt/exarobot/.env.agent`
- **Database**: `/opt/exarobot/exarobot.db`

---

## Service Management

**Panel:**
```bash
systemctl restart exarobot
journalctl -u exarobot -f
```

**Agent:**
```bash
systemctl restart exarobot-agent
journalctl -u exarobot-agent -f
```

---

## Frontend Module Deployment

Deploy geographically distributed frontend servers to improve censorship resistance:

**Option 1: Secure Installation (Recommended)**
```bash
# Download and verify
curl -sSLO https://raw.githubusercontent.com/semanticparadox/EXA-ROBOT/main/scripts/install.sh
curl -sSLO https://raw.githubusercontent.com/semanticparadox/EXA-ROBOT/main/scripts/install.sh.asc
gpg --verify install.sh.asc install.sh

# Run with frontnd role
sudo bash install.sh --role frontend \
  --domain frontend-eu.example.com \
  --token <AUTH_TOKEN_FROM_PANEL> \
  --region eu
```

**Option 2: Quick Install**
```bash
curl -sSL https://raw.githubusercontent.com/semanticparadox/EXA-ROBOT/main/scripts/install.sh | \
  sudo bash -s -- --role frontend \
  --domain frontend-eu.example.com \
  --token <AUTH_TOKEN_FROM_PANEL> \
  --region eu
```

> **Note:** The old `install-frontend.sh` script is deprecated. Use `install.sh --role frontend` instead.

**Architecture:**
- Frontend servers host the Telegram Mini App and user-facing assets
- Automatically register with the main Panel
- Use Caddy for automatic HTTPS
- Ideal for multi-region deployments to bypass regional blocks

---

## Environment Variables

### Panel (`.env`)
```bash
SERVER_DOMAIN=panel.example.com    # Your domain
ADMIN_PATH=/admin                  # Admin panel path
PANEL_PORT=3000                    # HTTP port
DATABASE_URL=sqlite://exarobot.db  # Database location
BOT_TOKEN=                         # Telegram Bot Token
PAYMENT_API_KEY=                   # Payment provider key
NOWPAYMENTS_KEY=                   # NowPayments API key
```

### Agent (`.env.agent`)
```bash
PANEL_URL=https://panel.example.com  # Panel URL
NODE_TOKEN=                          # Node authentication token
CONFIG_PATH=/etc/sing-box/config.json # Sing-box config path
```

**See also:** `.env.panel.example` and `.env.agent.example` for full templates.
