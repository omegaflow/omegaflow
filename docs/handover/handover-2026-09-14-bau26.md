<!--
  title: Handover — Bau & Code (Stand 2026-09-14, Bau26)
  session: Bau-Folge
  class: handover
  date: 2026-09-14
  sha256: 5a28b42a36a84d56eac9413d1f8e0b390458db9a3874104c5ed2446fffab09e0
  status: live
-->
# Handover — Bau & Code (2026-09-14, Bau26)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt.

## Clippy — gemessen, keine Decke

- `cargo clippy --all-targets -- -D warnings` ergibt 612 Funde; `unwrap_used` +
  `cast_precision_loss` ergeben 105. `cast_precision_loss` trifft legitime
  i64/usize→f64-Casts (Zeitstempel, Zählungen — Kern des Feldsystems); die
  Default-Lints widersprechen dem 0-Kanon (`match can be simplified with
  .unwrap_or_default()` schlägt genau die Fabrikation vor, die der commit_gate
  verbietet). Schritt: `cargo clippy --all-targets -- -D clippy::unwrap_used`
  einzeln zählen, kuratierte Deny-Liste festlegen, dann Zeile in `ci-check.yml`
  — nie `-D warnings` als Decke.

## archive_search — EarthData-Hook

- Der Token-Hook (`token.rs:21-22`) liest `EARTHDATA_USER`/`EARTHDATA_PASS`
  (Basic-Auth-Tausch); `.secrets.local` trägt nur `EARTHDATA_EDL_TOKEN`
  (682 Zeichen), das live 401 liefert (stale); der Hook feuert zudem nur auf 401,
  nicht auf 403/307 (das, was EarthData-Ports tatsächlich liefern). Schritt:
  frisches EDL-Token (urs-Profil) oder `EARTHDATA_USER`/`EARTHDATA_PASS` in die
  secrets; dann `token.rs` um den direkten `EARTHDATA_EDL_TOKEN`-Pfad erweitern
  und den 403/307-Auslöser nachziehen.

## ISC-EHB — Grammatik gewogen

- Das Legacy-CGI `cgi-bin/web-db-run` ist tot: HTML-Fehlerseite („request BULLETIN
  is not available"), `out_format=CSV` wird ignoriert. Der FDSN-Weg
  `https://www.isc.ac.uk/fdsnws/event/1/query?format=text` liefert Pipe-getrenntes
  `text/plain` mit denselben Feldern. Schritt: `phi/sources.φ` Zeile 2533 auf den
  FDSN-Endpunkt umzielen (SOURCE_PORT.md).

## Membran

- ESP32-Modul — on hold (Operator-Wort, 2026-09-13): das Gerät und sein Flash
  kommen zuletzt. Schritt, wenn an der Reihe: `espflash` installieren, Gerät
  anstecken, nn-Strom als `nn=<ms>` am ttyACM. BOM: `docs/specs/mantis-shrimp-bom.md`.

## Mail-Fang — Deployment offen (Operator-Wort)

- Der Webhook-Verlust-Fix (Retry + KV-Puffer + Cron-Drain in
  `cloudflare/email_worker.js` + `wrangler.toml`; Dedup über `seen_ids.φ` in
  `tools/service/src/bin/smail_recv.rs`) ist gebaut, nicht deployed. Schritt:
  `wrangler kv namespace create MAIL_QUEUE` → Id in `wrangler.toml` eintragen →
  `wrangler deploy`; `WEBHOOK_URL`/`WEBHOOK_TOKEN` prüfen.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
