<!--
  title: Handover — Bau & Code (Stand 2026-09-14, Bau31)
  session: Bau-Folge 31
  class: handover
  date: 2026-09-14
  sha256: dc8071716525ab30e376c46d0f6a0597dfbf49575258f90e6f3ab836e6bdd8a5
  status: live
-->
# Handover — Bau & Code (2026-09-14, Bau31)

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

## Parser-Gap-Programm C→A (Rats-Verdikt 5-0, 2026-09-14)

Der Compiler-Marathon (alle 62 Ledger-Kandidaten, Binding pending) prallte in
Welle 1 auf sechs Binärformate; sie fallen auf drei geteilte Parser-Gaps, keine
sechs Compiler. Der Rat ordnete B → C → A; kein Gap descoped; eine Quelle
descoped. Der B-FITS-Arm steht (git trägt ihn); `inflate.rs`
(gunzip/tar_members) steht bereits — nur der Dispatch fehlt.

- **C — tar.gz+YAML** (die nächste Sprosse): `inflate.rs` antwortet; `.tgz`
  holen, YAML-Prolog am `# End of YAML header` schneiden, ASCII-Serie parsen.
  Entblockt GRACE-FO L1B (range m / m/s / m/s² — das reine SI-Gravity;
  EARTHDATA_EDL_TOKEN liegt in .secrets.local, kein key-needed). (Schritt: den
  `fits`-Arm als Muster nehmen — `extract.rs`-Zweig + `main_flow.rs`-Branch,
  `inflate::tar_members` + Prolog-Schnitt am `# End of YAML header`; `cargo
  check` 0/0.)
- **A — DSN-Binärdecoder** (pending, erste Stufe benannt): ein Hexdump — die
  kleinste NH-REX-TNF-Datei (`pdssbn…/pds4-nh_rex:plutocruise_tnf`, TRK-2-34,
  182-Byte-SFDU) hexdumpen und die Felder gegen das TRK-2-34-Layout benennen.
  Staging TRK-2-34 → ODF/TNF (Juno, 36-Byte-MSB) → Voyager-Basisband (.ODR,
  5056-Byte, 14,36 GB, zuletzt). (Schritt: `curl -r` + `xxd` der kleinsten TNF,
  Feldnamen dokumentieren.)

## Disposition Welle 1 — sechs parser-def-Verdikte (nach ruhigem Baum)

- Die sechs Gravitation/Doppler-Proben (Juno ODF/TNF/RSR, LISA-PF DRS-FITS,
  NH-REX TNF, Voyager ODR, Voyager RSS, GRACE-FO L1B) tragen `parser-def` mit
  dem Dateityp; die Verdikte liegen in den Rat-/Proben-Notizen und gehen in
  `blocked_sources.φ`, sobald die Register-Migration eingelaufen ist. (Schritt:
  nach ruhigem Baum `blocked_sources.φ` + `ledger.φ` aktualisieren.)

## Descope — Voyager raw-ADC ODR

- `pds-rings.seti.org/voyager_rss_raw` — Befund: unitless ADC-Zählwerte, das
  Doppler/Opazitäts-Produkt ist NICHT im Bundle (der Faden bleibt; findet eine
  spätere Session das Produkt anderswo, re-registriert die Quelle dort). Das
  PPI-VG2/RSS-Basisband bleibt pending (das Signal selbst steckt in den Records).
  (Schritt: Befund als Handover-Zeile tragen, keine Register-Schreibarbeit.)

## Kraft-Korrektur (Register-Datum)

- Radio-Science-Doppler ist `em` (der Sensor ist der Funkempfänger), nicht
  `gravity` — Juno/NH-REX/Voyager; `gravity` trägt nur GRACE-FO + LISA-PF (dort
  IST die Messung die Schwerkraft). Die Notizen „gravity/Doppler" sind zu
  korrigieren. (Schritt: mit der Welle-1-Disposition in `blocked_sources.φ`.)

## Binding pending — die 62 Kandidaten

- Operator-Wort: alle 62 (Tier 1 + Tier 2) bauen, Konsumenten-Bindung bleibt
  als benannter offener Punkt — kein `sources.φ`-Block ohne Membran-Konsument,
  kein Debt. (Schritt: beim Operator — er benennt die Feld-Konsumenten.)

## TE-Gate n=1000 — residual+KSG messen und #13 schließen

- Die Shift-Null ist gemessen-falsifiziert (CI 34860002879, alle sechs Zellen
  fallen). residual+KSG ist die nächste Sprosse; der Lauf 34896734026 ist
  angestoßen (main). (Schritt: `residual_sweep_n1000` +
  `gate_fpr_autocorrelation_residual_null_ksg_n_1000` lesen; hält (FPR ≤ 8 %,
  Anstieg ≤ 2pp, ols_resolved = 600/600) → n=1000-Gate auf residual+KSG
  verankern, block/phase/shift-Gates streichen, `gh issue close 13`; hält
  nicht → nächste Sprosse nach Rat.)

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
