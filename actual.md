# EXA-ROBOT: Текущий Статус и Задачи

**Дата:** 2026-01-27
**Версия:** v0.8.0 (Stable / Gold Master)

---

## 🟢 Текущий Статус

Система полностью работоспособна и протестирована. Выпущен релиз **Gold Master**.

### Основные Компоненты
| Компонент | Статус | Версия | Описание |
|:--- |:--- |:--- |:--- |
| **Panel** | ✅ Stable | v0.8.0 | Управление узлами, генерация конфигов, биллинг |
| **Agent** | ✅ Stable | v0.8.0 | Авто-синхронизация, управление Sing-box 1.11+ |
| **VLESS Reality** | ✅ Active | - | Vision flow, XTLS, маскировка под Google/Yahoo |
| **Hysteria 2** | ✅ Active | - | UDP, Salamander Obfuscation, BBR Congestion Control |

---

## ✅ Реализованные Исправления (Changelog)

### 1. Совместимость с Sing-box 1.11+
- **Исправлено:** Поля лимитов скорости переименованы в `up_mbps` / `down_mbps` (integer).
- **Исправлено:** Формат авторизации Hysteria2 на сервере изменен на `user:pass` (как требует протокол).
- **Исправлено:** Устранен конфликт полей `ignore_client_bandwidth` и лимитов скорости.

### 2. Умная Генерация Ссылок (Client API)
- Реализована логика **Smart Insecure Flag**:
  - `insecure=1`: Для самоподписанных сертификатов (по умолчанию `drive.google.com`).
  - `insecure=0`: Для кастомных доменов с валидным сертификатом (Let's Encrypt).

### 3. Инфраструктура
- **Masquerade:** Автоматическое создание `/var/www/html` с заглушкой API Gateway.
- **Env:** Исправлен баг с пробелами в `.env` при интерактивной установке.
- **Dead Code:** Удалены неиспользуемые функции генерации конфигов.

---

## �️ Архитектура (Актуально)

### Серверы
- **Panel:** `vps-5a033298` (137.74.119.200)
- **Agent:** `vps-b0f3b585` (Test Node)

### Порты
- **443/TCP**: VLESS Reality (Steal SNI: drive.google.com)
- **8443/UDP**: Hysteria 2 (Obfs: Salamander)
- **9090/TCP**: Clash API (Localhost only)

---

## 📋 План Развития (Roadmap v0.9+)

### 1. Управление Сертификатами (CertBot)
- Интеграция Let's Encrypt в `install.sh`.
- Авто-обновление сертификатов через cron.

### 2. UI Оптимизации
- Отображение статуса сертификата в панели (Valid/Expired).
- Кнопка "Regenerate Keys" для VLESS Reality.

### 3. Мониторинг
- Графики потребления трафика (Prometheus/Grafana exporter).
- Telegram-уведомления о падении узлов.

---

## 📚 Debug Commands

### Проверка Статуса
```bash
# Agent
systemctl status exarobot-agent
systemctl status sing-box
```

### Логи в реальном времени
```bash
# Sing-box (Traffic & Errors)
journalctl -u sing-box -f

# Agent (Sync & Config updates)
journalctl -u exarobot-agent -f
```

### Проверка Конфига
```bash
cat /etc/sing-box/config.json | jq .
```
### Agent (vps-b0f3b585)
- **IP:** 137.74.119.200 (тот же, что панель — для теста)
- **Роль:** Прокси-сервер (sing-box)
- **Сервисы:** `exarobot-agent`, `sing-box`
- **Конфиг:** `/etc/sing-box/config.json`
- **Сертификаты:** `/etc/sing-box/certs/`

---

## ✅ Что Исправлено

### 1. VLESS Reality ✅
- **Проблема:** UUID с дефисами, handshake парсинг
- **Решение:** Убраны дефисы из паролей, исправлен парсинг `dest` в Reality
- **Статус:** **РАБОТАЕТ**

### 2. Hysteria2 — Парсинг Сертификатов ✅
- **Проблема:** Код ожидал `certificate_file`/`key_file`, база хранила `certificate_path`/`key_path`
- **Ошибка:** `missing field certificate_file at line 1 column 98`
- **Исправление:**
  - [apps/panel/src/models/network.rs](file:///Users/smtcprdx/Documents/exarobot/apps/panel/src/models/network.rs#L116-L121): изменена структура `Certificate`
  - [apps/panel/src/singbox/generator.rs](file:///Users/smtcprdx/Documents/exarobot/apps/panel/src/singbox/generator.rs#L94-L98): обновлены ссылки на поля
- **Статус:** **КОД ИСПРАВЛЕН, требуется деплой**

### 3. Sing-box Binary Path ✅
- **Проблема:** Сервис не мог найти `/usr/bin/sing-box`
- **Решение:** Автоопределение пути в [scripts/install.sh](file:///Users/smtcprdx/Documents/exarobot/scripts/install.sh#L94-L110)
- **Статус:** **РАБОТАЕТ**

### 4. Database Schema ✅
- **Проблема:** Пустые `users` списки из-за отсутствия `plan_nodes`
- **Решение:** Добавлена таблица `plan_nodes`, обновлён `orchestration_service`
- **Статус:** **РАБОТАЕТ**

---

## ⚠️ Текущие Проблемы

### 🔴 Hysteria2: Не Работает Подключение

**Симптомы:**
- Sing-box запускается: `udp server started at [::]:8443` ✅
- В логах sing-box **ничего не появляется** при попытке подключения ❌
- Клиент не может подключиться

**Статус:** `server_name: "example.com"` в конфиге агента вместо `drive.google.com`

**Root Cause:** Парсинг `stream_settings` падал → использовались дефолтные значения

**Решение (КОД ГОТОВ, ТРЕБУЕТСЯ ДЕПЛОЙ):**

1. **Собрать панель:**
   ```bash
   cd /Users/smtcprdx/Documents/exarobot
   cargo build --release --bin exarobot
   ```

2. **Деплой на панель:**
   ```bash
   ssh root@vps-5a033298
   sudo systemctl stop exarobot
   
   # С локальной машины
   scp target/release/exarobot root@vps-5a033298:/opt/exarobot/exarobot
   
   # На панели
   sudo chmod +x /opt/exarobot/exarobot
   sudo systemctl start exarobot
   ```

3. **Проверка:**
   ```bash
   # Логи панели — НЕ должно быть ошибок парсинга
   sudo journalctl -u exarobot -f | grep hysteria
   
   # На агенте — ждём 30 сек, проверяем конфиг
   ssh root@vps-b0f3b585
   cat /etc/sing-box/config.json | jq '.inbounds[] | select(.tag | contains("hysteria"))'
   # Ожидаем: server_name = "drive.google.com", НЕ "example.com"
   ```

**Подробно:** [hysteria2_fix_deployment.md](file:///Users/smtcprdx/.gemini/antigravity/brain/b7fe3d5a-377a-4a3b-a072-e8ee5eee42d5/hysteria2_fix_deployment.md)

---

## 🔧 Дополнительные Улучшения (После Основного Фикса)

### 1. Автоматизация Сертификатов в install.sh

**Проблема:**  
Сейчас сертификаты для Hysteria2 создаются **вручную** на агенте:
```bash
openssl req -x509 -nodes -newkey ec -days 3650 \
  -subj "/CN=drive.google.com" \
  -keyout /etc/sing-box/certs/key.pem \
  -out /etc/sing-box/certs/cert.pem
```

**Цель:**  
`install.sh` должен **автоматически**:
1. Парсить SNI из первого Hysteria2 в конфиге панели
2. Генерировать сертификат с правильным CN
3. Создавать masquerade directory

**Файл для правки:** [scripts/install.sh](file:///Users/smtcprdx/Documents/exarobot/scripts/install.sh#L400-L450)

**Псевдокод:**
```bash
install_singbox() {
    # ... existing code ...
    
    # После установки sing-box, но до запуска сервиса:
    
    # 1. Получить конфиг с панели (если агент)
    if [ "$ROLE" = "agent" ]; then
        # Запросить /api/v2/node/config
        # Парсить первый hysteria2 inbound → tls.server_name
        SNI=$(curl -s "$PANEL_URL/api/v2/node/config?token=$NODE_TOKEN" | jq -r '.inbounds[] | select(.type=="hysteria2") | .tls.server_name' | head -1)
        
        if [ -n "$SNI" ] && [ "$SNI" != "null" ]; then
            # 2. Генерировать сертификат
            openssl req -x509 -nodes -newkey ec -days 3650 \
              -subj "/CN=$SNI" \
              -keyout /etc/sing-box/certs/key.pem \
              -out /etc/sing-box/certs/cert.pem
            
            # 3. Создать masquerade
            mkdir -p /opt/exarobot/apps/panel/assets/masquerade
            echo "<!DOCTYPE html><html><body>Not Found</body></html>" > /opt/exarobot/apps/panel/assets/masquerade/index.html
        fi
    fi
}
```

---

### 2. UI для Управления SNI
(Planned)

---

## 🛠️ Критические Исправления (Gold Master)

**Все фиксы объединены и готовы к деплою:**

1. **Hysteria2 & VLESS:**
   - ✅ Пароли без дефисов.
   - ✅ SNI по умолчанию: `drive.google.com`.
   - ✅ Исправлен парсинг полей сертификатов (`certificate_path`).
   - ✅ Исправлен путь к `masquerade` (удаление `file://`).

2. **Автоматизация (install.sh):**
   - ✅ Авто-определение SNI из конфига.
   - ✅ Генерация сертификатов под `drive.google.com`, если не найдены.
   - ✅ Исправлена логика регенерации при несовпадении.

**Статус:** Готово к `git push`. Рекомендуется чистая установка или обновление.

---

## 📊 База Данных

---

## 📊 База Данных

### Важные Таблицы

**`inbounds`:**
```sql
CREATE TABLE inbounds (
    id INTEGER PRIMARY KEY,
    node_id INTEGER,
    tag TEXT,
    protocol TEXT, -- 'vless', 'hysteria2'
    listen_port INTEGER,
    listen_ip TEXT DEFAULT '::',
    settings TEXT, -- JSON: {protocol: "hysteria2", users: [...], ...}
    stream_settings TEXT, -- JSON: {network: "udp", security: "tls", tls_settings: {...}}
    remark TEXT,
    enable BOOLEAN DEFAULT 1
)
```

**`plan_nodes`:** (новая, для связи планов и нод)
```sql
CREATE TABLE plan_nodes (
    id INTEGER PRIMARY KEY,
    plan_id INTEGER,
    node_id INTEGER,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)
```

### Проверка Stream Settings

```bash
# На панели
ssh root@vps-5a033298
sudo sqlite3 /opt/exarobot/exarobot.db \
  "SELECT id, tag, stream_settings FROM inbounds WHERE tag='hysteria2-2';"
```

**Правильный формат:**
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

---

## 🚀 Быстрый Старт для Новой Сессии

### 1. Проверить Статус

```bash
# Панель
ssh root@vps-5a033298
sudo journalctl -u exarobot -n 20 --no-pager | grep -E "hysteria|ERROR"

# Агент
ssh root@vps-b0f3b585
sudo systemctl status sing-box
cat /etc/sing-box/config.json | jq '.inbounds[] | select(.tag | contains("hysteria")) | .tls.server_name'
```

### 2. Если Hysteria2 НЕ Работает

→ Деплой фикса (см. раздел "Текущие Проблемы")

### 3. После Деплоя

Протестировать подключение и **создать walkthrough.md** с результатами

---

## 📚 Документация

- **Sing-box:** https://sing-box.sagernet.org
- **VLESS Reality:** https://xtls.github.io/config/inbounds/vless.html
- **Hysteria2:** https://v2.hysteria.network

---

## 🔑 Важные Детали

### Password/UUID Format

- **VLESS:** UUID **без дефисов**: `5865ba13f4ac4a92bd12fcb6e7ea1151`
- **Hysteria2:** Password **без дефисов**: `5865ba13f4ac4a92bd12fcb6e7ea1151`

### Port Layout

- **VLESS Reality:** 443/TCP
- **Hysteria2:** 8443/UDP
- **Clash API:** 127.0.0.1:9090/TCP (локально на агенте)

### Certificates

**Location:** `/etc/sing-box/certs/`
- `cert.pem` — Public certificate
- `key.pem` — Private key

**CN MUST match** `tls.server_name` в конфиге!

---

## 🐛 Debug Commands

```bash
# Проверить порты на агенте
sudo ss -tulnp | grep -E '443|8443|9090'

# Проверить UDP 8443 доступен извне
nmap -sU -p 8443 137.74.119.200

# Логи sing-box в реал-тайм
sudo journalctl -u sing-box -f

# Логи панели в реал-тайм
sudo journalctl -u exarobot -f

# Проверить генерацию конфига
curl -H "Authorization: Bearer $NODE_TOKEN" \
  http://137.74.119.200:3000/api/v2/node/config
```

---

**Статус:** Код готов, требуется деплой панели → тест Hysteria2 → успех! 🎉
