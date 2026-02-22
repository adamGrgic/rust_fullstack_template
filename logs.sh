#!/bin/bash

# Atom Platform Log Viewer
# Shows live logs for API and/or Client with color-coded output

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

LOG_DIR=".logs"

show_usage() {
    echo "Usage: ./logs.sh [api|client|both]"
    echo ""
    echo "Options:"
    echo "  api    - Show API logs only"
    echo "  client - Show client logs only"
    echo "  both   - Show both logs side by side (default)"
    echo ""
    exit 1
}

MODE="${1:-both}"

case "$MODE" in
    api)
        echo -e "${CYAN}📋 Following API logs...${NC}"
        echo -e "${YELLOW}(Press Ctrl+C to exit)${NC}"
        echo ""
        tail -f "$LOG_DIR/api.log" | while read line; do
            if echo "$line" | grep -qi "error"; then
                echo -e "${RED}$line${NC}"
            elif echo "$line" | grep -qi "warn"; then
                echo -e "${YELLOW}$line${NC}"
            elif echo "$line" | grep -qi "info"; then
                echo -e "${GREEN}$line${NC}"
            else
                echo "$line"
            fi
        done
        ;;
    client)
        echo -e "${CYAN}📋 Following Client logs...${NC}"
        echo -e "${YELLOW}(Press Ctrl+C to exit)${NC}"
        echo ""
        tail -f "$LOG_DIR/client.log"
        ;;
    both)
        echo -e "${CYAN}📋 Following API and Client logs...${NC}"
        echo -e "${YELLOW}(Press Ctrl+C to exit)${NC}"
        echo ""
        tail -f "$LOG_DIR/api.log" "$LOG_DIR/client.log"
        ;;
    *)
        show_usage
        ;;
esac
