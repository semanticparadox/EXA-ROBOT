# VPN Control Panel

## Installation

### 1-Click Install
Run this command on your fresh server (Debian 12+ / Ubuntu 22.04+):

```bash
curl -sSL https://raw.githubusercontent.com/semanticparadox/EXA-ROBOT/main/scripts/install.sh | sudo bash
```

**Options:**
- **Standard Install**: Default. Clones source code to `/opt/exarobot/source` for easier debugging/modding.
- **Clean Install**: Adds `--clean`. Builds binaries in a temp folder and installs ONLY the executables. Keeps server clean.
  ```bash
  curl ... install.sh | sudo bash -s -- --clean
  ```

### Manual Arguments
You can skip the interactive menu by passing arguments:
```bash
# Install Panel Only (Clean)
... install.sh | sudo bash -s -- --clean --role panel --domain panel.example.com

# Install Agent Only
... install.sh | sudo bash -s -- --role agent --panel https://panel.example.com --token YOUR_TOKEN
```

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
systemctl restart exarobot-panel
journalctl -u exarobot-panel -f
```

**Agent:**
```bash
systemctl restart exarobot-agent
journalctl -u exarobot-agent -f
```
