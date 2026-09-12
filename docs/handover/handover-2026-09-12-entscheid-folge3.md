<!--
  title: Handover — Entscheid-Folge III (Stand 2026-09-12)
  session: Entscheid-Folge III
  class: handover
  date: 2026-09-12
  sha256: 4f84f74f3a81c3c4a4dc3ab55a48aa425249ef913e1172a8bcc81a9da07774a5
  status: live
-->
# Handover — Entscheid-Folge III (2026-09-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab, wie sie kann — Sub-Agenten tragen eigenen Kontext, die Anzahl
ist kein Aufwand. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

Jede Zeile trägt ihr Wiedervorlage-Datum — vor dem Datum wird sie nicht erwähnt;
nur fällige Zeilen (Datum ≤ heute) kommen auf den Tisch.

## GitHub-Konto

- Sicherheit — der exponierte Token `ghp_KHhu…` ist zu widerrufen
  (https://github.com/settings/tokens). Gemessen 2026-09-12: er ist NICHT der
  aktive `gh`-Token (aktiv ist `ghp_sfYHC…`, Account omegaflow, Scopes
  `admin:org, repo, workflow`) — der Push bricht also nicht; der Widerruf bleibt
  Operator-Akt. Nach dem Widerruf ist die Zeile erledigt.
- ToS / User→Org — der Support-Text ist formuliert (diese Session) und wartet
  auf das Sende-Wort; Senden ist Operator-Akt (https://support.github.com/contact).
  Nach dem Senden trägt die Zeile die Ticket-Nummer als Wiedervorlage.
- `ivoa` — Markus-Brief (Re-Invite `johannestyroller`/`omegaflow`) ist raus,
  Antwort offen.

## Votable-TAP

- Die 22 ehemals `blocked parser-def votable` sind vermessen: der TAP_SCHEMA-Pass
  (`tap_compiler --votable --index`) hat 18 Inventare als `phi/pipeline/catalog/
  tap_index_<label>.φ` versioniert (`.gitignore` trägt neu `!phi/pipeline/catalog/
  tap_index*.φ`, MANIFEST.φ die 18 Zeilen), 2 Endpunkte schlugen gemessen fehl
  (WGE-SDSS IllegalArgument, LIneA Schema-Name). Die Disposition steht in
  `phi/blocked_sources.φ`. Offen: die Compiler-Tranche je überlebendem Katalog —
  Feldblock + `<name>-cdn.yml` + CDN-Manifestation + sources.φ-Eintrag erst mit
  gemessenem Feldblock — für ALMA EU, SkyMapper, CASDA, MACHO, MUSE-Wide,
  WiggleZ, CADC youcat, LAMOST DR11; dazu die zwei Fehlschläge nachmessen.
- Bestand gemessen: die 54 älteren `tap_index_*.φ` (Pass 2026-09-10) sind nie
  versioniert gewesen — `MANIFEST.φ` listet sie `visible`, git trägt sie nicht.
  Die neue `.gitignore`-Zeile hebt die Klasse hervor; die 54 sind fremde Arbeit
  und bleiben untracked — das nächste Atom commitet sie mit oder revidiert die
  MANIFEST-Zeilen.

## Wiedervorlage

- 2026-09-14 — Desktop-Fork (GTX 970): 30-Jahres-Lauf.
- 2026-09-14 — de441 Re-Verifikation: der jup365-OOM-Fix (DafFile `pread`) liegt
  auf `main` (f67b14d); der nächste `kernel-flatten`-Dispatch verifiziert
  (bodies-Job grün + jup365-Release am CDN).
- 2026-09-14 — ned-Crawl: Void-Kegel-Fix + Listing-Void-Abbruch auf `main`; CDN
  1/40 Slices (gemessen 2026-09-11), nächster Cron-Lauf.
- 2026-09-14 — Token-Tausch verifizieren: `OMEGAFLOW_TOKEN` trägt jetzt einen
  classic PAT von `johannestyroller` (`public_repo`, eigenes 5.000/h-Bucket;
  API-gemessen 2026-09-11: `push:true` auf `omegaflow/sources`, Bucket frisch
  4999/5000). Der nächste `kernel-flatten`-Dispatch verifiziert (grün + kein
  Rate-Limit-Hit).
- 2026-09-14 — CI-Dedup (Endzustand der Concurrency-Arbeit, Commit 5d80061):
  `lead-geometry-cdn` soll das CDN-Asset `physionet.org/mitdb_arrhythmia.bin`
  konsumieren (`body_url`) statt es neu zu bauen; NOAA-Einzelstation
  (`noaa-ghcn/gsod/isd-cdn`) messen, ob sie in den 16 Shards der `-allstations`
  liegt — ja: falten, nein: Asset-Namen trennen (Namensverletzung). Bis dahin
  tragen die Namespace-Gruppen `mitdb-arrhythmia` / `noaa-*-assets`.
- 2026-09-14 — CI-Pending-Scans: Upload-Ziele der Probe-Workflows (`te-*`,
  `eikonal-tohoku`, …) lagen nicht im Contention-Scan — unverifiziert; die
  `17 5 1 * *`-Überlappung `kernel-flatten`/`signal-cone-audit` ist nur unter
  der unverifizierten Annahme „Audit liest nur" benannt.
- 2026-09-15 — CI-Sättigung messen: `ps1-cdn` + `allwise-cdn` (je ~180 min,
  stündlicher Cron) — Kadenz gegen Actions-Minuten/Rate-Limit messen; bewusst
  kein Per-Tag-Gate für `ssd.jpl.nasa.gov` (Drain 415 min/h verhungert).
- 2026-09-15 — LISA Pathfinder: Selbstregistrierung ab 15.09.
- 2026-09-16 — ned-objdir: IPAC-Auto-Bestätigung 2026-09-09, Antwort offen.
- 2026-09-18 — NOIRLab Data Lab: `jtyroller` registriert 2026-09-11, wartet auf
  menschliche Freigabe.
- 2026-09-18 — Rubin RSP-Datenrechte: Antrag in Prüfung.
- 2026-09-18 — papier-kleinpass (nach Merge).
- 2026-09-18 — sicherung-risiko-heime: wöchentliche Kopie.
- 2026-09-22 — AllWISE: CDN-Stand; `allwise_coverage.fp01` nach Abschluss
  verifizieren.
- 2026-09-25 — TOAR-Vollzugang: Jülich-Antwort (Schröder).
- 2026-09-25 — TNO-Kette: keine MPC-unabhängige Linie (not-published).
- 2026-09-28 — Nadel Ⅱ: JUICE-Flyby 28./29.9.
- 2026-12-02 — Nadel Ⅰ: Jeans-Residuum bis Gaia DR4.
- 2026-12-02 — gaia-dr4-iapetus.
- 2026-12-03 — Nadel Ⅱ: Europa Clipper 3.12.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check (`/abschluss`).
