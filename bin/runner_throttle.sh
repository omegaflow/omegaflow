#!/usr/bin/env bash
# runner_throttle — the self-hosted t420 yields to the shared WAN and the operator.
#
# The runner shares the home uplink (router 192.168.178.1) with the media PC. The
# heavy traffic is the runner's *downloads* (multi-GB harvests), so this shapes
# INGRESS (an IFB device — tc egress alone would only cap uploads). It caps the
# t420's download band and, in `adaptive` mode, backs further off when the WAN is
# stressed (a rising round-trip to a WAN host is the measured symptom of a full
# link / bufferbloat), then recovers when the link is idle. `pause`/`resume` stop
# the runner outright for a media session.
#
# Run on the t420 as root (sudo). Idempotent; applies CPU limits live via
# `systemctl set-property` (never restarts a running job) + a persistent drop-in.
#
# Usage: runner_throttle.sh install|uninstall|status|pause|resume|set <kbit>|adaptive
# Config: /etc/default/runner-throttle  (IFACE, MIN_KBIT, MAX_KBIT, GW, PROBE, HIGH_MS, LOW_MS)
set -euo pipefail

SERVICE="actions.runner.omegaflow-omegaflow.t420.service"
DROPIN_DIR="/etc/systemd/system/${SERVICE}.d"
DROPIN="${DROPIN_DIR}/10-throttle.conf"
CONF="/etc/default/runner-throttle"
ADAPTIVE_UNIT="/etc/systemd/system/runner-throttle-adaptive.service"
IFB="ifb0"

IFACE="${IFACE:-$(ip route show default 2>/dev/null | awk '/default/{print $5; exit}')}"
MIN_KBIT="${MIN_KBIT:-2000}"    # floor: keep the runner making progress
MAX_KBIT="${MAX_KBIT:-20000}"   # ceiling: never take the whole downlink
GW="${GW:-$(ip route show default 2>/dev/null | awk '/default/{print $3; exit}')}"
PROBE="${PROBE:-1.1.1.1}"       # WAN host for the bufferbloat probe
HIGH_MS="${HIGH_MS:-100}"       # above this, back off
LOW_MS="${LOW_MS:-30}"          # below this, recover
[ -f "$CONF" ] && . "$CONF"

need_root() { [ "$(id -u)" = 0 ] || { echo "runner_throttle: needs root (use sudo)" >&2; exit 2; }; }

ensure_ifb() {
  modprobe ifb numifbs=1 2>/dev/null || true
  ip link show "$IFB" >/dev/null 2>&1 || ip link add "$IFB" type ifb
  ip link set "$IFB" up
}

# The whole t420 ingress is redirected to $IFB and shaped there. (tc on the
# physical egress would cap only uploads — the wrong direction for downloads.)
apply_cap() {
  local rate="$1"
  ensure_ifb
  tc qdisc del dev "$IFACE" ingress 2>/dev/null || true
  tc qdisc add dev "$IFACE" handle ffff: ingress
  tc filter add dev "$IFACE" parent ffff: protocol all u32 match u32 0 0 \
    action mirred egress redirect dev "$IFB"
  tc qdisc del dev "$IFB" root 2>/dev/null || true
  tc qdisc add dev "$IFB" root handle 1: htb default 10
  tc class add dev "$IFB" parent 1: classid 1:1 htb rate "${MAX_KBIT}kbit"
  tc class add dev "$IFB" parent 1:1 classid 1:10 htb rate "${rate}kbit" ceil "${MAX_KBIT}kbit"
  echo "runner_throttle: ingress cap ${rate}kbit (ceiling ${MAX_KBIT}kbit) via ${IFB} on ${IFACE}"
}

tc_rate() {
  tc class change dev "$IFB" classid 1:10 htb rate "${1}kbit" ceil "${MAX_KBIT}kbit" 2>/dev/null || true
}

rtt_ms() {
  local h="$1"
  ping -c1 -W1 "$h" >/dev/null 2>&1 || { echo 9999; return; }
  ping -c1 -W1 "$h" | awk -F'time=' '/time=/{split($2,a," ");printf "%d",a[1]}'
}

cmd_install() {
  need_root
  mkdir -p "$DROPIN_DIR"
  cat > "$DROPIN" <<EOF
[Service]
Nice=10
CPUWeight=20
IOWeight=20
# leave half of the 2c/4t ThinkPad for the operator when the box is shared
CPUQuota=150%
EOF
  [ -f "$CONF" ] || cat > "$CONF" <<EOF
IFACE=${IFACE}
MIN_KBIT=${MIN_KBIT}
MAX_KBIT=${MAX_KBIT}
GW=${GW}
PROBE=${PROBE}
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
  install -m0755 "$0" /usr/local/bin/runner_throttle.sh 2>/dev/null || true
  systemctl daemon-reload
  # live CPU limits — no restart, a running harvest keeps running
  systemctl set-property "$SERVICE" CPUWeight=20 IOWeight=20 CPUQuota=150%
  systemctl enable --now runner-throttle-adaptive.service
  apply_cap "$MAX_KBIT"
  echo "runner_throttle: installed (drop-in, live CPU limits, adaptive service, ingress cap)"
}

cmd_uninstall() {
  need_root
  systemctl disable --now runner-throttle-adaptive.service 2>/dev/null || true
  rm -f "$ADAPTIVE_UNIT" "$DROPIN"
  rmdir "$DROPIN_DIR" 2>/dev/null || true
  tc qdisc del dev "$IFACE" ingress 2>/dev/null || true
  tc qdisc del dev "$IFB" root 2>/dev/null || true
  ip link del "$IFB" 2>/dev/null || true
  systemctl daemon-reload
  echo "runner_throttle: removed"
}

cmd_status() {
  echo "iface=${IFACE} gw=${GW} probe=${PROBE} band=${MIN_KBIT}..${MAX_KBIT}kbit"
  echo "runner: $(systemctl is-active "$SERVICE" 2>/dev/null)"
  echo "adaptive: $(systemctl is-active runner-throttle-adaptive.service 2>/dev/null)"
  tc qdisc show dev "$IFB" 2>/dev/null | sed 's/^/tc: /' || echo "tc: ifb absent"
  echo "rtt probe(${PROBE}): $(rtt_ms "$PROBE") ms"
}

cmd_pause()  { need_root; systemctl stop "$SERVICE"; echo "runner_throttle: runner paused"; }
cmd_resume() { need_root; systemctl start "$SERVICE"; echo "runner_throttle: runner resumed"; }

cmd_adaptive() {
  need_root
  local rate="$MAX_KBIT"
  while true; do
    local ms; ms=$(rtt_ms "$PROBE")
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
