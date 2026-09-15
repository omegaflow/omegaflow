<!--
  title: Handover — Entscheid-Folge VIII (Stand 2026-09-15)
  session: Entscheid-Folge VIII
  class: handover
  date: 2026-09-15
  sha256: 62d5567a1ddc4792f66b667a841a69fcb4b35264e9691e476699a404bb0ef8d6
  status: live
-->
# Handover — Entscheid-Folge VIII (2026-09-15)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht.

Dies ist die Entscheid-Linie: hier steht nur, was diese Linie autonom trägt —
Tasks, die nicht autonom hier erfolgen können, sind als Nachricht an ihre Linie
überführt (nie in ein fremdes Handover geschrieben).

## Nachricht an die Ernte-Linie — Notwendigkeits-Abgleich der Zugangsanfragen

Die Notwendigkeit einer Zugangsanfrage entscheidet sich am Quellen-Register, nicht
an der Korrespondenz — darum misst die Ernte-Linie je Anfrage zuerst; Entscheid
streicht danach nur die Briefe, die Ernte als „nicht mehr nötig" misst.

Versandte Mails im Ledger `state/mail/mail_ledger.φ` (57 Records, multi-line — auf
`^mail\t` schlüsseln; `smail.rs` loggt Versand nicht selbst, die Sent-Spur entsteht
nur durch manuelles `WG:`-Weiterleiten). 14 distinkte versandte Mails:
- 2026-08-01 JSOC (jsoc@sun.Stanford.EDU) — CONFIRM EXPORT ADDRESS
- 2026-08-18 TOAR (info@toar-data.org) — Registration as TOAR user
- 2026-08-23 BiSON (sarbani.basu@yale.edu) / CSES-Limadou (alessandro.sotgiu@roma2.infn.it) / NSE/Haug (B.Keimer@fkf.mpg.de)
- 2026-08-25 TOAR (s.schroeder@fz-juelich.de) / BiSON (A-M.Broomhall@warwick.ac.uk)
- 2026-09-08 LPF Δg-PSD (michele.armano@esa.int, CC paul.mcnamara@esa.int) / GAVO TAP-Async (msdemlei@ari.uni-heidelberg.de)
- 2026-09-10 IGETS SFTP (igets-support@gfz.de)
- 2026-09-11 NOIRLab Data Lab (robert.nikutta@noirlab.edu)
- 2026-09-12 GAVO ivoa-Umstellung (msdemlei@ari.uni-heidelberg.de)
- 2026-09-14 Rubin RSP (sheather@slac.stanford.edu) / 2026-09-15 Rubin RSP Folge (CC code@)

Register-Befund je Anfrage (gemessen, Verdikt = Ernte):
- NOIRLab: `ls_dr10.tractor` steht anonym in `phi/sources.φ:8498` → Anfrage vermutlich entbehrlich (ohnehin abgelehnt).
- JSOC: `jsoc.stanford.edu` AIA-Assets in `phi/sources.φ:8064+` → Registrierung evtl. entbehrlich.
- LPF: anonyme DRS-FITS-Route in `phi/blocked_sources.φ:38` gemessen → Anfrage evtl. redundant.
- GAVO TAP-Async: `gaia-xp-full-cdn.yml` gebaut, Sync-Kappung ~20000 → wahrscheinlich nötig.
- BiSON, IGETS: offene Ernte-Punkte (`handover-2026-09-15-ernte-folge28.md`) → nötig.
- CSES-Limadou: `docs/concepts/der-kausalpfeil.md:92` und `kybernetische-astrophysik.md:143` führen CSES als `descoped` (2026-09-12, kein anonymer Zugang); `folge28` trägt aber noch „CSES-SPA-Login" → Widerspruch, Ernte entscheidet.
- NSE/Haug: `survey-2026-09-14-warteliste-offene-alternativen.md:61` führt die Rohdaten als `descoped` (kein Deposit, null Datendateien); Forschung §Extern wartet aber noch auf Keimer → Widerspruch, Ernte entscheidet.
- Rubin RSP, TOAR: kein `phi`-Eintrag, kein Ernte-Punkt → Notwendigkeit ungeklärt.

(Schritt: Operator übergibt diese Nachricht der nächsten Ernte-Session; Ernte misst
je Anfrage und trägt das Verdikt in `folge28` ein.)

## NIM-Spezialmodelle — riva-translate angedockt, parse/embedding offen

- `nvidia/riva-translate-4b-instruct-v2` als reine Ruf-Stufe gebaut:
  `tools/service/src/bin/riva_translate.rs` (std+curl, system-Tag `<from>-<to>`,
  OpenAI-Antwortformat; 6 Tests grün, de→en am echten NIM-Pfad gemessen). Key:
  `NVIDIA_API_KEY` in env oder `.secrets.local` (dort noch nicht eingetragen).
  Kein Tool-Ruf → kein Agent. (Schritt: `NVIDIA_API_KEY` in `.secrets.local`
  eintragen; ersten Konsumenten benennen — welche fremdsprachige Quelle.)
- `nvidia/nemotron-parse-2.0` gemessen: VLM, leerer `content` auf Text-Chat →
  keine Text-Ruf-Stufe. Nur bei einem Dokument-/Bild-Parse-Konsumenten neu messen.
  (Schritt: Konsument benennen — Operator.)
- Embedding/Rerank über NIM: `404` gemessen, semantische Nähe nicht bedienbar →
  pending über einen anderen Weg. (Schritt: anderen Embedding-Pfad benennen.)

## Free-Gold-Recherche — `free-research` installiert, Transport instabil

- Rat-Verdikt (2026-09-15): der strikte Filter (Preis frei ∧ Vertrag trägt ∧
  Verhalten gemessen) schließt kilo (anonym), openrouter free (Prompt-Training),
  GLM (gestrichen) und den opencode-Provider (kein Key) aus; einziger
  free-gold-Kandidat ist `nvidia/nemotron-3-super-120b-a12b`. Kalibrier-Gate 7/7
  (Grounded-QA, Fabrikations-Falle mit ABSENT bestanden). Agent `free-research`
  in der Global-Config installiert (read-only: edit/task deny, bash nur
  archive_search + proton-wg). Offen: der Transport liefert 46 % HTTP 503 →
  Retry nötig; dauerhafte Zuordnung erst nach einer Stabilitäts-Messung.
  (Schritt: opencode neu starten; 503-Rate über N Calls messen.)

## Warten auf Rückmeldung (extern gebunden — kein Datum)

- adoption — Drei-Mail-Block (Toth/Turyshev/Markwardt) nicht gesendet; der Blocker
  Bande-Split ist geschlossen. (Schritt: senden — Operator.)
- GitHub Support — User→Org / HTTP 422: Antwort offen. (Schritt: Postfach prüfen.)
- Rubin RSP-Datenrechte — Antwort an Shaughnessy (SLAC) gesendet 2026-09-15;
  Antwort offen.
- NSE/Haug — Anfrage raus, Antwort offen (Keimer).
- GAVO — geschlossen: Demleitner hat die ivoa-Umstellung am 2026-09-14 bestätigt
  (`state/mail/mail_ledger.φ`).

## Termine (Wiedervorlage)

- 2026-09-22 — AllWISE-Coverage-Verifikation (CDN-Asset `allwise_coverage.fp01`).
- 2026-09-28 — JUICE-Flyby (Kernel 000113+); Feld-Zustand füllen
  (`papers/flyby-path-2-preregistration.md`).
- 2026-12-02 — NOIRLab Speisekammer-Frage (Gaia DR4).
- 2026-12-03 — Europa Clipper (Fenster); Feld-Zustand füllen.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
