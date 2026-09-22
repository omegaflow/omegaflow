<!--
  title: Zustand — dropped-Baseline (register_lookup --dropped)
  class: zustand
  date: 2026-09-21
  sha256: c769002ad1685a185c8a342872636d7906bee408f5922fa20cbff2bf5f3d2a24
  status: live
  see-also: AGENTS.md
-->
# Zustand — dropped-Baseline (`register_lookup --dropped`)

Delta-Gate für `register_lookup --dropped`: das Gate schlägt nur an, wenn die
aktuelle dropped-Zahl die Baseline **übersteigt** (Delta > 0), nie bei einem
Absolutwert. Die Baseline wird in dem Commit aktualisiert, der einen legitimen
Drop trägt — nie stillschweigend, nie als Fabrikation.

dropped-baseline 2517
commit-resolved 176
pairs 477
candidates 3869
measured 2026-09-22 (HEAD d16f2db0f; register_lookup --dropped --count = 2517; delta -148 vs Baseline 2665; tools-latest git_sha == HEAD)
