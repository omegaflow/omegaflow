<!--
  title: Handover — Sensory-Folge 227 (2026-10-03)
  session: Sensory-Folge 227
  class: handover
  date: 2026-10-03
  sha256: 9ca58d9cc6fa5054c45ef92507873921adf9718abb3a79972cf6445795659eb0
  status: live
-->
# Handover — Sensory-Folge 227 (2026-10-03)

Dieses Register trägt nur Offenes — Erledigtes ist gelöscht, nicht erklärt; git trägt,
was gemacht wurde. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks —
committet wird pfad-begrenzt, fremde uncommittete Arbeit wird nie überschrieben;
gepusht wird, sobald der eigene Commit steht und `origin/main` Vorfahr von HEAD ist.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** von Agenten abgearbeitet. Jeder Punkt trägt **Trigger** / **Lage** (mit
Messstempel) / **Blockade** / **Braucht**. Sortierung: **umsetzbar zuerst**; der
Akteur steht pro Punkt in `Bindung`.

Dieses Register konsumiert `handover-2026-10-03-sensory-folge226.md` (nach `archiv/`);
diese Folge ist 227. Der Stehende Pass wird zitiert, nie kopiert:
`state/zustand/standing-pass.md` (Stand HEAD `179167c66` == `origin/main`, gemessen
2026-10-03T17:52Z). Eigene Messung dieses Atoms (2026-10-03 F227): HEAD `1226e9082`
== `origin/main` (`git rev-parse`); im Arbeitsbaum fremde uncommittete Hünke
(river/mountain/grind, per `git status` gemessen — nicht berührt). Eigener Register-Pass: `register_lookup --fired sensory` = 2
(`hyperscanning-te zweite kohorte` FIRED_UNGEMESSEN — Trigger gemessen ungefeuert,
`ox64-m2c` FIRED); `--stale sensory --persist 3` = 0; `--addressed sensory` = 1
(mycelium-folge226, gefaltet); `--orphan-docs` = 0; `--orphans` = 1 (mycelium, fremd);
`open_points_check` der konsumierten F226 = 0 absent / 0 stale-citations /
0 owner-drift. `git_safety --snapshot` = `refs/safety/1791059050`.

Gefeuert: **ox64-m2c** (ETA 2026-10-02 verstrichen; unverändert `wartend` — die
physische Zustellung ist das Ereignis, keine Ferndiagnose; nicht neu gemessen, da
unverändert) und **hyperscanning-te zweite Kohorte** (`--fired` FIRED_UNGEMESSEN; der
Trigger „erster Validierungslauf ist ausgewertet" ist gemessen **ungefeuert**: Lauf
`37133961687` Step `screen` `in_progress`, `ci_manage jobs`). Nicht gefeuert:
europa-clipper (Termin 2026-12-03), rr-brustgurt (Operator-gebunden, an Future
geroutet).

**Messen und bearbeiten (Operator-Wort 2026-10-01, dauerhaft):** dieses Atom hat den
adressierten `## An sensory`-Block (mycelium-folge226) gefaltet: `src/archivar/fit.rs`
scoped formatiert und committet (`0b3292bc9`) — die `ci-gate`-Format-Rote meiner Linie
ist geheilt. Den einzigen autonom baubaren eigenen Punkt bis zur Kante gearbeitet: den
`ds007471`-Dyaden-Arm gebaut (`1226e9082`) — `brainvision_compiler --participant L|R`
trennt die 64-Kanal-`_L`/`_R`-Gruppen in je ein Teilnehmer-Bin (point-major, passend zu
`channel_series`), Workflow-Input `cohort` (ds007822|ds007471). Hardware LOCK, Termine
ohne gefeuerten Trigger.

## Haus — Sensorik, Hardware & Fundstellen (Stand 2026-10-03)

Diese Übergabe **ist** das Haus: jeder Sensor-/Hardware-Stand, jede Route, jeder Punkt steht hier
mit Zustand (Operator-Wort 2026-09-29). Ein Punkt, der nur in `state/` lebt und hier fehlt, ist
ein **verlorener Punkt**. `state/` wird mit `archive_search --root state` vermessen, **nie** mit
`sgrep` ohne `--all` (Falsch-Negative). Relevanz nie über zwei Stichwörter entscheiden —
breiter messen.

- **Hardware/Sensoren (Stand):** Polar H10/HRM-Dual — Live-BLE HR `NotSupported`, FIT nn=0;
  Hardware Operator-gebunden (Operator-Wort 2026-09-27, „erst bei Förderung") — der Brustgurt
  ist an die Future-Queue geroutet. [redacted]/DEMETER — die
  Daten/Datei verlassen das Gerät nie (Operator-Wort 2026-09-25). ox64-M2C Carrier — Routen
  gespeichert (`state/zustand/wartend.φ:12`); die deutsche DHL-Seite ist im echten Browser
  lesbar (F224/F225), die chinapost/17track-Route bleibt keyless + real-Chrome leer.
- **Fundstellen:** `state/zustand/standing-pass.md` · `state/zustand/external-state.md` ·
  `state/zustand/wartend.φ` · `state/zustand/ereignisse.φ` ·
  `state/operator-gespraeche/2026-10-02-sensory.md` · `state/operator-gespraeche/2026-10-03-sensory.md` · `state/sensory/archive-search-preset.txt`.
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

## Operator-Wort-Register (Stand 2026-10-03)

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
- **Wort:** „Erste Handlung: `sread docs/concepts/tool-forms.md` — die Form-Karte (verboten → kanonisch), damit die erlaubte Form am Punkt der Handlung steht." | 2026-10-01 | Operator (Session) — Session-Start-Befehl, Sensory-Folge 215.
- **Wort:** „Starte die Sensory-Linie in einem Pass … kein Consent-Stopp für Bekanntes. … Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort — Commit und Push trägt `/commit`." | 2026-10-01 | Operator (Session) — session-weiter Delegations-Consent, nicht das Commit-Wort, Sensory-Folge 215.
- **Wort:** „Erste Handlung: `sread docs/concepts/tool-forms.md` — die Form-Karte (verboten → kanonisch), damit die erlaubte Form am Punkt der Handlung steht." | 2026-10-01 | Operator (Session) — Session-Start-Befehl, Sensory-Folge 216.
- **Wort:** „Starte die Sensory-Linie **in einem Pass** — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes. … LOCK nie vorlegen. Wartend nie vorlegen. … `/consent` ist der session-weite Consent (Delegation), `/commit` ist das Commit-Wort — beide getrennt." | 2026-10-01 | Operator (Session) — session-weiter Delegations-Consent, nicht das Commit-Wort, Sensory-Folge 216.
- **Wort:** „nicht nur messen, messen und bearbeiten ist die prämisse und mein dauerhaftes wort" | 2026-10-01 | Operator (Session) — **dauerhaftes Haus-Wort:** eine Messung führt zur Bearbeitung bis zur Kante, nie zum Register allein; Ziel: alle live Übergaben; Origin: sensory-folge216.
- **Wort:** „die future punkte haben nichts bei dir zu suchen schon gar nicht mails nach extern" | 2026-10-01 | Operator (Session) — Future-Punkte (Operator-Akte, externe Mails, Send) gehören nicht in die Sensory-Übergabe; `laic-cses` als `## An future` geroutet, die Mail-Vorbereitung (`QUELLEN` in `state/mail/limadou-pi-nachfassen.md`) zurückgenommen; Origin: sensory-folge216.
- **Wort:** „Erste Handlung: `sread docs/concepts/tool-forms.md` — die Form-Karte (verboten → kanonisch), damit die erlaubte Form am Punkt der Handlung steht." | 2026-10-01 | Operator (Session) — Session-Start-Befehl, Sensory-Folge 217.
- **Wort:** „Starte die Sensory-Linie **in einem Pass** — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes. … LOCK nie vorlegen. Wartend nie vorlegen. … `/consent` ist der session-weite Consent (Delegation), `/commit` ist das Commit-Wort — beide getrennt." | 2026-10-01 | Operator (Session) — session-weiter Delegations-Consent, nicht das Commit-Wort, Sensory-Folge 217.
- **Wort:** „Erste Handlung: `sread docs/concepts/tool-forms.md` — die Form-Karte (verboten → kanonisch), damit die erlaubte Form am Punkt der Handlung steht." | 2026-10-01 | Operator (Session) — Session-Start-Befehl, Sensory-Folge 218.
- **Wort:** „Starte die Sensory-Linie **in einem Pass** — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes. … LOCK nie vorlegen. Wartend nie vorlegen. … `/consent` ist der session-weite Consent (Delegation), `/commit` ist das Commit-Wort — beide getrennt." | 2026-10-01 | Operator (Session) — session-weiter Delegations-Consent, nicht das Commit-Wort, Sensory-Folge 218.
- **Wort:** „Erste Handlung: `sread docs/concepts/tool-forms.md` — die Form-Karte (verboten → kanonisch), damit die erlaubte Form am Punkt der Handlung steht." | 2026-10-02 | Operator (Session) — Session-Start-Befehl, Sensory-Folge 219.
- **Wort:** „Starte die Sensory-Linie **in einem Pass** — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes. … LOCK nie vorlegen. Wartend nie vorlegen. … `/consent` ist der session-weite Consent (Delegation), `/commit` ist das Commit-Wort — beide getrennt." | 2026-10-02 | Operator (Session) — session-weiter Delegations-Consent, nicht das Commit-Wort, Sensory-Folge 219.
- **Wort:** „Erste Handlung: `sread docs/concepts/tool-forms.md` — die Form-Karte (verboten → kanonisch), damit die erlaubte Form am Punkt der Handlung steht." | 2026-10-02 | Operator (Session) — Session-Start-Befehl, Sensory-Folge 220.
- **Wort:** „Starte die Sensory-Linie **in einem Pass** — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes. Ist ein Name genannt, nimm diesen; sonst die neueste offene Sensory-Übergabe. … LOCK nie vorlegen. Wartend nie vorlegen. Der Send bleibt die Operator-Hand. `/consent` ist der session-weite Consent (Delegation), `/commit` ist das Commit-Wort — beide getrennt." | 2026-10-02 | Operator (Session) — session-weiter Delegations-Consent, nicht das Commit-Wort, Sensory-Folge 220.
- **Wort:** „Erste Handlung: `sread docs/concepts/tool-forms.md` — die Form-Karte (verboten → kanonisch), damit die erlaubte Form am Punkt der Handlung steht." | 2026-10-02 | Operator (Session) — Session-Start-Befehl, Sensory-Folge 221.
- **Wort:** „Starte die Sensory-Linie **in einem Pass** — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes. Ist ein Name genannt, nimm diesen; sonst die neueste offene Sensory-Übergabe. … LOCK nie vorlegen. Wartend nie vorlegen. Der Send bleibt die Operator-Hand. `/consent` ist der session-weite Consent (Delegation), `/commit` ist das Commit-Wort — beide getrennt." | 2026-10-02 | Operator (Session) — session-weiter Delegations-Consent, nicht das Commit-Wort, Sensory-Folge 221.
- **Wort:** „das was du machst ist völliger quatsch sschau dir an welche stimmen future fährt" | 2026-10-02 | Operator (Session) — Korrektur: die freien Stimmen fährt die Future-Flotte (`opencode run --agent voice`), nicht ein Sensory-/Mycelium-Swarm; Origin: Sensory-Folge 221.
- **Wort:** „also kannst du bitte nochmal unsere kostenlosen stimmen mit harten bandagen und archive search und die chat uis befragen ob es wirklich keine DEMETER alternative gibt und warum führst du das eigentlich sollte das nicht beser bei mycelium geführt werden?" | 2026-10-02 | Operator (Session) — Auftrag DEMETER-Alternativen + Ownership-Korrektur: DEMETER gehört nicht in die Sensory-Übergabe (Aufnehmer mycelium); Origin: Sensory-Folge 221.
- **Wort:** „Erste Handlung: `sread docs/concepts/tool-forms.md` — die Form-Karte (verboten → kanonisch), damit die erlaubte Form am Punkt der Handlung steht." | 2026-10-02 | Operator (Session) — Session-Start-Befehl, Sensory-Folge 222.
- **Wort:** „Starte die Sensory-Linie **in einem Pass** — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes. Ist ein Name genannt, nimm diesen; sonst die neueste offene Sensory-Übergabe. … LOCK nie vorlegen. Wartend nie vorlegen. Der Send bleibt die Operator-Hand (nie `smail --send`); jeder dritt-wirksame Akt ist per-Akt-Operator-Wort. `/consent` ist der session-weite Consent (Delegation), `/commit` ist das Commit-Wort — beide getrennt." | 2026-10-02 | Operator (Session) — session-weiter Delegations-Consent, nicht das Commit-Wort, Sensory-Folge 222.
- **Wort:** „Erste Handlung: `sread docs/concepts/tool-forms.md` — die Form-Karte (verboten → kanonisch), damit die erlaubte Form am Punkt der Handlung steht." | 2026-10-02 | Operator (Session) — Session-Start-Befehl, Sensory-Folge 223.
- **Wort:** „Starte die Sensory-Linie **in einem Pass** — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes. Ist ein Name genannt, nimm diesen; sonst die neueste offene Sensory-Übergabe. … LOCK nie vorlegen. Wartend nie vorlegen. Der Send bleibt die Operator-Hand (nie `smail --send`); jeder dritt-wirksame Akt ist per-Akt-Operator-Wort. `/consent` ist der session-weite Consent (Delegation), `/commit` ist das Commit-Wort — beide getrennt. **Die Übergabe IST der Stand.**" | 2026-10-02 | Operator (Session) — session-weiter Delegations-Consent, nicht das Commit-Wort, Sensory-Folge 223.
- **Wort:** „ich musste leider neustarten" | 2026-10-02 | Operator (Session) — Session-Neustart mitten im F223-Pass; kein eigener uncommitteter Standverlust (gemessen `git status`: nur fremd `src/archivar/fit.rs`).
- **Wort:** „Erste Handlung: `sread docs/concepts/tool-forms.md` — die Form-Karte (verboten → kanonisch), damit die erlaubte Form am Punkt der Handlung steht." | 2026-10-03 | Operator (Session) — Session-Start-Befehl, Sensory-Folge 224.
- **Wort:** „Starte die Sensory-Linie **in einem Pass** — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes. … LOCK nie vorlegen. Wartend nie vorlegen. Der Send bleibt die Operator-Hand (nie `smail --send`); jeder dritt-wirksame Akt ist per-Akt-Operator-Wort. `/consent` ist der session-weite Consent (Delegation), `/commit` ist das Commit-Wort — beide getrennt." | 2026-10-03 | Operator (Session) — session-weiter Delegations-Consent, nicht das Commit-Wort, Sensory-Folge 224.
- **Wort/Messung:** DHL-Sendungsverfolgung `…?piececode=LZ473049629CN` geliefert — „Freigabe der Sendung im Ursprungsland", Zielland Deutschland, letztes Event „Mi, 23.09.2026, 02:40, China VR" | 2026-10-03 | Operator (Session) — der DHL-Weg ist im echten Browser lesbar; headless `archive_search --playwright` rendert den JS-Status nicht.
- **Wort:** „Ehrliche Korrektur … Rivers eigene Papiere fälschlich als ‚fremd' bezeichnet … `## An sensory` in `river-folge84` gesetzt … Offen und nicht verschoben: Rivers eigene 2.9-Sprachheilung und die Antworten von Sensory/Mountain." | 2026-10-03 | Operator (Session, River-Folge 84) — Ownership-Korrektur + adressierter Block; in diesem Atom gefaltet und geheilt (2.8/2.9/2.10).
- **Wort:** „Erste Handlung: `sread docs/concepts/tool-forms.md` — die Form-Karte (verboten → kanonisch), damit die erlaubte Form am Punkt der Handlung steht." | 2026-10-03 | Operator (Session) — Session-Start-Befehl, Sensory-Folge 225.
- **Wort:** „Starte die Sensory-Linie **in einem Pass** — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes. … LOCK nie vorlegen. Wartend nie vorlegen. Der Send bleibt die Operator-Hand (nie `smail --send`); jeder dritt-wirksame Akt ist per-Akt-Operator-Wort. `/consent` ist der session-weite Consent (Delegation), `/commit` ist das Commit-Wort — beide getrennt." | 2026-10-03 | Operator (Session) — session-weiter Delegations-Consent, nicht das Commit-Wort, Sensory-Folge 225.
- **Wort:** „warum tragen mycellum und du eigentlich den brustgurt und kannst du bitte mal auf ebay kleinanzeigen und schauen welche guten und günstigen angebote es gibt und braucht es auch einen antstick?" | 2026-10-03 | Operator (Session) — Ownership-Frage: Dublette gemessen (sensory `state/zustand/wartend.φ:22` kanonisch; `mycelium-folge225:217` Dublette → `## An Mycelium`); Angebotslage Kleinanzeigen gemessen (45–70 € gebraucht, 81 € neu); ANT+-Stick nicht nötig (BLE-Leser `src/archivar/ble.rs`); Sensory-Folge 225.
- **Wort:** „warum sind die so teuer? und warum polar und nicht garmin?" | 2026-10-03 | Operator (Session) — Markt-/Marken-Frage: H10 = Referenz (neu ~90–100 €, hoher Restwert); für BLE-`0x2A37`-RR gleichwertig Garmin HRM-Dual/Pro/HRM-Fit; „Polar" war nur das Such-Stichwort, keine Absage an Garmin; Sensory-Folge 225.
- **Wort:** „und neu?" | 2026-10-03 | Operator (Session) — Neupreis-Messung (2026-10-03, Geizhals): Polar H10 ab 75,63 €; Garmin HRM-200 (HRM-Dual-Nachfolger) ab 63,50 €; Garmin HRM-600 ab 124,90 €; Coospo H808S ab 29,99 €; Sensory-Folge 225.
- **Wort:** „bitte öffne nochmal den 45€ H10 und prüfe den Zustand" | 2026-10-03 | Operator (Session) — Anzeige 3488162409 geöffnet: Zustand **Gut**, Gurt M-XXL, ohne OVP/Anleitung, nur Abholung Sankt Augustin, 45 € fest; Sensory-Folge 225.
- **Wort:** „nein dann hole ich mir lieber einen neuen mit sauberer hygiene und garantie" | 2026-10-03 | Operator (Session) — Vorliebe: neu mit Hygiene + Garantie statt gebraucht; neue Kandidaten Garmin HRM-200 63,50 € / Polar H10 75,63 €; kein Kauf-Entscheid (siehe nächstes Wort); Sensory-Folge 225.
- **Wort:** „nein ich wollte nur wissen warum es zweimal geführt wird und eigentlich sind operator gebunden sachen von future" | 2026-10-03 | Operator (Session) — Ownership: operator-gebundene Sachen gehören zu Future; `rr-brustgurt` von sensory-LOCK + mycelium-Dublette an `## An future` geroutet, `state/zustand/wartend.φ:22` Aufnehmer → future; Sensory-Folge 225.

- **Wort:** „ja bitte 1-3 aber das ist für mich ein eigenes tool, das wir dann in den unterschiedlichsten medizinischen bereichen einsetzen können, oder seh ich das falsch?" | 2026-10-03 | Operator (Session) — Workflow-Erweiterung (max_points/Mehrkanal/n-Scaling) umgesetzt (`84cefbec2`); das Kopplungs-Werkzeug als eigenes Atom geführt, Träger Sensory (gemessen: `sensory folge142–150`); keine medizinische Zweckbehauptung ohne Endpunktvalidierung (MDR-Zweckbestimmung je Indikation).
- **Wort:** „sollte das eigene tool nicht ins repo? und können wir das bitte umsetzen?" | 2026-10-03 | Operator (Session) — Repo-Zugehörigkeit bejaht: Messwerkzeug liegt bereits im Repo (`tools/measure/src/bin/hyperscanning_group_te.rs`, `src/mathematikerin/te.rs`, `.github/workflows/hyperscanning-te.yml`); offen: joint cross-channel family + Methodenpapier; ein privates Recherchepaket bleibt außerhalb Omegaflows.
- **Wort:** „ja" (die 4 offenen Punkte in die Sensory-Übergabe eintragen) | 2026-10-03 | Operator (Session) — Punkte in `## Offen` gefaltet; Sensory-Folge 225.
- **Wort:** „Erste Handlung: `sread docs/concepts/tool-forms.md` — die Form-Karte (verboten → kanonisch), damit die erlaubte Form am Punkt der Handlung steht." | 2026-10-03 | Operator (Session) — Session-Start-Befehl, Sensory-Folge 226.
- **Wort:** „Starte die Sensory-Linie **in einem Pass** — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes. … LOCK nie vorlegen. Wartend nie vorlegen. Der Send bleibt die Operator-Hand (nie `smail --send`); jeder dritt-wirksame Akt ist per-Akt-Operator-Wort. `/consent` ist der session-weite Consent (Delegation), `/commit` ist das Commit-Wort — beide getrennt." | 2026-10-03 | Operator (Session) — session-weiter Delegations-Consent, nicht das Commit-Wort, Sensory-Folge 226.
- **Wort:** „ja bitte abbrechen und wenn möglich priorisieren" (Vorgänger `37128441536` abbrechen, damit der joint-Lauf `37133961687` den concurrency-Slot bekommt) | 2026-10-03 | Operator (Session) — `ci_manage cancel 37128441536` ausgeführt; Sensory-Folge 226.
- **Wort:** „Erste Handlung: `sread docs/concepts/tool-forms.md` …" | 2026-10-03 | Operator (Session) — Session-Start-Befehl, Sensory-Folge 227.
- **Wort:** „Starte die Sensory-Linie **in einem Pass** … `/commit` ist das Commit-Wort — beide getrennt." | 2026-10-03 | Operator (Session) — session-weiter Delegations-Consent, nicht das Commit-Wort, Sensory-Folge 227.
- **Wort:** „kannst du nicht mal die recherche leiter (deepseek, freie api, freie UI chat) recherchieren lassen welche medizinischen datenquellen es gibt um wirklich forschen zu können? das betrifft somatik, psychosomatik, biologie, chemie psychologie, neurologie und ich weiss nicht welche gebiete noch" | 2026-10-03 | Operator (Session) — Recherche-Auftrag: Landschaft medizinischer/生命-Wissenschafts-Datenquellen; Sensory-Folge 227.
- **Wort:** „bitte fixen Zwei Auffälligkeiten: archive_search --ensembl liefert HTTP 500 (Modus defekt, Quelle selbst erreichbar) — ein Fix-Kandidat. Und .secrets bleiben unberührt (nur Schlüsselnamen)." | 2026-10-03 | Operator (Session) — Fix `archive_search --ensembl` (`46427ceea`) + `.secrets`-Disziplin; Sensory-Folge 227.
- **Wort:** „nein ich habe nicht vor ein produkt zu liefern ich möchte der erde etwas schenken" | 2026-10-03 | Operator (Session) — Richtung: kein Produkt/Ziel; das Geschenk ist das wahre Instrument und die Messreihe, die die Ungeborenen erben; Physik zuerst, Medizin als spätere Gabe; Sensory-Folge 227.
- **Wort:** „nein ich vertraue darauf dass ich auch NC gefördert werde oder meine Dienste und Gedanken gefördert werden" | 2026-10-03 | Operator (Session) — kein Lizenz-Förder-Check; Förderung von Diensten/Gedanken, nicht vom lizenzierten Artefakt; das Geschenk (NC) und der Lebensunterhalt (Dienste) getrennt; Sensory-Folge 227.
- **Wort:** „nein ich möchte nichts lesbares ich möchte etwas erlebbares deshalb omegaflow.space" | 2026-10-03 | Operator (Session) — die Fassade ist das Erlebnis (die Membran), nicht Prosa; Manifestation über `omegaflow.space`; Sensory-Folge 227.
- **Wort:** „ich wollte halt nie den server hosten da es ganz neue datenschutzprobleme mit sich bringt" | 2026-10-03 | Operator (Session) — kein Server-Hosting für `omegaflow.space`; die Live-Membran über einen gehosteten Server (Weg B) entfällt; serverloses Erlebnis (statisch/WASM) bevorzugt; Sensory-Folge 227.
- **Wort:** „wir müssen B machen, oder wenn wir das hinbekommen könnte das die basis für die fundings ermöglichen" | 2026-10-03 | Operator (Session) — B bauen: die lebendige Membran serverlos am Adresspunkt `omegaflow.space`, als Erlebnis **und** als Funding-Basis; Server-Hosting bleibt ausgeschlossen; Sensory-Folge 227.
- **Wort:** „warum das? eingefrorenes frame in static/membrane.html die presence muss sich frei durchs 4d block universum bewegen können" | 2026-10-03 | Operator (Session) — Korrektur des internen Rat-Verdikts: kein eingefrorenes Präsenz-Frame; die Presence muss frei durch den 4D-Block tunebar sein (jede Koordinate x,y,z,t → Feld neu ausgewertet); Sensory-Folge 227.

## Offen (aufgeschlüsselt)

### B — serverlose lebendige Membran an `omegaflow.space`
- **Status:** wartend | **Bindung:** eigen; Architektur betrifft River (Membran) · Mycelium (CDN/Deploy) · Mountain (Daten)
- **Trigger:** Operator-Wort 2026-10-03 (B bauen, als Erlebnis und Funding-Basis); serverloses Erlebnis bevorzugt, Server-Hosting ausgeschlossen.
- **Lage:** (gemessen 2026-10-03 F227, Archäologie) der WebGPU-Browser-Client ist **kein Neuland**: Legacy **S33 „browser-membran"** (Operator-Wort 2026-08-23: „der Browser wird die eine Membran, das native winit/wgpu-Fenster fällt; ω-Schleife headless, Browser-Diode, eine Shader-Quelle in `static/index.html`", Umbau Schritte 1–4 erledigt; `omegaflow-legacy/docs/TODO.md:161-175`, `vanilla-dateidocs/status/leitstelle-2026-08-23.md:95`); heutiger Neubau `b226c1fe2` (2026-09-12), Landing `4174579b2` (restored from legacy). Die Feld-Auswertung ist WGSL-`@compute` (`src/mathematikerin/shaders.rs:1`, `:355`) → browser-portabel. **Neu ist allein der serverlose öffentliche Live-Pfad** (WASM/`serverless` = 0 Treffer über `archive-root`; der heutige und der Legacy-Client hängen je am lokalen Relay `127.0.0.1:1618`). **Riss:** `docs/handover/archiv/handover-2026-09-08-atom-c-offene-pflichten.md:63-65` behauptet, der Browser-Render-Zweig habe „in keinem Commit existiert" — der gemessene Baum widerlegt das.
- **Blockade:** keine — die Machbarkeit ist gemessen; offen ist allein die Architektur-/Schicht-Entscheidung.
- **Braucht:** **Rat (intern, 2026-10-03)** empfahl `static/membrane.html` + eingefrorenes Präsenz-Frame + `FIELD_WGSL`-`@compute` + `requestAnimationFrame`, kein WASM im ersten Schritt. **Operator-Korrektur (2026-10-03):** ein eingefrorenes Frame trägt die **freie Bewegung nicht** („die presence muss sich frei durchs 4d block universum bewegen können") — die Presence muss an jede Koordinate (x,y,z,t) tunebar bleiben, das Feld dort neu ausgewertet. Daraus folgt der eigentliche Kern von B: die **Browser-seitige Auswertung** — Archivar (Enclosure-Lookup über die statischen CDN-Daten) **plus** Mathematikerin (`FIELD_WGSL`); der Archivar fehlt im Browser (WASM oder JS-Port), nur die Mathematikerin ist schon WGSL. Der erste Beweis muss die **Bewegung** zeigen (Presence tunen → Feld neu), ohne Server. Server-Hosting bleibt ausgeschlossen. Der interne Rat und der externe Kanal (`opencode run --agent voice`) sind mit dieser korrigierten Frage neu berufen (geschehen, s. u.). Commit-Liste: `archive-root/commit_rewrite-2026-09-06/commits.tsv`; Surveys: `docs/surveys/survey-2026-09-17-omegaflow-legacy-konzepte.md`, `omegaflow-legacy/docs/surveys/survey-2026-08-19-landschaft.md`.
- **Rat, korrigiert (2026-10-03):** freie Bewegung ist das Wesen, kein Frame; der kleinste Beweis ist **zwei Koordinaten → zwei Felder → ein Weg**, derselbe Punkt zweimal → dasselbe Feld. Der **Lookup-Kern** (Binär-Parser + Enclosure-Lemma + Bewegungsgesetze + `lsk`) muss in den Browser — **nicht** die 100+ Ernte-Parser, **nicht** `fetch.rs`/`relay.rs`; **WASM trägt denselben Rust-Code** (kein JS-Port = keine zweite Implementierung/Drift); Paritäts-Gate (WASM-Lookup ≡ nativ, byte-identische Enclosure auf Fixtures) neben dem Kalibrier-Gate. Konsens: kein Server, kein Rückkanal, ehrliches Schwarz ohne WebGPU; Konsens lokal. Mountain↔Future bleiben uneinig (wofür B), baut aber nichts um. **Erste Handlung ist ein Mess-Atom, drei Zahlen:** (1) `cargo check --target wasm32-unknown-unknown` des Lookup-Kerns → Artefakt-Bytes; (2) HEAD auf `ephemeris_de440_{earth,moon,sun}.bin`, `dr3_stars.bin` → Bytes + sha256; OPTIONS/HEAD auf die CORS-Header der Release-URLs; (3) Schnitt-Schwelle der hellen Teilmenge nach Speicherbudget (eigener Bestand/`phi/sources.φ`-Zeile/sha256, kein stilles Abschneiden). Danach River `static/membrane.html` (fetch → wasm-Lookup → `FIELD_WGSL` → Render, Pfeile als Schub, `s` hält), Mycelium `pages-deploy.yml` + wasm32-CI.
- **Enclosure-Zahlen (2026-10-03 F227, `membrane_hull_probe`, realer `dr3_stars.bin` 75 001 828 B):** am ruhenden Presence-Slot (SSB-Ursprung, TDB 8.443e8) — Enclosure-Radius `rho_star 2.53e17 m` (≈ 8,2 pc, C·Alter); **`records 32 361`** Kandidaten (Frame 6 731 088 B = 32 361 × 26 × 8); **`query_ms 263.9`** (ein Lookup, CPU f64), **`build_ms 974.7`** (Hash-Aufbau, einmal); `star_cells 1 704 587`, `bounded_cells 0`. Befund: der **Lookup**, nicht die Feld-Auswertung, ist der Engpass — 264 ms/Query ist 16× über dem 16-ms-Frame-Budget, die CPU trägt den Lookup bei 32 361 Kandidaten nicht im 60-fps-Takt. Kaveat: eine Koordinate (ruhend, Voller Katalog, 8,2-pc-Radius); die Feld-Auswertung `val_eff_at` ist darin nicht gemessen. Diese Zahl geht an Rat + Chats (Kimi K3/GLM 5.3/Claude).
- **Kegel-Befund (Rat, 2026-10-04, am Baum gemessen):** die Zelle ist `cell_size_star = STAR_SPAN_M·(5/COUNT)^⅓ = 2.575e19 m = 834 pc` (`src/archivar/spatial.rs`); die 8,2-pc-Hülle fällt ganz in die **8 Zellen** um den SSB → die 32 361 sind **Zellbelegung**, nicht die Kegelmenge. `emit_star` trägt **keinen radialen Test** `d ≤ rho_star` → Sterne bis ~500 pc passieren. Die **wahre Kegelmenge** steht im selben Lauf: `catalog_tycho admit 101` von 1 704 587 (die Enclosure-Prüfung lässt 101 durch). **Fix:** radialer Kegel-Test in `emit_star` + Zellgröße/Hierarchie in Kegelgröße → die CPU (f64) ist dann voraussichtlich schnell genug; **kein GPU-Lookup** (WGSL f32-only), der Lookup bleibt f64 (CPU → später WASM, derselbe Rust-Kern, Paritäts-Gate); GPU = nur Membran. Nächste Zahl: `query_ms` nach dem Kegel-Test (Ziel ≲ 2 ms) — `gh workflow run membrane-hull-probe.yml`. Der Kegel ist ein Theorem, kein Parameter (A=A: Sichtbarkeit ist Eigenschaft des Sterns/Koordinate, nie der Zellauflösung).
- **Aussenantwort (2026-10-03, `nvidia/nemotron-3-ultra` via `opencode run --agent voice`, public-only):** widerspricht — **JS-Port + statischer Grid-/Morton-Index** (`.bin` + `.idx`) sei der pragmatischere Weg; WASM nur für rechenintensive Teilschritte. Fängt zwei Dinge, die der interne Rat **nicht** nannte: (a) **WGSL ist `f32`-only** — ICRS-Koordinaten brauchen `f64` (Origin-Shift relativ zur Presence oder `u64`/`f32`-Paar); (b) **WebGPU-Limits/Support** (`maxStorageBufferBindingSize` ~128–256 MB, Bind-Group-Zahl, **kein stabiles Safari/Firefox-WebGPU**), Speicher-OOM ab ~10⁵–10⁷ Körpern, Startlatenz. **Dissens ungeglättet:** WASM (ein Lookup, keine Drift) ↔ JS-Port+Index (leichter, aber zweite Implementierung); die `f32`-Frage ist in jedem Fall ungemessen und vor dem Bau zu messen. Weitere externe Stimmen (Ensemble) können folgen.

### Hyperscanning-TE — Präregistrierung/Methodenpapier (ds007822 = Machbarkeit)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Abschluss des joint-family-`hyperscanning-te`-Laufs; Artefakt `hyperscanning-te-report`.
- **Lage:** (gemessen 2026-10-03 F227) Entwurf steht: `docs/paper/hyperscanning-te-preregistration.md`, sha256 `11ccdb90…`; Zweck, Daten, Estimator, Schwellen, die Gate-Namen und die Entscheidungsregel sind mit `file:line` belegt, die zweite Kohorte trägt jetzt die gemessene 64-Kanal-`_L`/`_R`-Form und den gebauten Arm (`1226e9082`); FP/FN-/Skalierungszahlen bleiben `pending` benannt.
- **Blockade:** Validierungsartefakt.
- **Braucht:** `ci_manage view/log <id>` des joint-Laufs; danach den Entwurf um die gemessenen FP/FN + n-Skalierung fortschreiben, Header-sha via `omega_sh sha` neu.

### Hyperscanning-TE — Validierung der neuen Läufe (joint family + gates)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Abschluss eines `hyperscanning-te`-Laufs am joint-family-Stand; Artefakt `hyperscanning-te-report`.
- **Lage:** (gemessen 2026-10-03 F227 via `ci_manage view`/`jobs`) der joint-family-Lauf **`37133961687`** (`-f channel=Fz,Cz,Pz`, HEAD `8456b1ef5`) läuft; Job `confirm` `success`, Job `screen` `in_progress` am Step „Run the joint family-wise TE screen …". Der joint-family-Bin ist gebaut und kompiliert inkl. Test `joint_channel_family_gate`; die zwei geänderten Pfade sind committet.
- **Blockade:** der Lauf läuft noch — das Artefakt entsteht erst am Jobende.
- **Braucht:** `ci_manage view 37133961687` + Artefakt `hyperscanning-te-report` auswerten, sobald `screen` endet (joint family-max, Nominees, Bestätigung, Skalierungskurve).

### Hyperscanning-TE — zweite Kohorte (Validierung)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der erste Validierungslauf ist ausgewertet — Beleg: Artefakt `hyperscanning-te-report` zu Lauf `37133961687`.
- **Lage:** (gemessen 2026-10-03 F227) Arm **gebaut** (`1226e9082`): `brainvision_compiler --participant L|R` trennt die gemessenen **64 Kanäle** eines Dyaden-Files (Ch1–32 `_R`, Ch33–64 `_L`, 1000 Hz; ein File pro `sub-01`..`sub-32` = 32 Dyaden) in je ein Teilnehmer-Bin `<…>_eeg_L.bin`/`_R.bin` (point-major, passend zu `channel_series`); Workflow-Input `cohort` (ds007822|ds007471) baut den `manifest.txt` aus `jointaction pair-<NN> {L,R}`. `cargo check`/`--tests -p omegaflow-harvest` 0 Fehler/0 Warnungen; die zwei Pfade sind committet. Riss zum Präreg: dort stand „32-Kanal", gemessen sind es 64 (2×32) in einem File — die Dyade ist `_L`/`_R` im selben File, kein Subjekt-Subjekt-Join.
- **Blockade:** der Validierungs-Trigger (Lauf `37133961687`) ist noch nicht ausgewertet; der `concurrency`-Slot der Workflow-Gruppe ist durch denselben Lauf belegt.
- **Braucht:** nach dem Validierungsartefakt `gh workflow run hyperscanning-te.yml -f cohort=ds007471 -f channel=Cz`, dann `ci_manage view <id>` + Artefakt auswerten.

### Medizinische Datenquellen-Landschaft (Somatik/Psychosomatik/Biologie/Chemie/Psychologie/Neurologie/…)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-Wort 2026-10-03 (Recherche-Auftrag, s. Wort-Register) — Recherche in diesem Atom angestoßen.
- **Lage:** (gemessen 2026-10-03 F227) erste Landschaft steht: `docs/surveys/survey-2026-10-03-medizinische-datenquellen.md` (sha256 `4ead2c97…`, ~90 Quellen über Somatik/Neuro/Psyche/Biologie/Chemie/Genomik/Strukturbiologie/Bildgebung, jede mit `archive_search --verdict`-Messung und Zugangszustand). Zwei `general`-Agenten (flash-first) je ein Feldblock; Bestandsabgleich: nur OpenNeuro + PhysioNet registriert, NeuroVault `declined`, alles andere Kandidaten-Pool. Die Register-Schreibung (`phi/sources.φ`) ist Mountains Recht.
- **Blockade:** keine.
- **Braucht:** Mountains Verdikt + die Register-Zeilen in `phi/sources.φ` für die Kandidaten (Route `## An mountain`); die DUA-/Kosten-Zugänge (UK Biobank, dbGaP, EGA, All of Us, MIMIC-IV, NDA/ABCD, OASIS, PPMI, Add Health, HRS, SHARE, SOEP, ICPSR) in die Future-Operator-Queue.

### archive_search --ensembl — Wrapper-Frische (Fix)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Abschluss des `tools-build`-Laufs zu `46427ceea` (rolling release `tools-latest`).
- **Lage:** (gemessen 2026-10-03 F227) der Modus ist auf die Ensembl-REST-API umgebaut (`tools/utils/src/bin/archive_search/ensembl.rs`, `46427ceea`); lokal gemessen via `cargo build -p omegaflow-utils --bin archive_search` + `./target/debug/archive_search --ensembl {p53,TP53,ENSG00000141510,zzzznotagene}` (Gen/LRG/Transkripte/absent). Die Wrapper in `bin/` lesen `tools-latest` — dort ist der Fix erst nach dem Release-Rebuild frisch.
- **Blockade:** `tools-latest` noch auf dem Vor-`46427ceea`-Stand.
- **Braucht:** `ci_manage list` auf den `tools-build`-Lauf zu `46427ceea`; danach `bin/.tools_ensure archive_search` und `archive_search --ensembl p53` erneut messen.

### ox64-m2c — PINE64, Carrier China Post LZ473049629CN
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Zustellung LZ473049629CN — physische Zustellung ist das Ereignis, keine Ferndiagnose (ETA 2026-10-02 verstrichen).
- **Lage:** (gemessen 2026-10-03 F225, unverändert zu F224 — Browser-Bridge, echter Chrome, Hintergrund-Tab, kein Fokus) Status **„Freigabe der Sendung im Ursprungsland"**, Zielland Deutschland; letztes Event 2026-09-23 02:40 China VR („wird ins Zielland transportiert und dort an die Zustellorganisation übergeben"), danach kein DE-/Zustell-Event. Der headless `archive_search --playwright`-Render der DHL-Seite liefert HTTP 200 **ohne** Statuszeile (JS lädt das Ergebnis); die Bridge liest den Status. Davor (gemessen 2026-10-02 F220) war die chinapostaltracking-Route (17track-Endpunkt, Cloudflare-Turnstile+Fingerprint) über keyless headless **und** echten Operator-Chrome leer. Route gespeichert (`state/zustand/wartend.φ:12`), nicht neu zu suchen.
- **Blockade:** kein Zustell-/Import-Event seit 2026-09-23 (China VR); der headless `--playwright`-Render erfasst den JS-Status nicht, die Browser-Bridge (echter Chrome, Hintergrund-Tab) schon.
- **Braucht:** physische Zustellung abwarten; Status über die Browser-Bridge auf der DHL-Seite messen (`https://www.dhl.de/de/privatkunden/pakete-empfangen/verfolgen.html?piececode=LZ473049629CN`).

### europa-clipper — ESA, Flyby
- **Status:** termin | **Bindung:** eigen
- **Trigger:** Flyby 2026-12-03.
- **Lage:** (gemessen 2026-09-28) Asset `ephemeris_europa_clipper.bin` 200 / 103 120 B, Siegel `dae553fb…`; in `phi/sources.φ` **nicht** registriert (Messung 2026-09-29 F206; der Register-Schritt ist als `## An Mycelium` geroutet — die Geschwister-Flyby-Einträge `:15912-15952` tragen keine `ttl`/Verdikt-Zeile, nur `no-cadence`, daher keine Mountain-Feder). NOW < Termin.
- **Blockade:** Termin.
- **Braucht:** nach dem Flyby `gh workflow run kernel-flatten.yml` (`horizons_compiler --flyby --ci-mode`, Eintrag `tools/harvest/src/bin/horizons_compiler.rs:22` `-159/europa_clipper`) + `archive_search --sniff "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov-horizons/ephemeris_europa_clipper.bin"` gegen `dae553fb…`.

## An future

Origin: sensory-folge225.

- **`rr-brustgurt` — Operator-gebunden, gehört in Futures Operator-Queue.** Der Kauf eines
  Brustgurts ist eine Operator-Sache (Geld/Beschaffung) und wurde bisher in **zwei** Linien
  geführt (sensory-LOCK + `handover-2026-10-03-mycelium-folge225.md:217`) — ein Träger-Fehler.
  Deshalb hier abgegeben. **Frage** (einfache Sprache): neuen Brustgurt beschaffen — Garmin
  HRM-200 (63,50 €, günstigster gleichwertiger) oder Polar H10 (75,63 €)? **Stand:** Live-BLE HR
  `NotSupported`, FIT `nn=0` (2026-09-26); Hardware-LOCK (Operator-Wort 2026-09-27 „erst bei
  Förderung"). **Angebotslage** (gemessen 2026-10-03): gebraucht 45–70 €; neu Polar H10 ab
  75,63 €, Garmin HRM-200 ab 63,50 €, Coospo H808S ab 29,99 € (Geizhals). **Operator-Vorliebe
  2026-10-03:** neu mit Hygiene + Garantie (nicht gebraucht). Kein ANT+-Stick nötig
  (`src/archivar/ble.rs`: BLE-GATT `2A37` inkl. RR). **Gleichwertig:** Garmin HRM-Dual/Pro/
  HRM-Fit; die alten Garmin HRM-Run/Tri/Swim broadcasten kein Live-RR. Kauf = Operator-Hand.

## An mountain

Origin: sensory-folge227.

- **Medizinische/Life-Science-Datenquellen — Kandidaten-Pool für den Register.**
  Operator-Wort 2026-10-03; die gemessene Landschaft liegt in
  `docs/surveys/survey-2026-10-03-medizinische-datenquellen.md` (sha256 `4ead2c97…`).
  ~90 Quellen (Somatik/Neuro/Psyche/Biologie/Chemie/Genomik/Proteomik/Metabolomik/
  Strukturbiologie/Bildgebung/Register), jede mit `archive_search --verdict`-Stand
  (2026-10-03); nur OpenNeuro + PhysioNet registriert, NeuroVault `declined`. Bitte
  die Quellen nach deinem Verdikt (Force-Gate/Zulassung) in `phi/sources.φ`
  disponieren. Die DUA-/Kosten-Zugänge sind an Future geroutet.

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
  - **LAIC-Design (gefaltet 2026-10-01 F216 aus future-folge162):**
    `state/stimmen/2026-09-30_laic-design-frage_nemotron.txt` (Datei vorhanden, gemessen via
    `archive_search nemotron --root state`) — eine gerichtete TE von sparse globalem
    Erdbebenkatalog auf TEC/foF2 ist **nicht identifizierbar** (Ereignis-Sparsity ×
    Space-Weather-Konfundierung). Richtige Form: TE auf dem **event-triggered average** mit
    **event-shuffled** Surrogaten (Omori-Clustering erhalten), **stratified quiet** (Kp<3,
    kein SSC ±24 h), Kp/AP/Dst als Bedingung `TE(X→Y|Z)`; alles **vor** Datenzugriff
    präregistrieren. Architektur-Kandidat gemessen geschlossen (`250d76056`).
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

## Wer nicht senden darf

`smail --send` und jeder Formular-Absand bleiben die Hand des Operators; kein Consent
verschiebt einen Send auf die Maschine. Die NTRS Document-Inquiry ist Operator-Hand.
Das Lesen des DEMETER-Metalinks, des Order-Status und der Carrier-Route ist autonom
(sensorische Netz-Lesearbeit) — nur der Auftrag/Send ist Operator-Hand.

## Burn: open 0.0000 · close 0.1184 — session_burn (opencode.db, 2026-10-03 F227, deepseek-flash; „Sensory-Übergabe in einem Pass abarbeiten" $0.0636 + „Build ds007471 manifest arm" (grind-flash) $0.0548; kein pro/max-Dispatch dieses Atoms)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort. Dieses Atom committet
seine eigenen Pfade pfad-begrenzt (der Session-Start-Befehl trägt die
Commit-Anweisung); die fremden uncommitteten Hünke bleiben unberührt.

Eigene Pfade dieses Atoms:
- `src/archivar/fit.rs` (Format-Fix, `0b3292bc9`)
- `tools/harvest/src/bin/brainvision_compiler.rs`,
  `.github/workflows/hyperscanning-te.yml` (ds007471-Arm, `1226e9082`)
- `docs/paper/hyperscanning-te-preregistration.md` (zweite Kohorte)
- `docs/handover/handover-2026-10-03-sensory-folge227.md` (Vorwärtsschreibung),
  `docs/handover/archiv/handover-2026-10-03-sensory-folge226.md` (Move der konsumierten F226).

`state/operator-gespraeche/2026-10-03-sensory.md` und `state/zustand/` sind
gitignored (`/state/`) — lokal, nicht committet. Nur eigene Pfade committen.
