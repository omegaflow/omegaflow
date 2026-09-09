<!--
  title: Befund — MiniSEED-Dopplung konsolidiert: laic_probe und mseed_measure lesen jetzt den kanonischen decode_body (kein Parser-Gap gemessen)
  class: befund
  date: 2026-09-09
  sha256: 6de080ebe58bf85209b553a6b9e411c848c105323f8245c968495b936146db52
  status: done
  see-also: docs/handover/archiv/handover-2026-09-09-zonen-flotte.md docs/befund/befund-2026-09-09-zonen-flotte.md
-->
# Befund: MiniSEED-Dopplung konsolidiert

## Frage & Bindung

Registerzeile des thematischen Handovers: „MiniSEED-Dopplung — `laic_probe.rs` trägt einen eigenen STEIM2/miniSEED-Parser parallel zu `tools/measure/src/miniseed.rs`; Konsolidierung `pending`." Der Zonen-Lauf trug mehr „no decodable record"-Skips (AU/S1/GE/PS) — die Dopplung war ein gemessener Engpass, kein bloßer Registereintrag.

## Was gebaut wurde

- `tools/measure/src/bin/laic_probe.rs`: der lokale Parser (`mseed_time`, `sign_extend`, `steim_decode`, `mseed_samples`, `parse_mseed`, 270 Zeilen) ist gelöscht; `harvest_mseed` liest jetzt `omegaflow_measure::miniseed::decode_body`.
- `tools/measure/src/bin/mseed_measure.rs`: dieselbe byte-idente Kopie (253 Zeilen, Zeilen 17–265) ist gelöscht; das Bin nutzt `decode_body`. `ymd_to_days` bleibt — ein echter Julian-Datum-Helfer des Bins selbst, kein Parser-Klon.
- Der kanonische `decode_body` geht die echte Blockette-Kette aus dem Header; die lokalen Kopien nahmen Blockette 1000 an festem Offset an.

## Die Messung

Kein Parser-Gap gemessen: das einzige Real-Record-Fixture (`/tmp/opencode/mseed_test.mseed`) ist abwesend, der Test läuft als benannter Skip („mseed test skipped … absent"), nie als fabrizierter Pass. Der kanonische Parser war bereits der, den der Fleet nutzte — die „no decodable record"-Skips (AU/S1/GE/PS) sind **Datenweg-Lücken**, kein Parser-Gap (0 geehrt: die Lücke bleibt Lücke, keine erfundene Tiefe).

## Verifikation

`cargo check -p omegaflow-measure` — null Fehler, null Warnungen. `cargo test -p omegaflow-measure --bin laic_probe` — 11 pass, 0 fail (der Real-Record-Test als benannter Skip).

## Verdikt

Die Dopplung ist gehoben: ein Parser, zwei Konsumenten mehr. Der gemessene Engpass („no decodable record" bei AU/S1/GE/PS) ist ein Datenweg, kein Parser — er bleibt als Datenlücke benannt, nicht als Dopplung.
