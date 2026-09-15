<!--
  title: Handover — Bau & Code (Stand 2026-09-15, Bau34)
  session: Bau-Folge 34
  class: handover
  date: 2026-09-15
  sha256: 42e42d525798fb161401755281bbfc8f0d5ec031b26cbea8e8588ce6eb228c1c
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
  Offen: die nächste Sprosse — feinere Bin-Auflösung ODER eine tragbare Null (auch die volle
  Y-Vergangenheits-Einbettung `EmbeddedPast` wurde gemessen degeneriert: FPR 10,5–12 % bei n=150)
  — dann das n=1000-Gate in CI messen. (Schritt: `src/mathematikerin/te.rs` Null-Konstruktion
  iterieren, `gh workflow run te-gate.yml`; `gh issue close 13` erst bei einem Gate, das hält.)

## Parser-Gap A — DRS-FITS offen; TNF/ODF/ODR gebaut, warten auf Konsument (Tor 1)

- DRS-FITS (LISA, `heasarc.gsfc.nasa.gov/FTP/lpf/data/fits/`): gemessen (2026-09-15) — Primary
  NAXIS=0 + BINTABLE `HOUSEKEEPING`; die Science-HDU `SCI_SCIENCE_1Hz` (140 Spalten) trägt die
  LTP-Kräfte (N) und LTP-Positionen (m), kein m/s². GEBAUT: `fits.rs` NAXIS=0-Primary + `TUNIT` +
  `drs_differential_acceleration` (Δg = (F2−F1)/1,928 kg aus TTYPE 80–88), 7 Tests grün. Offen: ein
  DRS-Compiler, der die Kette bis `EXTNAME='SCI_SCIENCE_1Hz'` läuft (der generische fits-Arm liest
  nur die erste BINTABLE), + Verifikation am echten Granulat (`pending`). (Schritt:
  `tools/harvest/src/bin/` DRS-Compiler nach fits-Compiler-Muster; echter FITS-Lauf per `curl -r`.)
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

## Register-Digest — Bau-Linie offen (aus register-digest, archiviert)

- `repo_surveillance` Branch-Verdict: `session-protokoll.md` §Wächter verlangt den Branch-Namen als
  eigene Verdict-Zeile; `repo_surveillance.rs:36-47` prüft nur den Remote. (Schritt: Branch in
  `git_verdict()` aufnehmen.)
- `--history`-Blindfleck: offene Zeilen, die in einer UMgeschriebenen (nicht gelöschten) Datei
  verschwanden, sind unsichtbar. (Schritt: `git log -S`-Modus je Thema, falls gebraucht.)
- `--live`-Namensstimme (Rat): „live" reibt an der Quellen-Registerklasse `live`.
  (Schritt: Operator entscheidet `--open` oder bleibt.)
- Legacy-Repo: 80 archivierte/gelöschte `docs/handover/*.md` ohne Gegenstück. (Schritt:
  `register_lookup --history --legacy /home/johannes/backup/archive/omegaflow/omegaflow-legacy`.)
- Sichtbarer Überseh-Korpus: `--live` meldet 106 Dokumente, ~630 offene Zeilen — die gehören je in
  ihre Linie (Ernte/Forschung), nicht gesammelt hier. (Schritt: je Linie ihr Digestschritt.)

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
