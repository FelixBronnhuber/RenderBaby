#!/bin/bash
set -e

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
BOLD='\033[1m'
RESET='\033[0m'

cd "$(dirname "$0")"

if [ -d "scenes" ]; then
    cd scenes
    
    for dir in */; do
        if [ -d "$dir" ]; then
            dirname=${dir%/}
            echo -e "${BLUE}${BOLD}Packing scene:${RESET} $dirname"
            
            if [ -f "${dirname}.rscn" ]; then
                echo -e "  ${GREEN}✓${RED} Deleting existing ${dirname}.rscn${RESET}"
            fi
            rm -f "${dirname}.rscn"
            
            # Create the .rscn file (renamed zip)
            # -r: recursive
            # -q: quiet
            (cd "$dirname" && zip -r -q "../${dirname}.rscn" .)
            echo -e "  ${GREEN}✓${RESET} Created ${dirname}.rscn"
        fi
    done
    echo -e "\n${GREEN}${BOLD}Done packing scenes.${RESET}"
else
    echo -e "${RED}${BOLD}Error: 'scenes' directory not found.${RESET}"
    exit 1
fi
