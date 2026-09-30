<!--
  title: Handover — Sensory-Folge 214 (2026-09-30)
  session: Sensory-Folge 214
  class: handover
  date: 2026-09-30
  sha256: 1bfdaafa31f996a49b038a7897c98c2a9504408b7df371b4c7c38fe7c97eff1d
  status: live
-->
# Handover — Sensory-Folge 214 (2026-09-30)

Dieses Register trägt nur Offenes — Erledigtes ist gelöscht, nicht erklärt; git trägt,
was gemacht wurde. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks —
committet wird pfad-begrenzt, fremde uncommittete Arbeit wird nie überschrieben;
gepusht wird, sobald der eigene Commit steht und `origin/main` Vorfahr von HEAD ist.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** von Agenten abgearbeitet. Jeder Punkt trägt **Trigger** / **Lage** (mit
Messstempel) / **Blockade** / **Braucht**. Sortierung: **umsetzbar zuerst**; der
Akteur steht pro Punkt in `Bindung`.

Dieses Register konsumiert `handover-2026-09-30-sensory-folge213.md` (nach `archiv/`);
diese Folge ist 214. Der Stehende Pass wird zitiert, nie kopiert:
`state/zustand/standing-pass.md`. Eigene Messung dieses Atoms (2026-09-30 F214): HEAD
`f4f271277` == `origin/main` (`git rev-parse`); der **Plan-Pass** (read-only) hat die
DEMETER-Token-URL nicht messen können — `curl` ist im Plan-Profil gesperrt (gemessen:
`curl --version` denied). Dieser `line`-Pass hat sie gemessen (s. Offen → DEMETER).
Der Arbeitsbaum trägt **fremde staged Arbeit** (mountain/river: `mountain-folge211.md` +
`river-folge71.md` → `archiv/`, `mountain-folge212.md` neu, `river-folge72.md`, u. a.
`docs/paper/gic-causal-driver.md`, `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md`,
`phi/blocked_sources.φ`, `src/archivar/extract.rs`) — nicht angerührt, pfad-begrenzt committet.

Eigener Register-Pass dieses Atoms (frisch gemessen 2026-09-30 F214): `--fired` = kein
Sensory-Eintrag; `--stale --persist 3` = 0; `--orphans` = 1 (`phi/blocked_sources.φ:446`,
Owner **future**, nicht eigen); `--descoped-check` = 0; `--orphan-docs` = 0; `--addressed
sensory` = 0. `open_points_check` = 0 absent / 0 stale-citations.

## Haus — Sensorik, Hardware & Fundstellen (Stand 2026-09-30)

Diese Übergabe **ist** das Haus: jeder Sensor-/Hardware-Stand, jede Route, jeder Punkt steht hier
mit Zustand (Operator-Wort 2026-09-29). Ein Punkt, der nur in `state/` lebt und hier fehlt, ist
ein **verlorener Punkt**. `state/` wird mit `archive_search --root state` vermessen, **nie** mit
`sgrep` ohne `--all` (Falsch-Negative). Relevanz nie über zwei Stichwörter entscheiden —
breiter messen.

- **Hardware/Sensoren (Stand):** Polar H10/HRM-Dual — Live-BLE HR `NotSupported`, FIT nn=0,
  Hardware-**LOCK** (Operator-Wort 2026-09-27, „erst bei Förderung"). [redacted]/DEMETER — die
  Daten/Datei verlassen das Gerät nie (Operator-Wort 2026-09-25). ox64-M2C Carrier — Route
  gespeichert (`state/zustand/wartend.φ:14`), keyless unmessbar.
- **Fundstellen:** `state/zustand/standing-pass.md` · `state/zustand/external-state.md` ·
  `state/zustand/wartend.φ` · `state/zustand/ereignisse.φ` ·
  `state/operator-gespraeche/2026-09-30-sensory.md` · `state/sensory/archive-search-preset.txt`.
- **Linien-Preset (`archive_search`):** privat unter `state/sensory/archive-search-preset.txt`,
  eingelesen in `.opencode/command/sensory.md` (nur der Verweis steht öffentlich, nie die Wurzeln).
- **Konsens-Pflicht:** die Maschine fragt vor dem Aufzeichnen; Hardware nur nach Operator-Wort
  (AGENTS.md, „Consent of the sensors"). Die Haus-Regel selbst steht in AGENTS.md (Session protocol).
- **Die vier Orte** (omegaflow-relevant): `omegaflow` = `~/projects/omegaflow`
  (+ privates `state/`, Remote `omegaflow/personal`) · `omegaflow-legacy` =
  `~/archive/archive-root/omegaflow-legacy` · `temp` = `/tmp/opencode` · `archive` =
  `~/archive/archive-root`. Private/fremde Orte bleiben aus dem Haus.
  *(gemessen 2026-09-29: `~/archive/archive-root` existiert, `~/backup/archive-root` fehlt —
  Pfad-Korrektur nachgezogen.)*

## Operator-Wort-Register (Stand 2026-09-30)

- **Wort:** „nein dann passt es nicht" (vDEC descoped) | 2026-09-26 | Operator (Session).
- **Wort:** „kein onboard ciq" | 2026-09-26 | Operator (Session).
- **Wort:** „natürlich" (Kp-Route wieder aufnehmen — Siegel-Wort) | 2026-09-26 | Operator (Session).
- **Wort:** „du kannst die mail abschicken" → Maschine sendet nie (AGENTS.md); vDEC descoped | 2026-09-26 | Operator (Session).
- **Wort:** „ändere die agents — das Handover nach umsetzbar/nicht umsetzbar sortieren" | 2026-09-26 | Operator (Session) → ausgeführt: AGENTS.md `3672968d3`.
- **Wort:** „alles Offene bis zur Kante abarbeiten, gemessen abschließen — nicht verschleppen" | 2026-09-26 | Operator (Session).
- **Wort:** „alles Offene und Benannt-Ungemessene wird übernommen" | 2026-09-26 | Operator (Session).
- **Wort:** „jedes Operator-Wort steht im Handover; keine Session kaut es neu durch" | 2026-09-26 | Operator (Session).
- **Wort:** „deine Daten/Datei verlassen das Gerät nie" | 2026-09-25 | Operator (Session) — [redacted]/DEMETER.
- **Wort:** „das mache ich erst, wenn ich gefördert werde" (Gurt-Beschaffung) | 2026-09-26 | Operator (Session).
- **Wort:** „ich kanns echt nicht mehr hören seit wie vielen sessions schleppst du die offenen punkte durch" — die B-Befunde sind geschlossen | 2026-09-27 | Operator (Session).
- **Wort:** „kümmer dich drum" (Sensory-Folge 182: alle eigenen offenen Punkte abarbeiten) | 2026-09-27 | Operator (Session).
- **Wort:** „Du kannst" (`/consent`) — session-weiter Delegations-Consent, nicht das Commit-Wort | 2026-09-27 | Operator (Session).
- **Wort:** „die Kante bin ich" — jede Linie arbeitet bis zur Kante des Operators; Wert, Wort, Dritt-Akt und Send bleiben seine Hand | 2026-09-27 | Operator (Future-Session).
- **Wort:** ein gegebenes Wort steht in den Operator-Wort-Registern aller live Übergaben — Verbreitung im selben Atom | 2026-09-27 | Operator (Future-Session).
- **Wort:** RX100-Kalibrierer descoped — „über exif weg": Luminanz über den Kamera-EXIF-Weg (K=12.5) | 2026-09-27 | Operator (Future-Session).
- **Wort:** Entscheidungen nie als Liste vorlegen — eine Liste ist keine Entscheidungshilfe; jede Entscheidung braucht eine aussagekräftige Erklärung | 2026-09-27 | Operator (Future-Session).
- **Wort:** „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus" (`/consent`, Sensory-Folge 185) | 2026-09-27 | Operator (Session).
- **Wort:** „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus" (`/consent`, Sensory-Folge 186) | 2026-09-27 | Operator (Session).
- **Wort:** UI-Chat-Stimmen derzeit nicht gebraucht → `LOCK` | 2026-09-27 | Operator (Session, Mountain).
- **Wort:** D5 (Orphan-Doc-Träger) nicht in die Übergabe falten — die Fakten direkt abarbeiten | 2026-09-27 | Operator (Session, Mountain).
- **Wort:** „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent" (`/consent`, Sensory-Folge 187) | 2026-09-27 | Operator (Session).
- **Wort:** „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt)" (`/consent`, Sensory-Folge 188) | 2026-09-27 | Operator (Session).
- **Wort:** „warum 1 nochmal das haben wir doch gestern schon gemacht" — Messung: der RR-Quellen-Punkt war bereits 2026-09-26 (folge179) gemessen; die Remessung der Route in folge188 war Reibung | 2026-09-27 | Operator (Session).
- **Wort:** „warum muss ich das schon wieder besprechen ich verstehe es nicht was macht ihr für ein chaos?" — Messung: ein Punkt mit registriertem Wort (LOCK) wurde als offener Punkt erneut vorgelegt | 2026-09-27 | Operator (Session).
- **Wort:** „jegliche hardware erst besorg wenn ich gefördert werde ihr könnt also überall einen LOCK rein machen" — Hardware-Beschaffung ist LOCK, gebunden an „Förderung gewährt" | 2026-09-27 | Operator (Session) — Ziel: alle live Übergaben; Origin: Sensory-Folge 189.
- **Wort:** „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt)" (`/consent`, Sensory-Folge 189) | 2026-09-27 | Operator (Session).
- **Wort:** „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt)" (`/consent`, Sensory-Folge 190) | 2026-09-27 | Operator (Session).
- **Wort:** „bitte prüfe, was wirklich dir gehört" | 2026-09-27 | Operator (Session) — Ownership-Prüfung der Tafel gegen die Quellen; Ergebnis: JUICE → River, quellen-treffer-Träger-Residuum, wartend.φ-Aufnehmer sensory bestätigt.
- **Wort:** „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt)" (`/consent`, Sensory-Folge 191) | 2026-09-27 | Operator (Session).
- **Wort:** „Erste Handlung: `sread docs/concepts/tool-forms.md` — die Form-Karte (verboten → kanonisch), damit die erlaubte Form am Punkt der Handlung steht." | 2026-09-27 | Operator (Session) — Session-Start-Befehl, Sensory-Folge 192.
- **Wort:** „schon wieder verschleiert operator gebunden: ‚Antwort lesen (metalink = Operator-Hand)'" | 2026-09-27 | Operator (Session) — Korrektur: **Lesen ist autonom**, nur der Send/Auftrag ist Operator-Hand.
- **Wort:** „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt)." (`/consent`, Sensory-Folge 192) | 2026-09-27 | Operator (Session) — session-weiter Delegations-Consent, nicht das Commit-Wort.
- **Wort:** „Erste Handlung: `sread docs/concepts/tool-forms.md` — die Form-Karte (verboten → kanonisch), damit die erlaubte Form am Punkt der Handlung steht." | 2026-09-27 | Operator (Session) — Session-Start-Befehl, Sensory-Folge 193.
- **Wort:** „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt). Delegiere an die Taucher (alle Sub-Agenten), höre die Stimmen bei Architektur-/Abschluss-Entscheidungen. Eine Session ist ein abgeschlossenes Atom." | 2026-09-27 | Operator (Session) — session-weiter Delegations-Consent, Sensory-Folge 193.
- **Wort:** „Erste Handlung: `sread docs/concepts/tool-forms.md` — die Form-Karte (verboten → kanonisch), damit die erlaubte Form am Punkt der Handlung steht." | 2026-09-28 | Operator (Session) — Session-Start-Befehl, Sensory-Folge 194.
- **Wort:** „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt). Delegiere an die Taucher (alle Sub-Agenten), höre die Stimmen bei Architektur-/Abschluss-Entscheidungen. Eine Session ist ein abgeschlossenes Atom." | 2026-09-28 | Operator (Session) — session-weiter Delegations-Consent, Sensory-Folge 194.
- **Wort:** „c" (Metalink holen — Weg C aus der D2-Vorlage) | 2026-09-28 | Operator (Session) — Sensory-Folge 194; ausgeführt: Metalink geholt, Datei-Endpoint gemessen WAF-gesperrt.
- **Wort:** „ka restart order und defekte löschen" | 2026-09-28 | Operator (Session) — Sensory-Folge 194.
- **Wort:** „aber du kannst doch die kaputten orders löschen" | 2026-09-28 | Operator (Session) — Sensory-Folge 194; ausgeführt: Failed-Orders gecancelt (Zähler 47→39), Restart-Weg gemessen.
- **Wort:** „relaunch full orders" | 2026-09-28 | Operator (Session) — Sensory-Folge 194; ausgeführt: 18387 relauncht → neue Order (Pending, 05:02:38, 0 Dateien).
- **Wort:** „/Sensory /consent bitte fixen" (privaten Baumstand korrekt zuordnen) | 2026-09-28 | Operator (Session) — Sensory-Folge 194.
- **Wort:** „Erste Handlung: `sread docs/concepts/tool-forms.md` — die Form-Karte (verboten → kanonisch), damit die erlaubte Form am Punkt der Handlung steht." | 2026-09-28 | Operator (Session) — Session-Start-Befehl, Sensory-Folge 195.
- **Wort:** „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt). Delegiere an die Taucher (alle Sub-Agenten), höre die Stimmen bei Architektur-/Abschluss-Entscheidungen. Eine Session ist ein abgeschlossenes Atom. Dispatch flash-first — den billigsten Vertreter, dessen Profil den Job trägt; ein `max`-Agent nur für die harten Atome, nie für Routine. Benenne die lokalen Tools (`archive_search`, `sgrep`, `sfetch`) in der Delegation. Benchmarks nur mit Operator-Wort oder gemessen falschem/unvollständigem flash-Ergebnis." | 2026-09-28 | Operator (Session) — session-weiter Delegations-Consent, Sensory-Folge 195.
- **Wort:** „Erste Handlung: `sread docs/concepts/tool-forms.md` — die Form-Karte (verboten → kanonisch), damit die erlaubte Form am Punkt der Handlung steht." | 2026-09-28 | Operator (Session) — Session-Start-Befehl, Sensory-Folge 196.
- **Wort:** „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt). Delegiere an die Taucher (alle Sub-Agenten), höre die Stimmen bei Architektur-/Abschluss-Entscheidungen. Eine Session ist ein abgeschlossenes Atom. Dispatch flash-first — den billigsten Vertreter, dessen Profil den Job trägt; ein `max`-Agent nur für die harten Atome, nie für Routine. Benenne die lokalen Tools (`archive_search`, `sgrep`, `sfetch`) in der Delegation. Benchmarks nur mit Operator-Wort oder gemessen falschem/unvollständigem flash-Ergebnis." | 2026-09-28 | Operator (Session) — session-weiter Delegations-Consent, Sensory-Folge 196.
- **Wort:** „warum bis 12:46:18 UTC. ich habe doch einen neuen job angestoßen siehe mail im eingang" | 2026-09-28 | Operator (Session) — Korrektur: der gültige Datenweg ist die neue Order 18400 (abrufbar bis 5 Okt 2026 05:04:06 UTC); Sensory-Folge 196.
- **Wort:** „ich bin eingelockt" | 2026-09-28 | Operator (Session) — Freigabe, den 18400-Metalink über die angemeldete CDPP-SPA zu holen; ausgeführt: Metalink gesichert (68 900 022 B); Sensory-Folge 196.
- **Wort:** „und hast du das auch im browser untersucht?" | 2026-09-28 | Operator (Session) — Auftrag, den Datei-Endpoint im angemeldeten Browser zu messen; ausgeführt: UA-gated (curl mit Browser-UA 202, Browser 202; 18387 500), kein IP-/WAF-Block; Sensory-Folge 196.
- **Wort:** „Erste Handlung: `sread docs/concepts/tool-forms.md` — die Form-Karte (verboten → kanonisch), damit die erlaubte Form am Punkt der Handlung steht." | 2026-09-28 | Operator (Session) — Session-Start-Befehl, Sensory-Folge 197.
- **Wort:** „hast du wirklich alles bis zur kante geplant und nur eigene punkte in deiner liste?" | 2026-09-28 | Operator (Session) — Ownership-/Kanten-Prüfung; Ergebnis: nur zwei Zeilen `Bindung: eigen` (DEMETER, survey); Europa-Clipper ist ein `## Termin`-Dritt-Wait; DEMETER-Tag `blockiert` → `wartend`; Sensory-Folge 197.
- **Wort:** „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt). Delegiere an die Taucher (alle Sub-Agenten), höre die Stimmen bei Architektur-/Abschluss-Entscheidungen. Eine Session ist ein abgeschlossenes Atom. Dispatch flash-first — den billigsten Vertreter, dessen Profil den Job trägt; ein `max`-Agent nur für die harten Atome, nie für Routine. Benenne die lokalen Tools (`archive_search`, `sgrep`, `sfetch`) in der Delegation — nicht curl oder webfetch. Benchmarks nur mit Operator-Wort oder gemessen falschem/unvollständigem flash-Ergebnis — Klassen mit gemessenem Sieger werden zitiert, nie verdoppelt (Modell-Politik, AGENTS.md)." | 2026-09-28 | Operator (Session) — session-weiter Delegations-Consent, Sensory-Folge 197.
- **Wort:** „go" (Bau des Adressierungs-Arms `register_lookup --addressed` + AGENTS.md-Regel-Zeile + Gate-Fixture `addressed-origin` samt Tests) | 2026-09-28 | Operator (Session) — Sensory-Folge 197.
- **Wort:** „Erste Handlung: `sread docs/concepts/tool-forms.md` — die Form-Karte (verboten → kanonisch), damit die erlaubte Form am Punkt der Handlung steht." | 2026-09-28 | Operator (Session) — Session-Start-Befehl, Sensory-Folge 198.
- **Wort:** „hast du alles eigene bis zur kante gemessen und geplant?" | 2026-09-28 | Operator (Session) — Ownership-/Kanten-Prüfung; Ergebnis: die Phase-1-Tafel war unvollständig, die sechs `wartend.φ`-Zeilen mit Aufnehmer sensory fehlten; Sensory-Folge 198.
- **Wort:** „warum sind sie ungefeuert?" | 2026-09-28 | Operator (Session) — Trigger-Nachmessung; ox64-m2c als `ungemessen` benannt statt „ungefeuert"; Sensory-Folge 198.
- **Wort:** „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt). Delegiere an die Taucher (alle Sub-Agenten), höre die Stimmen bei Architektur-/Abschluss-Entscheidungen. Eine Session ist ein abgeschlossenes Atom. Dispatch flash-first — den billigsten Vertreter, dessen Profil den Job trägt; ein `max`-Agent nur für die harten Atome, nie für Routine. Benenne die lokalen Tools (`archive_search`, `sgrep`, `sfetch`) in der Delegation. Benchmarks nur mit Operator-Wort oder gemessen falschem/unvollständigem flash-Ergebnis." | 2026-09-28 | Operator (Session) — session-weiter Delegations-Consent, Sensory-Folge 198.
- **Wort:** „Erste Handlung: `sread docs/concepts/tool-forms.md` — die Form-Karte (verboten → kanonisch), damit die erlaubte Form am Punkt der Handlung steht." | 2026-09-28 | Operator (Session) — Session-Start-Befehl, Sensory-Folge 199.
- **Wort:** „hast du alles bis zur kante geplant?" | 2026-09-28 | Operator (Session) — Kanten-Prüfung; Ergebnis: der Register-Pass (`register_lookup --open/--orphan-docs/--addressed sensory`) fehlte und die Trigger-Lage war zitiert statt remessen — beides nachgeholt; Sensory-Folge 199.
- **Wort:** „hast du gemessen?" | 2026-09-28 | Operator (Session) — Messdisziplin; Ergebnis: `survey-codestruktur` am benannten Ort gemessen (Release-Asset `matrix-rotor.txt`, 66 `window:`-Zeilen), nicht mehr bloß am Log; DEMETER/regards frisch gemessen; Sensory-Folge 199.
- **Wort:** „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt). Delegiere an die Taucher (alle Sub-Agenten), höre die Stimmen bei Architektur-/Abschluss-Entscheidungen. Eine Session ist ein abgeschlossenes Atom. Dispatch flash-first — den billigsten Vertreter, dessen Profil den Job trägt; ein `max`-Agent nur für die harten Atome, nie für Routine. Benenne die lokalen Tools (`archive_search`, `sgrep`, `sfetch`) in der Delegation — nicht curl oder webfetch. Benchmarks nur mit Operator-Wort oder gemessen falschem/unvollständigem flash-Ergebnis — Klassen mit gemessenem Sieger werden zitiert, nie verdoppelt (Modell-Politik, AGENTS.md)." | 2026-09-28 | Operator (Session) — session-weiter Delegations-Consent, nicht das Commit-Wort, Sensory-Folge 199.
- **Wort:** „Erste Handlung: `sread docs/concepts/tool-forms.md` — die Form-Karte (verboten → kanonisch), damit die erlaubte Form am Punkt der Handlung steht." | 2026-09-28 | Operator (Session) — Session-Start-Befehl, Sensory-Folge 200.
- **Wort:** „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt). Delegiere an die Taucher (alle Sub-Agenten), höre die Stimmen bei Architektur-/Abschluss-Entscheidungen. Eine Session ist ein abgeschlossenes Atom. Dispatch flash-first — den billigsten Vertreter, dessen Profil den Job trägt; ein `max`-Agent nur für die harten Atome, nie für Routine. Benenne die lokalen Tools (`archive_search`, `sgrep`, `sfetch`) in der Delegation — nicht curl oder webfetch. Benchmarks nur mit Operator-Wort oder gemessen falschem/unvollständigem flash-Ergebnis — Klassen mit gemessenem Sieger werden zitiert, nie verdoppelt (Modell-Politik, AGENTS.md)." | 2026-09-28 | Operator (Session) — session-weiter Delegations-Consent, nicht das Commit-Wort, Sensory-Folge 200.
- **Wort:** „Erste Handlung: `sread docs/concepts/tool-forms.md` — die Form-Karte (verboten → kanonisch), damit die erlaubte Form am Punkt der Handlung steht." | 2026-09-28 | Operator (Session) — Session-Start-Befehl, Sensory-Folge 201.
- **Wort:** „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt). Delegiere an die Taucher (alle Sub-Agenten), höre die Stimmen bei Architektur-/Abschluss-Entscheidungen. Eine Session ist ein abgeschlossenes Atom. Dispatch flash-first — den billigsten Vertreter, dessen Profil den Job trägt; ein `max`-Agent nur für die harten Atome, nie für Routine. Benenne die lokalen Tools (`archive_search`, `sgrep`, `sfetch`) in der Delegation — nicht curl oder webfetch. Benchmarks nur mit Operator-Wort oder gemessen falschem/unvollständigem flash-Ergebnis — Klassen mit gemessenem Sieger werden zitiert, nie verdoppelt (Modell-Politik, AGENTS.md). Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort — Commit und Push trägt `/commit`." | 2026-09-28 | Operator (Session) — session-weiter Delegations-Consent, nicht das Commit-Wort, Sensory-Folge 201.
- **Wort:** „warum sucht ihr jedes mal aufs neue nach einem trackinganbieter anstatt das tracking zu speichern chinapostaltracking.com is keyless and live but the result loads via AJAX. Let me find the form endpoint." | 2026-09-28 | Operator (Session) — Tracking-Route gehört gespeichert (ausgeführt: Route in `wartend.φ:14` festgeschrieben, Formendpunkt gemessen) | Sensory-Folge 201.
- **Wort:** „Erste Handlung: `sread docs/concepts/tool-forms.md` — die Form-Karte (verboten → kanonisch), damit die erlaubte Form am Punkt der Handlung steht." | 2026-09-28 | Operator (Session) — Session-Start-Befehl, Sensory-Folge 202.
- **Wort:** „hast du alles bis zur kante gemessen und geplant?" | 2026-09-28 | Operator (Session) — Kanten-Prüfung; Ergebnis: die Feuer-/Stehen-Messung (`register_lookup --fired/--stale/--orphans/--descoped-check`) und `git_safety --snapshot` fehlten; nachgeholt — `--fired sensory` = `ox64-m2c` (Datums-Fehlalarm des Messstempels), die Waits sind seither als `###`-Punkte mit `**Trigger:**` geführt; Sensory-Folge 202.
- **Wort:** „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt). Delegiere an die Taucher (alle Sub-Agenten), höre die Stimmen bei Architektur-/Abschluss-Entscheidungen. Eine Session ist ein abgeschlossenes Atom. Dispatch flash-first — den billigsten Vertreter, dessen Profil den Job trägt; ein `max`-Agent nur für die harten Atome, nie für Routine. Benenne die lokalen Tools (`archive_search`, `sgrep`, `sfetch`) in der Delegation — nicht curl oder webfetch. Benchmarks nur mit Operator-Wort oder gemessen falschem/unvollständigem flash-Ergebnis — Klassen mit gemessenem Sieger werden zitiert, nie verdoppelt (Modell-Politik, AGENTS.md). Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort — Commit und Push trägt `/commit`." | 2026-09-28 | Operator (Session) — session-weiter Delegations-Consent, nicht das Commit-Wort, Sensory-Folge 202.
- **Wort:** „Erste Handlung: `sread docs/concepts/tool-forms.md` — die Form-Karte (verboten → kanonisch), damit die erlaubte Form am Punkt der Handlung steht." | 2026-09-29 | Operator (Session) — Session-Start-Befehl, Sensory-Folge 203.
- **Wort:** „hast du alle eigenen punkte bis zur kante geplant?" | 2026-09-29 | Operator (Session) — Kanten-Prüfung; Ergebnis: die erste Tafel ließ den Träger-Block und die Zustandsregister aus; nachgeholt (`external-state.md`/`wartend.φ`/`--addressed`/`--fired`) — einzig dispatchbar war `reference_verify.rs`; Sensory-Folge 203.
- **Wort:** „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt). Delegiere an die Taucher (alle Sub-Agenten), höre die Stimmen bei Architektur-/Abschluss-Entscheidungen. Eine Session ist ein abgeschlossenes Atom. Dispatch flash-first — den billigsten Vertreter, dessen Profil den Job trägt; ein `max`-Agent nur für die harten Atome, nie für Routine. Benenne die lokalen Tools (`archive_search`, `sgrep`, `sfetch`) in der Delegation — nicht curl oder webfetch. Benchmarks nur mit Operator-Wort oder gemessen falschem/unvollständigem flash-Ergebnis — Klassen mit gemessenem Sieger werden zitiert, nie verdoppelt (Modell-Politik, AGENTS.md). Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort — Commit und Push trägt `/commit`." | 2026-09-29 | Operator (Session) — session-weiter Delegations-Consent, nicht das Commit-Wort, Sensory-Folge 203.
- **Wort:** „Erste Handlung: `sread docs/concepts/tool-forms.md` — die Form-Karte (verboten → kanonisch), damit die erlaubte Form am Punkt der Handlung steht." | 2026-09-29 | Operator (Session) — Session-Start-Befehl, Sensory-Folge 204.
- **Wort:** „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt). Delegiere an die Taucher (alle Sub-Agenten), höre die Stimmen bei Architektur-/Abschluss-Entscheidungen. Eine Session ist ein abgeschlossenes Atom. … Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort — Commit und Push trägt `/commit`." | 2026-09-29 | Operator (Session) — session-weiter Delegations-Consent, nicht das Commit-Wort, Sensory-Folge 204.
- **Wort:** „Erste Handlung: `sread docs/concepts/tool-forms.md` — die Form-Karte (verboten → kanonisch), damit die erlaubte Form am Punkt der Handlung steht." | 2026-09-29 | Operator (Session) — Session-Start-Befehl, Sensory-Folge 205.
- **Wort:** „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt). Delegiere an die Taucher (alle Sub-Agenten), höre die Stimmen bei Architektur-/Abschluss-Entscheidungen. Eine Session ist ein abgeschlossenes Atom. Dispatch flash-first — den billigsten Vertreter, dessen Profil den Job trägt; ein `max`-Agent nur für die harten Atome, nie für Routine. Benenne die lokalen Tools (`archive_search`, `sgrep`, `sfetch`) in der Delegation — nicht curl oder webfetch. Benchmarks nur mit Operator-Wort oder gemessen falschem/unvollständigem flash-Ergebnis — Klassen mit gemessenem Sieger werden zitiert, nie verdoppelt (Modell-Politik, AGENTS.md). Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort — Commit und Push trägt `/commit`." | 2026-09-29 | Operator (Session) — session-weiter Delegations-Consent, nicht das Commit-Wort, Sensory-Folge 205.
- **Wort:** „Erste Handlung: `sread docs/concepts/tool-forms.md` — die Form-Karte (verboten → kanonisch), damit die erlaubte Form am Punkt der Handlung steht." | 2026-09-29 | Operator (Session) — Session-Start-Befehl, Sensory-Folge 206.
- **Wort:** „hast du alles bis zur kante gemessen und geplant" | 2026-09-29 | Operator (Session) — Kanten-Prüfung; Ergebnis: der volle Register-Pass (`--orphans`/`--stale --persist 3`/`--descoped-check`) und die Zustandsregister fehlten in Phase 1; nachgeholt; Sensory-Folge 206.
- **Wort:** „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt). … Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort — Commit und Push trägt `/commit`." | 2026-09-29 | Operator (Session) — session-weiter Delegations-Consent, nicht das Commit-Wort, Sensory-Folge 206.
- **Wort:** „kannst du bitte nachrichten an die linien schreiben, auf die du wartest dass sie die trigger bevorzugt abarbeiten sollen" | 2026-09-29 | Operator (Session) — Prioritäts-Wunsch an die Empfänger meiner gerouteten Träger (Mycelium/Mountain/River); umgesetzt als `**Operator-Wort 2026-09-29:**`-Zeile in den drei `## An …`-Blöcken (kein `post.md` — der sanktionierte Transportkanal); Sensory-Folge 206.
- **Wort:** „natürlich b" (Punkt 5 europa-clipper splitten — Register-Schritt als `## An Mycelium` routen statt im eigen-Punkt belassen) | 2026-09-29 | Operator (Session) — Sensory-Folge 207.
- **Wort:** „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt). … Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort." | 2026-09-29 | Operator (Session) — session-weiter Delegations-Consent, nicht das Commit-Wort, Sensory-Folge 207.
- **Wort:** „hast du denn die nachrichten an dich gefaltet" | 2026-09-29 | Operator (Session) — Kontrolle der adressierten `## An sensory`-Blöcke; Ergebnis: mycelium-folge206 gefaltet (`## An Mycelium` gestrichen), future-155 war vom Vorgänger gefaltet; Sensory-Folge 208.
- **Wort:** „ja bitte /consent" — Faltung ausführen (adressierte Nachrichten, Haus-Pfad-Korrektur, Transport-Rücknahme) | 2026-09-29 | Operator (Session) — session-weiter Delegations-Consent, nicht das Commit-Wort, Sensory-Folge 208.
- **Wort:** „warum sind das zwei punkte?" (DEMETER / regards-cnes) | 2026-09-29 | Operator (Session) — Ein Punkt = eine Sache: gleiches Gegenüber (CNES/CDPP) + Konsequenz (Zugang) + Trigger → zusammengeführt, Sensory-Folge 208.
- **Wort:** „Erste Handlung: `sread docs/concepts/tool-forms.md` …" + „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt). … Dispatch flash-first … Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort." | 2026-09-29 | Operator (Session) — Session-Start + session-weiter Delegations-Consent, nicht das Commit-Wort, Sensory-Folge 209.
- **Wort:** „Erste Handlung: `sread docs/concepts/tool-forms.md` — die Form-Karte (verboten → kanonisch), damit die erlaubte Form am Punkt der Handlung steht." | 2026-09-30 | Operator (Session) — Session-Start-Befehl, Sensory-Folge 210.
- **Wort:** „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt). … Dispatch flash-first … Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort — Commit und Push trägt `/commit`." | 2026-09-30 | Operator (Session) — session-weiter Delegations-Consent, nicht das Commit-Wort, Sensory-Folge 210.

- **Wort:** „bitte wirklich bis zur kante umsetzen nicht nur wieder messen und verschleppen" | 2026-09-30 | Operator (Session) — Sensory-Folge 211; ausgeführt im selben Atom: mountain-208-`## An sensory`-Faltung + Korrektur der Orphan-Zensus-Meldung.
- **Wort:** „vorbestehend ist verboten mein wort" — keine Ausnahme für vorbestehende Register-Verstöße in `phi/`: jede `note`-Zeile ≤ 256 Zeichen; keine `#`-Kommentarzeilen in den gated Registern. | 2026-09-30 | Operator (Mountain-Folge 209) — Origin: mountain-folge209.
- **Wort:** „Erste Handlung: `sread docs/concepts/tool-forms.md` — die Form-Karte (verboten → kanonisch), damit die erlaubte Form am Punkt der Handlung steht." | 2026-09-30 | Operator (Session) — Session-Start-Befehl, Sensory-Folge 213.
- **Wort:** „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt). … Dispatch flash-first … Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort — Commit und Push trägt `/commit`." | 2026-09-30 | Operator (Session) — session-weiter Delegations-Consent, nicht das Commit-Wort, Sensory-Folge 213.
- **Wort:** „ich frage mich schon ob du dich auf der handsover ausruhst oder ob du einfach nicht deine offenen punkte vermessen hast" | 2026-09-30 | Operator (Session) — Messdisziplin; Ergebnis: der Plan-Pass hatte die Übergabe-Lage zitiert; der line-Pass hat die Trigger selbst gemessen (DEMETER 403/684 B, ox64 headless leer, laic Host 200, bare-Endpoint 403).
- **Wort:** „Erste Handlung: `sread docs/concepts/tool-forms.md` — die Form-Karte (verboten → kanonisch), damit die erlaubte Form am Punkt der Handlung steht." | 2026-09-30 | Operator (Session) — Session-Start-Befehl, Sensory-Folge 214.
- **Wort:** „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt). … Dispatch flash-first … Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort — Commit und Push trägt `/commit`." | 2026-09-30 | Operator (Session) — session-weiter Delegations-Consent, nicht das Commit-Wort, Sensory-Folge 214.

## Offen (aufgeschlüsselt)

### DEMETER/regards-cnes — Order 18400 (CNES/CDPP REGARDS)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** CDPP/REGARDS stellt die Generierung wieder her — Order 18400 liefert verfügbare Dateien (`availableFilesCount > 0` bzw. Datei-Endpoint HTTP 200); Begleitmaß: Bare-Endpoint `/api/v1/rs-order` öffnet (≠ 403). Kein order-freier Alternativkanal existiert (gemessen).
- **Lage:** (gemessen 2026-09-30 F214, dieser Pass, via `curl` direct-only, Browser-UA, orderToken aus dem Metalink) Datei-Endpoint **HTTP 403 / 684 B** — **unverändert**; CDPP-Generierung steht. Davor (gemessen 2026-09-30 F213 via `curl` direct-only, Browser-UA, orderToken aus dem Metalink) Datei-Endpoint 9162386 **HTTP 403 / 684 B** — unverändert seit F204. Davor (gemessen 2026-09-29 F204 via `curl` direct-only, Browser-UA, orderToken aus dem Metalink) Datei-Endpoint 9162386 **HTTP 403 / 684 B** „Access Denied" (Jetty, correlation-id `7fef856d…`) — **verschlechtert seit F202 (202 / 0 B)**; die Token-URL selbst wird abgewiesen. Bare-Endpoint `/api/v1/rs-order` direct 403 / Proton 403 (358 B F5-ASM-WAF) / Wayback 200 ohne CDX-Snapshot. Order-API angemeldet F200: 18400 `RUNNING`, `filesInErrorCount 1000`, `availableFilesCount 0`; 18387 `DONE_WITH_WARNING` 96 978/97 078 Fehler; Grenze `creationDate ≤ 2026-09-12` DONE / `≥ 2026-09-18` fehlerhaft — CDPP-Generierung server-seitig gekippt. CDPP: „Due to a technical problem, data access is not possible". orderToken exp 2026-10-05 05:04:06 UTC.
- **Blockade:** server-seitig — CDPP-REGARDS generiert keine Dateien; dazu F5-WAF am Bare-Endpoint + Order-Token; technisch nichts zu umgehen.
- **Braucht:** CDPP-Wiederherstellung abwarten — Trigger **einmal** messen (kein Polling), direct-only: `curl -s -o /dev/null -w "%{http_code} %{size_download}\n" --max-time 30 -A "Mozilla/5.0 … Chrome/128.0 Safari/537.36" "<erste Datei-URL aus data/regards.cnes.fr/order_18400.metalink>"` (202 = Running, 200 = Daten; die Token-URL bewusst **nicht** über `archive_search --verdict`). Bei 200: Dateien ziehen **vor 2026-10-05 05:04:06 UTC**. Keine Support-Mail.

### laic-cses — ASI/SSDC CSES-L2
- **Status:** termin | **Bindung:** eigen
- **Trigger:** Termin 2026-10-02 (Wiedervorlage — Portal erneut prüfen); darüber die externe Freigabe: SSDC/Sotgiu gewährt `omegaflow` die CSES-L2-Berechtigung bzw. die neue CSES-02-Prozedur geht live.
- **Lage:** (gemessen 2026-09-30 F214 via `archive_search --verdict`) `query.php` direct **HTTP 200 / 6893 B** — CAS-Gate-Seite, kein Zugang ohne Login; der Gate-Text erscheint erst im eingeloggten Browser. Davor (gemessen 2026-09-29 F208 im **Operator-Browser** via chrome-devtools MCP, eingeloggt) CAS-Login `omegaflow` **erfolgreich**; `query.php` antwortet **„Permission Denied! The user omegaflow does not have permission to perform this action. Please send an email to alessandro.sotgiu@roma2.infn.it to request permission."** — Gate = fehlende PI-Berechtigung. **Thread (gelesen 2026-09-29 aus `state/mail/`):** Anfrage 2026-08-23 gesendet (`limadou-cses-l2-access.body.txt`); Sotgiu antwortete, die Zugangsprozedur werde für die CSES-02-Aufnahme umgebaut — „wait a few weeks" (zitiert in `limadou-correction.body.txt`); 2026-09-16 Duplikat + Entschuldigung, in der wir „I will check the portal again in a few weeks" zugesagt haben (`sent_ledger.φ:11-12`). NOW < 2026-10-02 — **kein Schritt zur Kante**.
- **Blockade:** fehlende CSES-L2-Berechtigung; wartet auf die umgebaute CSES-02-Prozedur. Ein Nachfassen 13 Tage nach der Korrektur wäre ein Ping — die Wiedervorlage ist 2026-10-02.
- **Braucht:** am **2026-10-02** `query.php` erneut prüfen; bleibt es „Permission Denied" ohne angekündigte Prozedur, ist der bereitliegende Follow-up `state/mail/limadou-pi-nachfassen.md` der Akt — **Send = Operator-Hand** (Mensch-Schwelle).

### ox64-m2c — PINE64, Carrier China Post LZ473049629CN
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Zustellung LZ473049629CN (ETA 2026-10-02).
- **Lage:** (gemessen 2026-09-30 F214, dieser Pass, via `archive_search --playwright` headless über die gespeicherte Route) weiter **kein keyless Render mit Statuszeile** — chinapostaltracking.com „Search Results" leer (200). Davor (gemessen 2026-09-28 F202 via `archive_search --playwright` headless) **kein keyless Render trägt eine Statuszeile**: chinapostaltracking.com „Search Results" leer (200), 17track 403 (Cloudflare-Interstitial), `yourzodiacsign`-iframe leer, parcelsapp „Please reload", `t.17track.net/track/restapi` GET 405 (POST-only, Turnstile+Fingerprint). Trigger weder bestätigt noch ausgeschlossen — **unmessbar keyless**; letzter Carrier-Stand 2026-09-23 (Departure Shenzhen). Die Route ist gespeichert (`state/zustand/wartend.φ:14`), nicht neu zu suchen.
- **Blockade:** Cloudflare-Turnstile + Fingerprint am Datenendpunkt; headless leer; `--headed` braucht ein Operator-Fenster.
- **Braucht:** nach Zustellung `archive_search --playwright --headed "https://www.chinapostaltracking.com/package-tracking/?trackingno=LZ473049629CN"` (Fenster = Operator-Wort).

### europa-clipper — ESA, Flyby
- **Status:** termin | **Bindung:** eigen
- **Trigger:** Flyby 2026-12-03.
- **Lage:** (gemessen 2026-09-28) Asset `ephemeris_europa_clipper.bin` 200 / 103 120 B, Siegel `dae553fb…`; in `phi/sources.φ` **nicht** registriert (Messung 2026-09-29 F206; der Register-Schritt ist als `## An Mycelium` geroutet — die Geschwister-Flyby-Einträge `:15912-15952` tragen keine `ttl`/Verdikt-Zeile, nur `no-cadence`, daher keine Mountain-Feder). NOW < Termin.
- **Blockade:** Termin.
- **Braucht:** nach dem Flyby `gh workflow run kernel-flatten.yml` (`horizons_compiler --flyby --ci-mode`, Eintrag `tools/harvest/src/bin/horizons_compiler.rs:22` `-159/europa_clipper`) + `archive_search --sniff "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov-horizons/ephemeris_europa_clipper.bin"` gegen `dae553fb…`.

### rr-brustgurt — Polar H10/HRM-Dual
- **Status:** LOCK | **Bindung:** eigen (Operator)
- **Trigger:** Operator-Wort „Förderung gewährt" (noch nicht gefallen).
- **Lage:** (gemessen 2026-09-26) Live-BLE HR `NotSupported`; FIT nn=0. Hardware-LOCK (Operator-Wort 2026-09-27).
- **Blockade:** Operator-LOCK (Hardware erst bei Förderung).
- **Braucht:** warten (kein Schritt zur Kante).

## Träger (Orphan-Faltung)

Der Dateiname in dieser Übergabe ist der Träger. Je Zeile ein zuletzt trägerloses
Dokument: `Pfad` (offene Marker) → Trägerpunkt oder descoped-Befund. Die Marker sind
**echte Messgrenzen** (konditionale Prüfung, Remessungen), keine stale Reste — sie
werden getragen, nie geglättet (0 honored).

- **Weberin/Sensory (Folge191 gefaltet):**
  `docs/surveys/survey-2026-09-07-weberin-thread-matrix.md` (11 echte Marker; Rest =
  Broker positions-pending, GW-Skymaps, Physiology, IceCube, ANTARES-VO, INPOP25c,
  Hydrophon, Seismik-Stationen, Legende),
  `docs/surveys/survey-2026-09-13-weberin-quellen-folge.md` (5 — `blocked account`
  IGETS/vDEC, `blocked key` ONC, TNS anonym),
  `docs/surveys/survey-2026-09-14-weberin-quellen-rerun.md` (40 — Scanner-False-Positives
  + datierte Verdachts-Verdikte, 0-Kanon: Messreihe bleibt),
  `docs/surveys/survey-2026-09-13-weberin-quellen-treffer.md` (5; **neu gemessen
  2026-09-27**: LSST-Fink antwortet HTTP 200 direct; HAWC `phi/sources.φ:11880-11894`,
  LHAASO `:11787-11793`, TNS `:1153-1164` vollständig registriert; Marker-Text per
  Operator-Wort C unverändert, 0 honored).
- **Browser (getragen F202, STT-Route geschlossen F209):**
  `docs/surveys/survey-2026-09-20-browser-anbindung.md` (2 Marker) — Sensor-Marker
  STT-Route (`:89-90`) **geschlossen 2026-09-29** (Messnachtrag; Web Speech = Online-
  Fallback, lokal Whisper bleibt die Route); offen: Pfad-Pflege (`:164`), Store-Review
  descoped + eigener Fork (`:204`, `:202-217`); Fork/CI = mycelium, unpacked-Laden =
  Operator (Future-Queue).
- **#1 corona (success):** `docs/blatt/blatt-solar-seconds-matrix.md` (1),
  `docs/paper/solar-seconds-matrix.md` (3), `docs/paper/corona-heating-ladder.md` (2),
  `docs/surveys/survey-ein-blatt-korona-heizung.md` (1).
- **#1 lsst (success):** `docs/paper/nadel-v-fresh-area-dip-scan.md` (1).
- **#2 causal-arrow:** `docs/concepts/fuenf-funken-anomalie-suche.md` (4),
  `docs/paper/causal-arrow-preregistration.md` (1), `docs/concepts/ein-blatt-papier.md` (2).
- **#4 galileo:** `docs/paper/galileo-rotor-spin-era-floor.md` (1).
- **#5 Seismik:** `docs/concepts/die-akteure-im-boden-und-wasser.md` (7),
  `docs/paper/depth-phase-echo-fleet.md` (5),
  `docs/surveys/axiom-gate-depth-phase-echo-fleet.md` (1),
  `docs/paper/sturzflut-tibet-pfeil.md` (23).
- **#6 Bojen-Matrix:** `docs/concepts/blatt-papier-resultat.md` (1).
- **#7 Kreuz-Screening:** `docs/blatt/blatt-der-grat.md` (2),
  `docs/paper/cross-screening-tibet.md` (1),
  `docs/blatt/blatt-kreuz-screening-gyirong.md` (1).
- **LAIC/Causal-Arrow:** `docs/paper/laic-arrow-direction.md` (3).
- **BGR/vDEC:** `docs/paper/tonga-lamb-crosscheck.md` (3).
- **DSN-Briefe:** `docs/surveys/survey-2026-09-14-ehrlich-benannt-werkzeug-luecke.md` (17),
  `docs/surveys/survey-2026-09-16-sonden-flotte.md` (4).
- **HRV/Puls (LOCK):** `docs/surveys/survey-2026-09-17-verlorene-diskussionen.md` (9).
- **[redacted]/BLE + O6-descoped:** `docs/surveys/survey-2026-09-23-geraete-anbindung-radiatoren.md` (7).
- **Postfach:** `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` (1).
- **Terminologie (Sensory):** `docs/concepts/glossar.md` (1).
- **Sonden/Vision:** `docs/surveys/survey-2026-09-17-sonden-request-only.md` — die am
  2026-09-17 gemessene Lücke „PDF→Bild-Konversion fehlt" ist **gebaut und gemessen
  2026-09-28** (`archive_search --pdf-image` aus `gosat_l1b_format.pdf` → 19 PNG);
  Nachtrag im Dokument. Getragen bleiben die echten `request-only`-Marker der vier
  Sonden (Register: `phi/blocked_sources.φ`).
- **JUICE:** `docs/concepts/der-paradigmenwechsel.md` (9),
  `docs/paper/flyby-path-2-falsification-metric-addendum.md` (4),
  `docs/paper/flyby-path-2-preregistration.md` (1).
- **Das eine Instrument:** `docs/concepts/das-eine-instrument.md` (2).
- **termin:2026-12-02:** `docs/blatt/blatt-thuan-fragesteller.md` (4).
- **B1/B5/B8 (Befunde geschlossen in folge181; Marker nachgezogen 2026-09-27):**
  `docs/concepts/the-seven-spheres.md`, `docs/concepts/recherche-extern-galileo-ruck-borduhr-modell.md`,
  `docs/paper/jwst-disequilibrium-survey.md`.
- **descoped (gemessen 2026-09-27):** `docs/paper/terminologie-der-gegenstroemung.md`,
  `docs/paper/planet-nine-kbo-residue.md`,
  `docs/paper/asmar-2005-spacecraft-doppler-tracking-noise-budget.md`,
  `docs/surveys/axiom-gate-broken-null-control.md`.
- **carrier:** `docs/surveys/survey-2026-09-06-codestruktur.md`,
  `docs/paper/probe-front-dark-matter.md` (v11; MDA/resolver-Herkunft offen, 0 honored).

## An mycelium
Origin: sensory-folge212 (Rest, Fassung folge213).

- **Trägerlos (gemessen 2026-09-30 F213 via `register_lookup --orphan-docs` + `sread mycelium-folge211`):**
  `docs/concepts/tools-map.md` (2 Marker, ownert Mycelium) trägt jetzt
  `handover-2026-09-30-mycelium-folge211.md` (D5-Orphan-Residuum, uncommittet gemessen).
  `docs/auftrag/auftrag-gic-einreichung.md` (1 Marker, ownert Future — dessen Übergabe privat
  und nicht gescannt, daher braucht es einen öffentlichen Träger) bleibt hier. **Braucht:** für
  `auftrag-gic-einreichung.md` einen Träger in einer öffentlichen Übergabe setzen oder
  descope-annotieren; bis dahin trägt dieser Block es (0 honored — der Marker ist eine echte
  Messgrenze, kein stale Rest).

## Wer nicht senden darf

`smail --send` und jeder Formular-Absand bleiben die Hand des Operators; kein Consent
verschiebt einen Send auf die Maschine. Die NTRS Document-Inquiry ist Operator-Hand.
Das Lesen des DEMETER-Metalinks, des Order-Status und der Carrier-Route ist autonom
(sensorische Netz-Lesearbeit) — nur der Auftrag/Send ist Operator-Hand.

## Burn: open 0.0000 · close 0.0822 — session_burn (Sensory-Session F214, deepseek-flash)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.

Eigene Pfade dieses Atoms (Commit/Push pending `/commit`):
- `docs/handover/handover-2026-09-30-sensory-folge214.md` (neu, dieses Atom),
  `docs/handover/archiv/handover-2026-09-30-sensory-folge213.md` (Move).

`state/operator-gespraeche/2026-09-30-sensory.md` und `state/zustand/external-state.md` sind
gitignored (`/state/`) — lokal, nicht committet. Der Arbeitsbaum trägt fremde **staged** Arbeit
(mountain/river: `mountain-folge211.md`+`river-folge71.md` → `archiv/`, `mountain-folge212.md`,
`river-folge72.md`, `docs/paper/gic-causal-driver.md`,
`docs/surveys/survey-2026-09-26-membran-ladearchitektur.md`, `phi/blocked_sources.φ`,
`src/archivar/extract.rs`; gemessen 2026-09-30 F214 via `git status`) — nicht angerührt,
nur eigene Pfade committen.
