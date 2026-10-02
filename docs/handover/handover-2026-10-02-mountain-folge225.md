<!--
  title: Handover — Mountain-Folge 225 (Stand 2026-10-02)
  session: Mountain-Folge 225
  class: handover
  date: 2026-10-02
  sha256: 738f30ba61a447b618eb9323a69496286ddcf29eb2c9ca412ad16e16d4ab7415
  status: live
-->
# Handover — Mountain-Folge 225 (2026-10-02)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`; die
zitierte Runde steht auf `fb8ebc16c`). Diese Session konsumierte
`handover-2026-10-02-mountain-folge224.md` (→ `archiv/`).

Die adressierten `## An mountain`-Blöcke aus `future-folge168` sind gemessen und
gefaltet: die planetar/atmosphere/radio/zugang-Liste liegt als Verdikt in
`phi/blocked_sources.φ:521-548`; die bereits getragenen Quellen tragen
`pending`/`descoped`. Neu verdiktet (jede URL am 2026-10-02 per
`archive_search --verdict` nachgemessen): `psaftp.esac.esa.int` descoped (200,
Planck-Mirror ohne eigenen Arm), `pla.esac.esa.int` descoped (206, Redirect-Hülle),
`pdsimage.wr.usgs.gov/M3` pending (direct kein Response, nur Wayback 2018),
`data.kasi.re.kr` pending (200, Portal), `shadowcam.sese.asu.edu` descoped (200,
Projekt-Portal), `surveys.roe.ac.uk/ssa` descoped (206, Redirect-Hülle).

## Burn: open 0.0103 · close 0.2905 (line-agent kumulativ; Session-Delta 0.2802, gemessen 2026-10-02 via session_burn) · cap 0.35 (raised: operator-directed one-pass multi-atom) (reason: fired DE441-Granulat-Census, Eclipse-Epoch-Re-Run, JADES/DSN-sha, OSA-ATLASDR4-Admission, future-168-Fold)

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„erst messen" — Kandidaten vor jedem Verdikt messen | 2026-09-27 | Operator (Mountain 187)
„jeder Punkt trägt eine Empfehlung; wartende Linien erhalten eine bevorzugte Abarbeitungsbitte" | 2026-09-29 | Operator (Mountain 204)
„vorbestehend ist verboten mein wort" — alle über-256-Zeichen-`note`-Zeilen geheilt | 2026-09-30 | Operator (Mountain 209)
„braucht es wirklich pro?" — pro nur mit benanntem Hart-Atom oder gemessener flash-Fehllage | 2026-09-30 | Operator (Session, Mountain 211)
„die url/format-Zeilen sind ohne tragfähigen Arm vorzeitig" — kein url/format ohne deckenden Arm | 2026-09-30 | Operator (Session, Mountain 211)
„arbeite deine Liste bis zur Kante ab" — jeder eigene Punkt bis zur Kante, nichts Machbares liegen lassen | 2026-09-30 | Operator (Session, Mountain 212)
„verschleppen und nicht eigenes ist verboten" — Linienliste nur `eigen`, jeder Punkt im Atom bis zur Kante | 2026-09-30 | Operator (Session, Mountain 213)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-02 | Operator (Session, Mountain 225)
„Committe und pushe jetzt — nur deine eigene Arbeit, gemessen nicht beteuert … das Commit-Wort" | 2026-10-02 | Operator (Session, Mountain 221)
„braucht es max?" — pro/max nur mit benanntem Hart-Atom oder gemessener flash-Fehllage; flash-first | 2026-10-02 | Operator (Session, Mountain 222)

## Offen (aufgeschlüsselt)

### JWS2-Kontrakt — CI-Testlauf läuft
- **Status:** wartend | **Bindung:** eigen (CI: mycelium)
- **Trigger:** `ci-check` auf dem frischen Push (queued 2026-10-02T11:19:39Z, Run `37000386415`) trägt `jwst_bin_roundtrip*`, `_legacy_jws1`, `_sentinel`, `_refuses_malformed`.
- **Lage:** (gemessen 2026-10-02) der Wire trägt `z` (`src/archivar/jwst.rs`; `jwst_bin_roundtrip_carries_redshift` :502); JWS2-Record (Magic `JWS2`, Kopf 137 B), JWS1-Lesearm bleibt; Consumer `main_flow.rs jwst_spectrum_motion` unverändert. `jwst-cdn-watch` `37000087113` queued (Mycelium).
- **Blockade:** Lauf-Ende offen; JWS2-Bins unmanifestiert.
- **Braucht:** `ci_manage log 37000386415` einmal; Mycelium manifestiert die JWS2-Bins; danach `sha256` in `phi/sources.φ`.

### ONC-Hydrophon PSD — Arm ohne Fetch-Pfad
- **Status:** pending (parser-gap) | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-10-02) `tools/harvest/src/bin/onc_hydrophone_compiler.rs` gebaut (schreibt `COMP_ONC_PSD`), aber `--input <mat>`-getrieben und ohne Fetch-Pfad von `data.oceannetworks.ca/api/archivefile/download`; `declined_sources.φ:1417-1419` trägt die alte spectral-series-Ablehnung. Verdikt-Zeile in `phi/blocked_sources.φ` gesetzt.
- **Blockade:** Fetch-Arm fehlt.
- **Braucht:** Fetch-Arm (archivefile→MAT) oder expliziter `--input`-Harvest.

## Träger (Prosa, eigene)

- `docs/auftrag/auftrag-flyby2-kette.md` — σ-Metrik-Kette (3 Marker); Trigger JUICE In-Situ / Δ publiziert, `flyby_ephemeris_gate` (CI).
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` — Holdings-Inventur; Absolutpfade auf `~`-relativ gesetzt.
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` — Relevanz-Erstpass; die Pending-Einträge sind im `dead_sources.φ` disponiert.
- `docs/surveys/survey-raetsel-bestand.md` — zwölf Nadeln + Blätter + Kuprat, stehende Messreihe; jede Zelle mit `file:line`/`pending`.
- `docs/paper/eclipse-clock-worldlines.md` — `:45` trägt die 2024-04-08-Haus-Querprobe (`36989040013` @`769dfe8ff`: DE↔INPOP 22.25 km, DE↔EPM 16.40 km, INPOP↔EPM 31.47 km), Header-sha aktualisiert.
- `docs/paper/flyby-path-2-falsification-metric-addendum.md` — Anderson-Flyby-Klasse-Abschnitt (MESSENGER-Riss als Granulat-Naht), Header-sha aktualisiert.

## An mycelium

Origin: mountain folge225.

- **ASCAT/OSI-SAF CDN-Workflow fehlt:** `ascat_compiler.rs` gebaut und gemessen (Register-Zeile in `phi/sources.φ` steht); der Workflow `manati.star.nesdis.noaa.gov-ascat` fehlt → Manifestation hängt. Riss: `erddap.aoml.noaa.gov` (2026-10-01 HTTP 200) am 2026-10-02 TLS-Timeout + Proton; arbeitsfähig `manati.star.nesdis.noaa.gov`.
- **JWS2-Bins:** `jwst-cdn-watch 37000087113` queued; nach Lauf `sha256` der JWS2-Bins in `phi/sources.φ` nachtragen.
- **JADES/DSN/DE441/Eclipse re-manifestiert:** `jades-cdn 36988178053` + `dsn-cdn 36988181263` success (`a22edfa7`), `de44-cdn 36989695933` success (`45b03472`), `ephemeris-house-gate 36989040013` success (`769dfe8ff`); `sha256` JADES (`af728a13…`) + DSN (`5db02383…`) in `phi/sources.φ` eingetragen; DE441-Granulat-Census `overlapping pairs 0`.

## An future

Origin: mountain folge225.

- **future-folge168-Block vollständig gefaltet:** die sechs unregistrierten URLs sind verdiktet (`phi/blocked_sources.φ:521-548`); `pda.kasi.re.kr` bleibt `blocked account` (`:448`), `kari.re.kr/kpds` `descoped` (`:457`). Die SSA/OSA-Registerzeilen sind kanonisch sortiert (SSA-Block lag seit der letzten Session url-order-verletzend; `register_sort --write` geheilt).
- **DE441-Granulat-Naht-Befund:** `de_compiler.rs:215` dedupliziert per t0; Census auf den re-manifestierten CDN-Bins (`de441`/`inpop`/`epm`) zeigt `overlapping pairs 0`.

## LOCK

- Das private Experiment (Mountain 217) bleibt privat: Wort und Cut liegen nur im privaten `state/operator-gespraeche/2026-10-01-mountain.md`.
