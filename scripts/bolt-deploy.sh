#!/bin/bash

# GhostPanel Bolt Deployment Script
# Deploys GhostPanel using Bolt's native commands - just like Portainer!

set -e

echo "🚀 GhostPanel - Deploying to Bolt..."
echo "=================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if bolt is available
if ! command -v bolt &> /dev/null; then
    echo -e "${RED}❌ Bolt CLI not found. Please install Bolt first.${NC}"
    exit 1
fi

echo -e "${BLUE}📦 Creating GhostPanel volumes...${NC}"

# Create volumes using Bolt's volume commands
bolt volume create gpanel_data --size 10Gi --driver bolt || echo "Volume gpanel_data already exists"
bolt volume create gpanel_config --size 1Gi --driver bolt || echo "Volume gpanel_config already exists"
bolt volume create gpanel_logs --size 5Gi --driver bolt || echo "Volume gpanel_logs already exists"
bolt volume create gpanel_certs --size 100Mi --driver bolt || echo "Volume gpanel_certs already exists"

echo -e "${GREEN}✅ Volumes created successfully${NC}"

echo -e "${BLUE}🌐 Creating GhostPanel network...${NC}"

# Create network using Bolt's networking
bolt network create ghostpanel-net --driver bolt --subnet 10.99.0.0/16 || echo "Network already exists"

echo -e "${GREEN}✅ Network created successfully${NC}"

echo -e "${BLUE}🔧 Building GhostPanel services...${NC}"

# Build the Rust services
cargo build --release --workspace

echo -e "${GREEN}✅ Services built successfully${NC}"

echo -e "${BLUE}🚀 Deploying GhostPanel stack...${NC}"

# Deploy using Bolt's surge orchestration
bolt surge up --detach

echo -e "${GREEN}✅ GhostPanel deployed successfully!${NC}"

echo ""
echo -e "${YELLOW}📋 GhostPanel Access Information:${NC}"
echo "================================="
echo -e "🌐 Web Interface:  ${GREEN}https://localhost:9443${NC}"
echo -e "📊 Agent API:      ${GREEN}http://localhost:8000${NC}"
echo -e "⚡ CLI Bridge:     ${GREEN}http://localhost:9000${NC}"
echo -e "📈 Traefik UI:     ${GREEN}http://localhost:8080${NC} (dev only)"
echo ""
echo -e "${BLUE}🎮 Gaming Features:${NC}"
echo "==================="
echo -e "• GPU Monitoring: ${GREEN}Integrated with nvbind${NC}"
echo -e "• QUIC Networking: ${GREEN}Ultra-low latency${NC}"
echo -e "• Real-time Updates: ${GREEN}WebSocket + HTTP/3${NC}"
echo ""
echo -e "${YELLOW}📝 Next Steps:${NC}"
echo "=============="
echo "1. Visit https://localhost:9443 to access GhostPanel"
echo "2. Login with your Bolt credentials"
echo "3. Start managing your containers like Portainer, but better!"
echo ""
echo -e "${GREEN}🎯 GhostPanel is ready! Happy container management! 🚀${NC}"