<!--
  title: Handover — Ernte-Folge X (Stand 2026-09-12)
  session: Ernte-Folge X
  class: handover
  date: 2026-09-12
  sha256: 30b32dd073a2791da58c22ba5ea352a8c82d360cdfd556e37ca327f33c7dd8ad
  status: live
-->
# Handover — Ernte-Folge X (2026-09-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks; gepusht wird erst, wenn der Baum ruhig ist.

Die nächste Session schließt alle offenen Punkte dieser Linie ab — nicht
weiterschieben. Braucht sie Unterstützung, nimmt sie das Gremium (Rat) oder die
Taucher (Sub-Agenten, eigener Kontext). Fragen, die einen Webzugriff brauchen,
gehen über den opencode-Browser an den Operator.

## CDN-Manifestation (Duty)

- igets.bin — Wächter: Release `igetsftp.gfz.de` liest 404 (gemessen). Laufender
  Dispatch 34716901099 (20:22Z, cd4e963a). Der Vorgänger 34715203286 endet ohne
  Asset: compile(Vienna) scheiterte (exit 1, keine rustc-Annotationen — Script-Ebene),
  merge wird geskippt. Wächter misst die nächste Session.
- harps_rvcat.json — Wächter: Release `ssd.jpl.nasa.gov` liest 404 (304 Assets,
  keins harps). Void-Ursache gemessen (Job-Log 34715204877): `FORMAT=csv` ist dem
  ESO-TAP fremd → HTTP 400. Tabelle `safcat.HARPS_RVCAT_V1` + Spalten
  ra_simbad/dec_simbad/drs_ccf_rvc/plx_simbad mit `FORMAT=votable` verifiziert
  (QUERY_STATUS OK, Zeilen). Fix getragen: Workflow `--csv` → `--votable`;
  Redispatch 34717041472 (20:25Z, cd4e963a). Wächter misst die nächste Session.

## Ernte

- Hi-net — `HINET_PASS` weiter absent (.secrets.local gemessen) — Operator.

## Abschluss

- Baum beim Sessionsstart: HEAD == origin/main == cd4e963a (Refs gemessen). Der
  Baum trägt parallele Sessionsarbeit (uncommitted, fremd — unberührt). Dispatch
  getragen: 34716901099 (igets), 34717041472 (eso-harps). Nur eigene Dateien
  committet (.github/workflows/eso-harps-rvcat-cdn.yml + dieses Handover).
