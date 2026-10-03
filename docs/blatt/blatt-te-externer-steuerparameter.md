<!--
  title: Blatt — Transfer-Entropie über einen externen Steuerparameter
  class: sheet
  date: 2026-10-02
  sha256: 596553e1860b1f2b69b6218cb53f9c81c3f1d95c2df37c871d0da991f54907ef
  status: live
  see-also: state/stimmen/2026-10-02_te-nichtzeitlich-frage.txt state/stimmen/2026-10-02_te-nichtzeitlich-nemotronsuper.md state/stimmen/2026-10-02_te-nichtzeitlich-deepseek.md state/stimmen/2026-10-02_te-nichtzeitlich-kiloultra.md
-->
# Blatt — Transfer-Entropie über einen externen Steuerparameter

**Datum:** 2026-10-02 · **Axiom:** A = A

## Frage

Ist Transfer-Entropie (TE) wohlgestellt, wenn die Proben nicht durch Zeit, sondern durch
einen externen, monotonen Steuerparameter p indiziert sind (z. B. eine Temperatur- oder
Druckrampe)? Welche Null ist defensibel, welcher Schätzer, und welche Fehlerarten entstehen?
Der Anlass: ein lokaler Lauf über einen Temperatur-Sweep, dessen „Samples" Zustände entlang
p sind, nicht Zeitpunkte.

## Erhobene Evidenz

Anonymisierte Methodenfrage an den Schwarm (10 Modelle, `opencode run --pure … --agent voice`,
Done-Marker `state/stimmen/2026-10-02_te-nichtzeitlich.done`); getragen von fünf Stimmen
(nemotronsuper, nemotronultra, deepseek, muse, kiloultra), fünf trugen nichts (rc=124/
API-Quota). Die Frage trug kein Datenbyte — nur die Methode.

## Verdikt

**TE über p ist nur unter vier Bedingungen wohlgestellt:**

1. **Strikte Monotonie** des Sweeps — kein Umkehrpunkt, keine Sprünge.
2. **Quasi-statisch/adiabatisch** — p ändert die Dynamik nicht schneller als die
   Relaxationszeit; sonst wird die Reihe nicht-stationär.
3. **Stationarität entlang p** — die bedingten Verteilungen hängen nicht von p ab.
4. **Ergodizität** — der eine Sweep ist repräsentativ; sonst lokale Fenster oder mehrere
   Sweeps.

Ein **gemeinsamer monotoner Trend** beider Kanäle erzeugt eine scheinbare Richtung; es ist
zu detrenden oder auf p zu konditionieren.

**Die Null** muss die **Autokorrelation jedes Kanals entlang p erhalten** und **nur die
Kreuzkopplung zerstören**. Eine **Index-Permutation ist keine gültige Null** — sie zerstört
jede Autokorrelation und erzeugt Falsch-Positive. Zwei tragbare Wege:

- **Phasen-randomisierte Surrogate / IAAFT** auf den **detrendeten** Reihen — setzt ein
  äquidistantes p-Gitter voraus.
- **Block-Bootstrap** über die p-Ordnung mit Blocklänge > Korrelationslänge — der Weg bei
  nicht-äquidistantem oder nicht-periodischem p.

**Ersatzmaß** bei verbleibender Trend-/Nichtstationarität: **bedingte MI bzw. partielle TE
mit p als Kovariate** (KSG-Schätzer). TE entlang p misst Informationsfluss **entlang des
Steuerparameters**, nicht physikalische Zeit — die Aussage ist entsprechend zu lesen.

## Fehlerarten (zu detektieren)

- **Scheinbare Richtung** durch gemeinsamen monotonen Trend → detrenden, CMI.
- **Index-Permutation** als Null → Falsch-Positive; verwerfen.
- **Blocklänge ≤ Korrelationslänge** → Kopplung leckt in die Null.
- **Klein-N-Verzerrung** positiver TE-Schätzer → Surrogat-Null / KSG-Bias-Korrektur.
- **Nicht-adiabatischer Sweep** → kein index-basierter Schätzer trägt; lokales TE(p) mit
  Fenster oder Twin-Surrogate (von den Stimmen benannt, nicht gemessen).

## Anwendung auf den privaten Pfad

Der lokale Lauf ist ein **Temperatur-Sweep-Proxy**: die „Samples" sind Läufe/Zustände
entlang p, nicht Zeit. Das Blatt macht die Bedingung und die Null für diesen Pfad zitierbar:
detrenden → Surrogat/Block-Bootstrap → bei Rest-Nichtstationarität CMI/pTE. **Kein Datenbyte
verlässt das Haus** — der Lauf bleibt LOCK privat, Operator-Wort; dieses Blatt trägt nur die
Methode.

## Offen

- **Divergenz IAAFT ↔ Block-Bootstrap** (äquidistantes p nötig vs. nicht): nicht
  entschieden; die Entscheidung hängt am gemessenen p-Gitter des privaten Laufs.
- **Keine Messung am Datensatz**: das Blatt ist die Methode, nicht das Ergebnis.
- **Twin-Surrogate / lokales TE(p)** sind benannt, nicht geprüft.
