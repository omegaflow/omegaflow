<!--
  title: Handover — Uranus-Zentrum-Kopplung: Bau-Linie (a) gebaut (uranu_j gegen das Zentrum, Wobble 21.653 m) + die CI-Fixkette (drei Fehler, drei Fixes, eigener Workflow uranus-c-spk-cdn.yml)
  class: handover
  date: 2026-09-09
  sha256: 22a4e4cbbdaf80cebbbe1d9dfa908b1dab496d1cacd6eea7b7bc8f909d6dc013
  status: archived
  see-also: docs/befund/befund-2026-09-09-uranus-zentrum-kopplung.md docs/handover/handover-2026-09-09-uranus-riss-kontur.md docs/TODO.md
-->

# Handover — Uranus-Zentrum-Kopplung

Übergabe für die nächste Sitzung. Die Nacht hat Bau-Linie (a) gebaut und
gemessen — die Uranus-Astrometrie koppelt gegen das Planetenzentrum, die
Wobble wird Information — und die kernel-flatten-Kette auf drei
aufeinanderfolgende Fehler hin fixiert, bis `ephemeris_uranus_c.bin` auf dem
CDN stand. Die Komposition lebt jetzt in einem eigenen Workflow
(`uranus-c-spk-cdn.yml`, ~5 min) statt hinter dem 322k-Datei-Index-Crawl.

## 0. Die abgebende Sitzung ist abgeschlossen

Commits der Sitzung: `70f2414` (Commit-Step gitignore-Fix), `89b8df8`
(Push-Retry + continue-on-error), `7d0064d` (Bau-Linie (a) — `--center`),
`9db7a27` (Register-Schärfung (b)), `45621b7` (selbsttragender
uranus-c-spk-Schritt), `02bf4f4` (eigener Workflow, Schritt aus
kernel-flatten entfernt). CDN-Kriterium verifiziert:
`ephemeris_uranus_c.bin` → HTTP 200, 4.909.264 B (2026-09-09).

## 1. Bau-Linie (a) — gebaut und gemessen

Der Diurnal-Probe (`tools/measure/src/bin/uranus_diurnal_decomposition_probe.rs`)
trägt `--center`: die Planeten-Weltlinie komponiert zentrums-verankert
(DE441 = `ephemeris_uranus_c.bin` direkt; INPOP/EPM = Baryzentrum + 799−7 aus
ura111). Die Planeten-Tabelle (uranu_j, 3516 Positionen) reduziert gegen das
Zentrum:

| Linie    | c_par        | c_aber       | RMS roh → reduziert (mas) | c0 (ΔRA·cosδ, ΔDec, mas) |
|----------|--------------|--------------|---------------------------|--------------------------|
| de441    | 0.97 ± 0.01  | −0.11 ± 0.02 | 249.5 → 73.0              | (−14.2, −24.5)           |
| inpop19a | 0.96 ± 0.01  | −0.08 ± 0.02 | 226.8 → 69.5              | (+2.0, −46.1)            |
| epm2021  | 0.95 ± 0.01  | −0.07 ± 0.02 | 233.8 → 69.3              | (+10.5, +1.1)            |

- Die Wobble (Zentrum − Baryzentrum, DE441) ist jetzt ein getragener,
  gemessener Vektor: **21.653 m Mittel / 42.627 m max** — die Mond-Massen.
  Ohne die gestrige Nyquist-Korrektur (104-m-Kernel) wäre sie Meßrauschen
  geblieben; die 104-m-Präzision von gestern trägt die 42,6-km-Wobble von
  heute.
- Der Riß am Zentrum (27.0/35.5/47.9 mas) ist Skalen-konsistent mit dem
  Mond-Riß (27.0/31.5/47.3 nach der Reduktion).
- Kalibrier-Gate hält (injizierte Parallaxe + Aberration exakt
  zurückgewonnen: c_par 1.00, c_aber 1.00, RMS → 0.0). Die gestrigen
  Mond-Zahlen reproduzieren sich byte-identisch.
- Befund: `docs/befund/befund-2026-09-09-uranus-zentrum-kopplung.md`.

## 2. Die CI-Kette — drei Fehler, drei Fixes

- **Fehler 1:** „Commit index" addete `phi/sources_index.φ` — die Datei ist
  per A=A-Kuratierungs-Verdikt in `.gitignore` abgeleitet (Z. 39–44) → `git
  add` schlug fehl → bodies geskippt. Fix `70f2414`: nur noch
  `docs/reference/KERNEL_INDEX.md` wird committet; der obsolete
  „Commit body manifest" fiel.
- **Fehler 2:** `remote: fatal error in commit_refs` — das Push-Race zweier
  Sessions auf main. Fix `89b8df8`: 5 Push-Retries + `continue-on-error`,
  damit ein Commit-Race den bodies-Upload nie wieder überspringt.
- **Fehler 3:** `--uranus-c-spk: ephemeris_uranus.bin reads void` — der
  Flatten erzeugt still nichts (zwei gekoppelte Ursachen: `download_missing`
  verwirft jeden Download mit Index-Größe 0; das planets-System wählt die
  höchste DE-Nummer — seit Juli 2026 de442). Fix `45621b7`: der Schritt holt
  das DE441-Baryzentrum selbst vom CDN. Dann `02bf4f4`: die Komposition lebt
  im eigenen `uranus-c-spk-cdn.yml` (wie `de44-cdn.yml`), der Schritt ist aus
  kernel-flatten entfernt.
- **Die zwei Flatten-Bugs** trägt die Parallel-Session inzwischen selbst im
  Register („de442 size 0" + „de441 Range-Request"). Das gekoppelte Atom
  bleibt: die DE-Version ist zu pinnen, bevor der Größe-0-Check geschlossen
  wird — sonst schiebt der Flatten de442 auf das CDN.

## 3. Der Register-Stand — verdict (ii) hat seinen Grund gewechselt

Nach (a) liegt das reduzierte RMS auf Riß-Skala (69–73 mas gegen den
27–48-mas-Riß), aber die drei reduzierten RMS sind fast identisch — die
Linien bleiben ununterscheidbar. Verdict (ii) bleibt; sein Grund wechselte
von „zu unscharf" zu „scharf genug, trotzdem ununterscheidbar". Die
Schlichtung läuft nicht übers Gesamtmittel (dort sind alle gleich), sondern
über die per-Punkt-Struktur unter dem Rauschen — das ist Bau-Linie (b), der
Schlüssel für den Übergang (ii)→(i).

## 4. Offen (pending)

- (b) Versionen-Differenz als per-Punkt-Größe — der Schlüssel nach (a).
- (c) Neptun als zweiter Planet desselben Baus (`--neptune-c-spk`, Spiegel
  von `--uranus-c-spk`; `nep097xl-899.bsp` liegt im Index).
- Die Wobble-Periode (1,4-d-Mond-Signatur) als physikalische Form der Wobble
  separat prüfen.
- Das gekoppelte Flatten-Atom (DE-Version pinnen vor dem Größe-0-Check).

## 5. Arbeitsregeln für die bauende Sitzung

- Geteiltes Repo: die Parallel-Session committet weiter (sie hat inzwischen
  selbst `de44-cdn.yml`, „fail loudly on 0-written run" und mehrere Auszüge
  aus kernel-flatten gebaut). Nur eigene Dateien stagen; eigene Commits über
  einen separaten Worktree auf `origin/main` bringen; `.git/index.lock`
  kurz abwarten.
- Commit-Gate: `cargo check` 0 Fehler/0 Warnungen; `commit_check`.
- Der index-Job von kernel-flatten zählt bei jedem Dispatch 322k Dateien
  (~18 min) — frische Ernten gehören in eigene Workflows, nie in
  kernel-flatten.
