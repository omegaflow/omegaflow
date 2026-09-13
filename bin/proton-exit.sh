#!/usr/bin/env bash
# proton-exit.sh <CC> — bring up the Proton WireGuard exit for one country.
# The free plan offers no country choice through the CLI; the country exits are
# WireGuard configs, kept as <dir>/proton-<cc>.conf (default ~/.config/wireguard).
# Every Proton config shares Address 10.2.0.2/32 — only ONE exit runs at a time,
# so the helper brings the others down first.
# Free-plan countries: JP CA MX NL NO PL RO CH SG US
# usage: proton-exit.sh <cc> | off | list | status
set -u

WG="${PROTON_WG_DIR:-$HOME/.config/wireguard}"
run() { if [ "$(id -u)" -eq 0 ]; then "$@"; else sudo "$@"; fi; }
lc() { printf '%s' "$1" | tr '[:upper:]' '[:lower:]'; }

action="${1:-status}"
case "$action" in
  list)
    printf 'free countries: JP CA MX NL NO PL RO CH SG US\n'
    printf 'configs in %s:\n' "$WG"
    for f in "$WG"/proton-*.conf; do [ -e "$f" ] && printf '  %s\n' "$(basename "$f" .conf)"; done
    ;;
  status)
    ip -brief addr show 2>/dev/null | grep -E 'proton' || printf 'proton-exit: no tunnel up\n'
    ;;
  off | down | disconnect)
    for f in "$WG"/proton-*.conf; do
      [ -e "$f" ] || continue
      run wg-quick down "$f" 2>/dev/null || true
    done
    printf 'proton-exit: off\n'
    ;;
  *)
    cc=$(lc "$action")
    conf="$WG/proton-$cc.conf"
    if [ ! -e "$conf" ]; then
      printf 'proton-exit: no config %s — download a free-server WireGuard config from account.protonvpn.com/downloads\n' "$conf" >&2
      exit 2
    fi
    for f in "$WG"/proton-*.conf; do
      [ -e "$f" ] || continue
      run wg-quick down "$f" 2>/dev/null || true
    done
    run wg-quick up "$conf"
    ip=$(curl -s --max-time 15 https://ifconfig.me 2>/dev/null || printf 'absent')
    printf 'proton-exit: %s -> %s\n' "$cc" "$ip"
    ;;
esac
