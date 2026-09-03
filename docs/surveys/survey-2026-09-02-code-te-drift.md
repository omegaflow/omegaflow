<!--
  title: TE-Audit über Code, Ordner und Dateinamen
  class: survey
  date: 2026-09-02
  sha256: 5015c53251d77882cffe68ad7a57eb8f53d40d02c969a51f9625aa1cb7ca665b
  status: live
  see-also: src/mathematikerin/te.rs tools/measure/src/bin/code_drift_te.rs
-->

# TE-Audit über Code, Ordner und Dateinamen

Datum des Befunds: 2026-09-02. Die
Mathematikerin (`src/mathematikerin/te.rs`) misst selbst, ob deutsche
Zeichen und deutsche Wörter entlang der Lese-Reihenfolge entropisch
miteinander koppeln — die Reibung, die ein reines grep nur zählt.

## Werkzeug

`tools/measure/src/bin/code_drift_te.rs` — ein `omegaflow-work`-Bin, das die
echte Schätzung `omegaflow::te::transfer_entropy_lag` nutzt. Zwei Ebenen:

- **Zeilen-Ebene** (Lese-Reihenfolge): Einheit = jede `.rs`-Zeile in
  Pfad→Datei→Zeile-Ordnung. Serie X = deutsche Zeichen (ä ö ü ß Ä Ö Ü) pro
  Zeile, Serie Y = deutsche Lexikonwörter pro Zeile. Lag 1. Richtungsvergleich
  char→word und word→char.
- **Datei-Ebene** pro Ordner: Einheit = jede `.rs`-Datei in Namensordnung.
  Serie X = Deutschness des Dateinamens (Umlaut oder deutsches
  Namens-Stammwort: erfassen, messen, negativ, doppel, anomalie, korrelation),
  Serie Y = deutsche Zeichen des Datei-Inhalts. Richtung name→content.

Beide Ebenen gegen die Surrogat-Null (`surrogate_stats`, mean + 2σ über 10
Shuffle-Surrogate der Zielserie). SIGNIFICANT = gemessene TE überschreitet die
Schwelle.

## Befund

Aufruf: `cargo run --release -p omegaflow-work --bin code_drift_te src
tools/work/src/bin`

### src/ — der Kern ist sauber

- Zeilen-Ebene: `german_char_lines=1`, `german_word_lines=62` von 48236.
  `te_char->word` und `te_word->char` beide **null** — die 62 Wort-Treffer
  (Bindewörter wie `an`, `auf`, `de`, `ist` im Code) bilden kein strukturiertes
  Drift-Cluster entlang der Lese-Reihenfolge.
- Datei-Ebene: ein einziger deutsch benannter Datei-Gehalt — `doppel.rs`.

### tools/work/src/bin/ — die Reibung ist entropisch real

- Zeilen-Ebene: `german_char_lines=15`, `german_word_lines=143` von 61962.
  `te_char->word = 0.002405` (Schwelle 0.001322) und
  `te_word->char = 0.003053` (Schwelle 0.000856) — **beide SIGNIFICANT**. Die
  deutschen Wörter und Zeichen sind nicht verstreut, sie bilden entlang der
  Lese-Reihenfolge gekoppelte Cluster. Das ist die Reibung, die der frühere
  grep-Befund als Zeilenzahlen zählte, jetzt als Kopplung gemessen.
- Datei-Ebene: 5 deutsch benannte Werkzeuge, bestätigt:
  `doppel_anomalie_compiler.rs`, `mseed_messen.rs`,
  `pioneer11_negativ_fuzzy_probe.rs`, `pioneer_text_korrelation.rs`,
  `s1_post_erfassen.rs`. `te_name->content` ist **null** — der deutsche Name
  koppelt über die Datei-Ordnung nicht entropisch an den Inhalt. Er ist ein
  statischer Namens=Umsetzung-Defekt, kein Treiber; die Kopplung lebt auf der
  Zeilen-Ebene innerhalb der Dateien.

## Deutung

Die TE-Linse trennt zwei verschiedene Abweichungsklassen, die das bloße
Zählen vermengt:

1. **Echtes Drift-Regime** (tools/work/bin): deutsche Wort- und Zeichen-Serien
   koppeln entlang der Lese-Reihenfolge über die Surrogat-Null. Hier steht
   geschlossener deutscher Text — das ist der Gradient, der in einer Sitzung
   durchrutschte.
2. **Statische Namensdefekte** (src und tools/work): deutsche Dateinamen ohne
   Kopplung. Sie verletzen Name = Umsetzung, aber sie formen kein Regime.

`src/` fällt durch beide Ebenen: weder Kopplung noch Benennungsdrift (bis auf
`doppel.rs`). Der Kern hält die Sprach-Doktrin; die Werkzeug-Schicht trägt die
Abweichung.

## Grenzen der Messung

- Das Wörter-Lexikon ist eine kompakte Liste deutscher Bindewörter und
  Doktrin-Drifts; inhaltsbasierte deutsche Verben in Kommentaren (ohne Umlaut
  und ohne Listenwort) bleiben unerfasst. Ein False-Negativ ist möglich.
- Die Datei-Ebene deckelt bei n < 8 Dateien; die Zeilen-Ebene subsampled auf
  maximal 3000 Einheiten (Stride), um die O(n²)-Kosten der Schätzung zu
  begrenzen.
- Name→content ist über die Namensordnung (n = Dateien pro Ordner) schwach
  aufgelöst; eine koppelnde Benennungsmessung bräuchte mehr als 5 deutsche
  Namen über 150 Dateien.

## Folge

- Werkzeug verbleibt als reproduzierbare Messung (`tools/work/src/bin/
  code_drift_te.rs`), nicht als einmaliger Shell-Abriss.
- Die Datei-Ebene bleibt auf der Zeilen-Kopplung (Regime) und der
  Benennungszählung (statische Defekte); beide Befunde sind belegt, kein Edit
  wurde in dieser Sitzung an den Drift-Dateien vorgenommen.
