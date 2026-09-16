#!/usr/bin/env bash
# proton-wg.sh — a userspace Proton WireGuard exit via wireproxy (no root).
#
# The free Proton servers live as WireGuard configs in <WG>/proton-free/*.conf
# (one per server: US-FREE#2, NL-FREE#133, …). wireproxy speaks the WireGuard
# protocol in userspace and exposes a SOCKS5 inbound, so no wg-quick and no
# sudo are needed. Point any curl at it through the environment:
#
#   ALL_PROXY=socks5h://127.0.0.1:25344 curl https://data.lsst.cloud/
#
# The Archivar's fetch (src/archivar/fetch.rs) spawns curl without --proxy and
# without .env_clear(), so ALL_PROXY/HTTPS_PROXY route every fetch through the
# exit with zero code change.
#
# usage: proton-wg.sh list | status | off | <server|cc>
#   <server>  a config name, e.g. US-FREE#2 (the # may be typed as _)
#   <cc>      a country code, e.g. us / nl / jp — picks the first server
set -u

WG="${PROTON_WG_DIR:-$HOME/.config/wireguard}/proton-free"
BIN="${WIREPROXY_BIN:-$HOME/.local/bin/wireproxy}"
PORT="${PROTON_WG_PORT:-25344}"
RUNDIR="${XDG_RUNTIME_DIR:-/tmp}/proton-wg"
PIDF="$RUNDIR/wireproxy.pid"
CONF="$RUNDIR/active.conf"
LOG="$RUNDIR/wireproxy.log"

lc() { printf '%s' "$1" | tr '[:upper:]' '[:lower:]'; }

stop() {
  if [ -f "$PIDF" ]; then
    p=$(cat "$PIDF" 2>/dev/null || true)
    [ -n "$p" ] && kill "$p" 2>/dev/null || true
  fi
  pkill -x wireproxy 2>/dev/null || true
  sleep 1
  if pgrep -x wireproxy >/dev/null 2>&1; then
    printf 'proton-wg: wireproxy still running — pid file kept\n' >&2
  else
    rm -f "$PIDF"
  fi
}

exit_ip() { ALL_PROXY="socks5h://127.0.0.1:$PORT" curl -sS -m 15 https://api.ipify.org 2>/dev/null; }

action="${1:-status}"
case "$action" in
  list)
    printf 'free servers in %s:\n' "$WG"
    for f in "$WG"/*.conf; do [ -e "$f" ] && basename "$f" .conf; done
    ;;
  status)
    if pgrep -x wireproxy >/dev/null 2>&1; then
      printf 'proton-wg: up on socks5h://127.0.0.1:%s -> %s\n' "$PORT" "$(exit_ip || printf absent)"
    else
      printf 'proton-wg: no tunnel up\n'
    fi
    ;;
  off | down | disconnect)
    stop
    printf 'proton-wg: off\n'
    ;;
  *)
    name=$(printf '%s' "$action" | tr '#' '_')
    conf=""
    if [ -e "$WG/$name.conf" ]; then
      conf="$WG/$name.conf"
    else
      cc=$(printf '%s' "$(lc "$action")" | tr '[:lower:]' '[:upper:]')
      for f in "$WG"/*.conf; do
        case "$(basename "$f")" in
          "$cc"-FREE_*) conf="$f"; break ;;
        esac
      done
    fi
    if [ -z "$conf" ]; then
      printf 'proton-wg: no config for "%s" — run: proton-wg.sh list\n' "$action" >&2
      exit 2
    fi
    if [ ! -x "$BIN" ]; then
      printf 'proton-wg: wireproxy not found at %s\n' "$BIN" >&2
      exit 2
    fi
    mkdir -p "$RUNDIR"
    stop
    cp "$conf" "$CONF"
    printf '\n[Socks5]\nBindAddress = 127.0.0.1:%s\n' "$PORT" >> "$CONF"
    nohup "$BIN" -c "$CONF" >"$LOG" 2>&1 &
    echo $! > "$PIDF"
    sleep 3
    printf 'proton-wg: %s -> %s (socks5h://127.0.0.1:%s)\n' \
      "$(basename "$conf" .conf)" "$(exit_ip || printf absent)" "$PORT"
    ;;
esac
