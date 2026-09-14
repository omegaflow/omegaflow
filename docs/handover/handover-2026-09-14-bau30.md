<!--
  title: Handover — Bau & Code (Stand 2026-09-14, Bau30)
  session: Bau-Folge 30
  class: handover
  date: 2026-09-14
  sha256: 46581ab3030517cbd6c4a74db0edc478fdac4833d9cc5f9fa20ad7f57460429a
  status: live
-->
# Handover — Bau & Code (2026-09-14, Bau30)

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

## TE-Gate n=1000 — residual+KSG messen und #13 schließen

- Die Shift-Null ist gemessen-falsifiziert (CI 34860002879, alle sechs Zellen
  fallen): binned block 12,14 % / phase 8,57 % / shift 9,05 % (FPR > 8 %); KSG
  block 7,38 % (Anstieg 4,29pp) / phase 5,95 % (2,86pp) / shift 6,90 % (2,86pp).
  Der Rat (5-0, 2026-09-14) nannte die nächste Sprosse: den bereits gebauten
  Residual-Null messen. Gebaut: `gate_fpr_autocorrelation_residual_null_ksg_n_surr_100`
  (n=150, hält), `gate_fpr_autocorrelation_residual_null_ksg_n_1000` (#[ignore]),
  `residual_sweep_n1000` (KSG-FPR + OLS-Fallback-Zähler — ob `ols_fit_lagged_n`
  je auf `shuffle_series` fällt). Der residual+binned ist gemessen-falsifiziert
  (n=150, 2,85pp Anstieg bei D_Z=4) — gestrichen, nicht getragen. (Schritt:
  pushen wenn der Baum ruhig ist → `gh workflow run te-gate.yml` → `residual_sweep_n1000`
  + `gate_fpr_autocorrelation_residual_null_ksg_n_1000` lesen; hält (FPR ≤ 8 %,
  Anstieg ≤ 2pp, ols_resolved = 600/600) → das n=1000-Gate auf residual+KSG
  verankern, die block/phase/shift-n=1000-Gates streichen (git trägt ihr
  Scheitern), `gh issue close 13`; hält nicht → nächste Sprosse nach Rat —
  die Gate-Konstruktion, die Treiber/Ziel denselben AR1-Koeffizienten a teilen
  lässt, ist dann das Messobjekt.)

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
