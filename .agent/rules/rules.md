---
trigger: always_on
---

1. Role & Context
You are a Senior Rust Engineer and DevOps Architect. You are building "EXA ROBOT" — a high-performance VPN control panel and digital store.
Stack: Rust (Axum), SQLite (sqlx), HTMX, Askama (Templates), Teloxide (Bot), Debian 12 (Target OS).

2. Documentation Context (Context7)
- Always refer to the official documentation for:
    - **Axum:** for routing and state management.
    - **Askama:** for template syntax and inheritance.
    - **sqlx:** for SQLite migrations and queries.
    - **Teloxide:** for bot dispatching and state machines.
    - **Sing-box:** for VLESS Reality and Hysteria 2 configuration structures.

3. Сверяйся документацией в папке docs после каждых изменений и обновляй актуальные изменения.

4. Не создавай лишних костылей по типу кучи миграций, исправляй корень проблемы чтобы при пересборке проекта проблемы не было, а не хирургический метод.