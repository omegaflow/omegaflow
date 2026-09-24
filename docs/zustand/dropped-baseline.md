<!--
  title: Zustand — dropped-Baseline (register_lookup --dropped)
  class: zustand
  date: 2026-09-21
  sha256: 9fc3b1658596af3d9b6805e7bc7577ec856b2f083dd333ab5c6642a7e918bec6
  status: live
  see-also: AGENTS.md
-->
# Zustand — dropped-Baseline (`register_lookup --dropped`)

Delta-Gate für `register_lookup --dropped`: das Gate schlägt nur an, wenn die
aktuelle dropped-Zahl die Baseline **übersteigt** (Delta > 0), nie bei einem
Absolutwert. Die Baseline wird in dem Commit aktualisiert, der einen legitimen
Drop trägt — nie stillschweigend, nie als Fabrikation.

dropped-baseline 960
measured 2026-09-24 (tree @5179b438b; ci-check 36000930169 dropped-gate: baseline 678 | current 960 | delta 282; register-dropped 36000037355 @15ca40c46: 3340 dropped, 2657 commit-resolved, net 683. Die Differenz 960-683 sind die Handover-Übergänge der Planungs-Pässe zwischen den beiden SHAs. `register_lookup --dropped` zählt 683 `git: none`-Gruppen zurück bis 2026-09-09 — akkumulierte Historie, absorbiert durch den Baseline-Bump im annehmenden Commit, nicht durch Fabrikation)
