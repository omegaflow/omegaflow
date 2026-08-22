<!--
  title: Bz-Blatt — Selbst-Übergabe vor dem Refactoring (Stand + offene Atome des Bz-Tracks)
  class: handover
  date: 2026-08-22
  sha256: bfc4691bc76ebd1f31f260757717b2c434d7e64e78bba8a7ba4136b93061bd5a
  status: live
  see-also: docs/surveys/survey-2026-08-21-bz-kausalpfeil.md docs/handover/handover-2026-08-21-selbstuebergabe-enso-matrix.md
-->
# Bz-Blatt — Selbst-Übergabe vor dem Refactoring

Geschrieben von der schließenden Session (Bz-Track), gelesen von genau
einer empfangenden Session (Bz-Track, nach dem Refactoring). Der
Operator hat das Refactoring angekündigt; dieses Handover friert den
Stand des Bz-Tracks ein, damit die empfangende Session ohne
Wiederholung weitermacht. Die fünf Atome der ENSO-Selbst-Übergabe
(`handover-2026-08-21-selbstuebergabe-enso-matrix.md`: Ground-Truth-
Hénon, Literatur-Kalibrierung, Claims-Audit, Global-Akteure, offene
Operator-Fragen) gehören dem Nachbar-Track — hier nur referenziert,
nicht dupliziert.

## Was steht (Commits des Bz-Tracks)

- `97f78f6` Atom 1 — INTERMAGNET-Komponenten-Port: BGS-GIN HAPI XYZF
  1-min, ABK-Auroral-Block + Fanout, `hapi_fill`-Gate 99999.0,
  HAPI-Fallback trägt Vektor-Spalten ohne `parameters`-Array,
  `parse_iso_tdb` toleriert HH:MM.
- `258f3ed` Atom 2 — `src/bin/bz_blatt_probe.rs`: RTSW active-only ×
  ABK-dB/dt, 1-min-Gitter, Lag-Sweep 0–120 min, drei Nullkontrollen.
- `09e38d4` Atom 3 — das Blatt `docs/surveys/survey-2026-08-21-
  bz-kausalpfeil.md` (sha256 af567ec2…); Handover bz-paradoxon
  konsumiert (archive/handover/, status consumed), die vier
  Bz-Handover-Varianten geräumt.
- `589b39a` Recherche — Status-Matrix (definitive/quasi-def/reported/
  adjusted/best-avail × PT1M/PT1S × native/xyzf/hdzf/diff, 3074
  Datensätze; definitive →2021, quasi-def 2012→~1 Monat, P366D je
  Request).
- `cf9883e` Korrektur — der Fanout trug die 154 Observatorien schon
  über `stations GetCapabilities`: `fanout 154` (ein Block, Auroral-
  Ring eingeschlossen), der zweite Fanout entfernt.
- `bae72cd` Familien-Schwelle in `bz_blatt_probe` — fam = max
  Surrogat-TE der Runde (ENSO-Muster).
- `fb1f7a3` Retro-Urteil — `src/bin/bz_retro_probe.rs` (Ernte-Cache +
  `--stride` + `--harvest-only`), gemessen.

## Die zwei Blatt-Zeilen (der Stand der Wahrheit)

- Minuten-Skala (live, 22 h, 1-min): TE(Bz→dB/dt) 2.18e-1 über der
  eigenen Schwelle 2.08e-1 bei lag 60 min — gerichtet, aber fam
  3.74e-1 hält alle sechs Paare (family bound).
- Tages-Ensemble (1994→2026, stride 3, n 3916): still —
  TE(Bz→dB/dt) 1.25e-1 unter eigener Schwelle 1.39e-1 und fam
  1.89e-1; das Tagesmittel trägt den Treiber nicht, die Südwärts-
  Exkursion lebt sub-täglich (0 honored).

## Die offenen Atome des Bz-Tracks (Wort des Operators ausstehend)

1. **Der 1-h-Ensemble** — der fam-signifikante Minuten-Pfeil über
   Stürme. OMNI2-Recompile `--decimate-min 60` (der Compiler
   decimiert standardmäßig 1440 min → `omni2_serie.bin` ist täglich;
   der Recompile schreibt stündlich, die Fenster-Loop-Parameter stehen
   im Compiler) × INTERMAGNET stündlich (downsampled vom 1-min,
   dieselbe Monats-Chunk-Ernte wie `bz_retro_probe`). Lag 0/1 h
   straddeln die L1-Laufzeit. Register: TODO Bz-Eintrag trägt den
   Atom als offen.
2. **Kp-Vergleichszeile** — lebt als no-statement (n = 7 im
   1-min-Live-Fenster); die 3-h-Zeile wird erst mit dem 1-h-Ensemble
   tragfähig.

## Praktisches Session-Wissen (nur diese Session trägt es)

- **BGS-GIN-Eigenheiten:** Jahres-Requests (365 d, ~31 MB) werden vom
  Server mit Verbindungs-Reset zurückgewiesen — nur Monats-Chunks
  funktionieren. `stop` jenseits der Dataset-stopDate → HAPI-Error
  1405 — stop auf now−2 h klemmen. Lowercase-Stationscodes lösen auf
  (der Fanout-Pfad ist sauber).
- **Laufzeiten:** `bz_blatt_probe` ~2 min (release). `bz_retro_probe`
  Ernte ~40 min (396 Monats-Fetches), Compute bei vollem n ~35 min
  (O(n²)-KDE, ~15 s/Auswertung), `--stride 3` ~12 min — abgelöst
  laufen lassen (nohup + pollen), nie im 10-min-Shell-Cap.
- **Ernte-Cache:** `abk_dbdt_daily.tsv` (11889 Tage, untracked wie
  `omni2_serie.bin` — Datenartefakte bleiben untracked). Die Messung
  lädt den Cache automatisch; `--force-harvest` erzwingt neu.
- **Geteilter Baum:** parallele Sessions committen mit `git add -A` —
  unkommittete Edits werden in fremde Commits mitgezogen (diese
  Session: zwei TODO-Zeilen landeten in `eae987f`; `src/archivar.rs`
  wurde zweimal auf HEAD zurückgesetzt). Konsequenz: eigene Einheiten
  früh committen, gezielt stagen (nie `-A`), nach fremden Commits
  `git diff` gegen die eigenen Dateien prüfen.
- **Die Blatt-Kritik (Claude, drei Schichten: Methode legitim /
  Schlussfolgerungen überziehen / Stil):** der methodisch scharfe
  Punkt — die Minuten-Pfeile der Nadel III sind nicht fam-gereinigt —
  ist im Nachbar-Track als Claims-Audit registriert. Der Bz-Track ist
  der fam-Standard, auf den die anderen Blätter gehören; das Bz-Blatt
  wurde in der Kritik ausdrücklich gelobt.

## Gates für die empfangende Session

- `cargo check` 0 Fehler / 0 Warnungen (Stand der letzten Prüfung).
- Reproduzierbar: `cargo run --release --bin bz_blatt_probe` und
  `./target/release/bz_retro_probe --stride 3` (Cache vorhanden).
- Das Blatt trägt sha256 af567ec2… — jede Wort-Änderung rechnet die
  Summe neu (Körper ohne Header).

## Reihenfolge

Das Refactoring kommt zuerst (Operator). Danach, auf das Wort des
Operators: Atom 1 (1-h-Ensemble) — der letzte offene Schritt des
Bz-Pfeils. Atom 2 (Kp-Zeile) hängt daran.
