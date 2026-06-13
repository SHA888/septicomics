#!/usr/bin/env bash
# Setup pre-commit hooks for septicomics development

set -euo pipefail

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== septicomics pre-commit hook setup ===${NC}"

# Check if pre-commit is installed
if ! command -v pre-commit &> /dev/null; then
    echo -e "${YELLOW}⚠ pre-commit not found. Installing...${NC}"
    pip install pre-commit
fi

# Install git hooks
echo -e "${BLUE}Installing git hooks...${NC}"
pre-commit install
pre-commit install --hook-type commit-msg

# Run hooks on all files to validate setup
echo -e "${BLUE}Running initial validation...${NC}"
if pre-commit run --all-files; then
    echo -e "${GREEN}✓ Pre-commit hooks installed and validated${NC}"
else
    echo -e "${YELLOW}⚠ Some hooks failed on initial run (fix and re-run: pre-commit run --all-files)${NC}"
fi

echo -e "${BLUE}
Setup complete! Pre-commit hooks will now run on every commit.

Commands:
  pre-commit run --all-files      # Run all hooks on all files
  pre-commit autoupdate            # Update hook versions
  pre-commit run <hook-id>         # Run specific hook
  pre-commit uninstall             # Remove hooks (if needed)
${NC}"
