<!--
  title: Drift-Übergabe — die neue Leitstelle übernimmt
  class: handover
  date: 2026-09-03
  status: live
  see-also: TODO.md
-->

# Übergabe — Drift gemessen

Der Automat hat Drift gemessen und die Session eingefroren.

## Grund

limit phrase in model output

## Gemessen

the model spoke its own context loss

## Letzte Commits

```
4aebef5 register: cdn_reconcile — Registry↔CDN-Abgleich (sources.φ vs omegaflow/sources) als maschinenlesbare Tabelle; Orphan/Divergenz/missing/byte-Dupe-Klassen (406 Quellen, 200 Releases gemessen)
c1a5547 gate+ci: export_latex trägt mehrzeilige Blockzitate vollständig (Zahlen fallen nicht mehr weg); Paper-Kopf-shas abgeglichen (gyirong auf Standard-Kopf gehoben, kollab/sturzflut/causal-arrow aktualisiert, kollab-Titel gekürzt); health-check pages-verify checkt das Repo aus (Meldung vorher No-such-file)
c77b364 paper+ci: Kleinpass-Verdrahtung — corona 'steepest (5.47→5.57)' als formuliert widerlegt (größter log-T-Sprung 977→1032 Δ0.63, 1032→131 Δ0.10, aus Papier-Tabelle nachgemessen); neue CDN-Workflows lead-geometry-cdn.yml + signal-cone-audit-cdn.yml persistieren die Probe-Verdicts (lead_geometry_verdict.txt, signal_cone_audit_verdict.log) re-derivierbar auf den CDN-Releases
5afb77c auftrag: saubere Datenbank (sources.φ / CI / CDN-Assets) — registry-first, nie letzte Kopie, schrittweise Abnahme
a5fad95 gate+regel: CDN-Manifestation als Session-Duty — Granit-Grundsatz 7 (Manifestiert, nicht nur lokal) + AGENTS-Regel unter Source Curation (Register-Schuld statt Haken)

```

## Uncommittet im Baum

```
.github/workflows/flyby-odf-cdn.yml
docs/specs/cdn-ziel-schema.md
tools/measure/src/bin/odf_census_probe.rs
```

## Offene Punkte

Das Register TODO.md ist die Wahrheit — die neue Session liest es zuerst.
