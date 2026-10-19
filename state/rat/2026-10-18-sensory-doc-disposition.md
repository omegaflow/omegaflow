<!--
  title: Sensory — Besitz-Disposition der 16 Docs (2026-10-18)
  class: ref
  date: 2026-10-18
  sha256: 5812b28de25321035f866de6edbc3b6d6d6c59435cd5617531b91803dbf2bad5
  status: live
  see-also: docs/handover/handover-2026-10-14-sensory-folge259.md state/rat/2026-10-18-orphan-zuordnung.md
-->
# Sensory — Besitz-Disposition der 16 Docs (2026-10-18)

Grund: `b729a7f97` wies der Linie 16 Docs zu. Besitz ist kein Messauftrag; die
Zuordnung verlangt je Doc eine Disposition — schließen, delegieren oder mit
Begründung abgeben. `register_lookup --open` zählt Marker heuristisch (auch
Tabellen-/Prosa-Zeilen); die Linie entscheidet, was ein Punkt ist.

## Geschlossen (belegt)

| Doc | Beleg |
|---|---|
| `survey-2026-09-16-dead-sources-relevanz.md` | 4 Pending-Quellen gemessen; Commit `5e92f1e89` |
| `survey-2026-09-14-ehrlich-benannt-werkzeug-luecke.md` | drei Arme gebaut: `parquet_reader.rs`, `grib2_reader.rs`, `opendap_reader.rs` + `src/archivar/{parquet,grib2,opendap}.rs` |
| `survey-2026-09-16-sonden-flotte.md` | MAVEN registriert (`sources.φ:19111`); nativer Serien-Arm gebaut (`tnf_compiler.rs:71`, PODF) |
| `survey-2026-10-08-open-sources-delta.md` (3 Reste) | SSDC Limadou NXDOMAIN/kein Ledger; LiteBIRD 301 → JAXA/keine Datenroute; CSES TLS abgelaufen, Route via `--jina` **200** / `-k` **200** |
| `survey-2026-10-08-fmhy-research-landscape.md` | MiMo NXDOMAIN; Jina-Arm gebaut; SearXNG-Arm gebaut |
| `survey-2026-10-12-standort-lokalisierung-ortho-dsm.md` | Apple Look Around NXDOMAIN; NRCan canimage 404; Bing Streetside kein API-Doc |
| `survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` (Gravimetrie) | EQUIP-G 000/unregistriert; IGETS + BGI/SEDOO registriert/live |
| `survey-2026-10-08-open-sources-delta.md` Re-Audit 126–132 | Zitat Zeile 193–194: „13 Punkte, 13 gedeckt (2 nachgemessen 2026-10-18)"; Commit `5058ade61` |
| `survey-2026-10-08-fmhy-research-landscape.md` SearXNG-Instanz | Instanz gesetzt + gemessen: `SEARXNG_INSTANCES = ["https://search.lumy.live"]` (`tools/utils/src/bin/archive_search/net.rs:1802`); Commit `3e27dff0f` |
| `survey-2026-09-16-sonden-flotte.md` Zeilen 77/161/162 | kein offener Marker (`sed -n '77p;161p;162p'`), Historie; Commit `5058ade61` |

## Offen — mechanisch/Operator (benannt)

| Doc | Punkt | Zustand |
|---|---|---|
| `open-sources-delta` | Teilchen-/Dark-Matter-Experiment | Operator-Wort (Kuprat vs. Detektor) |
| `stoerungs-experiment` | CTBTO VDEC | Operator-Webform; root 404, request 403, unregistriert |

## Abgegeben (Grund)

| Doc | Empfänger | Grund |
|---|---|---|
| `daten-holdings-inventur` | future/Operator | Ziel-Layout-Wort steht; Byte-Messung/Migration ist Operator |
| `grosse-raetsel-und-maschine` | future | „Aufnahme-Wort" ist Operator |
| `geraete-anbindung-radiatoren` | Operator/Hardware | physische Sensoren ungemessen |
| `die-fuenf-archaeologie` | mountain | Code-Konstanten/Axiome, kein Sensor |
| `tauchbefunde-acht-faeden` | river/mountain | force-Keyword-Entscheid |
| `verlorene-diskussionen` | mountain/code | historische Bindings |

## Geroutet (Token + Brief, 2026-10-18)

| Token | An | Brief |
|---|---|---|
| `xfer:sensory-259…10~67b095df` | future | `post/in/2026-10-18-sensory-an-future-offene-entscheide.md` — 5 Entscheide |
| `xfer:sensory-259…11~22e38677` | mountain | `post/in/2026-10-18-sensory-an-mountain-zwei-docs.md` — 2 Docs |
| `xfer:sensory-259…12~a3bf3c25` | river | `post/in/2026-10-18-sensory-an-river-force-keyword.md` — force-Keyword |
| `xfer:sensory-259…13~99e00470` | mycelium | `post/in/2026-10-18-sensory-an-mycelium-searxng-instanz.md` — SearXNG-Instanz |
| `xfer:sensory-259…14~dfc6aa3a` | future | `post/in/2026-10-18-sensory-an-future-beobachtungen.md` — Meta-/Prosa, Command-Risse, ARVO, Working Tree, Gate |

## Kein Punkt (Meta/Prosa) — an future geroutet

`asservatenkammer-zensus` (Meta-Tabelle), `survey-raetsel-bestand` (Inventar),
`das-eine-instrument` (Konzept), `ehrlich-benannt:13` (Klassifikationssatz),
`sonden-flotte:77/161/162` (request-only-Historie). Diese Beobachtung steht nicht mehr
nur hier: Token `14` an future, Brief `post/in/2026-10-18-sensory-an-future-beobachtungen.md`.

## Linien-Punkte (nicht Docs)

- `ox64-m2c`: DHL 17.10. 05:57 „in der Region des Empfängers", Fortschritt 3/5 —
  nicht zugestellt.
- `europa-clipper`: Termin 03.12.2026; der Schritt ist ausdrücklich nach dem Flyby.
- `xfer:sensory-259…9~1da7eb85` an future: `SENT`/`unbeglichen` (kein future-Handover
  im Checkout).

## Pass-Abschluss (R-2026-10-18-c, 4/4 · 2026-10-18)

Messung (Linie `sensory-259`): Post 0 (Postfach in `$OMEGAFLOW_WORK/post`, dort 0 `*-an-sensory*`) · Trigger 0
(`register_lookup --addressed sensory-259` = 0) · Transfer 4 offen
(`xfer coverage --line sensory-259`: emitted 14 · disposed 10 · open 4). Delegiert
(4 freie Subagenten, done-when vom Pass), Belege geprüft, drei Punkte geschlossen:

- **Open-Sources-Delta Re-Audit 126–132** — geschlossen. Zitat Zeile 193–194:
  „13 Punkte, 13 gedeckt (2 nachgemessen 2026-10-18)." Commit `5058ade61`.
- **FMHY SearXNG-Instanz** — geschlossen. `tools/utils/src/bin/archive_search/net.rs:1802`
  `const SEARXNG_INSTANCES: &[&str] = &["https://search.lumy.live"];`; Commit `3e27dff0f`.
  Der Doc-Marker (Zeile 74) ist nachgetragen.
- **Sonden-Flotte Zeilen 77/161/162** — kein Punkt. Kein offener Marker
  (`sed -n '77p;161p;162p'`); Commit `5058ade61`.

Nicht geschlossen (Urteil bleibt bei der Linie, nicht delegierbar): `open-sources-delta`
Teilchen-/Dark-Matter-Experiment und `stoerungs-experiment` CTBTO VDEC (beide Operator-Wort).
Die vier Transport-Token `.9/.10/.13/.14` waren Empfänger-Arbeit (future/mycelium); `.13`
ist im Nachtrag geschlossen, `.9/.10/.14` bleiben offen (Urteil bei future/Operateur).

## Nachtrag-Pass (R-2026-10-18-c, 4/4 · 2026-10-18)

Nach dem mycelium-Pass (`0d93d380e`) liegt für Token `.13` der Beleg vor; der Emitter
setzt den Schluss. Messung (Linie `sensory-259`): Post 0 · Trigger 0 · Transfer
emitted 14 / disposed 10 / **open 4** → nach dem Schluss disposed 11 / **open 3**.

- **SearXNG `.13~99e00470` — geschlossen.** Zitat
  `tools/utils/src/bin/archive_search/net.rs:1802`:
  `const SEARXNG_INSTANCES: &[&str] = &["https://search.lumy.live"];` (Commit `3e27dff0f`);
  Zitat `state/rat/2026-10-18-mycelium-pass-rbn-manifestation.md:55`:
  „der **Emitter (sensory)** setzt den `CLOSED`-Eintrag". Ledger
  `state/ledger/sensory-259/ses_ec618129fffeLFawr4V3pQKpOl.log`:
  `CLOSED … .13~99e00470 by=sensory-259.ses_… ref=3e27dff0f`.

Nicht geschlossen: `.9~1da7eb85` sowie `.10~67b095df`/`.14~dfc6aa3a` an `future` —
kein `future`-Ledger im Baum (`ls state/ledger/`: mountain · mycelium · river ·
sensory-259), keine Empfänger-Quittung im Postfach; Empfänger-Arbeit, kein Ledger-Schluss.

### Kalibrierung (die zwei Command-Risse, gemessen 2026-10-18)

- **Postfach-Wurzel.** `post/in/*` löst relativ zum Repo auf und findet nichts; das
  Postfach liegt in `$OMEGAFLOW_WORK` (`bin/_roots.sh`, Default `~/projects/omegaflow-work`).
  Dort `ls -1 post/in/*-an-sensory*.md` → **0** (kein Eingang an sensory), aber die
  abgegebenen Briefe liegen dort, z. B. `post/in/2026-10-18-sensory-an-future-offene-entscheide.md`.
  Der frühere Befund „`post/` nicht im Baum" war der **Pfad-Riss**, kein leerer Korb
  (gespiegelt: `post/in/2026-10-18-sensory-an-future-postfach-pfad-gemessen.md`).
- **xfer-Linienname.** `xfer list --line sensory --open` ist leer; der Ledger liegt unter
  `state/ledger/sensory-259/`. Mit `--line sensory-259` erscheinen die Token.
  `.opencode/commands/**` ist `edit:deny` — die Korrektur ist als Beobachtung an `future`
  geroutet (Token `.14`), nicht in dieser Linie baubar.
