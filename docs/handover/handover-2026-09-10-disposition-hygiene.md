<!--
  title: Handover — Disposition-Hygiene (2026-09-10)
  class: handover
  date: 2026-09-10
  sha256: ad6597a1622128d26ef8d2c3cae7ffcb99250a201d319aab35aee7e7b5a31a75
  status: live
  see-also: docs/SOURCE_PORT.md docs/handover/handover-2026-09-10-nicht-autonom.md
-->
# Handover — Disposition-Hygiene (2026-09-10)

Die Hygiene der Endpoint-Zensus-Vollwelle ist getragen: die RegTAP-Artefakte
sind aufgelöst, die zwei dead_sources-Rechecks ausgetragen, die vier
kein-http-Endpoints nach Recherche disponiert (zwei Artefakte, ein dead, ein
URL-Korrektur-Fund). Der Import trägt beide Fixe (Charge-Dedupe +
relativer-access_url-Skip).

## Gebaut

- `vo-tap import --regtap` — (a) relative `access_url` (kein Host) wird als
  Artefakt übersprungen statt als Kandidat geschrieben; (b) Dedupe gegen die
  eigene Charge (derselbe URL aus zwei Interface-Zeilen → ein Block).
  Verifikation gegen die live RegTAP: `0 candidates after Bestand-Dedupe
  (2 relative access_url, 0 batch duplicates skipped)`.
- `cargo check -p vo-tap` 0 Errors, 0 Warnings.

## Disponiert

- Ledger: die zwei relativen `/tap`-Blöcke (ivoid `ivo://ovgso/tap`) getragen
  — der RegTAP-Record trägt keine absolute `access_url`, kein Kandidat; das
  Geschwister `ivo://ov-gso/climso` lebt (`https://epntap.climso.ovgso.fr/tap`).
  Die zwei `https://dachs.oca.eu/tap`-Doppel auf einen Block verschmolzen.
- dead_sources-Austrag: `mast.stsci.edu/vo-tap/api/v0.1/caom/sync` ausgetragen
  und als gewogener Kandidat in den Ledger zurück (HTTP 400 + TAP-JSON, lebt,
  braucht Query); `tapvizier.u-strasbg.fr/TAPVizieR/tap/sync` → `decline
  superseded-by-integrated` (400- und 501-Befund als Zeitpunkte desselben
  Hosts in einem Eintrag).
- kein-http: `arvo-registry.sci.am/tap` → `dead unreachable` (Peer-Reset nach
  Anfrage, DNS ok, Proton-VPN-Route offen). Der CADC-ARGUS war kein Tod: die
  RegTAP-`access_url` ist `/argus` ohne `/sync`, der TAP liegt unter
  `/argus/sync` und ist in `tap_index_cadc.φ` (21 Tabellen, --votable-Ernte
  2026-08-20) schon geerntet — als Kandidat mit korrigierter URL zurück im
  Ledger (TLS-Reset 2026-09-10, Proton-VPN offen).
- `docs/SOURCE_PORT.md` §16.4: der Hygiene-Befund.

## Register

- Ledger-Bestand nach Hygiene: 48 tap + 1 mast-Austrag + 1 cadc · 67 http,
  kein kein-http offen.

## Pending

- **tap-Klassifikations-Pass** — die 48 tap + der mast- und cadc-Austrag: je
  Endpoint zuerst Bestand-Dedupe gegen sources.φ/tap_index-Kataloge +
  Force-Gate-Vorentscheid; nur Überlebende in Schema-Discovery (`vo-tap
  tables`) + Draft.
- **http-Klassifikation (67, Wieger-Limit)** — POST-only/CSV-only/Redirect
  re-proben; die cefca-404er-Familie, `neocc.esa` 502 und `koa.ipac` 404 als
  §8-Recherche (toter Endpoint ist kein Endzustand).
- **Speisekammer-Fragen** (eigenes Atom) — aia2014, planck, eve, omni2, goes15,
  gebco-Stub: je Frage eigene Adress-Ernte, keine TAP-Klasse.
- **Proton-VPN-Recheck** — arvo-registry (endgültiges dead) + cadc.argus
  (TLS-Reset) — offene Eskalation in den Noten.
- **vo-tap/uvor Crate pushen** — Operator-Schritt (handover-nicht-autonom).

## Stray

- `tools/service/src/assets/job_dashboard.html` ist modifiziert im Working
  Tree — gehört zur supermag-Linie (e87894b), nicht zu diesem Atom; bleibt
  unangefasst für seinen Besitzer.
