<!--
  title: Zustand — dropped-Baseline (register_lookup --dropped)
  class: zustand
  date: 2026-09-21
  sha256: 994bfb0a5604eea2e6775c76a3b023139bae9e266cc0b9ca5f8a224f55ede22c
  status: live
  see-also: AGENTS.md
-->
# Zustand — dropped-Baseline (`register_lookup --dropped`)

Delta-Gate für `register_lookup --dropped`: das Gate schlägt nur an, wenn die
aktuelle dropped-Zahl die Baseline **übersteigt** (Delta > 0), nie bei einem
Absolutwert. Die Baseline wird in dem Commit aktualisiert, der einen legitimen
Drop trägt — nie stillschweigend, nie als Fabrikation.

dropped-baseline 2640
measured 2026-09-22 (tree @cd460f1ad + own folge148; register_lookup --dropped --count = 2640; delta +123 vs 2517; attribution mycelium 138→139 41, sensory 146→147 18, mountain 133→134 10, mountain 134→135 8, + 147→148 restructure; points carried)
