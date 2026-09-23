<!--
  title: Zustand — dropped-Baseline (register_lookup --dropped)
  class: zustand
  date: 2026-09-21
  sha256: ce9e7405b9851154798e01038fc2d5db94ea97598f2e51297b70f3c2b2ac6d5d
  status: live
  see-also: AGENTS.md
-->
# Zustand — dropped-Baseline (`register_lookup --dropped`)

Delta-Gate für `register_lookup --dropped`: das Gate schlägt nur an, wenn die
aktuelle dropped-Zahl die Baseline **übersteigt** (Delta > 0), nie bei einem
Absolutwert. Die Baseline wird in dem Commit aktualisiert, der einen legitimen
Drop trägt — nie stillschweigend, nie als Fabrikation.

dropped-baseline 3053
measured 2026-09-23 (tree @7ded53c; register_lookup --dropped --count = 3053; delta +373 vs 2680; ~95–96 % commit-resolved bzw. umformulierte Unterfeld-Zeilen — kein echter Verlust, gemessen grind-flash; mycelium folge148)
