<!--
  title: Kritik-Außenlesung — die Antwort des Bestands (LAIC/Bz/Korona)
  class: handover
  date: 2026-08-22
  sha256: 9325b5e8ca2704a91b6ec358b9b3eca2c3a60abb8c076d8db895d5ea47caf5a9
  status: live
  see-also: docs/surveys/survey-2026-08-21-laic-pfeilrichtung.md
    docs/surveys/survey-ein-blatt-korona-heizung.md
    docs/surveys/survey-2026-08-21-bz-kausalpfeil.md
-->
# Handover — Kritik-Außenlesung (2026-08-22)

## Auftrag

Eine externe Leserin (fremdes Modell) hat die Ein-Blatt-Dokumente
kritisiert: Methode legitim, Befunde teils überzogen, Stil als
Eigen-Ritual gelesen. Die Session hat die Kritik gegen den Bestand
geprüft (Ergebnis unten) und die Antwort vorbereitet. Fünf Schritte
warten auf das Wort des Operators — noch keiner begonnen:

1. **Kritik-Prüfung schreiben** — Verdikt-Tabelle Punkt für Punkt
   (zutreffend / bereits registriert / Fehl-Lesung / offen) als
   `docs/surveys/survey-2026-08-22-kritik-aussenlesung.md` + Register.
2. **Korona-Blatt korrigieren** — Minuten-Runde fam nachrechnen
   (`solar_dag_probe`-Maschinerie) oder die Lücke benannt lassen;
   Verdikt-Wortlaut schärfen („surrogat-signifikante TE bei lag 0/1“,
   nicht „die Pfeile der Korona-Heizung“).
3. **LAIC-Wording schärfen** — „auf dem Bestand abgeschlossen — die
   LAIC-Hypothese selbst bleibt Kanal-offen“ statt „abschließend
   gemessen“.
4. **Externe TE-Validierung** — Schreiber-2000-Standardbeispiel über
   die öffentliche API (te.rs unberührt) als Referenz-Probe.
5. **Literatur-Kalibrierung** — Subagent sammelt ENSO/Bz/LAIC-TE-
   Literatur → Vergleichs-Matrix (Kadenz, Lags, Surrogat-Konventionen)
   ins Register.

Gegenfrage der Leserin: welches Blatt zurückgeht — Empfehlung der
Session: **Korona** (größter Abstand Behauptung↔Messung); LAIC trägt
am saubersten. Entscheidung des Operators steht aus.

## Ist-Stand — die Prüfung der Kritik (verifiziert)

| Kritik-Punkt | Verdikt |
|---|---|
| TE-Methode legitim (Surrogate, 0 honored, Sensitivitäten) | zutreffend — kein Handeln |
| TE-Implementierung ungeprüft | teils Fehl-Lesung: `src/te.rs` trägt synthetische Ground-Truth-Tests (causal_positive, independent_near_zero, surrogate_threshold_below_causal_te, phase_surrogate_preserves_autocorrelation). Offen: externe Referenz-Validierung (Schreiber 2000), Einbettungs-/MI-Lag-Sensitivität des topologischen Pfads |
| LAIC: Hypothese nicht getestet, nur Boden-F-Proxy, 0–5 Ereignisse je Fenster | zutreffend und bereits im Befund-Register benannt („Was das Blatt nicht trägt“) |
| LAIC: Befund überzogen | Fehl-Lesung am Verdikt (der Befund ist Stille); zutreffend an der Endgültigkeits-Sprache („definitiv“, „abschließend gemessen“) → Schritt 3 |
| Korona: Pfeile überzogen, Alfvén/nanoflare ungedeckt | zutreffend: die Minuten-Runde trägt keine fam-Schwelle (fam der Tages-Runde deckt sie nicht — BLATT_PAPIER_RESULTAT §4 verlangt die Korrektur vor jedem Blatt); lag 0/1 bei 1-min-Zellen löst ~100 s nicht auf → Schritt 2 |
| Bz: Familien-Schwellen-Ehrlichkeit | Bestätigung der Leserin — kein Handeln |
| Kein Peer-Review, keine Literatur-Kalibrierung | zutreffend → Schritte 1 und 5 (die Kritik-Prüfung ist die erste benannte Außen-Lesung) |
| Ritual-Vokabular suggeriert Endgültigkeit | Außen-Lesart benannt; die Endgültigkeits-Wörter sind konkret schärfbar, das Identitäts-Vokabular bleibt Operator-Entscheid |

## Kontext — Refactoring

Der Operator kündigt ein kurzes Refactoring an, bevor die Schritte
laufen. Die empfangende Session prüft nach dem Refactoring: `cargo
check` 0/0, `laic_probe` kompiliert, `phi/pipeline/laic_harvest/`
(4.1 GB Ernte, 1726+60 Fenster) unversehrt, `src/te.rs` unberührt.

## Gates

- Jeder Schritt = eigener Commit; Registerzeile im selben Commit.
- sha256 aller berührten Docs neu rechnen (Body ohne Header).
- `src/te.rs`, `nobel_probe_corona`, `transfer_entropy_lag` bleiben
  unberührt — Validierung läuft über die öffentliche API.
- Nach eigenem Commit dieses Handover archivieren.
