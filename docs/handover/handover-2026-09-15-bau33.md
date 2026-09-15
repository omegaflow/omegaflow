<!--
  title: Handover — Bau & Code (Stand 2026-09-15, Bau33)
  session: Bau-Folge 33
  class: handover
  date: 2026-09-15
  sha256: bb83c0375aa903c02b753c3deb4f23773aafb6a8bc49235b61f68c27ff86126e
  status: live
-->
# Handover — Bau & Code (2026-09-15, Bau33)

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

## Parser-Gap A — DSN-Binärdecoder (TNF-Rahmen steht, Nutzlast offen)

- NH-REX TNF (`pdssbn.astro.umd.edu/holdings/pds4-nh_rex:plutocruise_tnf-v1.0/`):
  SFDU-Rahmen GEBAUT 2026-09-15 — `TnfSfdu`/`read_tnf_sfdu`/`scan_tnf_sfdus` in
  `src/archivar/odf.rs`, Test `nhrex_tnf_sfdu_framing_decodes` gegen die
  gemessenen Head-Bytes von `lunocc2012.tnf` (35 770 168 B, 5 SFDU-Blockbreiten
  182/378/144/220/236 B). Widerlegt (gemessen): das TNF trägt KEINE
  9-Wort-/36-Byte-Orbit-Records — `orbit_record` dekodiert das ODF; die
  TNF-Observable liegt im Sekundär-CHDO (66 B für DT0). Offen:
  Sekundär-CHDO-Payload Ende-zu-Ende dekodieren, dann Juno ODF/TNF
  (`atmos.nmsu.edu/PDS/data/jnogrv_1001/`) → Voyager-Basisband (.ODR, 5056 B).
  (Schritt: `lunocc2012.lblx` für das DT0-Sekundär-CHDO-Layout lesen und gegen
  die gemessenen 66 B dekodieren.)

## TE-Gate n=1000 — Zähler entkontaminiert, nächste Sprosse benannt (#13 offen)

- Lauf 34896734026 gelesen: ALLE n=1000-Gates fallen (residual+KSG a=0,9
  13,33 %, Anstieg 10,71 pp; block/phase/shift ebenso; `ols_resolved=600/600`).
  Der `||`-Zweig in `te-gate.yml` verschluckt den Testausfall — ein
  success-Step beweist das Halten nicht. Rat 2026-09-15: Zähler entkontaminieren
  (DGP-True-Edges als `tp`, nie `fp`) — GEBAUT (`GateCell.tp` + true_edge-
  Ausschluss in `gate_fpr_cells_from`, `src/mathematikerin/te.rs`, 15 Zeilen).
  Das bias-korrigierte KSG (`transfer_entropy_ksg_conditional_n_bc`, lokale
  Y_past-Permutation, M=16) wurde ZWEIMAL gemessen degeneriert (FN-Kalibrier-
  Gate 0/20, Konstante ≈ ψ(k_eff); die fixed-eps-Permutation lässt n_xyc
  invariant) → ehrlich zurückgerollt. Offen: nächste Sprosse — restricted-
  permutation Null (Y innerhalb Bins seiner eigenen Vergangenheit permutieren)
  oder eine tragfähige Bias-Korrektur; dann das n=1000-Gate auf dem
  entkontaminierten Zähler in CI messen. `gh issue close 13` erst, wenn ein
  Gate auf einem Zähler hält, der nur zählt, was er behauptet. (Schritt:
  `gh workflow run te-gate.yml`, dann
  `gate_fpr_autocorrelation_residual_null_ksg_n_1000` auf dem entkontaminierten
  Zähler lesen — Heavy-Lauf, nie lokal.)

## GRACE-FO L1B — Streaming gebaut, Real-Granulat offen

- `tar_gz_yaml` GEBAUT 2026-09-15: `gunzip_tar_members` (streamender Tar-Scan
  über `gunzip_stream`, nur das gewünschte Mitglied im Speicher) und
  `tar_gz_yaml_to_json` (Pfad-Streaming) in `src/archivar/inflate.rs` +
  `src/archivar/extract.rs`; der 720-MB-Vollpuffer fällt. Tests
  `tar_gz_yaml_format_extracts_member_last_row` +
  `gunzip_tar_members_streams_only_wanted_member` grün. Offen: Verifikation am
  echten 142-MB-Granulat (`real_gracefo_l1b_tarball_parses_a_member`,
  `OMEGAFLOW_GRACE_TGZ`, heavy → CI). (Schritt: den ignored Test in CI mit
  geladenem Granulat laufen lassen.)

## Binding — 62 verloren, Arbeitssatz = 107er-Ledger

- Rat 2026-09-15 (5-0): die „62 Kandidaten" sind NICHT rekonstruierbar — der
  bau31-Ledger war ab 2026-09-11 untracked und ist nicht in git; die Zahl
  existiert nirgends im Baum (`phi/pipeline/ledger.φ` trägt 107
  `kandidat`-Einträge: 49 verifiziert / 36 ausstehend / 17 parser-gap / 5 void;
  das einzige „Tier 1/2" ist ein fremder astroquery-Import in der alten
  `phi/pipeline/queue/master.φ`, 17 Quellen). Eine rekonstruierte 62er-Liste
  wäre fabrizierte Spezifität (A = A: ein verlorenes Datum wird als verloren
  benannt, nie rekonstruiert). Der Operator-Wille bleibt als Politik gültig:
  alle Kandidaten bauen, Konsumenten-Bindung bleibt benannter offener Punkt,
  kein `sources.φ`-Block ohne Membran-Konsument, kein Debt. Arbeitssatz = der
  107er-Ledger (Stand 2026-09-15); Tor 1 je Kandidat gemessen (grep des
  Lesepunkts). (Schritt: Marathon von Parser-Gap A fortsetzen; die
  Consumer-Benennung bleibt beim Operator.)

## Mail-Fang — Deployment offen (Operator-Wort)

- Der Webhook-Verlust-Fix (Retry + KV-Puffer + Cron-Drain in
  `cloudflare/email_worker.js` + `cloudflare/wrangler.toml`; Dedup über
  `seen_ids.φ` in `tools/service/src/bin/smail_recv.rs`) ist gebaut, nicht
  deployed. (Schritt: `wrangler kv namespace create MAIL_QUEUE` → Id in
  `wrangler.toml` eintragen → `wrangler deploy`; `WEBHOOK_URL`/`WEBHOOK_TOKEN`
  prüfen.)

## Membran

- ESP32-Modul — on hold (Operator-Wort, 2026-09-13): das Gerät und sein Flash
  kommen zuletzt. (Schritt: ruht beim Operator — er löst die Wiedervorlage aus.)
  BOM: `docs/specs/mantis-shrimp-bom.md`.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
