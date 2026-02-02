# 🔄 Auto-Update System Guide

The EXA ROBOT Auto-Update System allows deployed agents to automatically upgrade themselves to the latest version without manual intervention.

## How it Works
1.  **Version Check**: The Agent sends its version in every `Heartbeat` (e.g., `0.2.0`).
2.  **Panel Response**: If a newer version is available (configured in Panel Settings), the Panel includes `latest_version: "0.2.1"` in the response.
3.  **Update Info**: The Agent notices the version mismatch and queries `/api/v2/node/update-info`.
4.  **Download & Verify**: The Agent downloads the new binary from the URL and verifies the SHA256 checksum.
5.  **Self-Replace**: The Agent atomically replaces its own binary executable.
6.  **Restart**: The Agent triggers `systemctl restart exarobot-agent`.

## Configuration (Panel)
To enable updates, the Admin must set the following in **Settings** (App Settings table):

| Key | Value Example | Description |
| :--- | :--- | :--- |
| `agent_latest_version` | `0.2.1` | The target version agents should update to. |
| `agent_update_url` | `https://github.com/user/repo/releases/download/v0.2.1/exarobot-agent` | Direct download URL for the binary. |
| `agent_update_hash` | `a1b2c3d4...` | SHA256 Checksum of the binary. |

## Prerequisities (Agent)
*   The Agent must be running as a systemd service named `exarobot-agent`.
*   The Agent binary must have write permissions to its own file (or run as root).
*   The system must have `systemctl` available.

## Troubleshooting
*   **Logs**: Check agent logs (`journalctl -u exarobot-agent -f`) for "Self-update failed" or "Binary replaced successfully".
*   **Looping**: The agent protects against update loops by strictly checking `target_ver != current_version`. Ensure `agent_latest_version` matches the actual version string inside the new binary.
