Betreff: Zugangsanfrage GAVO-DC-Konto für TAP-Async (Gaia DR3 XP-Spektren)

Sehr geehrter Herr Dr. Demleitner,

ich betreibe ein quelloffenes Feld-System (omegaflow, kybernetische
Astrophysik), das Gaia-DR3-XP-Mittelwerkspektren (gdr3spec.spectra) in
Spektral-Oszillatoren zerlegt. Der Sync-Zugang über https://dc.g-vo.org/tap
funktioniert; für die Ernte der vollständigen Survey-Ernte (Millionen Quellen,
statt der Sync-Kappung von ~20000 Zeilen pro Abfrage) benötige ich den
asynchronen TAP-Zugang (UWS) auf dem GAVO-DC.

Anonym erzeugte Async-Jobs bleiben bei mir im Zustand PENDING (ownerId leer,
auch nach PHASE=RUN), daher die Frage nach einem Datenzentrums-Konto.

Könnten Sie mir bitte Zugangsdaten (Benutzername/Passwort) für den
GAVO-Datenzentrum-TAP einrichten — zweckgebunden für den asynchronen
Lesezugriff auf gdr3spec.spectra? Ich nutze ausschließlich lesende ADQL-Abfragen.

Vielen Dank und beste Grüße
Johannes Tyroller

## Antwort (Markus Demleitner, 2026-09-08) und Befund (2026-09-09, gemessen)

Kein Konto nötig — GAVO verlangt keine Authentifikation; die Anfrage ist
damit erledigt. Der PENDING-Stand war ein Client-Fehler, kein
Berechtigungs-Problem. Gemessen (anonym, ein IP): POST REQUEST=doQuery an
/tap/async erzeugt den Job in PENDING (DaCHS, TAP-1.1-konform —
Location …/__system__/tap/run/async/&lt;id&gt;); erst der explizite POST
PHASE=RUN an {job}/phase startet ihn → COMPLETED, Resultat geholt, Job
gelöscht (303). Der frühere Hang entstand, weil tap_async keinen
PHASE=RUN postete und ewig auf COMPLETED pollte. Gebaut (tap_compiler):
PHASE=RUN-Post, sobald die Phase PENDING ist, und FORMAT=votable/td
(DaCHS liefert Standard-votable als BINARY, das der Parser nicht
dekodiert — votable/td = TABLEDATA; end-to-end verifiziert). UWS-Jobs
bleiben IP-gebunden: stirbt der CI-Runner, verwaist der Job.

## Antwort (Markus, 2026-09-09) — VO-Client-Keim unter github.com/ivoa

Markus bietet an, ein Repo unter github.com/ivoa für einen Rust-VO-Client
einzurichten und zu übergeben („Kristallisationskeim"). Gebaut (2026-09-09):
das eigenständige Crate `tools/vo-tap` (std-only + curl, kein omegaflow-Dep),
gegen GAVO verifiziert (sync + async, COUNT = 219.196.404). Antwort an Markus
GESENDET (2026-09-09) — der Operator fragt nach der IVOA-üblichen Lizenz
(MIT / Apache-2.0 / dual). OFFEN: Markus' Lizenzantwort + Repo-Übergabe;
danach LICENSE + Cargo.toml-`license`-Feld.
