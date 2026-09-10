<!--
  title: Handover — Endpoint-Zensus: Wieger gebaut, RegTAP-Import (149 TAP-Dienste), Pilot gewogen
  class: handover
  date: 2026-09-10
  sha256: c312a0145e2307d9b6dfbe7b88800c17eafeba5f3fbb95cf4f9822411f4e272a
  status: live
  see-also: docs/SOURCE_PORT.md docs/handover/handover-2026-09-10-autonom.md docs/handover/handover-2026-09-10-nicht-autonom.md
-->
# Handover — Endpoint-Zensus (2026-09-10)

Der Endpoint-Zensus („Adreßbuch statt Schnüffeln", Rat-Verdikt 2026-09-10) ist
gebaut und der Pilot gewogen. Zwei neue Modi in `tools/vo-tap`; die fünf
CDN-Tore stehen als SOURCE_PORT.md §16.

## Gebaut

- `vo-tap census <url> …` — der Wieger: je Endpoint HTTP-Code (nackter GET),
  Antwortzeit (`time_total`), Redirect-Ziel (`final_url`), Datum und die
  Inhaltsprobe (`tap` = minimale DoQuery antwortet TAP-JSON, `http` = HTTP
  antwortet ohne TAP, `kein-http` = keine HTTP-Antwort). Ein 400 auf einen
  TAP-Root ist „lebt, braucht eine Query" — die Inhaltsprobe ist der Puls.
- `vo-tap import --regtap <root> [--ledger <pfad>]` — erntet die RegTAP-Liste
  der TAP-Dienste: COUNT(*)-Roundtrip, Bestand-Dedupe je Host + URL, trägt
  Entdecktes als `ausstehend kandidat <url>` in den Ledger.
- `cargo check -p vo-tap` 0 Errors, 0 Warnings.

## Gemessen (Pilot, 2026-09-10 — die Vollerhebung des Eigenen, 10 Endpoints)

| Endpoint | http | probe | Befund |
|---|---|---|---|
| tapvizier.cds.unistra.fr/TAPVizieR/tap/sync | 400 | tap | lebt |
| tapvizier.u-strasbg.fr/TAPVizieR/tap/sync | 400 | tap | lebt — alter Host antwortet; dead_sources-Notiz rechecken |
| ned.ipac.caltech.edu/tap/sync | 400 | tap | lebt |
| exoplanetarchive.ipac.caltech.edu/TAP/sync | 400 | tap | lebt |
| irsa.ipac.caltech.edu/TAP/sync | 200 | http | lebt; Inhaltsprobe GET-JSON ohne TAP-JSON (CSV/VOTable-Route) |
| mast.stsci.edu/portal/api/tap/sync | 200 | http | Redirect auf ErrorMsg.aspx; POST-only — Register-Notiz bestätigt |
| mast.stsci.edu/vo-tap/api/v0.1/caom/sync | 400 | tap | TAP spricht — Divergenz zur dead_sources-Notiz („404 nginx", 2026-09-06); Recheck pending |
| datalab.noirlab.edu/tap/sync | 200 | http | lebt; CSV-antwortend (footprints-Notiz bestätigt) |
| cdms.astro.uni-koeln.de/vamdc/tap/rest/VAMDC-TAP/sync | 404 | http | tot (dead_sources bestätigt) |
| reg.g-vo.org/tap/sync | 000 | kein-http | https = TLS-Mismatch (curl 60); http spricht TAP (149 Dienste) |

## Register

- `phi/pipeline/ledger.φ`: +120 `ausstehend kandidat`-Zeilen (149 RegTAP-Zeilen,
  29 Bestand-Dedupe, COUNT(*)-Roundtrip 149=149). ivoid in der note.
- `docs/SOURCE_PORT.md` §16: die fünf CDN-Tore + der Endpoint-Zensus +
  RegTAP-Messung.

## Pending

- **Vollwelle** — der Zensus über die 120 RegTAP-Kandidaten als Fruchtfolge/
  Myzel-Welle mit Pause; eigenes Folge-Atom. Pilot-Verdikt (zweigeteilt): der
  Modus wiegt korrekt, die Verteilung des Eigenen hat diese Form; die Welle
  wird aus ihr abgeleitet, nicht aus 10 gefolgert.
- **Wieger-Limit (benannt, kein Fehler):** die Inhaltsprobe ist GET-only +
  JSON-only. POST-only (MAST) und CSV/VOTable-only (IRSA, datalab) Endpoints
  fallen als `http`, nicht `tap` — `http` ist kein Tod, der nackte Code zählt.
- **MAST vo-tap-Recheck** — Divergenz „404 nginx" (2026-09-06) vs „tap spricht"
  (2026-09-10) am selben Pfad.
- **Speisekammer-Fragen** (aia2014, planck, eve, omni2, goes15, gebco-Stub):
  keine VO-TAP-Klasse — der RegTAP-Import adressiert sie nicht; je Frage eine
  eigene Adress-Ernte (offen).
- **vo-tap/uvor Crate pushen** — bleibt Operator-/Markus-Schritt
  (handover-2026-09-10-nicht-autonom).
