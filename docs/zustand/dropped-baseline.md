<!--
  title: Zustand — dropped-Baseline (register_lookup --dropped)
  class: zustand
  date: 2026-09-21
  sha256: 96864c5c90930902c742c3331a70607eaa1ae0e33af47eddf722394858c4918b
  status: live
  see-also: AGENTS.md
-->
# Zustand — dropped-Baseline (`register_lookup --dropped`)

Delta-Gate für `register_lookup --dropped`: das Gate schlägt nur an, wenn die
aktuelle dropped-Zahl die Baseline **übersteigt** (Delta > 0), nie bei einem
Absolutwert. Die Baseline wird in dem Commit aktualisiert, der einen legitimen
Drop trägt — nie stillschweigend, nie als Fabrikation.

dropped-baseline 2286
commit-resolved 176
pairs 477
candidates 3869
measured 2026-09-21 (Forschung-Folge 138, HEAD 379ca31b; delta 67 über 2219 — Baseline im annehmenden Commit nachgezogen)
