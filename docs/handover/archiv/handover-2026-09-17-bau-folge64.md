<!--
  title: Handover — Bau-Folge 64 (Stand 2026-09-17)
  session: Bau-Folge 64
  class: handover
  date: 2026-09-17
  sha256: f2b1838a54cb5ce455fc6e298410e7ecfb38957a1a05353969a8293b27671d09
  status: live
-->
# Handover — Bau-Folge 64 (2026-09-17)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht.
Der Planungs-Pass nennt die offenen Punkte als nummerierte Auswahl (der erste ist
der härteste undatierte); die Session arbeitet so viele ab wie möglich.

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (2026-09-17, HEAD 817ddc08)

- Postfach: kein neuer externer Eingang seit 2026-09-16 (Sotgiu); die offenen
  Anfragen (Rubin-Review, NSE/Haug, die fünf Sonden) unverändert — letzte
  Ledger-Zeile `state/mail/mail_ledger.φ` = Rubin-Password-Reset (1789307656),
  kein Eingang. `docs/handover/post.md` trägt keine bau-Zeile (nur „An alle
  Linien" und „An forschung").
- CI am HEAD `817ddc08`: Watchdog-Snapshot 2026-09-17T16:01 +02:00 — aktiv
  demeter-cdn, ned-cdn, physionet-cdn (2×), gaia-xp-full, planetary-odf,
  health-check, allwise; queued pii-exposure `35230489744`, paper-check
  `35230433910`; failed paper-check `35228278279`, `35224760511`.
- Zustand-Ledger: `docs/zustand/external-state.md` trägt weiter **fremde
  uncommittete** Zeilen (Postfach/PII/CI @ `bc9d6a0b`) — nicht überschrieben.
  (Schritt: die CI-Zeile auf `817ddc08` fortschreiben, sobald der fremde Delta
  committet ist — fremde Arbeit, kein eigener Hunk.)

## Fast-ttl-Mirror — die eine offene Kante (härtester undatierter Punkt)

Gebaut: `tools/harvest/src/bin/cdn_mirror_compiler.rs` — 1:1-Passthrough-Mirror,
leitet den Asset-Namen über **die Laufzeitfunktionen selbst** ab
(`cdn_manifest_map` → `source_name_from_url` auf der marker-substituierten URL,
identisch zu `fetch_one`) → null Drift. Plausibilitäts-Gate: void oder falsches
Format → Asset unangetastet, Lauf rot (Absenz überschreibt nie eine Messung).
Drei Workflows: `vires-hapi-cdn.yml` (dispatch-only, 7 statische HAPI-Fenster),
`swpc-mirror-cdn.yml` (cron `11 * * * *`, 9 Bodies), `quake-feeds-cdn.yml`
(cron `29 * * * *`, 8 JSON + 1 XML). Cadence: vires einmalig (festes
Vergangenheitsfenster), SWPC/Beben stündlich (rolling windows, Register-ttl
60–180 s).

Offen:
- Die drei Workflows sind registriert, aber **nicht manifestiert** (kein
  Dispatch, kein Push). (Schritt: nach dem Push `gh workflow run
  vires-hapi-cdn.yml` (einmalig), `swpc-mirror-cdn.yml`, `quake-feeds-cdn.yml`;
  danach die Assets per `--sniff`/`gh release view --repo omegaflow/sources`
  gegen die Laufzeitnamen prüfen.)
- **Die Positions-Klasse bleibt ohne Mirror**:
  `earthquake.usgs.gov/fdsnws/event/1/query?...&starttime={hour_ago}&latitude={lat}&longitude={lon}`
  — der Laufzeitname ist sekunden-/positions-granular, ein Mirror-Asset würde
  nie gelesen. (Schritt: entscheiden — CDN-Fallback für diese Quelle abschalten
  (0 honored) oder einen positions-stabilen Cache-Schlüssel bauen; erste
  Messung: welche `{lat}`/`{lon}` die Presence heute liefert.)

## Werkzeug-Reibung — offener Rest

Gebaut: `omega_sh sha <file>` (Header-sha256 über den Body ohne `<!-- … -->`-
Header) + `omega_sh check` (cargo-check-Zusammenfassung) in
`tools/utils/src/bin/omega_sh.rs`; `git_safety --close [<own-path>…]`
(Abschluss-Check in einem Aufruf) in `tools/utils/src/bin/git_safety.rs`.
Profil-Freigaben in `opencode.json`: `echo`/`printf` in explore/general/
research-max/council, `ci_manage list|view` ins Plan-Profil;
`docs/concepts/tools-map.md` nachgezogen.

Offen:
- Die Linien-Prompts (`.opencode/command/*.md`) nennen `omega_sh sha`/
  `git_safety --close` noch nicht. (Schritt: die Phase-2-Preamble in `start.md`
  um die zwei Aufrufe ergänzen.)
- Die neuen Unterkommandos sind Quell-gebaut, aber **nicht auf PATH** —
  `service-build.yml` baut nur `omegaflow-service` + `ci_manage`, nicht
  `omega_sh`/`git_safety`. (Schritt: den Install-Weg der `~/.local/bin`-Binaries
  messen und für die zwei erweitern; erste Messung: woher kommen heute
  `omega_sh`/`git_safety` auf PATH.)

## Benchmark (gemessen, session_burn)

Drei getrennte Atome, kein Doppel-Lauf: Fast-ttl-Mirror `grind-max` **$0.0802**
(Urteil+Bau in einem Kontext, 4 neue Dateien); Asset-Alter `grind-pro`
**$0.1612** (teurerer Lauf trotz kleinerem Delta — cache_read-dominiert);
Werkzeug-Reibung `grind-flash` **$0.0177**. Die Mirror-Klasse (judgment
source-port) hat noch keinen registrierten Sieger; der flash-first-Aufstieg ist
nicht gemessen — der nächste Mirror-Punkt kann flash-first laufen.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
