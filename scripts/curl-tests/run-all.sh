#!/usr/bin/env bash
# DimDim Health - Run All API Tests
# Executes all curl-based API tests in sequence

set -euo pipefail

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Resolve script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

API_BASE_URL="${API_BASE_URL:-http://localhost:3000}"

echo "=========================================="
echo -e "${BLUE}DimDim Health API Test Suite${NC}"
echo "=========================================="
echo ""
echo "API URL: $API_BASE_URL"
echo ""

PASSED=0
FAILED=0
SKIPPED=0

run_test() {
    local test_script="$1"
    local test_name
    test_name=$(basename "$test_script")
    
    echo ""
    echo -e "${YELLOW}Running: $test_name${NC}"
    echo "----------------------------------------"
    
    if bash "$test_script"; then
        ((PASSED++))
    else
        ((FAILED++))
        echo -e "${RED}Test failed: $test_name${NC}"
    fi
}

# Run tests in order
for test_file in "$SCRIPT_DIR"/[0-9][0-9]-*.sh; do
    if [ -f "$test_file" ]; then
        run_test "$test_file"
    fi
done

# Summary
echo ""
echo "=========================================="
echo -e "${BLUE}Test Summary${NC}"
echo "=========================================="
echo -e "  ${GREEN}Passed:${NC}  $PASSED"
echo -e "  ${RED}Failed:${NC}  $FAILED"
echo -e "  ${YELLOW}Skipped:${NC} $SKIPPED"
echo ""

if [ "$FAILED" -eq 0 ]; then
    echo -e "${GREEN}All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}Some tests failed.${NC}"
    exit 1
fi
