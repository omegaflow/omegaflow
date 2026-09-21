<!--
  title: Zustand — dropped-Baseline (register_lookup --dropped)
  class: zustand
  date: 2026-09-21
  sha256: 63f13c849fe62c7867a001365a672d3d819f6e85be3e1d784a01115dd827749f
  status: live
  see-also: AGENTS.md
-->
# Zustand — dropped-Baseline (`register_lookup --dropped`)

Delta-Gate für `register_lookup --dropped`: das Gate schlägt nur an, wenn die
aktuelle dropped-Zahl die Baseline **übersteigt** (Delta > 0), nie bei einem
Absolutwert. Die Baseline wird in dem Commit aktualisiert, der einen legitimen
Drop trägt — nie stillschweigend, nie als Fabrikation.

dropped-baseline 2219
commit-resolved 176
pairs 477
candidates 3869
measured 2026-09-21 (Forschung-Folge 137, HEAD 096ed904; delta 61 über 2158 — Baseline im annehmenden Commit nachgezogen)
