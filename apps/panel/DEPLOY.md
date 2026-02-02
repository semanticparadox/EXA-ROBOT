# 🚀 EXA ROBOT Production Deployment Guide

This guide details how to build and deploy the EXA ROBOT Panel for production use, ensuring maximum performance, security, and stability.

## 🏗️ Build Optimization

The `Cargo.toml` has been configured with a high-performance `[profile.release]`:
- **LTO (Link Time Optimization)**: Enabled (`true`) for smaller binaries and better runtime optimization.
- **Codegen Units**: Set to `1` to maximize optimization (trades build time for runtime speed).
- **Strip**: Enabled (`true`) to reduce binary size by removing symbols.
- **Panic**: `abort` for smaller binary size (no unwinding).

### Manual Build
```bash
cd apps/panel
cargo build --release
```
Artifact: `target/release/exarobot-panel`

## 🐳 Docker Deployment (Recommended)

The included `Dockerfile` uses a multi-stage process to create a minimal image (~80MB compressed).

### 1. Build Image
```bash
docker build -t exarobot-panel:latest -f apps/panel/Dockerfile .
```

### 2. Run with Docker Compose
Ensure you have `docker-compose.yml` set up (Phase 2 artifact).

```yaml
version: '3.8'
services:
  panel:
    image: exarobot-panel:latest
    restart: always
    ports:
      - "3000:3000"
    volumes:
      - ./data:/data
    environment:
      - PANEL_URL=https://your-domain.com
      - BOT_TOKEN=your_bot_token
      - DATABASE_URL=sqlite:///data/exarobot.db
```

```bash
docker-compose up -d
```

## 🌍 Geo-Optimization & Latency

The panel now supports **Auto SNI Rotation** and **Smart Server Selection**.

1. **GeoIP**: The panel automatically fetches Node coordinates upon heartbeat using `ip-api.com`.
2. **Smart Sort**: Client endpoints (`/api/client/servers`) automatically sort VPN nodes by distance to the user's IP.
   - *Note*: Ensure your load balancer/proxy passes real user IPs via `X-Forwarded-For`.

## 🛡️ Security Checklist for Production

- [ ] **SSL/TLS**: Use Nginx or Traefik as a reverse proxy with Let's Encrypt.
- [ ] **Firewall**: proper UFW rules (allow 22, 80, 443, block others).
- [ ] **Backups**: Periodically backup `/data/exarobot.db`.
- [ ] **SSH Keys**: Ensure `id_rsa` / `id_rsa.pub` are secure in the container (used for node management if applicable).

## 🔄 Maintenance

### Updating
To update the panel:
1. `git pull`
2. `docker-compose build --no-cache`
3. `docker-compose up -d`

### Logs
```bash
docker-compose logs -f panel
```
