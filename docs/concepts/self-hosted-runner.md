<!--
  title: Self-hosted Runner (t420) — Routing, intelligente Drosselung, Grenzen
  class: concept
  date: 2026-10-10
  sha256: ade23c27c0d3be505be282226c45c70490459a138b636b3f0a4d8dff6e0cf291
  status: live
  see-also: docs/concepts/github-pipeline.md bin/runner_throttle.sh .github/workflows/ci-gate.yml
-->
# Self-hosted Runner (t420)

Der freie zweite CI-Rechner der Linie. Diese Karte ist der gemessene Stand und das
Wie der Drosselung. (Azure-OSS ist **descoped**: discontinued + CC BY-NC-SA ist keine
OSI-Lizenz — s. `survey-2026-10-10-github-ci-cdn-optimierung.md`.)

## Der Runner (gemessen 2026-10-05, `state/zustand/ereignisse.φ:81058`)

- Registriert als **`t420`**, Host `johannes-ThinkPad-T420` (2011er ThinkPad), Linux Mint 22.3.
- Labels `self-hosted, Linux, X64`; GitHub-Runner **v2.337.0** unter `/home/actions/actions-runner`.
- Nutzer **`actions`**, **ohne sudo**; systemd `actions.runner.omegaflow-omegaflow.t420.service`.
- Sleep/Suspend/Hibernate **maskiert**, Lid-close ignoriert → dauerhaft an.
- Bereits gedrosselt: `Nice=10`, `CPUWeight=20`, `IOWeight=20` (systemd, soft/fair-share).

**Der XPS 13 (`johannes-XPS-13-9350`) ist der Arbeits-Rechner des Operators und trägt
keine Jobs** (Operator-Wort 2026-10-08: „auf meinem XPS13 dürfen sie auf keinen Fall laufen").

## Routing (gemessen 2026-10-10)

- **20 Workflows** fahren auf `runs-on: [self-hosted, Linux]` — die netz-lastigen CDN-Compiler
  (`cmb-cdn`, `gaia-xp-cdn`, die Ephemeriden-Compiler, `dust`, `wod`, `mpcobs-shard`, `tnbfits`,
  `twomass`, …). Sie profitieren, weil bei ihnen die **Bandbreite** der Engpass ist, nicht die CPU.
- **`ci-gate.yml`-`subset`** (`cargo test --lib`) fährt auf `[self-hosted, Linux]` (Operator-Wort
  2026-10-08 „nur der subset-Job auf den T420" + 2026-10-10 „gerne mehr Last"). Der Job hält nur
  den **Tip** (per-ref concurrency, `cancel-in-progress`); superseded SHAs bleiben `pending`.
- **Sicherheit:** self-hosted nur für **trusted** Trigger. `ci-gate` läuft auf `push`/
  `workflow_dispatch` (kein `pull_request`) → kein ungeprüfter Fork-Code auf dem Rechner.
  Nie `pull_request_target` + Fork-Checkout auf self-hosted.

## Intelligente Drosselung (`bin/runner_throttle.sh`)

Der t420 teilt den Uplink mit dem Media-PC. Das Werkzeug (auf dem t420 als root zu installieren,
`install`) tut drei Dinge:

1. **CPU/IO (soft):** systemd-Drop-in `Nice=10`, `CPUWeight=20`, `IOWeight=20`, **`CPUQuota=150%`** —
   die Hälfte des 2c/4t-ThinkPads bleibt dem Operator, der Kernel bevorzugt bei Bedarf jeden
   anderen Prozess (fair-share; das ist die „intelligente" CPU-Seite ohne Skript).
2. **Netz-Kappe mit Headroom:** `tc`/HTB auf dem Default-Interface, **Ceiling `MAX_KBIT`** (Standard
   20 Mbit). Der Runner nimmt nie den ganzen Uplink — der Media-PC behält Luft.
3. **Adaptiv (das Intelligente):** ein systemd-Dienst misst alle 5 s die **Round-Trip-Zeit zum
   Gateway** (Bufferbloat = gemessenes Symptom eines ausgelasteten Links) und regelt die Kappe
   zwischen `MIN_KBIT` und `MAX_KBIT` herunter/hoch — steigt die Latenz **≥ `HIGH_MS`** (Media
   streamt, Link satt), weicht der Runner zurück; fällt sie **≤ `LOW_MS`**, erholt er sich.
   `pause`/`resume` stoppen den Runner ganz für eine Media-Session.

Konfiguration: `/etc/default/runner-throttle` (`IFACE`, `MIN_KBIT`, `MAX_KBIT`, `GW`, `HIGH_MS`,
`LOW_MS`). `status` zeigt Interface, Band, tc-Zustand und Latenz.

## Grenzen (0 honored)

- **Der Media-PC ist ein eigenes Gerät** — der t420 sieht dessen Verkehr nicht. Die ehrliche,
  lokale Regel ist deshalb **Headroom lassen** (Kappe + Adaption über die gemessene Gateway-Latenz);
  die **wirklich** adaptive Stelle bei getrennten WLAN-Clients ist **QoS am Router** (Bufferbloat-
  Management, `cake`/SQM) — das ist Operator-Config, nicht t420-Config, und bleibt hier `pending`.
- Der Runner ist ein 2011er ThinkPad: für CPU-schwere Jobs **langsamer** als GitHub-hosted. Deswegen
  laufen dort die **netz-lastigen** Compiler und der kleine `--lib`-Subset, nicht die volle Suite.
- Ein Runner = ein Job. Der `subset`-Tip hält die Queue; bei Offline-Gerät hängt ein Required-Check
  (`pending`, nie fabriziert grün).
