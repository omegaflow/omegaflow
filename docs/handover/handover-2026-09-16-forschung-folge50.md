<!--
  title: Handover — Forschung-Folge 50 (2026-09-16)
  class: handover
  date: 2026-09-16
  sha256: ffe77dbe95d53f8d367e303ff9db2917acdf308a4efbb48fae7bb5b16395f7d9
  status: live
-->
# Handover — Forschung-Folge 50 (2026-09-16)

## ODF-Bande-Split — Konsument (undatiert, mechanisch — CI-blockiert)

- Lauf `35139594201` (`planetary-odf-cdn`) ohne Job-Start (gemessen 2026-09-16:
  `jobs total_count = 0`, Run pending/queued, Log nicht verfügbar). (Schritt:
  `gh run view 35139594201 --log`, sobald der Run einen Job trägt — den
  gedruckten φ-Block nehmen und **jeden ganzen 5-Zeilen-Block** in
  `phi/sources.φ:6723–6733` ersetzen; nur `mro_odf` und `odyssey_odf` tragen
  einen Shard-Zweig; `refuse_shard_overlaps` in `src/archivar/parse.rs`
  verweigert Überlappung gleichen `format`.)

## Paper / Präregistrierung (datiert)

- Flyby Path 2 — datiert (JUICE 28./29.09., Clipper 03.12.), schweigt vor dem
  Datum. Gemessen 2026-09-16: `docs/paper/flyby-path-2-preregistration.md` nennt
  nur das *Was* (Felder plasma-pressure gradient, IMF-Bz, Kp, Swarm, RTSW@L1 mit
  L1-Transitzeit) — **kein ausführbares Schritt-Detail** (welcher Kanal über
  welches Tool abgerufen wird). (Schritt: vor dem 28.09. den konkreten
  Abruf-Schritt je Kanal in `docs/paper/flyby-path-2-preregistration.md` setzen.)
