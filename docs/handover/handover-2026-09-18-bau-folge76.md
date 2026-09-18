<!--
  title: Handover — Bau-Folge 76 (Stand 2026-09-18)
  session: Bau-Folge 76
  class: handover
  date: 2026-09-18
  sha256: b5b9b0c697e802b1504494e7790f799da8beed563a9ba8d1e110fb61ae51b858
  status: live
-->
# Handover — Bau-Folge 76 (2026-09-18)

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

## Stehender Pass (gemessen 2026-09-18, HEAD 45190022)

- **Postfach** — letzter `state/mail/mail_ledger.φ`-Eingang `1789689115`
  (2026-09-18, Rubin-Forum-Thread, **kein Agenten-Eingang**); `post.md` trägt nur
  zwei `An entscheid`-Zeilen (Lasair, Pipeline-Port), **keine `An bau`-Zeile**.
- **CI am HEAD** — `ci-check` `35313990442` @`45190022` pending; `ci-check`
  `35313966498` @45190022 cancelled; `planetary-odf-cdn` `35313968728` in_progress;
  `te-gate` `35313041295` @`b8bb7b01` in_progress; `ci-check` `35313075218`
  @`cdb720d8` cancelled; `ci-check` `35307448019` failure @69c3ae96 (format/clippy,
  Wurzel in folge75 behoben); `pii-exposure` `35287195140` failure (exit 2 =
  Exposition bleibt, erwartet); `release-build` `35302966400` failure (veraltete
  `ci_manage`-Kopie, folge74); sonst fremde Linien.
- **HEAD** `45190022` == `origin/main` (FF-ready); zwischen Session-Start und jetzt
  pushte die Forschung-Linie (`ca58d3c9`, `45190022`) — fremde Commits, eigener
  Baum unberührt.
- **Sicherheitsnetz** — `refs/safety/1789711293` (Session-Start).

## TE-Gate — Restrisiken (a)(b) geschlossen, Gate-Verdikt ausstehend (härtester undatiert)

- **Restrisiko (b) geschlossen:** `arx_restricted_surrogate` (te.rs:1062) gibt
  jetzt `Option<Vec<f32>>` zurück; der `shuffle_series`-Fallback bei te.rs:1079
  (nicht-finite Rekonstruktion) und :1086 (OLS-Fit fehlgeschlagen) ist `None`
  (Verweigerungsarm). Aufrufstellen te.rs:1098 + :2719 angepasst; Test
  `arx_restricted_surrogate_refuses_instead_of_shuffling` (te.rs:3843).
  `residual_surrogate_conditional` + skalarer `transfer_entropy_lag`-Pfad
  unberührt (AGENTS.md: skalare Probe behält ihren broken-null-control-Record).
- **Restrisiko (a) geschlossen:** bidirektionaler Gate-Arm
  `gate_conditional_arx_fpr_fn_reversed_n1000` (te.rs:3937, `#[ignore]`) +
  `gate_conditional_driver_reversed` (te.rs:3706) + `gate_conditional_fpr_reversed`
  (te.rs:3757) — deckt TE x→y, spiegelbildlich zum y→x-Arm; Kriterien identisch
  (FPR ≤ 8 %, Rise ≤ 2pp, FN `found/meas ≥ 0.5`, `neg > 0`). te-gate.yml-Zeile
  ergänzt. Null-/Schätzer-Design unangetastet.
- **Rat-Einwand (registriert, kein Blocker):** der volle Fit orthogonalisiert die
  Residuen gegen x; die Nullen-Rekonstruktion stellt die x-getragene c-Leistung
  nicht wieder her → Null-Verteilung kann zu eng werden (FPR-Inflation). Das
  Kalibrier-Gate ist der Schiedsrichter.
- **Restrisiko (c):** Name = Implementation — Rename
  `arx_restricted_surrogate_conditional` nach grünem Gate (fittet jetzt das volle
  Modell).
- (Schritt: neuen `te-gate`-Lauf nach dem Bau-76-Commit einmal lesen
  (`ci_manage view <id>`/`ci_manage log <id>`); grün → Rename (c), rot → rote Zelle
  + FN-Arm `found/meas ≥ 0.5` beider Richtungen ist die Messung, Design verbreitern,
  nie die Verweigerung aufweichen.) · `wartend`

## Red main / Kanon-Gate — CI-Verifikation ausstehend

- clippy/format in folge75 behoben; `ci-check` `35313075218` @`cdb720d8` pending.
  Kanon-Gate (`phi/canon.φ` + Gate in `commit_check.rs`) steht am Baum, wartet auf
  den grünen `ci-check`-Lauf.
- (Schritt: `ci-check` des gepushten Bau-76-SHA einmal lesen; grün → beide
  geschlossen.) · `wartend`

## Offen (kein Handlungsschritt)

- **Bindings-Prosa** `phi/bindings/{dust-maske,bathymetrie-gebco}.φ` —
  Architektur-Akt mit Operator-/Council-Wort. · `operator-gebunden`
- **`opencode.json`** — fremder uncommitteter Hunk. · `operator-gebunden`
- **Scanned-/bild-only-PDFs → `vision`-OCR** und **`--pdf-text`
  Type0/Identity-H ohne ToUnicode** — kein Bau nötig. · `pending`

## Geteilter Baum — eigener Pfad-Satz

- **Eigener Commit-Pfad:** `src/mathematikerin/te.rs`,
  `.github/workflows/te-gate.yml`, `phi/supermag_stations.φ`,
  `docs/zustand/external-state.md`,
  `docs/handover/handover-2026-09-18-bau-folge76.md`,
  `docs/handover/archiv/handover-2026-09-18-bau-folge75.md` (Move).
- **Fremd (nicht anfassen):** `opencode.json`, `.github/workflows/harvest.yml`,
  `.github/workflows/planetary-odf-cdn.yml`,
  `tools/harvest/src/bin/gedi_l2a_compiler.rs`, die Forschung-Linien-Handover
  (`forschung-folge74`→archiv, neu `forschung-folge75`), die drei
  Handover-Archiv-Renames/Deletes der anderen Linien. Nie ein nacktes `git commit`.

## Benchmark

- **TE-Gate-Atom** (Restrisiken a/b) → `grind-max` (harte Klasse TE-/Null-Konstruktion,
  kein Flash-Double: kein aufgezeichneter Sieger in der Klasse); Ergebnis korrekt
  (cargo check sauber, beide Arme + Verweigerungstest gebaut). **supermag-Atom**
  → `grind-pro` (Urteil) → `descoped` gemessen. Burn nicht gemessen (Sub-Agent-Kontext).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
