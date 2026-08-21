# Handover: Offene Atome — Sample-Budget, Fetch-Sturm, GLADE+/NED/2MASS, grüner Lauf

Registriert 2026-08-21. Die nächste Session liest genau dieses eine Dokument und
beginnt. Selbsttragend — interpretierbar mit null Vorkontext. Der Auftrag ist
nicht die Ausführung; ausgeführt wird erst auf das Wort des Operators.

## Best Practice — die vier offenen Atome

Kurz: vier Posten blieben aus früheren Sessions offen und sind im Register
(TODO.md) benannt. Dieses Dokument ist ihr Auftrag — die Reihenfolge ist Teil
des Auftrags, weil die Atome sich bedingen: erst der stille Lauf, dann die
Messung, dann die großen Kataloge.

### 1. Sample-Budget-Messung (Vorbedingung für GLADE+)

- **Messung:** wie viele der ~9–10 Mio Katalogzeilen überleben die Kappe.
  `MAX_SAMPLES = 1 << 22` = 4 194 304 (`src/archivar.rs:9038`); der Rebuild
  sortiert epoch-absteigend und wirft die ältesten (`archivar.rs:15543`) —
  epoch 0.0 = die Katalogzeilen, sie sind die ersten Verlierer.
- **Mechanik:** Membran im ω-Loop. Die Zeile
  `sample cap … reached — … dropped (newest kept)` ist die Wahrheit; ohne sie
  bleibt die Zahl ungemessen (keine Fabrikation).
- **Blocker (gemessen):** der volle Membran-Lauf sättigt die Heimleitung
  (Atom 2); zwei Läufe brachen vor dem Katalog-Load ab.
- **Weg:** `OMEGAFLOW_HIDDEN=1` (Fenster + acoustic/seismic sind seit Atom 11
  still), `/tmp/omegaflow_catalog_dr3_stars.bin` + `dastcom_asteroids.bin`
  vorbefüllen (kein Download), Lauf begrenzen/drosseln (Atom 2), die cap-Zeile
  greppen.

### 2. ω-Loop-Fetch-Sturm-Reparatur

- **Befund:** der Live-Source-Zyklus fischt ~200 Quellen kontinuierlich —
  4 Retries × 23 s + ttl/Φ-Backoff — ein unbegrenzter Churn, der die Leitung
  bei jedem Membran-Lauf sättigt (Neustart erzwungen, 2026-08-21).
- **Reparatur:** Retry-Exponent (2ⁿ), Pause je Quelle, begrenztes
  Fetch-Budget je Tick, Cooldown toter Quellen.
- **Reihenfolge:** Atom 2 vor Atom 1 abschließen, damit die Messung still läuft.

### 3. GLADE+ / NED / 2MASS

- **GLADE+** (`VII/291/gladep`, 22 M; Spalten RAJ2000/DEJ2000/Bmag/zhelio/zcmb/
  dL[Mpc] live verifiziert): drei Blocker — Schrittboden-Kappung des
  `--mag-bands`-Banders (~180 k Zeilen je 0.25-mag-Band), ~2.4 GB JSON über dem
  2-GB-Release-Limit, 22 M über MAX_SAMPLES. Weg: RA-Slices/async +
  Quadranten-Assets + Budget-Entscheidung (Atom 1 zuerst).
- **NED** (Root `https://ned.ipac.caltech.edu/tap/sync`, Tabelle
  `NEDTAP.objdir`, Spalten ra/dec/z/prefname/type_key verifiziert): sync-COUNT
  läuft in den 60-s-Timeout (Server: async) → async-Slice-Counts messen, dann
  RA-Slice-Chunk-Schritt.
- **2MASS** (`II/246/out`, 470 M; sync-COUNT > 60 s gemessen): Bulk-Kompilator-
  Atom (cdsarc-ftp) — eigenes Atom, kein Fake-Subset.

### 4. Ein voller grüner chunk_catalogs-Lauf

- `kernel_flatten.yml` → Job `chunk_catalogs`: RAVE (24 RA-Slices à 15°,
  rv-Gate HRV) + pastel/wds/mktypes/denis (RA-Slices). Alle fünf Assets auf dem
  CDN verifizieren (rave_dr5/denis/mktypes/pastel/wds).
- **Stand:** zwei Läufe wurden extern abgebrochen; die Kompilate wurden lokal
  nachgeholt und liegen valide auf dem CDN — der volle grüne Lauf fehlt noch.

## Einstieg

```bash
cd /home/johannes/projects/omegaflow
git status                        # sauber oder fremde Arbeit nennen
grep -n 'sample cap\|MAX_SAMPLES' src/archivar.rs   # die Kappe
grep -n 'OMEGAFLOW_HIDDEN' src/mathematikerin.rs    # der stille Schalter
```

Referenzen (stehend): `TODO.md` (alle vier Posten sind dort benannt),
`docs/surveys/chunk-plan-2026-08-20.md` (GLADE+-Blocker + Pfeiler-Matrix),
`docs/surveys/fischplan-kataloge-2026-08-20.md` (Tabellen/Spalten),
`phi/pipeline/ledger.φ` (Kompilat-Zählungen).

## Gates

- Atom 2 (still) → Atom 1 (Messung) → Atom 3 (GLADE+ erst nach Budget).
- cargo check 0/0; Register: TODO.md + ledger.φ + dieses Dokument.
- Ein Commit je Atom; das letzte schließt die Posten und archiviert dieses
  Dokument nach `/home/johannes/projects/archive/handover/`.

## Nicht anfassen

Die kebab-case-Umbenennung von `concepts/` (läuft in der Parallel-Session),
`reference/`-Drittanbieter, bare UPPER_SNAKE-Konzeptnamen in Prosa.
