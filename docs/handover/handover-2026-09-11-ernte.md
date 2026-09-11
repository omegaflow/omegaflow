<!--
  title: Handover — Ernte & Register (Stand 2026-09-11)
  class: handover
  date: 2026-09-11
  sha256: 1308701a989136ed6c5e8f0622ab2d045c2257a94c63e55f67b7bbd1d237fe97
  status: live
-->
# Handover — Ernte & Register (2026-09-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab, wie sie kann — Sub-Agenten tragen eigenen Kontext, die Anzahl
ist kein Aufwand. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## Ernte (Harvest)

- ESO-TAP — Ernte + Duty (kein Register-Eintrag).
- Discovery-Download-Workflow (Korpora).
- Pioneer-10-ATDF-Restbestand — 238 Dateien / 77 Tage ernten (SPDF
  `ATDF_Data-Files_CMarkwardt_Readable`).
- IGETS/GGP — Vollernte läuft (igets-cdn.yml, run 34571077129); Asset
  `igets.bin` am CDN verifizieren. Compiler `igets_compiler` gebaut, Katalog
  `gfz_igets_catalog.φ`.

## CDN-Manifestation (Duty)

- jup365 — kernel-flatten bodies-Job bricht weiter (8× failure). Gemessen
  2026-09-11 (run 34551994683): 103/103 Downloads, dann „The operation was
  canceled" im Flatten-Schritt (9m39s), nach den großen Satelliten-Kernels
  (jup365/ura184/nep097/plu060). Ursache (Memory vs. Disk) ungemessen — lokale
  Reproduktion oder df/mem-Probe vor dem Flatten. Wächter offen.
- Dispatched 2026-09-11: supermag-magstid, noaa-ghcn/gsod/isd-allstations,
  noaa-dcdb, noaa-keo-papa, copernicus-icoads/cuon — Asset-Landung am CDN
  verifizieren.

## TAP-Klassifikation (Rest)

- 12 TAP-Source-Drafts — Spalten-Namen Best-Effort unverifiziert; Verifikation
  gegen `tap_schema.columns` je Endpoint, dann sources.φ (Verweis
  `phi/pipeline/research/agent_output/tap_klassifikation_2026-09-11.φ`,
  Abschnitt A „Draft").
- 26 live-with-query — Query-Probe (FORMAT=json) → Klassifikation.

## Parser-Pending (Compiler gebaut, Granularität offen)

- AMS-02 — Endpoint pending (Site 200, Fluss-Tabellen-URL unverifiziert);
  field-Block erst nach URL-Probe.
- noaa-ccor — Render-Pfad für Skalar-Frame-Bin (MAGIC_CCOR) pending; Compiler
  steht.
- noaa-jpss — Produkt wählen (VIIRS-SDR oder OMPS-SDR), dann Hdf5File-Reader.

## Register

- bucket_litmus auf weitere Inventare (Copernicus u. a.).
- Step-5-Folge — destruktiver Schnitt (verifiziert).
- R2 — Archiv-Zählung (archive-root + lokales Backup) als Grundwahrheit in
  `number_audit.rs` verdrahten.
- docs-reference-verteilung — Bewegung + Referenz-Rewiring, Seeds → Survey-Heimat.
- Probe-Einheit-Autoableitung.
- Korpora-Verdikt cmr + dataone messen (CMR aggregiert Partner-Metadaten;
  dataone terms-Seite broken).

## Speisekammer / Nachlese

- Speisekammer-Fragen — aia2014, planck, eve, omni2, goes15, gebco.
- Proton-VPN-Recheck — arvo-registry (endgültiges dead), cadc.argus (TLS-Reset).
