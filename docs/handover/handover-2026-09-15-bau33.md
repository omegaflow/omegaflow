<!--
  title: Handover — Bau & Code (Stand 2026-09-15, Bau33)
  session: Bau-Folge 33
  class: handover
  date: 2026-09-15
  sha256: a3057ac45aa15a2c737988fbe889eee9a75732569ef8f2bd9eb14d1a027aa307
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

## Parser-Gap A — DSN-Binärdecoder (TNF-Sekundär-CHDO zuerst)

- **TNF (NH REX, `pdssbn.astro.umd.edu/holdings/pds4-nh_rex:plutocruise_tnf-v1.0/`)** —
  SFDU-Rahmen GEBAUT (`TnfSfdu`/`read_tnf_sfdu`/`scan_tnf_sfdus`, `src/archivar/odf.rs`,
  Test `nhrex_tnf_sfdu_framing_decodes`). Gemessen: `lunocc2012.tnf` = 5 SFDU-Blockbreiten
  182/378/144/220/236; das TNF trägt keine 9-Wort-Orbit-Records (das ist ODF); die Observable
  liegt im Sekundär-CHDO (66 B für DT0). Offen: Sekundär-CHDO-Payload Ende-zu-Ende dekodieren.
  (Schritt: `lunocc2012.lblx` für das DT0-Sekundär-CHDO-Layout lesen, gegen die gemessenen 66 B dekodieren.)
- **ODF (Juno, `atmos.nmsu.edu/PDS/data/jnogrv_1001/`)** — der TRK-2-34-ODF-Record-Decoder
  steht (`odf::parse_odf`/`orbit_record`, produktiv via `galileo_odf_compiler` +
  `pioneer11_odf_compiler`; `sources.φ` trägt `galileo_odf.bin`). Offen: Juno-Rohdatei →
  `parse_odf` (erste Messung: direkt oder SFDU-gerahmt), dann Juno-Compiler + CDN + Konsument.
  (Schritt: kleinste Juno-ODF per `curl -r` + `xxd` messen, gegen `parse_odf` halten.)
- **ODR (Voyager, `pds-rings.seti.org/pds4/bundles/voyager_rss_raw/`)** — `galileo_odr.rs`
  liest 2666-B-Galileo-ODR; Voyager ist 5056 B. Offen: 5056-B-Reader.
  (Schritt: kleinste Voyager-ODR-Record messen, Header-Geometrie benennen.)
- **RSS (Voyager, `pds-ppi.igpp.ucla.edu/mission/Voyager/VG2/RSS`)** — offen; Geschwister
  SPDF VG1/VG2 Saturn (UNIVAC-1108, 26 Tracking-Records/Block). (Schritt: Format messen.)
- **DRS-FITS (LISA, `heasarc.gsfc.nasa.gov/FTP/lpf/data/fits/`)** — der `fits.rs`-Arm (Gap B)
  steht vollständig; offen ist die DRS-Semantik. (Schritt: ein echtes DRS-FITS per `curl -r`
  messen, HDU-Struktur gegen `FitsHeader` benennen.)

## TE-Gate n=1000 — nächste Sprosse, #13 offen

- Lauf 34896734026: ALLE n=1000-Gates fallen (residual+KSG a=0,9 13,33 %, Anstieg 10,71 pp;
  `ols_resolved=600/600`). Der `||`-Zweig in `te-gate.yml` verschluckt den Testausfall.
  Zähler entkontaminiert (GEBAUT: `GateCell.tp` + true_edge-Ausschluss, `src/mathematikerin/te.rs`).
  Das bias-korrigierte KSG (`transfer_entropy_ksg_conditional_n_bc`, lokale Y_past-Permutation)
  wurde ZWEIMAL gemessen degeneriert (FN-Kalibrier-Gate 0/20, Konstante ≈ ψ(k_eff)) → zurückgerollt.
  Offen: nächste Sprosse nach Rat — restricted-permutation Null (Y innerhalb Bins seiner eigenen
  Vergangenheit permutieren) oder eine tragfähige Bias-Korrektur; dann das n=1000-Gate auf dem
  entkontaminierten Zähler in CI messen. (Schritt: `gh workflow run te-gate.yml`, dann
  `gate_fpr_autocorrelation_residual_null_ksg_n_1000` lesen; `gh issue close 13` erst bei einem
  Gate, das hält.)

## RINEX/CORS — Parser gebaut, Quelle offen

- Der RINEX-2.11- + Hatanaka-Parser ist GEBAUT und grün (`src/archivar/rinex.rs`:
  `parse_rinex_obs`/`is_hatanaka`/`crx2rnx`; `cors_compiler`/`cors_rinex_compiler`; 15 lib +
  3 + 1 Tests grün, u. a. `collect_builds_records_from_rinex_211_obs`). Der `blocked parser-def
  rinex`-Eintrag in `phi/blocked_sources.φ` ist damit fehlklassifiziert (die Notiz „parse_rinex_obs
  ist RINEX-3-geformt" ist überholt). Offen: die CORS-Beobachtungsquelle (`s3://noaa-cors-pds`,
  `.24o.gz` RINEX-2.11 / `.24d.gz` CRINEX) registrieren + CDN + Konsument. (Schritt:
  `blocked_sources.φ`-Eintrag richtigstellen, `cors_compiler` an einen `sources.φ`-Block binden.)

## Die 14 akzeptierten Quellen — mergen (TAP-Tafeln vorher messen)

- Der Draft `phi/pipeline/research/agent_output/sources14_2026-09-15.φ` ist NICHT im Baum.
  Format-Register gemessen: `superdarn_fitacf` + `nexrad_level2` + `iss_lis.bin` sind registriert;
  `atdf`, `gk2a_ami`, `goes_abi`, `himawari_hsd`, `gdp_drifter`, `lis_otd` NICHT — obwohl die
  Compiler-Arme existieren (`gk2a_ami_compiler`, `goes_abi_compiler`, `himawari_hsd_compiler`,
  `gdp_drifter_compiler`, `iss_lis_compiler`, `atdf.rs`). Offen: die 14 Blöcke mergen; die TAP-Tafeln
  vor dem Merge probe-verifizieren. (Schritt: `tap_compiler` je TAP-Tafel, dann `sources.φ`-Block
  je Quelle mit Feld + Kraft + Konsument.)

## GRACE-FO L1B — Real-Granulat-Verifikation

- `tar_gz_yaml`-Streaming GEBAUT (`gunzip_tar_members` + `tar_gz_yaml_to_json` in
  `src/archivar/inflate.rs`/`extract.rs`); synthetische Tests grün. Offen: Verifikation am echten
  142-MB-Granulat (`real_gracefo_l1b_tarball_parses_a_member`, `OMEGAFLOW_GRACE_TGZ`, heavy → CI).
  (Schritt: den ignored Test in CI mit geladenem Granulat laufen lassen.)

## Binding — 62 verloren, Arbeitssatz = 107er-Ledger

- Rat 2026-09-15 (5-0): die „62 Kandidaten" sind NICHT rekonstruierbar — der bau31-Ledger war ab
  2026-09-11 untracked und ist nicht in git; die Zahl existiert nirgends im Baum (`phi/pipeline/ledger.φ`
  trägt 107 `kandidat`-Einträge: 49 verifiziert / 36 ausstehend / 17 parser-gap / 5 void; das einzige
  „Tier 1/2" ist ein fremder astroquery-Import in der alten `phi/pipeline/queue/master.φ`, 17 Quellen).
  Eine rekonstruierte 62er-Liste wäre fabrizierte Spezifität (A = A: ein verlorenes Datum wird als
  verloren benannt, nie rekonstruiert). Der Operator-Wille bleibt als Politik gültig: alle Kandidaten
  bauen, Konsumenten-Bindung bleibt benannter offener Punkt, kein `sources.φ`-Block ohne
  Membran-Konsument, kein Debt. Arbeitssatz = der 107er-Ledger (Stand 2026-09-15); Tor 1 je Kandidat
  gemessen (grep des Lesepunkts). (Schritt: Marathon von Parser-Gap A fortsetzen; die Consumer-Benennung
  bleibt beim Operator.)

## Mail-Fang — Deployment offen (Operator-Wort)

- Der Webhook-Verlust-Fix (Retry + KV-Puffer + Cron-Drain in `cloudflare/email_worker.js` +
  `cloudflare/wrangler.toml`; Dedup über `seen_ids.φ` in `tools/service/src/bin/smail_recv.rs`) ist
  gebaut, nicht deployed. (Schritt: `wrangler kv namespace create MAIL_QUEUE` → Id in `wrangler.toml`
  eintragen → `wrangler deploy`; `WEBHOOK_URL`/`WEBHOOK_TOKEN` prüfen.)

## Membran

- ESP32-Modul — on hold (Operator-Wort, 2026-09-13): das Gerät und sein Flash kommen zuletzt.
  (Schritt: ruht beim Operator — er löst die Wiedervorlage aus.) BOM: `docs/specs/mantis-shrimp-bom.md`.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene Abschluss-Check mit
Commit und Push (`/commit`).
