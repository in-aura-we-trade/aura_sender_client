#!/usr/bin/env bash
set -euo pipefail

iface="${1:-}"

if [[ -z "$iface" ]]; then
  echo "usage: $0 <iface>" >&2
  exit 1
fi

if [[ ! -d "/sys/class/net/$iface" ]]; then
  echo "interface '$iface' does not exist" >&2
  exit 1
fi

gateway_ip="0.0.0.0"

if [[ "$iface" != "lo" ]]; then
  resolved_gw="$(ip route show default dev "$iface" 2>/dev/null | awk '/default/ {
    for (i = 1; i <= NF; i++) {
      if ($i == "via") {
        print $(i + 1)
        exit
      }
    }
  }')"

  if [[ -n "${resolved_gw:-}" ]]; then
    gateway_ip="$resolved_gw"
  fi
fi

echo "gateway_ip = \"$gateway_ip\""
