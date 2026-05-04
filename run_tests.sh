#!/usr/bin/env bash
set -e

echo "==============================="
echo "  Running Elixir tests (ex_aoc)"
echo "==============================="
(cd ex_aoc && mix test)

echo ""
echo "==============================="
echo "  Running Rust tests (rs_aoc)"
echo "==============================="
(cd rs_aoc && cargo test)

echo ""
echo "==============================="
echo "  All tests complete!"
echo "==============================="
