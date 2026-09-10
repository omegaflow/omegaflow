<!--
  title: Handover — tap-Klassifikations-Pass (2026-09-10)
  class: handover
  date: 2026-09-10
  sha256: defef3b2f98ae629d59a36359b91075b40b71de18706f3f0dc7f90cb90045207
  status: archived
  see-also: docs/SOURCE_PORT.md docs/handover/handover-2026-09-10-disposition-hygiene.md
-->
# Handover — tap-Klassifikations-Pass (2026-09-10)

Der Pass ist getragen: die 50 Endpoints des tap-Zensus (48 RegTAP-tap +
mast-/cadc-Austrag) sind je Endpoint durch Bestand-Dedupe, Force-Gate und
Schema-Discovery disponiert. 44 Überlebende tragen ihr Tafel-Inventar
(`tap_index_<label>.φ`), zwei Verzeichnisse sind declined, zwei tote Routen
gehen als §8-Recherche in die http-Linie, zwei sind Bestand. Ein Werkzeug-Fix
macht die DaCHS-Schema-Discovery überhaupt erst lesbar.

## Gebaut

- `tools/vo-tap/src/lib.rs::tables` — lief vorher auf jedem DaCHS-Host in
  `tables returned void` (queried den HTML-Root statt `/sync`, las Rows nur
  als Objekte, während Standard-TAP-JSON Array-of-Arrays liefert). Jetzt:
  `sync_candidates` (wie `tap_speaks`, `/sync` vor Root) + `parse_json_rows`
  (beide Formen, Spalten-Case-insensitive). Verifikation: 46/46 clean-tap-
  Endpoints liefern ein parsebares Inventar, 0 void.
- `cargo check -p vo-tap` 0 Errors, 0 Warnings.

## Disponiert (50)

- **Bestand-Dedupe:** mast + cadc sind schon geerntet (`tap_index_mast.φ` 15,
  `tap_index_cadc.φ` 21) → bestand; keiner der 48 RegTAP-Hosts trägt Bestand
  in sources.φ/tap_index.
- **decline registry (2)** → dead_sources.φ: registry.euro-vo.org (RegTAP-
  Service-Verzeichnis, rr.*) und voparis-rr.obspm.fr (rr.* + glots, kein
  Messwert-Feld, nur ivoa.obscore-View). Der euro-vo-Endpoint bleibt die
  RegTAP-Lesepfade (`vo-tap import`).
- **§8-Recherche (2)** → http-Linie: neocc.esa (502), koa.ipac (404) — toter
  Endpoint ist kein Endzustand.
- **Überlebende (44):** Schema-Discovery getragen, je Endpoint
  `tap_index_<label>.φ` in `phi/pipeline/catalog/` (7–486 Tabellen); die
  Ledger-Note trägt Verweis + „Draft pending".

## Register

- Ledger: 50 Noten klassifiziert (44 Draft-pending, 2 decline, 2 §8, 2 bestand).
- dead_sources.φ: +2 decline-registry-Blöcke.
- catalog/: +44 tap-index-Dateien, MANIFEST.φ um die 44 Einträge erweitert.
- docs/SOURCE_PORT.md §16.5: der Pass-Befund.

## Pending

- **Source-Drafts der 44 Überlebenden** — je Endpoint field/force/τ nach §8.
  Die Klassifikation ist getragen (jede Ledger-Note trägt ihren tap-index-
  Verweis); der Draft je Katalog ist die Bau-Linie, kein Regal.
- **http-Klassifikation (67)** — unberührt von diesem Atom: POST-only/CSV-only/
  Redirect, cefca-404-Familie, neocc 502 + koa 404 §8-Recherche (hier nur
  geroutet, nicht erledigt).
- **Speisekammer-Fragen (eigenes Atom)** — aia2014, planck, eve, omni2, goes15,
  gebco-Stub: je Frage eigene Adress-Ernte, keine TAP-Klasse.
- **Proton-VPN-Recheck** — arvo-registry (endgültiges dead) + cadc.argus
  (TLS-Reset), offen.

## Stray

- `tools/service/src/assets/job_dashboard.html` — modifiziert im Working Tree,
  gehört zur supermag-Linie (e87894b); bleibt unangefasst für seinen Besitzer.
