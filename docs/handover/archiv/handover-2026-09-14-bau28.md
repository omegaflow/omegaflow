<!--
  title: Handover — Bau & Code (Stand 2026-09-14, Bau28)
  session: Bau-Folge 28
  class: handover
  date: 2026-09-14
  sha256: dacb99668531439bbf6c519b522476b657c778c2e99f7d21767ebf17182e8d7f
  status: live
-->
# Handover — Bau & Code (2026-09-14, Bau28)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht.

## TE-Gate n=1000 — die KSG-Entscheidung ist ungemessen

- Der Block-Sweep (CI 34829602389) hat die Block-Längen-Fixierung falsifiziert:
  bei n=1000, a=0,9, d_z=4 steigt die FPR nie unter 8,57 % (Block 32/48), der
  Anstieg nie unter ~4,3pp; Phase- und Block-Null lecken beide am Binned-Schätzer.
  Der Rat (5-0, 2026-09-14): KSG bei n=1000 messen, Schwellen unangetastet. Die
  KSG-Gates (`gate_fpr_autocorrelation_{block,phase}_null_ksg_n_1000`) + `ksg_sweep_n1000`
  sind gebaut (`src/mathematikerin/te.rs`, `.github/workflows/te-gate.yml`), aber
  un-gepusht. **Schritt:** pushen (wenn der Baum ruhig ist), `gh workflow run
  te-gate.yml`, das `ksg_sweep_n1000`-Ergebnis lesen; hält KSG → die n=1000-Gates
  auf KSG umstellen; hält es nicht → die Shift-Null als nächste Sprosse (der Rat);
  #13 mit dem Ergebnis schließen.

## Push — der Baum ist nicht ruhig

- Vier eigene Commits liegen un-gepusht (`541cbda` te-gate, `b762c0e` readers,
  `0a3fbda` tar+LZW, `fea9973` las-laz→pending) hinter fremden Commits
  (`8bb9dce`, `7dfccbc`, `29fdc88`) und fremder uncommitteter Arbeit
  (`src/archivar/{extract,geo,hsd,noaa_nodd,range,tests,zeuge}.rs`, `phi/sources.φ`,
  die Harvest-Compiler). **Schritt:** warten, bis der Baum ruhig ist, dann
  `git push`; danach `git rev-parse HEAD` == `git rev-parse origin/main`.

## Mail-Fang — Deployment offen (Operator-Wort)

- Der Webhook-Verlust-Fix (Retry + KV-Puffer + Cron-Drain in
  `cloudflare/email_worker.js` + `wrangler.toml`; Dedup über `seen_ids.φ` in
  `tools/service/src/bin/smail_recv.rs`) ist gebaut, nicht deployed. **Schritt:**
  `wrangler kv namespace create MAIL_QUEUE` → Id in `wrangler.toml` eintragen →
  `wrangler deploy`; `WEBHOOK_URL`/`WEBHOOK_TOKEN` prüfen.

## Membran

- ESP32-Modul — on hold (Operator-Wort, 2026-09-13): das Gerät und sein Flash
  kommen zuletzt. BOM: `docs/specs/mantis-shrimp-bom.md`.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
