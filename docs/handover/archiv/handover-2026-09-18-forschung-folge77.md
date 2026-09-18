<!--
  title: Handover — Forschung-Folge 77 (Stand 2026-09-18)
  session: Forschung-Folge 77
  class: handover
  date: 2026-09-18
  sha256: 3e598365f50eacfd99d2e71572b466406b9e3dc80553a827e89ca8c625505267
  status: live
-->
# Handover — Forschung-Folge 77 (2026-09-18)

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
Wartestellungen (`wartend`) sind kein Auswahlpunkt — sie nennen nur ihren Auslöser
und werden nie als Handlungsschritt geführt; gibt es keinen abarbeitbaren
undatierten Punkt, sagt die Session das. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (gemessen 2026-09-18)

- **HEAD** `627ccea5`. Fremd uncommittet (nicht angefasst): `AGENTS.md`,
  `docs/SOURCE_PORT.md`, `opencode.json` (fremde Hunks + eigener Hunk),
  `docs/handover/post.md`, `phi/declined_sources.φ`,
  `src/archivar/fetch.rs`, `main_flow.rs`, `port.rs`,
  `tools/register/src/bin/register_lookup.rs`, die gestagten
  `handover-2026-09-16-*`-Renames, `handover-2026-09-18-entscheid-folge45/46.md`.
  Safety-Net `refs/safety/1789713482`.
- **CI** (`ci_manage list`/`view`): LRO-`harvest` `35312009465` (format `lro_trk`,
  head `8d1264bb`) **in_progress** → LRO-Trigger nicht gefeuert. Sonst fremde
  Linien (`te-gate` `35314110831`, `ci-check` `35314164053`, `*-cdn`).
- **Postfach** — `post.md` leer (nur Template); letzter Ledger-Eingang `1789689115`.

## Kein abarbeitbarer undatierter Punkt (`—`)

- Alle Punkte sind Wartestellungen, operator-gebunden oder datiert. Diese Session
  hat den BepiColombo-Auth-Befund gemessen (unten) und die Auth-Kanten-Pflicht in
  die Taucher-Profile gebaut (`opencode.json`); kein undatierter Selbst-Punkt.

## BepiColombo (`blocked account` + Route `pending`)

- **Gemessen 2026-09-18:** `bc_mpo_more` anon SFTP/Mirror HTTP 200 = `bundle…lblx`
  + readme + `document/`, **kein `data/`** (`data_raw/` 404). Login mit lokalem
  `ESA_USER`/`ESA_PASS` → SFTP rc 67 / Web 530 **abgewiesen**. TAP: 89517
  registrierte LIDs (proprietär, `proprietary_end_date` 2088/2099);
  `download_path /distribution/getProduct?id=…` an allen geprüften Hosts **404**
  (kein 401/403). Register: `phi/blocked_sources.φ` (`blocked account`).
  (Schritt: entscheid/Operator — gültiges PSA-/psaftp-Konto prüfen/beschaffen;
  Route-Endpunkt bleibt `pending`, erste Messung: PSA-API-/Distribution-Endpoint-Doku.)

## LRO utF (`wartend`)

- Trigger = Abschluss `harvest` `35312009465` (`format lro_trk`, in_progress).
  (Schritt: nach Abschluss `archive_search --sniff` auf `lro_trk.bin`,
  sha256/Größe, `phi/harvest.φ` (`format lro_trk`) + `phi/sources.φ`-Block auf present.)

## NSE/Haug — Rohdaten zugesagt (`wartend`)

- Keller-Antwort (17.09.), sendet „in einigen Tagen". Trigger = Dateieingang.
  (Schritt: bei Eingang `nse_haug_trisp`-Quelle + Compiler + `sources.φ`;
  0-Kanon: kein Asset ohne Datei.)

## Legacy-Konzepte (`operator-gebunden`, hintenangestellt)

- Silence-Map-Probe, vC-Definition L:53, Certainty, TDA/Betti-0, Minkowski als 4.,
  Nostr hinten (`survey-2026-09-17-omegaflow-legacy-konzepte.md`). (Schritt: bei
  Wiederaufnahme den Rat-Erster-Atom bauen —
  `tools/measure/src/bin/silence_map_probe.rs`.)

## Paper / Präregistrierung (`termin`)

- Flyby Path 2 — datiert (JUICE 28./29.09., Clipper 03.12.), schweigt vor dem
  Datum. (Schritt: vor dem 28.09. den konkreten Abruf-Schritt je Kanal in
  `docs/paper/flyby-path-2-preregistration.md` setzen.)

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `opencode.json` (nur eigener Hunk — Auth-Kanten-Pflicht in
  `general`/`research-max`/`grind-flash`/`grind-pro`/`grind-max`),
  `phi/blocked_sources.φ`, `docs/handover/handover-2026-09-18-forschung-folge77.md`
  (+ archivierte `handover-2026-09-18-forschung-folge76.md`).
- **Fremd (nicht anfassen):** `AGENTS.md`, `docs/SOURCE_PORT.md`,
  `docs/handover/post.md`, `phi/declined_sources.φ`, `src/archivar/fetch.rs`,
  `main_flow.rs`, `port.rs`, `tools/register/src/bin/register_lookup.rs`, die
  gestagten `handover-2026-09-16-*`-Renames, `handover-2026-09-18-entscheid-folge45/46.md`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
