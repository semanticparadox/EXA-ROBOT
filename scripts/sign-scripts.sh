#!/bin/bash
#
# GPG Key Generation and Script Signing Guide
# For EXA-ROBOT Installation Scripts
#

set -e

echo "🔐 EXA-ROBOT GPG Signing Setup"
echo "================================"
echo ""

# Configuration
KEY_NAME="EXA-ROBOT Release Team"
KEY_EMAIL="release@your-domain.com"
KEY_COMMENT="Official install script signing key"
KEY_EXPIRE="1y"  # 1 year expiration

# Step 1: Check if GPG is installed
if ! command -v gpg &> /dev/null; then
    echo "❌ GPG not found. Installing..."
    if [ -f /etc/debian_version ]; then
        apt-get update && apt-get install -y gnupg
    elif [ -f /etc/redhat-release ]; then
        yum install -y gnupg
    else
        echo "Please install GPG manually"
        exit 1
    fi
fi

# Step 2: Generate GPG key if it doesn't exist
if ! gpg --list-keys "$KEY_EMAIL" &> /dev/null; then
    echo "📝 Generating new GPG key..."
    echo "   Name: $KEY_NAME"
    echo "   Email: $KEY_EMAIL"
    echo ""
    
    # Create key generation batch file
    cat > /tmp/gpg-gen.txt <<EOF
%echo Generating EXA-ROBOT GPG key
Key-Type: RSA
Key-Length: 4096
Subkey-Type: RSA
Subkey-Length: 4096
Name-Real: $KEY_NAME
Name-Comment: $KEY_COMMENT
Name-Email: $KEY_EMAIL
Expire-Date: $KEY_EXPIRE
%no-protection
%commit
%echo Done
EOF
    
    gpg --batch --generate-key /tmp/gpg-gen.txt
    rm /tmp/gpg-gen.txt
    
    echo "✅ GPG key generated"
else
    echo "✅ GPG key already exists"
fi

# Step 3: Export public key
echo ""
echo "📤 Exporting public key..."
gpg --armor --export "$KEY_EMAIL" > gpg-key.asc
echo "✅ Public key saved to: gpg-key.asc"

# Step 4: Sign installation scripts
echo ""
echo "✍️  Signing installation scripts..."

for script in install.sh install-frontend.sh; do
    if [ -f "scripts/$script" ]; then
        # Create detached signature
        gpg --armor --detach-sign --default-key "$KEY_EMAIL" \
            --output "scripts/${script}.asc" "scripts/$script"
        echo "✅ Signed: $script → ${script}.asc"
    fi
done

# Step 5: Display public key fingerprint
echo ""
echo "🔑 Public Key Fingerprint:"
gpg --fingerprint "$KEY_EMAIL" | grep -A 1 "Key fingerprint"

echo ""
echo "📋 Next Steps:"
echo "=============="
echo ""
echo "1. Publish the public key:"
echo "   cp gpg-key.asc apps/panel/static/"
echo ""
echo "2. Commit signatures to repository:"
echo "   git add scripts/*.asc gpg-key.asc"
echo "   git commit -m 'Add GPG signatures for install scripts'"
echo ""
echo "3. Users can verify scripts before installation:"
echo "   curl -sSLO https://your-panel.com/install.sh"
echo "   curl -sSLO https://your-panel.com/install.sh.asc"
echo "   curl -sSL https://your-panel.com/gpg-key.asc | gpg --import"
echo "   gpg --verify install.sh.asc install.sh"
echo "   sudo bash install.sh [options]"
echo ""
echo "4. Re-sign scripts after each update:"
echo "   ./scripts/sign-scripts.sh"
echo ""
echo "✅ Setup complete!"
