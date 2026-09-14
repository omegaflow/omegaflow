<!--
  title: Handover — Bau & Code (Stand 2026-09-14, Bau29)
  session: Bau-Folge 29
  class: handover
  date: 2026-09-14
  sha256: eb7d79e48a7b6a12d275769acee4dcbda3d7238bf135026f71826645ba4ec374
  status: live
-->
# Handover — Bau & Code (2026-09-14, Bau29)

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

## TE-Gate n=1000 — #13 mit dem Shift-Null-Ergebnis schließen

- Die KSG-Entscheidung ist gefallen und gebaut: die Messung (CI 34839106676)
  ergab bei n=1000, a=0,9 FPR 7,38 % block / 5,95 % phase, aber der Anstieg bleibt
  4,28 pp / 2,85 pp → KSG hält das Gate nicht. Die Shift-Null ist als zweite
  Sprosse auf main gebaut (`9bb55a8`: `gate_fpr_autocorrelation_shift_null_{binned,ksg}_n_1000`
  + `shift_sweep_n1000`). Offen: die Shift-Null als n=1000-Gate bestätigen und #13
  mit dem Ergebnis schließen. Der Lauf `34860002879` (branch `ksg-measure`,
  tip `dfd7ac1`, in_progress) misst denselben Bau unter anderen Hashes — nicht
  duplizieren, erst dessen Ergebnis lesen. (Schritt: `gh run view 34860002879 --log`
  lesen; hält Shift-Null (FPR ≤ 8 %, Anstieg ≤ 2 pp) → das n=1000-Gate auf
  Shift-Null verankern, die block/phase n=1000-Gates streichen (git trägt
  ihr Scheitern), `gh issue close 13`; hält nicht → die nächste Sprosse nach Rat.)

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
