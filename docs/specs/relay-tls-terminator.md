<!--
  title: Relay-TLS-Terminator — the wireless sensor secure context
  class: concept
  date: 2026-09-25
  sha256: 01ed85c855fe03706957492bfeefc8c1da1c8e4d3fee59ba429e1579c7b05828
  status: live
  see-also: AGENTS.md, src/archivar/relay.rs, src/archivar/main_flow.rs, static/sensorium.js, bin/relay-tls.stunnel.conf
-->

# Relay-TLS-Terminator — the wireless sensor secure context

> The `sha256` header is computed over the body without the header via
> `omega_sh sha docs/specs/relay-tls-terminator.md` (set 2026-09-25). A later
> body change recomputes it.

## The problem

The browser sensorium needs a secure context. `static/sensorium.js` calls the
secure-context-gated APIs at three sites: `Accelerometer`/`Gyroscope`/
`Magnetometer` (line 14), `DeviceMotionEvent` (line 88), and
`navigator.mediaDevices.getUserMedia` (line 137). In the tethered case,
`adb reverse` maps the phone's `localhost` to the dev machine — `localhost` is
already a secure context, so no TLS is needed there. In the wireless case the
relay binds `0.0.0.0:1618` (`relay.rs:9/11`, `RELAY_BIND_DEFAULT`), serves plain
HTTP over the LAN IP, and the browser withholds every sensor API: a LAN IP is not
a secure context.

The relay is plain HTTP end to end — WebSocket over `0.0.0.0:1618`, consent at
`/consent?ja` (`relay.rs:396`), started by `bin/omegaflow` when `OMEGAFLOW_HIDDEN`
is unset (`main_flow.rs:762`, `if !hidden`).

## Why the core stays untouched

The relay's TLS wrapper is transport, not field: the wire contract, the consent
gate, and the sensorium are byte-identical behind the terminator. Nothing in
`src/` changes, and the `std + curl + serialport` bound holds — no rustls, no
native-tls, no new core dependency. The terminator is a separate process bound to
the same interface, in front of `127.0.0.1:1618`.

## The choice: stunnel (over Caddy)

stunnel is purpose-built for exactly this job — TLS termination plus a TCP
forward — and nothing else. The private-LAN case is its native case: it takes a
static `cert`/`key` pair directly. Caddy is a full web server whose default ACME
auto-HTTPS assumes a public domain; a private LAN IP has no public name to
validate, so Caddy would need `tls internal` (self-signed, re-issued per run,
awkward to trust on the phone) or an explicit static-cert block to reach the same
result. stunnel has no such machinery: one file, no site-block nesting, no
web-server baggage. The relay is already the HTTP server; stunnel only wraps the
transport.

## Trust model — one local CA

The phone must trust the leaf cert. The right shape is a local CA (a self-signed
root) whose certificate is installed on the phone, plus a leaf certificate for
the LAN IP (SAN `IP:<lan-ip>`) signed by that CA. Installing one CA trusts every
future LAN certificate — one trust step, never per-cert.

## Operator sequence

1. Generate the local CA (once).
2. Generate the leaf cert for the LAN IP (SAN `IP:<lan-ip>`).
3. Start the terminator: `stunnel bin/relay-tls.stunnel.conf` (it serves
   `https://0.0.0.0:1619` → `127.0.0.1:1618`).
4. Start the relay: `bin/omegaflow` with `OMEGAFLOW_HIDDEN` unset.
5. Install `ca.pem` on the phone (Android: Settings → Security → Encryption &
   credentials → Install a certificate → CA certificate; iOS: open the profile,
   then enable full trust).
6. Open `https://<lan-ip>:1619/consent?ja` — the consent gate, now over TLS.

## Reachability / verification

- `archive_search --verdict https://<lan-ip>:1619/` — the reachability ladder
  (direct → Proton → Wayback).
- `archive_search --sniff https://<lan-ip>:1619/` — magic bytes + sha256 of what
  the terminator serves.
- `archive_search --verdict https://<lan-ip>:1619/consent?ja` — the consent route.

## Measured

- The certificate material is generated under `state/tls/` (gitignored), once, with
  OpenSSL 3.0.13 (2026-09-26): `ca.pem`/`ca.key` (root `CN=omegaflow-relay-ca`,
  valid to 2036-09) and `relay-leaf.pem`/`relay-leaf.key` (leaf `CN=omegaflow-relay`,
  SAN `IP:<lan-ip>`, signed by the CA, valid to 2028-12). `bin/relay-tls.stunnel.conf`
  points its `cert`/`key` at this material.
- The LAN IP is measured; it lives in the leaf SAN (gitignored material), never in
  tracked prose. The TLS port 1619 is the chosen convention, adjacent to 1618.

## pending

- The operator's act: install `ca.pem` on the phone, start `stunnel`, start the relay
  with `OMEGAFLOW_HIDDEN` unset, verify `https://<lan-ip>:1619/consent?ja`. No
  installation, no live run was performed.
