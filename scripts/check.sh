#!/usr/bin/env bash
# ==============================================================================
# Runway: Code Quality & Static Analysis Pipeline
# - Formatter (cargo fmt)
# - Linter & Static Analysis (cargo clippy)
# - Compilation & Target Validation (Native + WASM)
# - Unit & Integration Tests (cargo test)
# ==============================================================================

set -euo pipefail

# ANSI Color Codes
CYAN='\033[0;36m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BOLD='\033[1m'
NC='\033[0m' # No Color

# Workspace Root
SCRIPT_DIR="$(cd "$(dirname "$(readlink -f "${BASH_SOURCE[0]}" 2>/dev/null || echo "${BASH_SOURCE[0]}")")" && pwd -P)"
if [[ -f "${SCRIPT_DIR}/Cargo.toml" ]]; then
    ROOT_DIR="${SCRIPT_DIR}"
else
    ROOT_DIR="$(cd "${SCRIPT_DIR}/.." && pwd -P)"
fi
cd "$ROOT_DIR"

MODE="fix"
if [[ "${1:-}" == "--check" ]] || [[ "${1:-}" == "-c" ]]; then
    MODE="check"
fi

echo -e "${BOLD}${CYAN}======================================================${NC}"
echo -e "${BOLD}${CYAN} 🔍 Runway Code Quality & Static Analysis          ${NC}"
echo -e "${BOLD}${CYAN}======================================================${NC}"
echo -e "Mode: ${BOLD}${YELLOW}${MODE}${NC} (Pass --check for strict check-only / CI mode)"
echo ""

# ------------------------------------------------------------------------------
# 1. Rustfmt (Formatter)
# ------------------------------------------------------------------------------
echo -e "${BOLD}[1/4] 🎨 Formatting (cargo fmt)...${NC}"
if [[ "$MODE" == "fix" ]]; then
    cargo fmt --all
    echo -e "  ${GREEN}✓ Formatter applied to all workspace crates.${NC}"
else
    if cargo fmt --all -- --check; then
        echo -e "  ${GREEN}✓ Code formatting check passed.${NC}"
    else
        echo -e "  ${RED}✗ Code formatting errors detected. Run './check.sh' or 'cargo fmt --all' to fix.${NC}"
        exit 1
    fi
fi
echo ""

# ------------------------------------------------------------------------------
# 2. Clippy Static Analysis (Native Workspace Crates)
# ------------------------------------------------------------------------------
echo -e "${BOLD}[2/4] 🔬 Static Analysis & Linting - Native Crates (cargo clippy)...${NC}"
echo -e "  Scanning common, backend, desktop..."
if cargo clippy --workspace --exclude frontend --all-targets -- -D warnings; then
    echo -e "  ${GREEN}✓ Native crates passed clippy static analysis without warnings.${NC}"
else
    echo -e "  ${RED}✗ Clippy warnings found in native workspace crates.${NC}"
    exit 1
fi
echo ""

# ------------------------------------------------------------------------------
# 3. Clippy Static Analysis (Frontend WASM Crate)
# ------------------------------------------------------------------------------
echo -e "${BOLD}[3/4] 🌐 Static Analysis & Linting - Frontend WebAssembly (cargo clippy)...${NC}"
echo -e "  Scanning frontend (wasm32-unknown-unknown target)..."
if cargo clippy -p frontend --target wasm32-unknown-unknown -- -D warnings; then
    echo -e "  ${GREEN}✓ Frontend Leptos crate passed clippy static analysis without warnings.${NC}"
else
    echo -e "  ${RED}✗ Clippy warnings found in frontend crate.${NC}"
    exit 1
fi
echo ""

# ------------------------------------------------------------------------------
# 4. Automated Tests
# ------------------------------------------------------------------------------
echo -e "${BOLD}[4/4] 🧪 Running Unit & Integration Tests (cargo test)...${NC}"
if cargo test --workspace --exclude frontend; then
    echo -e "  ${GREEN}✓ All workspace unit and integration tests passed.${NC}"
else
    echo -e "  ${RED}✗ One or more tests failed.${NC}"
    exit 1
fi
echo ""

echo -e "${BOLD}${GREEN}======================================================${NC}"
echo -e "${BOLD}${GREEN} 🎉 All lint, formatting, and static checks PASSED!   ${NC}"
echo -e "${BOLD}${GREEN}======================================================${NC}"
