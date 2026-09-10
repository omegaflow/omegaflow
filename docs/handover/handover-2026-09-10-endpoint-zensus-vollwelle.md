<!--
  title: Handover — Endpoint-Zensus Vollwelle (2026-09-10)
  class: handover
  date: 2026-09-10
  sha256: acb7d0798ea02959dcee27b83dd5ba510bc2af6503d48e0f266a829065d2192b
  status: live
  see-also: docs/SOURCE_PORT.md docs/handover/handover-2026-09-10-nicht-autonom.md
-->
# Handover — Endpoint-Zensus Vollwelle (2026-09-10)

Die Vollwelle ist gewogen: alle 120 RegTAP-Kandidaten tragen den Befund in der
Ledger-Note. Der `wave`-Modus steht; zwei dead_sources-Notizen sind recheckt
und korrigiert.

## Gebaut

- `vo-tap wave --ledger <pfad> [--probe-timeout 60] [--pause 2]` — liest die
  `ausstehend kandidat`-Blöcke, wiegt in Host-Fruchtfolge (derselbe Host nie
  zweimal direkt), schreibt den Befund atomar in die Ledger-Note zurück
  (`gewogen <datum>: http <code> probe <probe>`); Status bleibt `ausstehend`.
  Idempotent: nur Notizen mit `ungewogen` werden gewogen. `--probe-timeout`
  trennt die Wiege-Probe (60 s) von der Ernte-Probe (`query_sync` 300 s).
- `cargo check -p vo-tap` 0 Errors, 0 Warnings.

## Gemessen (Vollwelle, 2026-09-10)

Verteilung der 120 Kandidaten: **49 tap · 67 http · 4 kein-http**.
Zwei RegTAP-Artefakte, getragen (nicht geflickt): zwei `/tap`-Blöcke (relative
`access_url`, ivoid `ivo://ovgso/tap` — kein Host, wiegt `kein-http`) und zwei
Doppel-Blöcke `https://dachs.oca.eu/tap` (ivoid `ivo://purx/dachs/tap` zweimal
— RegTAP liefert zwei Interface-Zeilen, der Import dedupliziert nicht gegen die
eigene Charge; `known_hosts_and_urls` dedupliziert nur gegen den Bestand).

## Rechecks (dead_sources korrigiert)

- `mast.stsci.edu/vo-tap/api/v0.1/caom/sync` — 400 + TAP-JSON (lebt); die
  404-nginx-Messung (2026-09-06) war ein Zeitpunkt. Notiz korrigiert.
- `tapvizier.u-strasbg.fr/TAPVizieR/tap/sync` — 400 + TAP-JSON (lebt); die
  Browser-Historie-Diagnose war falsch. Notiz korrigiert.
- Der Austrag der beiden aus dead_sources + der Port gehören ins
  Dispositions-Atom.

## Register

- `phi/pipeline/ledger.φ`: 120 Kandidaten-Notizen `ungewogen` → `gewogen`
  (Befund je Endpoint).
- `docs/SOURCE_PORT.md` §16.3: Vollwelle + Verteilung + die zwei Artefakte.

## Pending

- **Dispositions-Atom** — die 49 `tap`-Endpoints (lebt, TAP spricht) in
  Port-Drafts überführen (Force-Gate, Fruchtfolge der Register); die 67
  `http`-Endpoints (POST-only/CSV-only/Redirect) nach dem Wieger-Limit
  klassifizieren; die 4 `kein-http` → dead_sources. Die zwei RegTAP-Artefakte
  (`/tap` relativ, `dachs.oca.eu` doppelt) dabei auflösen.
- **Wieger-Limit (benannt, kein Fehler):** die Probe ist GET-only + JSON-only;
  `http` ist kein Tod.
- **Speisekammer-Fragen** (aia2014, planck, eve, omni2, goes15, gebco-Stub):
  keine TAP-Klasse — je Frage eigene Adress-Ernte.
- **vo-tap/uvor Crate pushen** — Operator-/Markus-Schritt
  (handover-2026-09-10-nicht-autonom).
