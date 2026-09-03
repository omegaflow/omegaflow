<!--
  title: Auftrag — verify-references Regel-Runde: CASE 5 (Archiv-Exklusion) und CASE 6 (Fließtext-Drift)
  class: auftrag
  date: 2026-09-03
  status: pending
  see-also: tools/register/src/bin/path_reference_scan.rs, docs/concepts/das-eine-instrument.md
-->

# Auftrag: die verify-references Regel-Runde

## Ausgangsmessung (2026-09-03)

Der Scanner `tools/register/…/path_reference_scan.rs` (main `55648b4`) läuft
als CI-Test und erzwingt: 0 tote in-Repo-Referenzen, 0 Absolutpfade. Der
CASE-5-Filter-Bug der alten `^/`-Regel ist damit funktional erledigt — der
Scanner verbietet jetzt jede leading-home/user-Absolutpfad-Stelle
(`/home/`, `/Users/`, `/root/`, `/srv/`, `/mnt/`) und die eine
inline-Archiv-Absolutpfad-Stelle (`survey-auswertung`) wurde entfernt.

## Zwei offene Regel-Entscheidungen — je Kalibrationslauf, je Commit

- **CASE 5 — Archiv-Absolutpfade (Entscheidung abweichend vom Chat-Vorschlag).**
  Der Verlauf schlug vor: „Exklusion nur auf `/archive/`-Substrings" — d. h.
  legitime Absolutpfade in ein Archiv bleiben erlaubt und sind nur von der
  Prüfung freigestellt. Umgesetzt ist das Strengere: Absolutpfade ins Archiv
  sind ganz ausgeschlossen (Referenzen ins `archive-root` laufen als bare
  Namen, nicht als Absolutpfade; `archive-root` liegt außerhalb des Baums).
  Zu entscheiden, nicht still zu übernehmen: ist „kein Absolutpfad in den
  committeten Baum, auch nicht ins Archiv" die gewollte Regel — oder soll
  eine dokumentierte Archiv-Ausnahme (Absolutpfad nur unter einer benannten
  Archiv-Wurzel) bestehen? Kalibrationslauf gegen die lebenden Dokumente.
- **CASE 6 — Fließtext-Drift (Resthälfte).** `file_refs()` extrahiert nur
  see-also-Tokens und `](…)-Links; Backtick-lose Referenzen im Fließtext
  (z. B. `survey-fortschritt.md` in Prosa) werden nicht geprüft. Der
  historische Anteil ist dokumentierte Grenze (historische Anker werden nicht
  zurückverfolgt). Offen die echte Resthälfte: **Fließtext-Drift in lebenden
  Dokumenten** — wird die Klasse gejagt (Backtick-lose Pfade in lebenden
  Dokumenten erkennen und auflösen) oder als dokumentierte Grenze
  festgeschrieben? Entscheidung + Kalibrationslauf + Commit, getrennt vom
  CASE-5-Commit.

## Sequenz

Zwei Entscheidungen, zwei Kalibrationsläufe, zwei Commits — nicht vermischt
mit dem main-Umzug oder anderen Pflichten. Die Regel-Entscheidungen werden
registriert (je Zeile im Register), nicht still gefällt. main-Guard
respektieren; Commits der Feldarbeit eigenverantwortlich im Repo.

## Lieferung

Für CASE 5 und CASE 6 je eine registrierte Regel-Entscheidung mit
Kalibrationslauf (bekannte Fundzahl gegen die lebenden Dokumente) und je
ein Commit; der Scanner-Kommentar nennt das, was der Code tut (die Lehre
von CASE 5: „der Kommentar lügt über den Code" darf nicht wiederkommen).

## Abschluss

`path_reference_scan` und sein Kommentar sind konsistent mit der
entschiedenen Regel; die zwei offenen Entscheidungen stehen als Zeilen im
Register, nicht als stille Übernahmen.
