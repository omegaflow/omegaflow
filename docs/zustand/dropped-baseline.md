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

dropped-baseline 678
measured 2026-09-23 (tree @48ae5e734; register_lookup --dropped: 3186 dropped, 2508 commit-resolved, N = Z − R = 678; run 35922188109; mycelium folge148-Drops aus note-drop/φ-Commits, kein echter Verlust)
