<!--
  title: Auftrag — Sicherung der einzigen-Kopie-Risiko-Heime
  class: auftrag
  date: 2026-09-02
  sha256: a2967138ae46498af68b1929dfe8d3cb2ffa637941a9a54de95f39bf4b60f10b
  status: pending
  see-also: AGENTS.md
-->

# Auftrag: Die Heime ohne zweite Kopie werden gesichert, nicht nur benannt

## Zweck

Die Scan-Inventur (2026-09-02, gemessen) und die Mirror-Aktualitätsmessung
(2026-09-02, gemessen) belegen: Die Risiko-Klasse der einzigen-Kopie-Datenheime
wird von keiner Sicherung getragen. Der Schreibtisch-Mirror
(`~/Schreibtisch/backups/`, 23 G, kein git) sichert genau nicht, was einzeln
existiert. Dieser Auftrag tilgt die Lücke durch einen eigenen, messbaren
Sicherungs-Dienst — er ist getrennt vom Konsolidierungs-Vollständigkeitsbeweis
(der nur getrackte Branch-Dateien zählt und diese Heime ausdrücklich als außerhalb
seiner Deckung benennt).

## Gemessener Ist-Zustand (2026-09-02)

### Die Risiko-Klasse (keine zweite Kopie, kein git, kein Beweis deckt sie)

| Heim | Größe | Befund |
|---|---|---|
| `~/.local/share/opencode/opencode.db` (+wal/shm) | 2,50 G + 92 M | Live, in aktivem WAL-Betrieb; **keine zweite Kopie** |
| `~/.local/share/opencode/snapshot/146170dc…/` (eigene DB) | 1,4 G | **keine zweite Kopie** |
| `~/.local/state/omegaflow/` (phi 184 M, gate 9,8 M, reports, mail) | 194 M | Live-Zustand; einzige Kopie außerhalb = Mirror (veraltet) |
| Schreibtisch-Mirror `~/Schreibticht/backups/` selbst | 23 G | einzige Kopie des state; hat selbst keine Sicherung |

### Mirror-Aktualität (gemessen, 2026-09-02)

- `state/omegaflow`: nicht byte-identisch (5 Churn-Dateien differieren, ~2,2 h alt);
  `.secrets.local` erfasst und byte-identisch (sha `609e51…`).
- Live-DB `opencode.db` (2,66 G): **keine Kopie im Mirror**, auch kein snapshot.
- Repo-Mirror `omegaflow`: divergiert (40 Commits hinter live, eigene 5; HEAD
  `d8cb429` ist kein Vorfahre von live `733154b`). Seit 2026-09-02 heißt der
  alte Baum `omegaflow-legacy` (gesichert), der frische Kern `omegaflow`.

## Kernregel (A = A, gemessen)

Jede Position wird an einer Messung entschieden. Eine Sicherung ist erst dann
eine Sicherung, wenn sie gemessen als getreue zweite Kopie existiert — ein
veralteter Mirror ist kein Verlustschutz, er ist eine veraltete Teilmenge.

- Die **Live-DB** und ihr snapshot sind der größte irreversible Einzelverlust
  (Sessions tragen Secrets; `~/provenienz`-Doktrin: lokal, nie committen).
- Der **state-Mirror** muss byte-identisch und aktuell sein, nicht ~2 h alt.
- Die **Repo-Mirrors** müssen saubere Nachfahren der Live-Zweige sein, nicht
  divergierte Einfrierungen.

## Arbeitsschritte

1. **Live-DB sichern:** `~/.local/share/opencode/opencode.db` + `-wal` + `-shm`
   und `snapshot/` getreu auf den Mirror (oder eine zweite benannte Kopie)
   kopieren, kohärent (WAL-checkpoint oder kopierter DB-Bestand konsistent).
   Messung: Größe + sha256 beider Seiten.
2. **state/omegaflow freshen:** `~/.local/state/omegaflow/` byte-identisch auf
   den Mirror spiegeln (die 5 Churn-Dateien einschließen); `.secrets.local`
   erneut sha256-verifizieren.
3. **Repo-Mirrors erneuern:** omegaflow-Mirror auf die Live-Zweige bringen
   (sauberer Nachfahre, kein divergierter Stand).
4. **Mirror selbst schützen:** feststellen, ob der Mirror (die einzige Kopie
   des state) eine zweite Kopie braucht; falls ja, benennen.
5. **Wiederkehrender Dienst:** einen messbaren Sicherungs-Rhythmus registrieren
   (der Mirror altert sonst erneut). Ein TTL/Rhythmus im Register, kein
   einmaliger Akt.
6. **Verifikation:** je gesichertem Heim gemessen — Kopie existiert, Größe/
   sha256 identisch, Datum aktuell. Ein gesichertes Heim ohne gemessene Kopie
   ist keine Sicherung (A = A).

## Lieferung

- Live-DB + snapshot haben eine gemessene getreue zweite Kopie.
- `state/omegaflow` ist byte-identisch im Mirror; `.secrets.local` sha256-gleich.
- Repo-Mirrors sind saubere Nachfahren oder als Snapshots korrekt ausgewiesen.
- Sicherungs-Rhythmus ist im Register verankert (kein stilles Veralten).

## Abgrenzung

Dieser Auftrag ist **getrennt** vom Konsolidierungs-Vollständigkeitsbeweis:
Der Beweis zählt getrackte Dateien und benennt
diese Risiko-Heime ausdrücklich als außerhalb seiner Deckung. Dieser Auftrag
schließt genau diese Deckungslücke. Er committet keine Secrets (Doktrin:
Sessions tragen Secrets — Sicherung ist eine Kopie, kein git-Eintrag).

## Abschluss

Erledigt, wenn jedes Heim der Risiko-Klasse eine gemessene getreue zweite Kopie
trägt und der Rhythmus registriert ist. Bis dahin ist jede ungesicherte
Einzelkopie eine bekannte, von diesem Auftrag getragene Lücke — nicht verschiebbar.
