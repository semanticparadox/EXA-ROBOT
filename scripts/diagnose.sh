#!/bin/bash
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${YELLOW}=== EXAROBOT DEEP DIAGNOSTIC TOOL ===${NC}"
echo "Date: $(date)"
echo "Hostname: $(hostname)"
echo ""

check_cmd() {
    if command -v "$1" &> /dev/null; then
        echo -e "[${GREEN}OK${NC}] Command '$1' found"
    else
        echo -e "[${RED}FAIL${NC}] Command '$1' NOT found"
    fi
}

echo "--- 1. System Checks ---"
check_cmd "sing-box"
check_cmd "ufw"
check_cmd "openssl"

echo ""
echo "--- 2. Service Status ---"
systemctl status sing-box --no-pager | grep -E "Active:|Main PID|Status"
systemctl status exarobot-agent --no-pager | grep -E "Active:|Main PID|Status"

echo ""
echo "--- 3. Network Ports (Listening) ---"
if command -v ss &> /dev/null; then
    echo "TCP Ports:"
    ss -tulpn | grep "sing-box"
    echo "UDP Ports:"
    ss -uulpn | grep "sing-box"
else
    netstat -tulpn | grep "sing-box"
fi

echo ""
echo "--- 4. Firewall (UFW) ---"
if command -v ufw &> /dev/null; then
    ufw status | grep -E "443|8443|2053|9090"
else
    echo "UFW not found or not active"
fi

echo ""
echo "--- 5. Certificate Verification ---"
CERT_DIR="/etc/sing-box/certs"
if [ -d "$CERT_DIR" ]; then
    ls -l $CERT_DIR
    if [ -f "$CERT_DIR/cert.pem" ]; then
        echo "Certificate details:"
        openssl x509 -in "$CERT_DIR/cert.pem" -noout -subject -dates -fingerprint
    else
        echo -e "${RED}cert.pem MISSING${NC}"
    fi
else
    echo -e "${RED}Certificate directory $CERT_DIR MISSING${NC}"
fi

echo ""
echo "--- 6. Config Validation ---"
CONFIG_FILE="/etc/sing-box/config.json"
if [ -f "$CONFIG_FILE" ]; then
    echo "Checking config syntax..."
    sing-box check -c "$CONFIG_FILE"
    
    echo "Dumping Config Structure (Sanitized IDs):"
    # Show structure but hide UUIDs roughly
    cat "$CONFIG_FILE" | grep -v "uuid" | grep -v "password" | head -n 50
else
    echo -e "${RED}Config file $CONFIG_FILE MISSING${NC}"
fi

echo ""
echo "--- 7. Recent Logs (Errors) ---"
echo "Sing-box Logs:"
journalctl -u sing-box -n 20 --no-pager
echo ""
echo "Agent Logs (Config Generation):"
journalctl -u exarobot-agent -n 20 --no-pager | grep -i "error"

echo ""
echo -e "${YELLOW}=== END OF DIAGNOSTICS ===${NC}"
