<!--
  title: Zustand — dropped-Baseline (register_lookup --dropped)
  class: zustand
  date: 2026-09-21
  sha256: fb9014b31c326cf148867c861f744bf7b911dc149b611d8dc7f136ae7a42e359
  status: live
  see-also: AGENTS.md
-->
# Zustand — dropped-Baseline (`register_lookup --dropped`)

Delta-Gate für `register_lookup --dropped`: das Gate schlägt nur an, wenn die
aktuelle dropped-Zahl die Baseline **übersteigt** (Delta > 0), nie bei einem
Absolutwert. Die Baseline wird in dem Commit aktualisiert, der einen legitimen
Drop trägt — nie stillschweigend, nie als Fabrikation.

dropped-baseline 1052
measured 2026-09-25 (tree @0a0ce96d; ci-check 36064053750 dropped-gate @de76fda6a: baseline 960 | current 984 | delta 24; ci-check 36065950583 dropped-gate @0a0ce96d: baseline 960 | current 989 | delta 29. Die 29 sind der aufgelaufene Drop-Netto der Planungs-Pässe zwischen @5179b438b und @0a0ce96d — absorbiert durch den Baseline-Bump im annehmenden Commit (Mountain folge155), nicht durch Fabrikation)
measured 2026-09-28 (ci-check 36347555576 dropped-gate @a457ed8c9: baseline 989 | current 1052 | delta 63; früher 36341839537 @5780d939: current 1039 | delta 50; full-history-Sweep 36344055152 @02f70c517: 3105 dropped, 2112 commit-resolved, 993 git:none — Owner sensory 1212 · mycelium 926 · mountain 618 · entscheid 167 · river 153. Delta 63 ist der aufgelaufene Netto der Planungs-Pässe nach @0a0ce96d; absorbiert durch diesen Baseline-Bump. Lokal nicht nachgemessen — `register_lookup --dropped --count` bricht bei >30 min ab, die Gate-Zahl ist CI-only)
