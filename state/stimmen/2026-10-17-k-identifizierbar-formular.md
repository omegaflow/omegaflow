# Rat-Frage R-2026-10-17-j — Ist `K` mit der 1981–83-WDC-Reihe identifizierbar?

**Die Frage in einem Satz:** Ändert die neu geerntete Boden-/Index-Reihe (AE/AL/AU/AO, Dst, Kp) den Befund, dass `K` aus den DE-2-Daten **nicht identifizierbar** ist — oder bleibt die Impulsbilanz-Lücke bestimmend?

## Was der Rat am 2026-10-17 bereits entschieden hat (R-d)

- TE misst **Abhängigkeit, nicht Kraft**.
- `K` ist aus `de2_ua16s_all.csv` **nicht identifizierbar**: kein `T`, kein `∇p`, kein `B/j`, Ein-Satellit.
- Nächster gebundener Schritt: **bedingte TE `TE(NVE→IVE | Z)`** gegen gemessene Treiberproxies.

## Was neu ist (gemessen 2026-10-17)

- **Die 1981–83-WDC-Reihe liegt vor**, für genau das DE-2-Fenster: Lauf `39024807968` — AE/AL/AU/AO 1 min, Dst 1 h, Kp 3 h; `wdc_ae.bin` **1 634 952 records, 93 MB**.
- **Dst 13 512 rows (0 Lücken), AE 810 720 rows (0 fill), 13 Observatorien** definitiv (`wdc.kugi.kyoto-u.ac.jp/hapi`; keyless).
- **Die DE-2-Größen bleiben:** `de2_ua16s_all.csv` 64 143 643 B, sha `c128daa8…`; Größen `u_i, u_n, n_i, n_N2` — **kein `T`, kein `B/j`**.

## Die Fragen

1. Ist `K` mit der Boden-/Index-Reihe **identifizierbar** — oder bleibt die Impulsbilanz-Lücke (∇p, B/j, Ein-Satellit undurchquerbar) bestimmend?
2. Ist der gebundene Schritt jetzt **bedingte TE `(NVE→IVE | AE/Kp/Dst/LT/Höhe)`** — und bleibt das ein **Indikator** (kein `K`, keine Kraft)?
3. Oder ist die ehrliche Register-Zeile **`K = pending`** (Ernte da, Wert nicht) — und der TE-Indikator ein **eigener** Punkt?

## Die Optionen

- **A** — `K` bleibt nicht identifizierbar; die WDC-Reihe stützt nur die **bedingte TE** als Indikator. Register: `K = pending`, Indikator eigener Punkt.
- **B** — Die Index-Reihe macht `K` **teil-identifizierbar** (z. B. als Regressions-/Korrelationsmaß gegen AE/Kp) — dann braucht es eine benannte Definition, was `K` dann *ist*.
- **C** — `K` fällt ganz aus dem Register (kein Punkt, keine Zeile); die Frage lebt nur als Messdesign.

Bitte: **Verdikt + der stärkste Riss** (insbesondere: ist „bedingte TE" noch Physik oder schon Statistik?). Keine Umsetzung — die ist Folge, nicht Frage.
