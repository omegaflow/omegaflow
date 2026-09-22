<!--
  title: Zustand — dropped-Baseline (register_lookup --dropped)
  class: zustand
  date: 2026-09-21
  sha256: 796f630846f52a1be98b309674797542bb641221b20af01da7a4176d272a1c19
  status: live
  see-also: AGENTS.md
-->
# Zustand — dropped-Baseline (`register_lookup --dropped`)

Delta-Gate für `register_lookup --dropped`: das Gate schlägt nur an, wenn die
aktuelle dropped-Zahl die Baseline **übersteigt** (Delta > 0), nie bei einem
Absolutwert. Die Baseline wird in dem Commit aktualisiert, der einen legitimen
Drop trägt — nie stillschweigend, nie als Fabrikation.

dropped-baseline 2680
measured 2026-09-23 (tree @eea5867e1 + own folge138; register_lookup --dropped --count = 2680; delta +40 vs 2640; attribution mountain 135→136 13 (HDF5-Test-Rewrite + ci-check-Routing, resolved), mountain 136→137 (HDF5 CI-grün geschlossen), mountain 137→138 (archive restructure, Punkte getragen), sensory 148→150 restructure; points carried)
