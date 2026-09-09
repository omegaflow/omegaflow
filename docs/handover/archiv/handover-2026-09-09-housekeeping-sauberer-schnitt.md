<!--
  title: Handover — Housekeeping: sauberer Schnitt vollzogen, vo-tap/uvor-Seed gebaut, TODO-Auflösung beschlossen
  class: handover
  date: 2026-09-09
  sha256: 60bba09ef310fb41ded217de80dbcce3095aec78f75df00ee27120e6ed65df1a
  status: archived
  see-also: docs/auftrag/gavo-dc-account-anfrage.md tools/vo-tap/ docs/TODO.md
-->
# Handover — Housekeeping: sauberer Schnitt vollzogen, vo-tap/uvor-Seed gebaut

Übergabe für die nächste Sitzung. Diese Sitzung hat das Repo aus dem
Sieben-Session-Chaos in einen sauberen Zustand gebracht und den
GAVO/VO-Client-Faden bis zur Übergabe an Markus geführt.

## Vollzogen — der saubere Schnitt

- Git: `main` == `origin/main`, Working Tree leer, drei Stashes gedroppt,
  `release-fix`-Branch und `backup-pre-rebase`-Tag gelöscht.
- Die Historie der sieben Parallel-Sessions ist versöhnt (Commit-Rewrite
  abgeschlossen). Der eine echte Stash-Orphan (inpop-epm sha256-Log) ist
  committet; der Rest der Stashes war per Session-Wort stale oder
  verstrickt und wurde gedroppt.
- Sieben Commits gepusht: Release-channel-Fix (6,9-GB-Bündel raus, Landing
  verlinkt das CDN), meteo-Rename `config/meteo → phi/meteo`, TAP-async-Fix
  (PHASE=RUN + votable/td), `gaia_xp_compiler --source-range`, vo-tap-Seed,
  GAVO-Register, inpop sha256-Log.

## Der vo-tap/uvor-Seed (Markus-Faden)

- `tools/vo-tap`: BSD-3-Clause, Copyright Johannes Tyroller, Repository
  `github.com/ivoa/uvor`, frischer JSON-Parser (kein PolyForm-Abzug),
  README + CLI. Verifiziert gegen GAVO (sync + async, COUNT 219.196.404).
- Crate-Name `vo-tap`, Repo-Name `uvor` (Markus' Unterscheidung). Kein
  Rückverweis auf das omegaflow-Repo im Crate.
- OFFEN: Markus legt `ivoa/uvor` an (Handle `omegaflow`); danach den Crate
  dorthin pushen. Lizenz + Name sind mit Markus geklärt (BSD-3, uvor).

## Beschlüsse (noch nicht ausgeführt)

- TODO auflösen: Git ist das Register, die Übergabe der Mechanismus, eine
  TODO ist doppelte Buchführung. Regel: Session-Ende → Übergabe schreiben →
  die letzte Übergabe ins Archiv.
- Archiv-Ordner: `docs/{handover,auftrag,befund,blatt}/archiv/` (flach).
- Session-Protokoll: Planungsmodus liest das Handover; zweiter Prompt
  „Du kannst. Delegiere an die Taucher (alle Sub-Agenten), höre die Stimmen
  bei Architektur-/Abschluss-Entscheidungen. Eine Session ist ein
  abgeschlossenes Atom."

## Offen für die Folge-Session

1. TODO-Auflösung ausführen (Triage: erledigt → archiv, offen → auftrag,
   Bau-Linie → die-weberin.md), dann die Regel nach AGENTS.md.
2. Die vier Folge-Sessions aus den sieben Übergaben (drei Atome:
   tiefenphasen-Flotte, TE-Atom-4, Membran-Sonde; plus ein konsolidierter
   Follow-up der mechanischen Reste).
3. Markus: uvor-Übergabe abwarten, dann den Crate pushen.
