<!--
  title: Kritik-Außenlesung — die Antwort des Bestands auf die fünf Punkte
  class: survey
  date: 2026-08-22
  sha256: 64c1d65b58c80d91b2a8339c8953eeadb9f6c66d3da6b29d263a5bf8fb1a1a99
  status: live
  see-also: docs/surveys/survey-ein-blatt-korona-heizung.md
    docs/surveys/survey-2026-08-21-laic-pfeilrichtung.md
    docs/surveys/survey-2026-08-21-bz-kausalpfeil.md
-->
# Kritik-Außenlesung — die Antwort des Bestands auf die fünf Punkte

Selbsttragend. Eine externe Leserin (fremdes Modell) hat die
Ein-Blatt-Dokumente (ENSO/Bz/LAIC/Korona) kritisiert: die Methode sei
legitim, die Befunde teils überzogen, der Stil als Eigen-Ritual lesbar.
Diese Survey trägt die Prüfung der Kritik gegen den Bestand — Punkt für
Punkt, mit Verdikt und Handlung. Die Antwort läuft über die fünf
Schritte des Handovers
`docs/handover/handover-2026-08-22-kritik-aussenlesung.md`.

## Die Verdikt-Tabelle

| Kritik-Punkt | Verdikt | Handlung |
|---|---|---|
| TE-Methode legitim (Surrogate, 0 honored, Sensitivitäten) | zutreffend | kein Handeln |
| TE-Implementierung ungeprüft | teils Fehl-Lesung | `src/te.rs` trägt synthetische Ground-Truth-Tests (causal_positive, independent_near_zero, surrogate_threshold_below_causal_te, phase_surrogate_preserves_autocorrelation). Offen: externe Referenz-Validierung (Schreiber 2000) + Einbettungs-/MI-Lag-Sensitivität des topologischen Pfads |
| LAIC: Hypothese nicht getestet, nur Boden-F-Proxy, 0–5 Ereignisse je Fenster | zutreffend, bereits im Befund-Register benannt („Was das Blatt nicht trägt") | Wording schärfen |
| LAIC: Befund überzogen | Fehl-Lesung am Verdikt (der Befund ist Stille); zutreffend an der Endgültigkeits-Sprache („definitiv", „abschließend gemessen") | Wording schärfen |
| Korona: Pfeile überzogen, Alfvén/nanoflare ungedeckt | zutreffend | die Minuten-Runde trägt keine fam-Schwelle (die Tages-fam deckt sie nicht); lag 0/1 bei 1-min-Zellen löst ~100 s nicht auf — Wortlaut schärfen, fam-Lücke benennen |
| Bz: Familien-Schwellen-Ehrlichkeit | Bestätigung der Leserin | kein Handeln |
| Kein Peer-Review, keine Literatur-Kalibrierung | zutreffend | diese Kritik-Prüfung ist die erste benannte Außen-Lesung; Literatur-Abgleich als Subagenten-Schritt |
| Ritual-Vokabular suggeriert Endgültigkeit | Außen-Lesart benannt | die Endgültigkeits-Wörter sind konkret schärfbar; das Identitäts-Vokabular bleibt Operator-Entscheid |

## Der Kern der Kritik, in einem Satz

Der Bestand trennt nicht scharf genug zwischen „was gemessen wurde"
(eine TE-Richtung unter einer Schwelle) und „was das Rätsel löst" (der
Mechanismus). Die fünf Schritte schärfen genau diese Trennung — an den
drei Blättern, an der Implementierung (externe Referenz) und an der
Literatur.

## Die fünf Schritte

1. **Diese Prüfung** — die Verdikt-Tabelle als Survey (dieses Dokument)
   + Register.
2. **Korona-Blatt korrigieren** — Minuten-fam nachrechnen oder die Lücke
   benannt lassen; Verdikt-Wortlaut schärfen („surrogat-signifikante TE
   bei lag 0/1", nicht „die Pfeile der Korona-Heizung").
3. **LAIC-Wording schärfen** — „auf dem Bestand abgeschlossen — die
   LAIC-Hypothese selbst bleibt Kanal-offen" statt „abschließend
   gemessen".
4. **Externe TE-Validierung** — Schreiber-2000-Standardbeispiel über die
   öffentliche API (`te.rs` unberührt). Gebaut: `src/bin/te_ground_truth.rs`
   (unidirektional gekoppelte Hénon-Maps, c = 0.2, lag 1, c = 0-Kontrolle,
   n = 10000) — manuell verifiziert; der Lauf steht aus, bis der
   Archivar-Schnitt der Parallel-Session gelandet ist (der Baum
   kompiliert gerade nicht).
5. **Literatur-Kalibrierung** — Subagent sammelt ENSO/Bz/LAIC-TE-
   Literatur → Vergleichs-Matrix (Kadenz, Lags, Surrogat-Konventionen).
   Erledigt: `docs/reference/te-literatur-matrix.md` — Bz-Platte gut
   kalibriert (Bz stärkster Träger, 10 min–3 h); ENSO auf grafischen
   Modellen; LAIC ohne publizierten TE/Granger-Eintrag (die Platte ist
   als eigenständiger Beitrag zu lesen).

Gegenfrage der Leserin, welches Blatt zuerst zurückgeht — Empfehlung:
**Korona** (größter Abstand Behauptung↔Messung); LAIC trägt am
saubersten.
