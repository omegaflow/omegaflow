<!--
  title: Self-hosted Runner (t420) — Routing, intelligente Drosselung, Grenzen
  class: concept
  date: 2026-10-10
  sha256: 52f2354dacf5ffd242de0e74e63f7d845c4813aadcd73c4d4802144b28556d20
  status: live
  see-also: docs/concepts/github-pipeline.md bin/runner_throttle.sh .github/workflows/ci-gate.yml
-->
# Self-hosted Runner (t420)

Der freie zweite CI-Rechner der Linie. Diese Karte ist der gemessene Stand und das
Wie der Drosselung. (Azure-OSS ist **descoped**: discontinued + CC BY-NC-SA ist keine
OSI-Lizenz — s. `survey-2026-10-10-github-ci-cdn-optimierung.md`.)

## Der Runner (gemessen 2026-10-05, `state/zustand/ereignisse.φ:81058`)

- Registriert als **`t420`**, Host `johannes-ThinkPad-T420` (2011er ThinkPad), Linux Mint 22.3.
- Labels `self-hosted, Linux, X64`; GitHub-Runner **v2.337.0** unter dem Home des `actions`-Nutzers (`~/actions-runner`).
- Nutzer **`actions`**, **ohne sudo**; systemd `actions.runner.omegaflow-omegaflow.t420.service`.
- Sleep/Suspend/Hibernate **maskiert**, Lid-close ignoriert → dauerhaft an.
- Bereits gedrosselt: `Nice=10`, `CPUWeight=20`, `IOWeight=20` (systemd, soft/fair-share).

**Der XPS 13 (`johannes-XPS-13-9350`) ist der Arbeits-Rechner des Operators und trägt
keine Jobs** (Operator-Wort 2026-10-08: „auf meinem XPS13 dürfen sie auf keinen Fall laufen").

## Routing (gemessen 2026-10-10)

- **20 Workflows** fahren auf `runs-on: [self-hosted, Linux]` — die netz-lastigen CDN-Compiler
  (`cmb-cdn`, `gaia-xp-cdn`, die Ephemeriden-Compiler, `dust`, `wod`, `mpcobs-shard`, `tnbfits`,
  `twomass`, …). Sie profitieren, weil bei ihnen die **Bandbreite** der Engpass ist, nicht die CPU.
- **`ci-gate.yml`-`subset`** (`cargo test --lib`) fährt seit Operator-Wort 2026-10-10 („mach A")
  auf `ubuntu-24.04-arm` — GitHub-hosted, gratis + unbegrenzt für ein public Repo
  (`survey-2026-10-10-github-ci-cdn-optimierung.md:59`), mit per-SHA-Concurrency
  (`ci-gate-subset-${{ github.sha }}`). Der frühere t420-Tip (per-ref concurrency,
  `cancel-in-progress`) wurde im Commit-Burst dauerhaft abgebrochen; jetzt läuft jeder SHA
  parallel und bekommt ein echtes `subset`-Verdikt.
- **Sicherheit:** self-hosted nur für **trusted** Trigger. `ci-gate` läuft auf `push`/
  `workflow_dispatch` (kein `pull_request`) → kein ungeprüfter Fork-Code auf dem Rechner.
  Nie `pull_request_target` + Fork-Checkout auf self-hosted.

## Intelligente Drosselung (`bin/runner_throttle.sh`)

Der t420 teilt den Uplink mit dem Media-PC. Das Werkzeug (auf dem t420 als root zu installieren,
`install`) tut drei Dinge:

1. **CPU/IO (soft):** systemd-Drop-in `Nice=10`, `CPUWeight=20`, `IOWeight=20`, **`CPUQuota=150%`** —
   die Hälfte des 2c/4t-ThinkPads bleibt dem Operator, der Kernel bevorzugt bei Bedarf jeden
   anderen Prozess (fair-share; das ist die „intelligente" CPU-Seite ohne Skript).
2. **Netz-Kappe mit Headroom (Ingress):** Der schwere Verkehr ist der **Download** (Multi-GB-Ernten) —
   `tc` auf dem physischen Egress würde nur Uploads drosseln. Deshalb wird der gesamte Eingang von
   `wlp3s0` auf ein **IFB-Gerät** (`ifb0`) umgeleitet und dort per HTB gekappt: **Ceiling `MAX_KBIT`**.
   Der Runner nimmt nie den ganzen Downlink — der Media-PC behält Luft.
3. **Adaptiv (das Intelligente):** ein systemd-Dienst misst alle 5 s die **Round-Trip-Zeit zu einem
   WAN-Host** (Bufferbloat = gemessenes Symptom eines ausgelasteten Links) und regelt die Kappe
   zwischen `MIN_KBIT` und `MAX_KBIT` herunter/hoch — steigt die Latenz **≥ `HIGH_MS`** (Media
   streamt, Link satt), weicht der Runner zurück; fällt sie **≤ `LOW_MS`**, erholt er sich.
   `pause`/`resume` stoppen den Runner ganz für eine Media-Session. CPU-Grenzen werden **live** per
   `systemctl set-property` gesetzt — kein laufender Job wird gekillt.

Konfiguration: `/etc/default/runner-throttle` (`IFACE`, `MIN_KBIT`, `MAX_KBIT`, `GW`, `PROBE`,
`HIGH_MS`, `LOW_MS`). `status` zeigt Interface, Band, tc-Zustand und Latenz.

**Stand 2026-10-10 (gemessen auf dem t420):** `install` ausgeführt — Drop-in + Live-CPU
(`CPUQuotaPerSecUSec=1.5s`, `CPUWeight=20`, `IOWeight=20`), `runner-throttle-adaptive.service`
**active**, Ingress-Cap 20 Mbit auf `ifb0` (`wlp3s0`), RTT-Probe 5 ms; Runner-Dienst unangetastet
(läuft weiter).

## Grenzen (0 honored)

- **Der Media-PC ist ein eigenes Gerät** — der t420 sieht dessen Verkehr nicht. Die ehrliche,
  lokale Regel ist deshalb **Headroom lassen** (Kappe + Adaption über die gemessene Gateway-Latenz);
  die **wirklich** adaptive Stelle bei getrennten WLAN-Clients ist **QoS am Router** (Bufferbloat-
  Management, `cake`/SQM) — das ist Operator-Config, nicht t420-Config, und bleibt hier `pending`.
- Der Runner ist ein 2011er ThinkPad: für CPU-schwere Jobs **langsamer** als GitHub-hosted. Deswegen
  laufen dort die **netz-lastigen** Compiler und der kleine `--lib`-Subset, nicht die volle Suite.
- Ein Runner = ein Job. Der `subset`-Tip hält die Queue; bei Offline-Gerät hängt ein Required-Check
  (`pending`, nie fabriziert grün).
