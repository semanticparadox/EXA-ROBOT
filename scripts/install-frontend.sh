#!/bin/bash
#
# DEPRECATED: This script has been merged into install.sh
# Use: curl -sSL .../install.sh | sudo bash -s -- --role frontend ...
#

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "⚠️  DEPRECATION NOTICE"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "This script (install-frontend.sh) is deprecated."
echo "Please use the main installer instead:"
echo ""
echo "  curl -sSL {PANEL_URL}/install.sh | \\"
echo "    sudo bash -s -- --role frontend \\"
echo "    --domain <domain> \\"
echo "    --token <token> \\"
echo "    --region <region> \\"
echo "    --panel <panel_url>"
echo ""
echo "Redirecting to new installer in 5 seconds..."
echo "Press Ctrl+C to cancel."
echo ""

sleep 5

# Extract panel URL from script location if possible
SCRIPT_URL="${BASH_SOURCE[0]}"
if [[ "$SCRIPT_URL" =~ ^https?:// ]]; then
    PANEL_URL=$(echo "$SCRIPT_URL" | sed 's|/install-frontend.sh||')
else
    PANEL_URL="https://panel.example.com"
fi

# Redirect to new installer with same arguments
curl -sSL "$PANEL_URL/install.sh" | sudo bash -s -- --role frontend "$@"
