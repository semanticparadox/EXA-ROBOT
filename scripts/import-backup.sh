#!/bin/bash
#
# ExaRobot Panel Backup Import Script
# Restores database and configuration from exported backup
#

set -e

BACKUP_FILE="$1"
INSTALL_DIR="/opt/exarobot"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${GREEN}ExaRobot Backup Import${NC}"
echo "=========================="
echo ""

# Validate arguments
if [ -z "$BACKUP_FILE" ]; then
    echo -e "${RED}Error: No backup file specified${NC}"
    echo ""
    echo "Usage: $0 <exarobot_backup_YYYYMMDD_HHMMSS.tar.gz>"
    echo ""
    echo "Example:"
    echo "  $0 exarobot_backup_20260201_190500.tar.gz"
    exit 1
fi

if [ ! -f "$BACKUP_FILE" ]; then
    echo -e "${RED}Error: Backup file not found: $BACKUP_FILE${NC}"
    exit 1
fi

# Check root permissions
if [ "$EUID" -ne 0 ]; then
    echo -e "${YELLOW}Warning: Not running as root. Some operations may fail.${NC}"
    echo "Consider running: sudo $0 $BACKUP_FILE"
    echo ""
fi

echo "📦 Backup file: $BACKUP_FILE"
echo ""

# Create temp directory for extraction
TEMP_DIR=$(mktemp -d)
echo "🔄 Extracting backup..."
tar xzf "$BACKUP_FILE" -C "$TEMP_DIR"

# Find extracted directory
EXTRACT_DIR=$(find "$TEMP_DIR" -maxdepth 1 -type d -name "exarobot_export_*" | head -1)

if [ -z "$EXTRACT_DIR" ]; then
    echo -e "${RED}Error: Invalid backup archive structure${NC}"
    rm -rf "$TEMP_DIR"
    exit 1
fi

echo "✅ Archive extracted"
echo ""

# Verify backup contents
if [ ! -f "$EXTRACT_DIR/exarobot.db" ]; then
    echo -e "${RED}Error: Database file missing in backup${NC}"
    rm -rf "$TEMP_DIR"
    exit 1
fi

# Backup existing database
if [ -f "$INSTALL_DIR/exarobot.db" ]; then
    BACKUP_NAME="exarobot.db.backup.$(date +%s)"
    echo "📦 Backing up existing database..."
    echo "   $INSTALL_DIR/$BACKUP_NAME"
    cp "$INSTALL_DIR/exarobot.db" "$INSTALL_DIR/$BACKUP_NAME"
    echo "✅ Existing database backed up"
    echo ""
fi

# Restore database
echo "💾 Restoring database..."
cp "$EXTRACT_DIR/exarobot.db" "$INSTALL_DIR/exarobot.db"

# Verify database integrity
if command -v sqlite3 &> /dev/null; then
    echo "🔍 Verifying database integrity..."
    if sqlite3 "$INSTALL_DIR/exarobot.db" "PRAGMA integrity_check;" | grep -q "ok"; then
        echo "✅ Database integrity: OK"
    else
        echo -e "${YELLOW}⚠️  Database integrity check returned warnings${NC}"
    fi
else
    echo "ℹ️  sqlite3 not installed - skipping integrity check"
fi

echo ""

# Show environment configuration
if [ -f "$EXTRACT_DIR/env_sanitized.txt" ]; then
    echo "⚙️  Environment Configuration (Sanitized):"
    echo "=========================================="
    cat "$EXTRACT_DIR/env_sanitized.txt"
    echo "=========================================="
    echo ""
    echo -e "${YELLOW}⚠️  IMPORTANT: Merge the above with $INSTALL_DIR/.env${NC}"
    echo ""
    echo "   Sensitive keys were REDACTED and need to be added back:"
    echo "   - BOT_TOKEN"
    echo "   - PAYMENT_API_KEY"
    echo "   - NOWPAYMENTS_KEY"
    echo "   - SESSION_SECRET"
    echo ""
fi

# Show README if available
if [ -f "$EXTRACT_DIR/README.txt" ]; then
    echo "📖 Restoration Instructions:"
    echo "============================"
    cat "$EXTRACT_DIR/README.txt"
    echo ""
fi

# Cleanup
rm -rf "$TEMP_DIR"

echo ""
echo -e "${GREEN}✅ Database restored successfully!${NC}"
echo ""
echo "Next steps:"
echo "  1. Review and merge env_sanitized.txt with $INSTALL_DIR/.env"
echo "  2. Add back sensitive credentials (BOT_TOKEN, API keys)"
echo "  3. Restart panel: systemctl restart exarobot"
echo "  4. Verify: journalctl -u exarobot -f"
echo ""
