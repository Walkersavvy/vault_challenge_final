#!/bin/bash
set -e

echo "Checking environment variables..."
echo "RUSTFLAGS=$RUSTFLAGS"
echo "CARGO_BUILD_JOBS=$CARGO_BUILD_JOBS"

echo "Checking validator health..."
if solana cluster-version --url localhost >/dev/null 2>&1; then
  echo "Validator RPC is live"
else
  echo "Validator RPC not responding at localhost:8899"
  exit 1
fi

echo "Checking program ID alignment..."
IDL_ADDR=$(grep '"address"' target/idl/blueshift_anchor_vault.json | cut -d '"' -f4)
TOML_ADDR=$(grep blueshift_anchor_vault Anchor.toml | cut -d '"' -f2)
LIB_ADDR=$(grep declare_id programs/blueshift_anchor_vault/src/lib.rs | cut -d '"' -f2)

echo "IDL:   $IDL_ADDR"
echo "TOML:  $TOML_ADDR"
echo "LIBRS: $LIB_ADDR"

if [[ "$IDL_ADDR" == "$TOML_ADDR" && "$TOML_ADDR" == "$LIB_ADDR" ]]; then
  echo "Program IDs match"
else
  echo "Program ID mismatch"
  exit 1
fi

echo "Checking deployment..."
if solana program show "$IDL_ADDR" --url localhost >/dev/null 2>&1; then
  echo "Program deployed"
else
  echo "Program not deployed"
  exit 1
fi

echo "All checks passed! Running tests..."
anchor test --skip-local-validator
