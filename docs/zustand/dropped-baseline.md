<!--
  title: Zustand — dropped-Baseline (register_lookup --dropped)
  class: zustand
  date: 2026-09-21
  sha256: c820c851e3acf673c4c04ffa5bf29ac3d822fc99ee94f5747af84939099ffbb0
  status: live
  see-also: AGENTS.md
-->
# Zustand — dropped-Baseline (`register_lookup --dropped`)

Delta-Gate für `register_lookup --dropped`: das Gate schlägt nur an, wenn die
aktuelle dropped-Zahl die Baseline **übersteigt** (Delta > 0), nie bei einem
Absolutwert. Die Baseline wird in dem Commit aktualisiert, der einen legitimen
Drop trägt — nie stillschweigend, nie als Fabrikation.

dropped-baseline 989
measured 2026-09-25 (tree @0a0ce96d; ci-check 36064053750 dropped-gate @de76fda6a: baseline 960 | current 984 | delta 24; ci-check 36065950583 dropped-gate @0a0ce96d: baseline 960 | current 989 | delta 29. Die 29 sind der aufgelaufene Drop-Netto der Planungs-Pässe zwischen @5179b438b und @0a0ce96d — absorbiert durch den Baseline-Bump im annehmenden Commit (Mountain folge155), nicht durch Fabrikation)
