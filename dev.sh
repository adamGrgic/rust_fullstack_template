#!/bin/bash

# Atom Platform Development Server
# Starts and monitors API and Client services

set -e

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color
BOLD='\033[1m'

# Service configuration
API_PORT=8080
CLIENT_PORT=8081
LOG_DIR=".logs"

# Create log directory
mkdir -p "$LOG_DIR"

# Cleanup function
cleanup() {
    echo ""
    echo -e "${YELLOW}🛑 Shutting down services...${NC}"
    
    # Kill any processes on our ports
    lsof -ti:$API_PORT | xargs -r kill 2>/dev/null || true
    lsof -ti:$CLIENT_PORT | xargs -r kill 2>/dev/null || true
    sleep 1
    
    # Force kill if still running
    lsof -ti:$API_PORT,$CLIENT_PORT | xargs -r kill -9 2>/dev/null || true
    
    echo -e "${GREEN}✅ All services stopped${NC}"
    exit 0
}

# Set up trap for cleanup
trap cleanup SIGINT SIGTERM

# Check if ports are already in use
check_ports() {
    if lsof -ti:$API_PORT >/dev/null 2>&1; then
        echo -e "${RED}❌ Port $API_PORT already in use${NC}"
        echo "   Run 'make stop' to kill existing services"
        exit 1
    fi
    
    if lsof -ti:$CLIENT_PORT >/dev/null 2>&1; then
        echo -e "${RED}❌ Port $CLIENT_PORT already in use${NC}"
        echo "   Run 'make stop' to kill existing services"
        exit 1
    fi
}

# Check service status with color
check_service_status() {
    local name=$1
    local port=$2
    printf "  %-20s" "$name"
    if lsof -ti:$port >/dev/null 2>&1; then
        echo -e "${GREEN}●${NC} running     http://localhost:$port"
    else
        echo -e "${RED}●${NC} stopped"
    fi
}

# Display status dashboard
show_status() {
    clear
    echo -e "${BOLD}${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BOLD}${CYAN}         🚀 Atom Platform - Development Environment${NC}"
    echo -e "${BOLD}${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    echo -e "${BOLD}Services Status:${NC}"
    check_service_status "Platform API" $API_PORT
    check_service_status "Client UI" $CLIENT_PORT
    echo ""
    
    # Show recent errors if any
    if [ -f "$LOG_DIR/api.log" ]; then
        local api_errors=$(grep -i "error" "$LOG_DIR/api.log" | tail -1)
        if [ ! -z "$api_errors" ]; then
            echo -e "${BOLD}${RED}Recent API Error:${NC}"
            echo -e "${RED}  $(echo $api_errors | cut -c1-70)...${NC}"
            echo ""
        fi
    fi
    
    echo -e "${BOLD}Logs:${NC}"
    echo -e "  ${CYAN}→${NC} API:    tail -f $LOG_DIR/api.log"
    echo -e "  ${CYAN}→${NC} Client: tail -f $LOG_DIR/client.log"
    echo ""
    echo -e "${BOLD}Quick Actions:${NC}"
    echo -e "  ${CYAN}→${NC} Test API:    curl http://localhost:$API_PORT/api/todos"
    echo -e "  ${CYAN}→${NC} Open Client: open http://localhost:$CLIENT_PORT"
    echo ""
    echo -e "${YELLOW}Press Ctrl+C to stop all services${NC}"
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
}

# Main execution
main() {
    echo -e "${CYAN}🚀 Starting Atom Platform Development Environment...${NC}"
    echo ""
    
    # Check ports
    check_ports
    
    # Start API server
    echo -e "${BLUE}📦 Starting Platform API on port $API_PORT...${NC}"
    (cd platform/api && cargo run 2>&1) > "$LOG_DIR/api.log" 2>&1 &
    
    # Start Client server
    echo -e "${BLUE}📦 Starting Client on port $CLIENT_PORT...${NC}"
    (cd client && unset NO_COLOR && trunk serve 2>&1) > "$LOG_DIR/client.log" 2>&1 &
    
    # Wait for services to start
    echo -e "${YELLOW}⏳ Waiting for services to start...${NC}"
    
    MAX_WAIT=30
    WAITED=0
    API_STARTED=false
    CLIENT_STARTED=false
    
    while [ $WAITED -lt $MAX_WAIT ]; do
        if lsof -ti:$API_PORT >/dev/null 2>&1; then
            if [ "$API_STARTED" = false ]; then
                echo -e "${GREEN}  ✓ API started${NC}"
                API_STARTED=true
            fi
        fi
        
        if lsof -ti:$CLIENT_PORT >/dev/null 2>&1; then
            if [ "$CLIENT_STARTED" = false ]; then
                echo -e "${GREEN}  ✓ Client started${NC}"
                CLIENT_STARTED=true
            fi
        fi
        
        if [ "$API_STARTED" = true ] && [ "$CLIENT_STARTED" = true ]; then
            break
        fi
        
        sleep 1
        WAITED=$((WAITED + 1))
    done
    
    echo ""
    
    # Check for startup failures
    if [ "$API_STARTED" = false ]; then
        echo -e "${RED}❌ API failed to start after ${WAITED}s${NC}"
        echo -e "${RED}   Check logs: tail -20 $LOG_DIR/api.log${NC}"
        echo ""
        tail -20 "$LOG_DIR/api.log"
        cleanup
    fi
    
    if [ "$CLIENT_STARTED" = false ]; then
        echo -e "${RED}❌ Client failed to start after ${WAITED}s${NC}"
        echo -e "${RED}   Check logs: tail -20 $LOG_DIR/client.log${NC}"
        echo ""
        tail -20 "$LOG_DIR/client.log"
        cleanup
    fi
    
    # Show status dashboard
    show_status
    
    echo ""
    echo -e "${GREEN}✅ All services running successfully!${NC}"
    echo -e "${YELLOW}   Monitoring... (updates every 5s)${NC}"
    echo ""
    
    # Monitor services and update status
    while true; do
        sleep 5
        
        # Refresh status display
        show_status
        
        # Check if services are still running
        if ! lsof -ti:$API_PORT >/dev/null 2>&1; then
            echo ""
            echo -e "${RED}⚠️  API stopped unexpectedly!${NC}"
            echo -e "${RED}   Last 20 lines of log:${NC}"
            tail -20 "$LOG_DIR/api.log"
            cleanup
        fi
        
        if ! lsof -ti:$CLIENT_PORT >/dev/null 2>&1; then
            echo ""
            echo -e "${RED}⚠️  Client stopped unexpectedly!${NC}"
            echo -e "${RED}   Last 20 lines of log:${NC}"
            tail -20 "$LOG_DIR/client.log"
            cleanup
        fi
    done
}

main
