<!--
  title: Handover — Sensory-Folge 200 (2026-09-28)
  session: Sensory-Folge 200
  class: handover
  date: 2026-09-28
  sha256: 293a906b68a14625fbda16b8164fa33f387251a2cecaeec8d0cd88698e92d5d2
  status: live
-->
# Handover — Sensory-Folge 200 (2026-09-28)

Dieses Register trägt nur Offenes — Erledigtes ist gelöscht, nicht erklärt; git trägt,
was gemacht wurde. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks —
committet wird pfad-begrenzt, fremde uncommittete Arbeit wird nie überschrieben;
gepusht wird, sobald der eigene Commit steht und `origin/main` Vorfahr von HEAD ist.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** von Agenten abgearbeitet. Jeder Punkt trägt **Trigger** / **Lage** (mit
Messstempel) / **Blockade** / **Braucht**. Sortierung: **umsetzbar zuerst**; der Akteur
steht pro Punkt in `Bindung`.

Dieses Register konsumiert `handover-2026-09-28-sensory-folge199.md` (nach `archiv/`);
diese Folge ist 200. Der Stehende Pass wird zitiert, nie kopiert:
`state/zustand/standing-pass.md` (gemessen 14:46Z, HEAD dort `ab2b14c9e`; der Baum steht
auf `138b785a5` — der Pass ist damit stale, die Meta-Linie zieht ihn in ihrer Runde nach).

Dieses Atom maß die einzige lebende externe Abhängigkeit der Linie frisch an ihre Kante:
**DEMETER (Order 18400)** — der Datei-Endpoint `…/files/9162386` antwortet direct
**202 / 0 B** und nach Proton **202 / 0 B** (Wayback 200 ohne CDX-Snapshot); der
Bare-Endpoint `/api/v1/rs-order` bleibt 403/358 B (F5-ASM-WAF). **Direktmessung in der
angemeldeten SPA `Command` (2026-09-28 ~16:07 UTC):** Order 18400 `Running` **0 %**, 97 078
Dateien / 34,71 GB, Ablauf 2026-10-05 05:04:06 UTC; die Vor-Order (21.09., `Done with
warning`) ist am 2026-09-28 12:46:18 UTC abgelaufen. **Ursache gemessen (Order-API
`/api/v1/rs-order/user/orders/18400?token=…`):** `status RUNNING`, `percentCompleted 0`,
`availableFilesCount 0`, **`filesInErrorCount 1000`**; die Vor-Order 18387 endete
`DONE_WITH_WARNING` mit **96 978 von 97 078 Dateien im Fehler** / 0 verfügbar. Grenze: alle
Orders mit `creationDate ≤ 2026-09-12` sind `DONE`/0 Fehler, alle `≥ 2026-09-18` tragen
Fehler — **die CDPP-REGARDS-Generierung ist zwischen 12.09. und 18.09. server-seitig
gekippt**, kleine wie große Orders. CDPP meldet auf der Seite selbst: „Due to a technical
problem, data access is not possible" (Operator-Wort, 2026-09-28). Kein order-freier
Alternativkanal (SPDF/CDAWeb 404 verifiziert; SPASE-AccessURL nennt nur REGARDS; AMDA ohne
DEMETER-Datensatz; Recherche `general`). Kein Datenbyte, Trigger **nicht gefeuert** — es gibt
technisch nichts zu umgehen; die einzige Bewegung ist die Server-Wiederherstellung.
`regards-cnes` ist dieselbe Sache (Token nur über die angemeldete SPA). Die übrigen vier
Aufnehmer-Waits (`laic-cses`, `ox64-m2c`, `europa-clipper`, `rr-brustgurt`) hängen an
Terminen bzw. LOCK, alle > NOW. `survey-2026-09-06-codestruktur` bleibt geschlossen
(folge199). Die `wartend.φ`-Zeile (Aufnehmer sensory) und
`state/zustand/external-state.md:49` wurden mit dem F200-Stempel nachgezogen. **Kein eigener
dispatchbarer Punkt in diesem Atom** — der Plan (Phase 1) hielt mit genannten Triggern.

## Operator-Wort-Register (Stand 2026-09-28)

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

## Offen (aufgeschlüsselt)

### DEMETER (Order 18400) — CDPP-REGARDS-Generierung server-seitig defekt (0 verfügbare Dateien)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** CDPP/REGARDS stellt die Generierung wieder her — Order 18400 liefert verfügbare Dateien (`availableFilesCount > 0` bzw. Datei-Endpoint HTTP 200). Kein order-freier Alternativkanal existiert (gemessen).
- **Lage:** (gemessen 2026-09-28 via `archive_search --verdict` + Order-API, Sensory-Folge 200; Vorwert Folge 199) Order-API `/api/v1/rs-order/user/orders/18400?token=…` (angemeldete SPA): `status RUNNING`, `percentCompleted 0`, `availableFilesCount 0`, **`filesInErrorCount 1000`**; Datasets `DMT_N1_1143` (39 318) + `DMT_N1_1144` (57 760). Vor-Order 18387: `DONE_WITH_WARNING`, **96 978/97 078 Dateien im Fehler**, 0 verfügbar. Grenze: alle Orders `creationDate ≤ 2026-09-12` = `DONE`/0 Fehler, alle `≥ 2026-09-18` tragen Fehler — die CDPP-Generierung kippte zwischen 12.09. und 18.09., kleine wie große Orders. Datei-Endpoint `…/files/9162386` direct+Proton **202 / 0 B** (kein Datenbyte); Bare-Endpoint `/api/v1/rs-order` 403/358 B (F5-WAF). CDPP meldet auf der Seite selbst: „Due to a technical problem, data access is not possible" (Operator-Wort, 2026-09-28). Kein order-freier Alternativkanal: SPDF/CDAWeb `pub/data/demeter/` **404** (verifiziert), SPASE-AccessURL nennt nur REGARDS, AMDA ohne DEMETER-Datensatz (Recherche `general`). Lesen/Metalink autonom; Send/Order-/Retry-Akt Operator-Hand.
- **Blockade:** Server-seitig — CDPP-REGARDS generiert keine Dateien (`availableFilesCount 0`, Fehlerstatistik steigt). Technisch nichts zu umgehen; der Fehler liegt in der Archiv-Pipeline, nicht beim Client.
- **Braucht:** CDPP-Wiederherstellung abwarten — Trigger messen (einmal, kein Polling): `archive_search --verdict` bzw. Order-API `/user/orders/18400?token=…` → `availableFilesCount > 0`/Datei-Endpoint `200` → Dateien aus `data/regards.cnes.fr/order_18400.metalink` ziehen, **vor 2026-10-05 05:04:06 UTC**. Keine Support-Mail (CDPP erklärt den Defekt selbst, Operator-Wort 2026-09-28). Offene Order-Retry-Links (`…/18373/retry`) bleiben Operator-Hand.

## Aufnehmer-Waits (Aufnehmer sensory, Wohnort `state/zustand/wartend.φ`)

Jede Zeile ein eigener Akt; die Trigger wurden am 2026-09-28 frisch gemessen.
Der Aufnehmer (sensory) arbeitet eine Zeile im selben Atom, in dem ihr Trigger feuert —
kein Punkt wird bloß benannt, wo ein Schritt zur Kante existiert.

- `demeter-18400` (`wartend.φ:4`) — Trigger Order-18400-Done → Datei-Endpoint 200; Lage 2026-09-28 (Folge 200): Datei-Endpoint 9162386 202/0 B (direct+Proton), Wayback 200 ohne Snapshot (Running).
- `regards-cnes` (`wartend.φ:5`) — Trigger Order abrufbar / Route öffnet; Lage 2026-09-28 (Folge 199): `/api/v1/rs-order` 403/358 B (direct+Proton), Wayback 200 ohne Snapshot.
- `laic-cses` (`wartend.φ:11`) — termin 2026-10-02; PI-Berechtigung offen (ASI/SSDC). NOW < Termin.
- `ox64-m2c` (`wartend.φ:13`) — Trigger Zustellung LZ473049629CN; Lage: China Post „On route", letztes Event 2026-09-23 Shenzhen, ETA 2026-10-02 (noch nicht zugestellt).
- `europa-clipper` (`wartend.φ:15`) — termin 2026-12-03; Epoche ernten. NOW < Termin.
- `rr-brustgurt` (`wartend.φ:23`) — LOCK (Hardware erst bei Förderung); Trigger „Förderung gewährt" (Wort nicht gefallen).

## Termin

- **Europa-Clipper** | termin:2026-12-03 | Trigger: 02./03.12. | Lage (gemessen
  2026-09-28): Dritt-Wait in `state/zustand/wartend.φ:15` (Aufnehmer sensory) |
  Braucht: Epoche ernten.

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
  2026-09-28**: `archive_search --pdf-image` lieferte aus `gosat_l1b_format.pdf`
  19 PNG; Nachtrag im Dokument. Getragen bleiben die echten `request-only`-Marker der
  vier Sonden (Register: `phi/blocked_sources.φ`).
- **JUICE:** `docs/concepts/der-paradigmenwechsel.md` (9),
  `docs/paper/flyby-path-2-falsification-metric-addendum.md` (4),
  `docs/paper/flyby-path-2-preregistration.md` (1).
- **Das eine Instrument:** `docs/concepts/das-eine-instrument.md` (2).
- **termin:2026-12-02:** `docs/blatt/blatt-thuan-fragesteller.md` (4).
- **B1/B5/B8 (Befunde geschlossen in folge181; Marker nachgezogen 2026-09-27):**
  `docs/concepts/the-seven-spheres.md` (B8-Marker nachgezogen; Restmarker echt),
  `docs/concepts/recherche-extern-galileo-ruck-borduhr-modell.md` (B1 §3-Verdikt auf
  `absent` geschärft),
  `docs/paper/jwst-disequilibrium-survey.md` (B5-Schließung steht; Rest-pendings = echte
  ungemessene Kanäle).
- **descoped (gemessen 2026-09-27):** `docs/paper/terminologie-der-gegenstroemung.md`
  (Definitions-Kopf), `docs/paper/planet-nine-kbo-residue.md` (Negation),
  `docs/paper/asmar-2005-spacecraft-doppler-tracking-noise-budget.md` (Scanner-False-Positive),
  `docs/surveys/axiom-gate-broken-null-control.md` (nur Pendings des Papers).
- **carrier:** `docs/surveys/survey-2026-09-06-codestruktur.md` (übrige offene
  Vermessungs-Dimensionen; der `confirm-rendering`-Marker ist **erledigt** — 66
  `φ window:`-Zeilen im `matrix-rotor.txt`-Asset gemessen, Folge 199),
  `docs/paper/probe-front-dark-matter.md` (v11; die MDA/resolver-Herkunft bleibt im
  Paper offen, 0 honored — der NTRS-/PTTI-Volltext ist gemessen: der DTIC-Band
  `ADA149163` trägt kein `MDA`/`resolver`/`Mark IV`).

## An Mycelium (gemessen, fremde Feder)

- **DEMETER — CDPP-WAF-Route (Order 18400 aktiv, 18387 abgelaufen):** (gemessen 2026-09-28 via
  `archive_search --verdict`/`--sniff`) die Route
  `regards.cnes.fr/api/v1/rs-order` liefert am Token-Datei-Endpoint die 247-B-F5-WAF-Seite
  „Request Rejected" (`magic unrecognized`); die anonymen Arme sind WAF-403 (direct und
  Proton 403, Wayback 503 — gemessen 06:22 UTC); kein Spiegel (SPDF/CDAWeb/NCEI
  `pub/data/demeter/` → 404).
  **Korrektur (gemessen 2026-09-28 11:00 UTC; Lesekante erneut Folge 199):** der
  Datei-Endpoint ist **UA-gated, kein IP-Block** — `curl` ohne Browser-UA → `403`, mit
  Browser-UA → `202` (nur ein `orderToken`, kein Login); `archive_search --verdict`
  passiert das Gate (202, nicht 403); im angemeldeten DevTools-Browser dieselbe Antwort
  (18400 `202`, 18387 `500`). Die Dispositions-Klasse `blocked ip-blocked` (CDPP) ist damit
  zu prüfen (eher `parser-def`/Route-Gate). Der Dispositions-Eintrag der Sache steht als
  `pending` in `phi/blocked_sources.φ:81-83`; Myceliums Feder: die Klasse korrigieren.
  **Nachtrag (gemessen 2026-09-28 via Order-API, Folge 200):** die Route ist intakt; die
  eigentliche Blockade ist **server-seitig** — die CDPP-REGARDS-Generierung liefert
  `availableFilesCount 0` und wirft Fehler (18400 `filesInErrorCount 1000`; 18387
  `DONE_WITH_WARNING` 96 978/97 078 Fehler; Grenze 12.–18.09.2026). CDPP meldet „Due to a
  technical problem, data access is not possible". Die Klasse ist damit weder `ip-blocked`
  noch `parser-def`, sondern „CDPP-Generierung defekt — wartend".
  Der Wait steht `state/zustand/wartend.φ:4` (Aufnehmer sensory).
  Origin: sensory-folge194/195/196/197/199/200.

## An Mountain (gemessen, fremde Feder)

**Sonden-Flotten-Erweiterung — Asien/Russland, anonym offen** (gemessen 2026-09-28,
`research-max`, harte Bandagen; Details + Belege in
`docs/surveys/survey-2026-09-16-sonden-flotte.md` `## Nachtrag 2026-09-28`).
Alle Routen 200 anonym. Korrigierte Vorprüfung (`archive_search` über `phi/` + `docs/`,
2026-09-28): **teils schon im Haus** — `phi/sources_index.φ` trägt NAIF-Kernel-Dirs für
BepiColombo, ExoMars2016, Hayabusa, Hayabusa2 (`hyb2`), Vega, LunarOrbiter, SELENE;
BepiColombo ist registriert (`phi/sources.φ bc_mpo_mag`, `phi/blocked_sources.φ:53`).
**Neu** (`archive_search`-Zähler 0): Akatsuki-RS, Chandrayaan, Venera 15/16, Phobos-2,
Danuri/KASI, CNSA. Mountains Feder: die `url`/`origin`/`compiler`-Zeilen (Mycelium:
Aufnahme/Compiler) — je Route ein Port-Schritt nach `docs/SOURCE_PORT.md`. **Eingetragen
2026-09-28 als `ausstehend kandidat` in `phi/pipeline/ledger.φ`** (owner mycelium; der
Pass liest sie per `register_lookup --open`).

- **Akatsuki Radio Science (JAXA/ISAS)** — `https://data.darts.isas.jaxa.jp/pub/pds4/data/vco/vco_rs/` — PDS4-Bundle `urn:jaxa:darts:vco_rs`, SIS `…/vco_rs/document/vco_rs_sis_v13.pdf`. Einziges asiatisches Radio-Science-Roh mit offener Tür.
- **Hayabusa PDS4 (JAXA/ISAS)** — `https://sbnarchive.psi.edu/pds4/hayabusa/` (AMICA/LIDAR/NIRS/Mission/SPICE).
- **Kaguya/SELENE LRS (JAXA/ISAS)** — `https://ode.rsl.wustl.edu/moon/pagehelp/Content/Missions_Instruments/KAGUYA%20(SELENE)/LRS/Raw_Data.htm`; DARTS-SLN `darts:sln-l-rise-5-traj-rstar-v1.0`.
- **Chandrayaan-1 PDS3 (ISRO)** — `https://pds-geosciences.wustl.edu/missions/chandrayaan1/` (+ SPICE `spiftp.esac.esa.int/data/SPICE/CHANDRAYAAN-1/`).
- **Venera 15/16 (UdSSR, PDS-Spiegel)** — `pds-geosciences.wustl.edu/venera/mpi-venus-alt.dat` (10 398 160 B) + `…/mpi-radiometry.dat` (5 949 650 B).
- **Vega 1/2 Halley + Ballons (UdSSR)** — `pds-smallbodies.astro.umd.edu/holdings/vega2-c-{tvs,ducma,sp1,sp2,puma,pm1,mischa}-*` + `atmos.nmsu.edu/PDS/data/vega_5001/`.
- **Phobos 2 (UdSSR)** — `pds-smallbodies.astro.umd.edu/holdings/phb2-m-{krfm-3-photometry,vsk-2-edr}-v1.0/`.
- **ExoMars TGO (russ. Instrumente ACS/FREND)** — `archives.esac.esa.int/psa/ftp/ExoMars2016/`.
- **Danuri/KPLO (KASI)** — `pda.kasi.re.kr/` + `shadowcam.im-ldi.com/`.

Origin: sensory-folge195.

## An Future (operator-gebunden)

**Account-gated Sonden-Quellen — Konto/Antrag = Operator-Hand** (gemessen 2026-09-28,
`research-max`; Details im Flotten-Nachtrag). Die Daten existieren; Zugang nur per Konto.
Aufnehmer future (wartend.φ gesetzt). Future legt sie dem Operator in einfacher Sprache vor.

- **CNSA Chang'e 1–6** — `moon.bao.ac.cn` (GRAS): Portal 200, Download Konto-gated (Gate nicht end-to-end gemessen).
- **CNSA Tianwen-1** — `nssdc.ac.cn`: Portal 200, Antrag/Konto.
- **ISRO Chandrayaan-2/3, MOM, Aditya-L1** — `pradan.issdc.gov.in` / `mrbrowse.issdc.gov.in`: account nötig.
- **MBRSC Hope/Al-Amal (VAE)** — `sdc.emiratesmarsmission.ae`: kostenloses Konto.

**Eingetragen 2026-09-28 als `blocked account` in `phi/blocked_sources.φ`** +
`state/zustand/wartend.φ` (Aufnehmer future).

Origin: sensory-folge195.

## Wer nicht senden darf

`smail --send` und jeder Formular-Absand bleiben die Hand des Operators; kein Consent
verschiebt einen Send auf die Maschine. Die NTRS Document-Inquiry ist Operator-Hand.
Das Lesen des DEMETER-Metalinks und des Order-Status ist autonom (sensorische
Netz-Lesearbeit) — nur der Auftrag/Send ist Operator-Hand.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.

Eigene Pfade dieses Atoms (Commit/Push pending `/commit`):
- `docs/handover/handover-2026-09-28-sensory-folge200.md` (neu),
  `docs/handover/archiv/handover-2026-09-28-sensory-folge199.md` (Move).

`state/operator-gespraeche/2026-09-28-sensory.md`, `state/zustand/wartend.φ` und
`state/zustand/external-state.md` sind gitignored (`/state/`) — lokal, nicht committet.
