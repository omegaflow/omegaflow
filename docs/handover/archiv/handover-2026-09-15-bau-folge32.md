<!--
  title: Handover — Bau & Code (Stand 2026-09-15, Bau32)
  session: Bau-Folge 32
  class: handover
  date: 2026-09-15
  sha256: 9fa27ba7a295c6c8a195fe2459f769c8dc79e18dca40b5ccba58928dc211eb21
  status: live
-->
# Handover — Bau & Code (2026-09-15, Bau32)

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

## Parser-Gap A — DSN-Binärdecoder (erste Stufe benannt)

- Die kleinste NH-REX-TNF-Datei (`pdssbn.astro.umd.edu/holdings/pds4-nh_rex:plutocruise_tnf-v1.0/`,
  TRK-2-34, 182-Byte-SFDU) hexdumpen und die Felder gegen das TRK-2-34-Layout
  benennen. Staging TRK-2-34 → ODF/TNF (Juno `atmos.nmsu.edu/PDS/data/jnogrv_1001/`)
  → Voyager-Basisband (.ODR, 5056-Byte, zuletzt). Die fünf parser-def-Verdikte
  stehen in `phi/blocked_sources.φ`; Kraft korrigiert (Juno/NH-REX/Voyager `em`,
  LISA-PF `gravity`). (Schritt: `curl -r` + `xxd` der kleinsten TNF, Feldnamen
  dokumentieren.)

## GRACE-FO L1B — Streaming statt 720 MB in-memory

- Der `tar_gz_yaml`-Parser (Bau32, `extract.rs`) gunzippt die 142-MB-Tagesgranule
  vollständig in-memory (720 MB entpackt, bit-weiser Inflater) und scannt das
  Tar; erst danach wird nur das `KBR1B`-Mitglied geparst. Die Quelle ist
  registriert (`phi/sources.φ`, `format tar_gz_yaml`, Felder
  `gracefo_kbr_range_rate_m_s` + `gracefo_kbr_range_accl_m_s2`, `at earth`,
  gravity, statische gemessene Granule 2026-08-14). (Schritt:
  `inflate::gunzip_stream` + ein streamender Tar-Scan, der nur das gesuchte
  Mitglied hält — `tar_gz_yaml_to_json` in `src/archivar/extract.rs`.)

## TE-Gate n=1000 — residual+KSG messen und #13 schließen

- Die Shift-Null ist gemessen-falsifiziert (CI 34860002879, alle sechs Zellen
  fallen). Der Lauf 34896734026 ist angestoßen (main). (Schritt:
  `residual_sweep_n1000` + `gate_fpr_autocorrelation_residual_null_ksg_n_1000`
  lesen; hält (FPR ≤ 8 %, Anstieg ≤ 2pp, ols_resolved = 600/600) → n=1000-Gate
  auf residual+KSG verankern, block/phase/shift-Gates streichen,
  `gh issue close 13`; hält nicht → nächste Sprosse nach Rat.)

## Binding pending — die 62 Kandidaten

- Operator-Wort: alle 62 (Tier 1 + Tier 2) bauen, Konsumenten-Bindung bleibt als
  benannter offener Punkt — kein `sources.φ`-Block ohne Membran-Konsument, kein
  Debt. (Schritt: beim Operator — er benennt die Feld-Konsumenten.)

## Mail-Fang — Deployment offen (Operator-Wort)

- Der Webhook-Verlust-Fix (Retry + KV-Puffer + Cron-Drain in
  `cloudflare/email_worker.js` + `cloudflare/wrangler.toml`; Dedup über `seen_ids.φ` in
  `tools/service/src/bin/smail_recv.rs`) ist gebaut, nicht deployed. (Schritt:
  `wrangler kv namespace create MAIL_QUEUE` → Id in `wrangler.toml` eintragen →
  `wrangler deploy`; `WEBHOOK_URL`/`WEBHOOK_TOKEN` prüfen.)

## Membran

- ESP32-Modul — on hold (Operator-Wort, 2026-09-13): das Gerät und sein Flash
  kommen zuletzt. (Schritt: ruht beim Operator — er löst die Wiedervorlage aus.)
  BOM: `docs/specs/mantis-shrimp-bom.md`.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
