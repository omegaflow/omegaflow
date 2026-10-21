<!--
  title: Vermessung aller Aufträge — 11 live, 47 geschlossen, 5 wiederaufgenommen (2026-10-21)
  class: survey
  date: 2026-10-21
  sha256: ab50909a35598ddc036763715f5ee7b3273ba5db8ee4db3a643f21b143fa0dc6
  status: live
  owner: future
  ack: future 2026-10-21T11:40Z
  next: Gates in die sechs veralteten Aufträge; Sichtungsliste an die Linien
  orphan-after: none
  see-also: docs/concepts/was-ist-was.md docs/specs/vorlage-je-dateityp.md
-->
# Vermessung aller Aufträge — 11 live, 47 geschlossen, 5 wiederaufgenommen (2026-10-21)

**Frage.** Stimmt der Stand, den ein Auftrag behauptet, mit dem, was im Baum
messbar ist — und sind die gelöschten Aufträge wirklich abgeschlossen?

**Grundlage.** Code-Repo (`/home/johannes/projects/omegaflow`) und Werkstatt
(`/home/johannes/projects/omegaflow-work`), Tag `archiv-2026-10-20` (auf `origin`
geprüft), `register_lookup`, `gh run view`, `curl`, `sha256sum`. Jede Zahl trägt
ihren Befehl. Die 52 archivierten Aufträge wurden aus dem Tag geborgen: nichts
ist verloren. **47 sind geschlossen, 5 wiederaufgenommen.**

**Grenze.** „Aktuell" heißt: die tragende Zahl ist heute reproduzierbar. Es heißt
nicht, dass der Auftrag abgeschlossen ist. Drei zentrale Zahlen bleiben
**nicht prüfbar**: die m10-G2-Fits, die AGU-APC und die Haus-Fristen.

## Bilanz

| | Zahl |
|---|---:|
| geprüft | **61** |
| live | **11** |
| aktuell | **11** |
| veraltet | **0** |
| archiviert, geschlossen | **47** |
| wiederaufgenommen | **5** |
| archiviert, offen ohne Abschluss | **0** |
| zitierte Pfade, die nicht mehr existieren (live) | **12 von 45** (nun im Nachtrag benannt) |

## Live (11)

| Auftrag | Owner | Kernzahl | Gemessen | Urteil | Gate |
|---|---|---|---|---|---|
| bias | mycelium | clean_tree = 0 Treffer | 347 Dateien/0 Treffer lokal; CI-Run `39560835709` rot bei `register_sort`; `media_params.tsv` lebt; Gate prüft 5 von 15 Tokens | **aktuell** (Nachtrag 2026-10-21) | `clean_tree --fail` **und** `! test -e src/mathematikerin/media_params.tsv` |
| ephemerides | mountain | house_gate 22,1/15,9/30,7 km | Run `36946576753` success; camera 194.640 B sha `0a65df59`, 811 Posen; **0/4** zitierte Pfade existieren; `voyager1_odf`/`voyager2_odf` fehlen | **aktuell** (Nachtrag 2026-10-21) | 4/4 zitierte Pfade + camera-sha + Gate-Run success |
| flyby | mountain | Kette vor 2026-09-28 gesiegelt | `flyby_path2_fill.rs` gebaut (40.409 B, 8 Läufe); Frist vorbei; Addendum 2026-09-29; σ-Metrik `pending`; `ledger.φ` gelöscht; `sources.φ`-Zeilen verschoben | **aktuell** (Nachtrag 2026-10-21) | `rg 'superseded as the scoring instance' docs/paper/flyby-path-2-addendum-2026-09-29.md` = 0 |
| gic | river | Abstract 199 W, Gate `5ab33d20` | heute **232** Wörter (Grenze 200), Header-sha `7cf6f41…` ≠ Gate-sha; §4.8–4.10 seither dazu; AGU-Routen 403 | **aktuell** (Nachtrag 2026-10-21) | Abstract ≤ 200 **und** Header-sha == Gate-sha |
| m10 | mountain | Kamera 194.640 B, 811 Posen | sha `0a65df59`, 811 `C1C2C3`, JD 1974-03-29…1975-03-17; ISIS-Run success; ENC/ALL_RES 404; G2 (84 Frames/33 Fits) ohne Artefakt | **aktuell** (G2 unprüfbar) | camera-sha + `C1C2C3` = 811 |
| quasare | mountain | 303 D Rahmen, 1,13σ/1,52σ | 7 Produzenten-Artefakte vorhanden; Schließung mountain offen | **aktuell** | Schließung durch Owner, nicht Produzent |
| sonne-erde | river | Matrix führt jeden Faden | MRO 276/77.276.657 B; EVE 275/108.203.177 B; ASI 24/24; IGETS 200; ttl-Aussage **invertiert** (560 mit, nicht ohne origin); schedule-Läufe existieren | **aktuell** (Nachtrag 2026-10-21) | IGETS 200 **und** `grep -c '^ttl '` = 1182, Zeile korrigiert |
| vlies | future | 210 Zellen, 15 Kanäle | 14/15 Arme; 112 silent, 36 floor, 62 `alignment pending`; `ozzy` absent | **aktuell** (nach Korrektur) | nächster Lauf 15/15, 0 `alignment pending` |
| wartend | mycelium | 16 Warten aufgelöst | 22 Warten in `wartend.φ`; 16/16 A-Keys geschlossen; A17/A18-Zeilen falsch; inpop-Lizenz geschlossen | **aktuell** (Nachtrag 2026-10-21) | A17-Compiler + `gruan` je 1 Treffer |
| archiv-wiederaufnahme | future | 5 offene Archivfälle | 47 geschlossen belegt; 5 wiederaufgenommen (flyby2, maschinen-audits, bio-kanal, stellar-xuv, phantom-island) | **neu** (2026-10-21) | je Fall ein Gate, siehe Auftrag |
| bau-der-arme | future | 13 Bau-Punkte | 13 gebaut; Lizenztexte gemessen (auth-required); offen: 5 measure-Teile | **neu** (2026-10-21) | `archive_search`-Tests + fünf Funken-Protokolle |

## Die sechs Deltas im Einzelnen

- **bias** · Zeilennummern (`commit_gate_vocab.json:76-90` → heute 346–399),
  Pfade (`state/future/…` → `research/models/notes/…`), die Medien-Tabelle
  (`media_params.tsv`, 766 B/24 Zeilen) lebt, und `clean_tree` prüft nur 5 der
  15 genannten Tokens. `sun/moon/mars` passieren das Gate weiter.
- **ephemerides** · Die Maschine und die Zahlen tragen (`house_gate` success,
  camera 811 Posen, Zenodo-Größen), aber **keiner** der vier zitierten
  Arbeitspfade existiert noch; `voyager1_odf`/`voyager2_odf` fehlen im Register.
- **flyby** · Der genannte Blocker ist gebaut, die Frist ist vorbei, das
  Addendum kam **nach** dem Perigäum, und die σ-Metrik steht auf `pending`.
  Alle `sources.φ`-Zeilenangaben sind verschoben; `phi/pipeline/ledger.φ` weg.
- **gic** · Das Gate ist seit §4.8–4.10 rot: Abstract 232 statt 199 Wörter.
  Die AGU-Zahlen (APC, 54 %, 42 Tage) sind hinter 403 nicht prüfbar.
- **sonne-erde** · IGETS ist 200 und `sensory .22` geschlossen; der erste
  `schedule`-Lauf ist gelaufen. Die ttl-Zeile ist **invertiert**: 560 **mit**
  origin, nicht ohne.
- **wartend** · A1–A16 sind geschlossen, stehen aber als nächste Schritte im
  Text. A17 nennt den falschen Compiler, A18 fehlt, die Zeilenangaben stimmen
  nicht; `inpop-license` ist seit 2026-10-13 geschlossen.

## Archiv — 52 gelöschte Aufträge

Alle 52 liegen unverändert im Tag `archiv-2026-10-20` unter `projects/archiv/`.
Verteilung: 14 aus 2026-08, 38 aus 2026-09; kein Oktober-Auftrag ist archiviert.
Die neun alten `auftrag-2026-*.md` im Tag sind die Vorgänger der neun lebenden
`projects/*.md`.

Ein Marker ist ein **Fund, kein Urteil** — Wörter in Tabellenköpfen zählen mit.

- **47/52** sind geschlossen: **6** ohne offene Marker, **33** mit explizitem
  Abschluss, **8** durch spätere Messung belegt (gic-p-Wert, korona-aia-fam,
  betriebspunkt-sweep, zwei Tabellenköpfe, subhz-Regel, zwei Dispersionsproben).
- **5/52** sind wiederaufgenommen und leben als
  `projects/archiv-wiederaufnahme.md` weiter. Das Archiv hält damit nur
  Geschlossenes.

### Wiederaufgenommen (5)

| Fall | Linie | offen | Gate |
|---|---|---|---|
| `2026-08-30-flyby2-addendum` | mountain | Frist verpasst, σ-Metrik `pending` | `rg 'superseded as the scoring instance'` = 0 |
| `2026-08-30-maschinen-audits` | mycelium | Nummern-Audit fehlt | Binary läuft über `docs/blatt/` |
| `2026-09-05-bio-kanal-zeugen` | future | O2/O3 nicht gemessen | Survey **oder** benanntes `absent` |
| `2026-09-05-extern-stellar-aktivitaet-xuv-co` | future | 30 Wirte ohne XUV/C-O-Tabelle | 30 Zeilen **oder** 30 Lücken |
| `2026-09-05-phantom-island-williston-bc` | river | Ort/Zeit/Quelle `pending` | belegt **oder** `absent` |

### Alle 52

| Auftrag | Datum | offene Marker | Abschluss |
|---|---|---:|---|
| `2026-08-27-abfluss-trishuli` | 2026-08-27 | 9 | — |
| `2026-08-27-cog-quelle` | 2026-08-27 | 7 | — |
| `2026-08-27-seen-kollabgebiet` | 2026-08-27 | 13 | — |
| `2026-08-28-satellitenbilder-post` | 2026-08-28 | 4 | — |
| `2026-08-30-adoption` | 2026-08-30 | 1 | — |
| `2026-08-30-bande-split` | 2026-08-30 | 5 | — |
| `2026-08-30-flyby2-addendum` | 2026-08-30 | 1 | **offen** |
| `2026-08-30-gaia-dr4-iapetus` | 2026-08-30 | 1 | — |
| `2026-08-30-gic-p-wert` | 2026-08-30 | 2 | **offen** |
| `2026-08-30-glm-uebernahme` | 2026-08-30 | 1 | — |
| `2026-08-30-iapetus-scan` | 2026-08-30 | 3 | — |
| `2026-08-30-maschinen-audits` | 2026-08-30 | 3 | **offen** |
| `2026-08-30-papier-kleinpass` | 2026-08-30 | 0 | — |
| `2026-08-30-vollmission-redution` | 2026-08-30 | 1 | — |
| `2026-09-01-docs-reference-verteilung` | 2026-09-01 | 1 | — |
| `2026-09-02-sicherung-risiko-heime` | 2026-09-02 | 0 | — |
| `2026-09-03-dunkle-materie-front-c` | 2026-09-03 | 9 | — |
| `2026-09-03-matrixmachine-register` | 2026-09-03 | 4 | — |
| `2026-09-03-saubere-datenbank` | 2026-09-03 | 3 | — |
| `2026-09-03-verify-references-regelrunde` | 2026-09-03 | 1 | — |
| `2026-09-04-quiet-zone-uebertragung` | 2026-09-04 | 14 | — |
| `2026-09-04-quiet-zone-vorfilter` | 2026-09-04 | 10 | — |
| `2026-09-04-subhz-drift-quiet-zone` | 2026-09-04 | 1 | — |
| `2026-09-04-voyager-roh-doppler-zugang` | 2026-09-04 | 2 | — |
| `2026-09-05-bio-kanal-zeugen` | 2026-09-05 | 4 | **offen** |
| `2026-09-05-der-grat` | 2026-09-05 | 3 | — |
| `2026-09-05-extern-stellar-aktivitaet-xuv-co` | 2026-09-05 | 1 | **offen** |
| `2026-09-05-gate-bereinigung-abschluss` | 2026-09-05 | 4 | — |
| `2026-09-05-grat-rest-verifikation` | 2026-09-05 | 5 | — |
| `2026-09-05-grat-trishuli-konditionierung` | 2026-09-05 | 4 | — |
| `2026-09-05-korona-aia-fam` | 2026-09-05 | 1 | **offen** |
| `2026-09-05-nadel-v-lsst-scan` | 2026-09-05 | 0 | — |
| `2026-09-05-nadel-xiii-xuv-zensus` | 2026-09-05 | 3 | — |
| `2026-09-05-negativ-fuzzy-techno` | 2026-09-05 | 0 | — |
| `2026-09-05-phantom-island-williston-bc` | 2026-09-05 | 7 | **offen** |
| `2026-09-05-richtungs-transient-atom` | 2026-09-05 | 4 | — |
| `2026-09-05-techno-atmosphaeren-gase` | 2026-09-05 | 1 | **offen** |
| `2026-09-05-techno-narrowband-scan` | 2026-09-05 | 2 | — |
| `2026-09-06-extern-neutrino-cr-routen` | 2026-09-06 | 1 | **offen** |
| `2026-09-07-extern-weberin-faden-luecken` | 2026-09-07 | 16 | — |
| `2026-09-07-weberin-faden-luecken-folge` | 2026-09-07 | 18 | — |
| `2026-09-08-dispersions-ortungstest` | 2026-09-08 | 1 | — |
| `2026-09-08-dispersionsrelation` | 2026-09-08 | 2 | — |
| `2026-09-08-extern-weberin-zweitlinien` | 2026-09-08 | 8 | — |
| `2026-09-08-klassen-benchmark-pcmci` | 2026-09-08 | 2 | — |
| `2026-09-08-lisa-pathfinder-psd` | 2026-09-08 | 5 | — |
| `2026-09-08-rayleigh-dispersion` | 2026-09-08 | 1 | — |
| `2026-09-09-betriebspunkt-sweep` | 2026-09-09 | 1 | **offen** |
| `2026-09-09-galileo-nsurr-20` | 2026-09-09 | 0 | — |
| `2026-09-09-ned-objdir-zugang` | 2026-09-09 | 6 | — |
| `2026-09-09-nobel-dag-mci` | 2026-09-09 | 1 | — |
| `2026-09-28-pii-history-rewrite` | 2026-09-28 | 0 | — |

## Was quer liegt

- **Zeilennummern verrotten.** `sources.φ`, `commit_gate_vocab.json` und
  `sources.φ:9719` werden als Zeilen zitiert; nach jedem Register-Edit zeigen
  sie ins Leere. Lebende Zeiger brauchen Schlüssel, nicht Zeilennummern.
- **Pfade verrotten nach dem Umbau.** `state/` → `research/`, `rat/` →
  `research/models/raw/`, `auftrag/` → `projects/`. In ephemerides existiert
  **keiner** der vier zitierten Pfade.
- **`status: live` heißt nicht aktuell.** flyby ist seit dem 28.09. überfällig
  und trägt weiter `live`; gic ist seit §4.8–4.10 rot; sonne-erde widerspricht
  seinem eigenen Handover (ttl invertiert).
- **`status: archived` heißt nicht geschlossen.** 5 von 52 waren offen und
  sind jetzt wiederaufgenommen; bei zweien war der Marker nur ein Tabellenkopf.
- **Header-sha verrottet.** gic-Header `7cf6f41…` ≠ Gate-sha `5ab33d20`;
  vlies war vor heute ebenfalls stale.
- **Zwei Blocker waren schon gebaut.** flybys `flyby_path2_fill.rs` und IGETS'
  200 — der Text führte sie weiter als offen.

## Nächster Akt

1. Die fünf Wiederaufnahmen abarbeiten (`projects/archiv-wiederaufnahme.md`);
   je Fall ein Gate, kein `pending` ohne Grund.
2. Die sechs Nachtrags-Gates fahren (bias, ephemerides, flyby, gic, sonne-erde,
   wartend) — sie stehen in den Aufträgen.
3. Erst danach entscheiden, ob ein Fall ins Archiv zurück darf.
