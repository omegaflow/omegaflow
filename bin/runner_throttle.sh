#!/usr/bin/env bash
# runner_throttle — the self-hosted t420 yields to the shared WAN and the operator.
#
# The runner shares the home uplink with the media PC. This tool caps the t420's
# egress and, in `adaptive` mode, backs the cap further off when the WAN is
# stressed (a rising round-trip to the gateway is the measured symptom of a
# saturated link), then recovers when the link is idle again. `pause`/`resume`
# stop the runner outright for a media session.
#
# Run on the t420 as root (or via sudo). Idempotent. Nothing here runs in a
# session — it installs a systemd unit on the machine, like the watchdogs.
#
# Usage: runner_throttle.sh install|uninstall|status|pause|resume|set <kbit>|adaptive
#
# Config: /etc/default/runner-throttle  (IFACE, MIN_KBIT, MAX_KBIT, GW, HIGH_MS, LOW_MS)
set -euo pipefail

SERVICE="actions.runner.omegaflow-omegaflow.t420.service"
DROPIN_DIR="/etc/systemd/system/${SERVICE}.d"
DROPIN="${DROPIN_DIR}/10-throttle.conf"
CONF="/etc/default/runner-throttle"
ADAPTIVE_UNIT="/etc/systemd/system/runner-throttle-adaptive.service"

# defaults — a conservative band; override in $CONF
IFACE="${IFACE:-$(ip route show default 2>/dev/null | awk '/default/{print $5; exit}')}"
MIN_KBIT="${MIN_KBIT:-2000}"    # floor: keep the runner making progress
MAX_KBIT="${MAX_KBIT:-20000}"   # ceiling: never take the whole uplink
GW="${GW:-$(ip route show default 2>/dev/null | awk '/default/{print $3; exit}')}"
HIGH_MS="${HIGH_MS:-80}"        # above this, back off
LOW_MS="${LOW_MS:-20}"          # below this, recover
[ -f "$CONF" ] && . "$CONF"

need_root() { [ "$(id -u)" = 0 ] || { echo "runner_throttle: needs root (use sudo)" >&2; exit 2; }; }

tc_rate() { tc qdisc show dev "$IFACE" | grep -q htb && tc class change dev "$IFACE" classid 1:10 htb rate "${1}kbit" ceil "$MAX_KBIT"kbit 2>/dev/null || true; }

apply_cap() {
  local rate="$1"
  tc qdisc del dev "$IFACE" root 2>/dev/null || true
  tc qdisc add dev "$IFACE" root handle 1: htb default 10
  tc class add dev "$IFACE" parent 1: classid 1:1 htb rate "${MAX_KBIT}kbit"
  tc class add dev "$IFACE" parent 1:1 classid 1:10 htb rate "${rate}kbit" ceil "${MAX_KBIT}kbit"
  echo "runner_throttle: egress cap ${rate}kbit on ${IFACE} (ceiling ${MAX_KBIT}kbit)"
}

cmd_install() {
  need_root
  mkdir -p "$DROPIN_DIR"
  cat > "$DROPIN" <<EOF
[Service]
Nice=10
CPUWeight=20
IOWeight=20
# leave half of the 2c/4t ThinkPad for the operator/media when it shares the box
CPUQuota=150%
EOF
  [ -f "$CONF" ] || cat > "$CONF" <<EOF
IFACE=${IFACE}
MIN_KBIT=${MIN_KBIT}
MAX_KBIT=${MAX_KBIT}
GW=${GW}
HIGH_MS=${HIGH_MS}
LOW_MS=${LOW_MS}
EOF
  cat > "$ADAPTIVE_UNIT" <<EOF
[Unit]
Description=runner-throttle adaptive WAN back-off for the t420 runner
After=network-online.target
[Service]
ExecStart=/usr/local/bin/runner_throttle.sh adaptive
Restart=always
RestartSec=5
[Install]
WantedBy=multi-user.target
EOF
  install -m0755 "$0" /usr/local/bin/runner_throttle.sh
  systemctl daemon-reload
  systemctl restart "$SERVICE"
  systemctl enable --now runner-throttle-adaptive.service
  apply_cap "$MAX_KBIT"
  echo "runner_throttle: installed (drop-in, adaptive service, cap)"
}

cmd_uninstall() {
  need_root
  systemctl disable --now runner-throttle-adaptive.service 2>/dev/null || true
  rm -f "$ADAPTIVE_UNIT" "$DROPIN"
  rmdir "$DROPIN_DIR" 2>/dev/null || true
  tc qdisc del dev "$IFACE" root 2>/dev/null || true
  systemctl daemon-reload
  systemctl restart "$SERVICE"
  echo "runner_throttle: removed"
}

cmd_status() {
  echo "iface=${IFACE} gw=${GW} band=${MIN_KBIT}..${MAX_KBIT}kbit"
  systemctl is-active "$SERVICE" 2>/dev/null | sed 's/^/runner: /'
  tc qdisc show dev "$IFACE" 2>/dev/null | sed 's/^/tc: /'
  ping -c1 -W1 "$GW" >/dev/null 2>&1 && echo "rtt: $(ping -c1 -W1 "$GW" | awk -F'time=' '/time=/{split($2,a," ");print a[1]" ms"}')" || echo "rtt: gateway unreachable"
}

cmd_pause()  { need_root; systemctl stop "$SERVICE"; echo "runner_throttle: runner paused"; }
cmd_resume() { need_root; systemctl start "$SERVICE"; echo "runner_throttle: runner resumed"; }

cmd_adaptive() {
  need_root
  local rate="$MAX_KBIT"
  while true; do
    local ms=9999
    if ping -c1 -W1 "$GW" >/dev/null 2>&1; then
      ms=$(ping -c1 -W1 "$GW" | awk -F'time=' '/time=/{split($2,a," ");printf "%d",a[1]}')
    fi
    if [ "$ms" -ge "$HIGH_MS" ] && [ "$rate" -gt "$MIN_KBIT" ]; then
      rate=$(( rate * 70 / 100 )); [ "$rate" -lt "$MIN_KBIT" ] && rate="$MIN_KBIT"
      tc_rate "$rate"
    elif [ "$ms" -le "$LOW_MS" ] && [ "$rate" -lt "$MAX_KBIT" ]; then
      rate=$(( rate * 110 / 100 )); [ "$rate" -gt "$MAX_KBIT" ] && rate="$MAX_KBIT"
      tc_rate "$rate"
    fi
    sleep 5
  done
}

case "${1:-}" in
  install)   cmd_install ;;
  uninstall) cmd_uninstall ;;
  status)    cmd_status ;;
  pause)     cmd_pause ;;
  resume)    cmd_resume ;;
  adaptive)  cmd_adaptive ;;
  set)       need_root; apply_cap "${2:?kbit}" ;;
  *) echo "usage: runner_throttle.sh install|uninstall|status|pause|resume|set <kbit>|adaptive" >&2; exit 2 ;;
esac
