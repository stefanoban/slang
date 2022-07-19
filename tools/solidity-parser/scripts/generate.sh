#!/bin/bash
set -euo pipefail

THIS_DIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")")
PROJECT_DIR=$(dirname "$THIS_DIR")
REPO_ROOT=$(realpath "$PROJECT_DIR/../..")

# shellcheck source=/dev/null
[[ "${HERMIT_ENV:-}" == "$PROJECT_DIR" ]] || source "$PROJECT_DIR/bin/activate-hermit"

###################################################
# Solidity from the intended source manifest
###################################################

cargo run --manifest-path "$PROJECT_DIR/../syntax-schema/Cargo.toml" --bin "validate_manifest" -- \
  --manifest-input "$PROJECT_DIR/syntax/solidity/manifest.yml"

cargo run --manifest-path "$PROJECT_DIR/../syntax-schema/Cargo.toml" --bin "manifest_to_ebnf" -- \
  --manifest-input "$PROJECT_DIR/syntax/solidity/manifest.yml" \
  --ebnf-output "$PROJECT_DIR/src/generated/derived.ebnf"

cargo run --manifest-path "$PROJECT_DIR/../syntax-schema/Cargo.toml" --bin "manifest_to_chumsky" -- \
  --manifest-input "$PROJECT_DIR/syntax/solidity/manifest.yml" \
  --chumsky-output "$PROJECT_DIR/src/generated"

cargo run --manifest-path "$PROJECT_DIR/../syntax-schema/Cargo.toml" --bin "manifest_to_spec" -- \
  --manifest-input "$PROJECT_DIR/syntax/solidity/manifest.yml" \
  --documentation-folder "$REPO_ROOT/documentation"
