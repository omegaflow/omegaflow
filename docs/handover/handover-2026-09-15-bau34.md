<!--
  title: Handover — Bau & Code (Stand 2026-09-15, Bau34)
  session: Bau-Folge 34
  class: handover
  date: 2026-09-15
  sha256: 347566d3ab783b7f6337df603c73176fbe7953fc5f4e31e36288b24a3d82ed1f
  status: live
-->
# Handover — Bau & Code (2026-09-15, Bau34)

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

## TE-Gate n=1000 — restricted-permutation Null degeneriert, #13 offen

- Lauf 34896734026: ALLE n=1000-Gates fallen (residual+KSG a=0,9 13,33 %, Anstieg 10,71 pp).
  Der `||`-Zweig in `.github/workflows/te-gate.yml` ist repariert (jetzt `if: failure()`-Step —
  der Testausfall wird rot, das Issue-Skript meldet zusätzlich, statt den Ausfall zu verschlucken).
  Die restricted-permutation Null (`TeNull::RestrictedPermutation`, Y innerhalb von Bins der eigenen
  Vergangenheit permutiert) wurde GEBAUT und gemessen degeneriert: das n=150-Gate fällt schon
  (FPR 13,78 % bei a=0,9 D_Z=4, Anstieg 4 pp über a) → zurückgerollt wie das bias-korrigierte KSG.
  Offen: die nächste Sprosse — feinere Bin-Auflösung oder volle Y-Vergangenheits-Einbettung — dann
  das n=1000-Gate in CI messen. (Schritt: `src/mathematikerin/te.rs` Null-Konstruktion iterieren,
  `gh workflow run te-gate.yml`; `gh issue close 13` erst bei einem Gate, das hält.)

## Parser-Gap A — DRS-FITS offen; TNF/ODF/ODR gebaut, warten auf Konsument (Tor 1)

- DRS-FITS (LISA, `heasarc.gsfc.nasa.gov/FTP/lpf/data/fits/`): gemessen (2026-09-15) — Primary
  NAXIS=0 + BINTABLE `HOUSEKEEPING` (116 Spalten); die Science-HDU `SCI_SCIENCE_1Hz` (140 Spalten)
  liegt in größeren Dateien; kein Feld trägt m/s² (differentielle Beschleunigung ist aus
  LTP1/LTP2-Positionen m bzw. LTP-Kräften N zu bilden). Der `fits.rs`-Arm liest die Kette; zwei
  Lücken: `FitsImage::parse` weist NAXIS=0 ab, `TUNIT` wird nicht gelesen. (Schritt:
  `src/archivar/fits.rs` NAXIS=0-Primary + TUNIT, dann DRS-Semantik.)
- TNF (NH REX) Stufe 2 GEBAUT: `tnf_dt0` in `src/archivar/odf.rs` dekodiert die Uplink-Carrier-Phase
  (verschachteltes Daten-CHDO Typ 10 Länge 76: hi/lo/frac_phs_cycles + ramp_freq 7,15 GHz X-Band).
  (Schritt: TNF-Compiler + Membran-Konsument — der Konsument bleibt beim Operator.)
- ODF Juno: `parse_odf` trägt die Rohdatei (60 records gemessen, GRV_JUGR_2017033_…);
  `juno_odf_compiler` gebaut. (Schritt: sources.φ-Block + CDN + Konsument — Konsument beim Operator.)
- ODR Voyager: `src/archivar/voyager_odr.rs` liest den 5056-B-Record (RSC-11-6, 56-B-Header,
  5000 8-Bit-Samples, BCD-Zeit-Tag); RSS teilt den Reader. (Schritt: Konsument — Operator.)

## Die 14 akzeptierten Quellen — Draft verloren, Leser fehlen

- Der Draft `phi/pipeline/research/agent_output/sources14_2026-09-15.φ` ist NICHT im Baum (das ganze
  Verzeichnis `phi/pipeline/research/` fehlt) — die Zahl 14 ist verloren, nicht rekonstruierbar
  (A = A). Messbar sind 6 nicht registrierte Kandidaten: `gk2a_ami` (GKA1), `goes_abi` (GAB1),
  `himawari_hsd` (kein Magic), `gdp_drifter` (GDPT), `atdf` (PASF/GASR), `lis_otd` (kein Compiler).
  Befund: GKA1/GAB1/GDPT existieren nur im Compiler — KEIN Leser in `src/`, ein sources.φ-Block machte
  die Assets nicht lesbar; zwei Compiler-Ziele sind tot (iss_lis `ghrc.nasa.gov` DNS tot →
  `data.ghrc.earthdata.nasa.gov`; gdp `gdac.aoml.noaa.gov` DNS tot → `www.aoml.noaa.gov/phod/gdp/`).
  Eine Parallel-Session hat 2026-09-15 vier ANDERE TAP-Quellen gemergt (ESO VHS Ks, JVO akari/irsf,
  NOIRLab ls_dr10). (Schritt: je Kandidat den Leser in `src/archivar/` bauen, tote Routen umstellen,
  dann sources.φ-Block je Quelle mit Konsument.)

## GRACE-FO L1B — CI-Workflow steht, Dispatch offen

- `gunzip_tar_members`/`tar_gz_yaml_to_json` grün; der ignored Test
  `real_gracefo_l1b_tarball_parses_a_member` ist jetzt in `.github/workflows/gracefo-l1b-verify.yml`
  verdrahtet (Download `podaac-ops-cumulus-protected` via `curl -L` + EDL-Bearer, gemessen 206).
  (Schritt: nach Push `gh workflow run gracefo-l1b-verify.yml`, das Ergebnis lesen.)

## RINEX/CORS — Eintrag richtiggestellt, Konsument offen

- `blocked_sources.φ`-Eintrag ist jetzt `pending` (der Parser `parse_rinex_obs`/`is_hatanaka`/
  `crx2rnx` ist gebaut und grün; `cors_compiler`/`cors-cdn.yml` stehen). Der Rat hält den
  sources.φ-Block bis zur Konsument-Benennung zurück. (Schritt: Konsument benennen — Operator.)

## Binding — Konsument-Benennung bleibt beim Operator

- Alle Kandidaten bauen, Konsumenten-Bindung bleibt benannter offener Punkt, kein sources.φ-Block
  ohne Membran-Konsument, kein Debt. (Schritt: Marathon von Parser-Gap A fortsetzen; die
  Consumer-Benennung bleibt beim Operator.)

## Mail-Fang — Deployment offen (Operator-Wort)

- Der Webhook-Verlust-Fix (Retry + KV-Puffer + Cron-Drain in `cloudflare/email_worker.js` +
  `cloudflare/wrangler.toml`; Dedup über `seen_ids.φ` in `tools/service/src/bin/smail_recv.rs`) ist
  gebaut, nicht deployed. (Schritt: `wrangler kv namespace create MAIL_QUEUE` → Id in `wrangler.toml`
  eintragen → `wrangler deploy`; `WEBHOOK_URL`/`WEBHOOK_TOKEN` prüfen.)

## Membran

- ESP32-Modul — on hold (Operator-Wort, 2026-09-13): das Gerät und sein Flash kommen zuletzt.
  (Schritt: ruht beim Operator.) BOM: `docs/specs/mantis-shrimp-bom.md`.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene Abschluss-Check mit
Commit und Push (`/commit`).
