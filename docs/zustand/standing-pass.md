<!--
  title: Standpass — öffentliche Projektion
  class: zustand
  date: 2026-10-15
  sha256: 1eef6146808c4debccfdc2627ddc4b2c0f25074479b91f14fea97dfdbb43fac7
  status: live
  see-also: AGENTS.md
-->
# Standpass — öffentliche Projektion (privat↔öffentlich, A)

Die öffentliche Fassung des Stehenden Passes (Operator-Wort A, 2026-10-14). Sie trägt
**nur, was öffentlich ist** — HEAD und rote CI-Läufe. Mailstand, Operator-Akte und
Personenbezug bleiben im privaten Haus (`state/zustand/standing-pass.md`, Linie Mycelium).
Die öffentlichen Linien zitieren **diese** Fassung, nie die private.

| Feld | Wert | gemessen |
|---|---|---|
| HEAD | `ec2f6efde` (== `origin/main`) | 2026-10-15 |
| rote CI-Läufe | **653** non-success — Träger: `ci-gate` (345) · `register-coverage` (184) · `tools-build` (41) · `quake-feeds-cdn` (11) · `health-check` (9) · `matrix-rotor` (8) · `de2-cdn` (8) · `gic-al-network` (5) · `sources-repo-licence` (4) · `m10-mdis-raw-cdn` (4) · `entsoe-cdn` (3) · `dsn-cdn` (3) · `ci-check` (3) · je 2: `ssd-jpl-raw-cdn` · `modvolc-cdn` · `kasi-cdn` · `intermagnet-xyz-cdn` · `hdf5-real-granule` · `clpds-cdn` · je 1: `zenodo-raw-cdn` · `vnp46a3-cdn` · `venus-radar-cdn` · `themis_asi-fodg-cdn` · `swpc-mirror-cdn` · `swpc-efield-cdn` · `sources-refresh` · `ps1-cdn` · `nohrsc_snowfall-cdn` · `nasa-power-t2m-cdn` · `knmi-cdn` · `hips-png-cdn` · `cdaweb-tec-cdn` | 2026-10-15 |

**Der Transport-Grundriss für die 80,9-GB-Basemap steht.** `raw_mirror` splittet den Body
stromweise in `.parts` (statt ihn ganz in den Speicher zu lesen); ein inkrementelles,
std-only md5 trägt das **Quell-Tor** (Zenodo veröffentlicht md5), sha256 bleibt der
Haus-Fingerabdruck je Teil. Der `.parts`-Manifest-Aktor liest jetzt auch die `md5`-Zeile.
Der 80,9-GB-Lauf selbst ist **nicht gefahren** — die Standard-Runner-Platte trägt Temp +
Teile (≈ 162 GB) nicht; der direkte curl→parts-Strom ist der nächste begrenzte Bau.

**Noch unread:** die tiefen Log-Gründe der fremden roten Läufe (kein Polling); die
Push-Läufe am neuen HEAD sind queued. Die Logs liest `ci_manage log <id>`.
