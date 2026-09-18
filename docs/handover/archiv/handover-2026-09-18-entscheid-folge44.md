<!--
  title: Handover — Entscheid-Folge 44 (Stand 2026-09-18)
  session: Entscheid-Folge 44
  class: handover
  date: 2026-09-18
  sha256: cc80468da40e6f69bdc0a002d68b743e9e7c385b62f5b1ae03ae6225fcb75a78
  status: live
-->
# Handover — Entscheid-Folge 44 (2026-09-18)

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

## Stehender Pass (gemessen 2026-09-18, Entscheid-Folge 44)

- **HEAD** `8b8dfd00` bei Messung (== `origin/main`); der Baum ist während der
  Session auf `ad1a05e9` weitergezogen (forschung: post-only an bau). Folge 43
  nannte `985bee3c`.
- **Postfach** — letzter `state/mail/mail_ledger.φ`-Eingang `1789689115`
  (2026-09-18, Rubin-Forum-Migrations-Thread, **kein Agenten-Eingang**); seit
  Folge 43 **kein neuer Eingang**. `sent_ledger`: GitHub-Privacy-Löschantrag
  `1789662499` (privacy@github.com, Ref `01a0b032`) gesendet.
- **CI** — `ci_manage list`/`view`: `ci-check` `35310847988` pending @`8b8dfd00`;
  davor `ci-check` `35307448019` failure @`69c3ae96` (Jobs format/test/clippy),
  `35310810512` cancelled @`7dc620a7`; `release-build` `35302966400` failure
  @`e64cc21b` (Wurzel war die veraltete `ci_manage`-Datei-Kopie auf PATH, nicht
  der Release-Build — bau-folge74); `xp-pilot-cdn` `35305640254` failure;
  `pii-exposure` `35287195140` failure (exit 2 = Exposition bleibt, erwartet);
  `pioneer-odf-cdn` `35309762909` success; sonst fremde Linien.
- **Zustand-Ledger** — `docs/zustand/external-state.md`: CI-Zeile auf `8b8dfd00`
  (neuer pending-Lauf, Release-Wurzel korrigiert), Postfach-Zeile measured-at
  Folge 44 (kein neuer Eingang), TE-Gate-Zeile um die gemessene rote Zelle
  (`FPR rise 6.00pp over a at rho=0.5 exceeds 2pp` @`te.rs:3726`, bau-folge74)
  ergänzt.
- **Post** — die eigene überholte Zeile `An bau: ci_manage log …` gelöscht
  (Werkzeug-Gap laut bau-folge74 geschlossen: Symlink `~/.local/bin/ci_manage`).

## Handlungsfähig — Auswahlpunkte

**Kein session-abarbeitbarer undatierter Punkt.** Die drei offenen Punkte sind
operator-gebunden/blockiert/wartend; die Session fertigt keine Arbeit aus einem
Warten.

- `operator-gebunden` — **ESP32-Modul**, physischer Träger für Puls/HRV; BOM
  `docs/specs/mantis-shrimp-bom.md`. (Schritt: Operator-Wort.)
- `blockiert` — **adoption-Block** (Toth/Turyshev/Markwardt, 20-s-Bande): §4 ist
  geschlossen (forschung `b4e70b1d`, Paper v10). Die Entwürfe
  `state/mail/adoption-{toth,turyshev,markwardt}.body.txt` liegen vor; **Senden
  ist verboten** (Operator-Wort 2026-09-17, bekräftigt). Kein Sendeschritt, keine
  Sende-Anfrage, keine Register-Zeile.
- `wartend` — **GitHub PII-Exposition**: Wert 45 @`3b7aa5a1` (exit 2 = Exposition
  bleibt; `commits reachable 15/15`, `PII retrievable: yes`, `exposed
  combinations 45`). Trigger jetzt nur GitHub-GC-Antwort (HEAD-invariant). GC
  offen (Ticket #4761801); Privacy-Löschantrag gesendet (Ref `01a0b032`).
  (Auslöser: GitHub-GC-Antwort.) (Schritt: `gh run download 35287195140 -n
  pii-exposure` bei neuem Lauf; Wert in `docs/zustand/external-state.md`
  fortschreiben.)

## Wartend (extern) — kein Auswahlpunkt

- `wartend` — GitHub GC `#4761801`, GitHub Privacy-Löschung (Ref `01a0b032`,
  gesendet 2026-09-17), NSE/Haug I(q,t) (TRISP/MLZ Keller; Daten zugesagt), fünf
  Sonden-Anfragen (NSSDC Voyager/Mariner 10/Viking, Cassini, Juno), Rubin-Review
  (Umzug `rubin.community` 2026-09-24), CSES-Limadou (Sotgiu 2026-09-16: neue
  Antragsprozedur nach CSES-02, in einigen Wochen). (Auslöser: Postfach-Eingang.)
- Offene Alternativen: `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md`.

## Termine (Wiedervorlage)

- 2026-09-22 — AllWISE-Coverage (`allwise_coverage.fp01`).
- 2026-09-24 — Rubin-Forum-Umzug auf `rubin.community`.
- 2026-09-28 — JUICE-Flyby (Kernel 000113+); Feld-Zustand füllen.
- 2026-09-30 — EDL-Token-Erneuerung (`EARTHDATA_EDL_TOKEN`, Konto `omegaflow.space`).
- ~2026-10-07 — CSES-Limadou: neue Antragsprozedur nach CSES-02-Umstellung.
- 2026-12-02 — NOIRLab Speisekammer-Frage (Gaia DR4).
- 2026-12-03 — Europa Clipper (Fenster).

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `docs/zustand/external-state.md`,
  `docs/handover/post.md`,
  `docs/handover/handover-2026-09-18-entscheid-folge44.md`,
  `docs/handover/archiv/handover-2026-09-18-entscheid-folge43.md` (Move).
- Fremd uncommittet (nicht angefasst): die drei `handover-2026-09-16-*`-Renames
  (`entscheid-folge24`, `forschung-folge44`, `forschung-folge51`), `opencode.json`.
- Nicht getrackt (`state/`, gitignored): `state/mail/adoption-*.body.txt` und
  `state/mail/adoption-mails.md`.

## Benchmark

- Kein Doppellauf: der Stehender Pass (CI-/Postfach-Zeile, TE-Rotzelle, Post-
  Aufräumen) ist Routine (flash-Klasse), keine gemessene Benchmark-Klasse; kein
  pro/max-Dispatch nötig.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
