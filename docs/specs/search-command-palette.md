<!--
  title: Command Palette Terminal
  class: concept
  sha256: 1b544bf7c4a118a5a87fd7352c0c48d64357db2e3cd203dc51a87573b226baa2
-->
STATUS: DESCOPED (2026-09-12) — kein autonomer Bauweg: das Substrat (`load_sources` + SIMBAD) lebt, aber der Wirt fehlt (kein Client). Geht in den Client-Wirt auf.

# Command Palette Terminal

## Concept

⌘K fuzzy search for the block universe.

Three namespaces in one input:

```
┌──────────────────────────────────────┐
│ █ m31                                │
├──────────────────────────────────────┤
│ ★ M31 / Andromeda         ra 10.68   │
│   dec 41.27 → jump presence          │
│                                      │
│ □ gaia_dr3        cone 0.5°          │
│ □ mast_caom        cone 0.1°          │
│ □ simbad          resolve            │
│                                      │
│ ○ oac_astrocats   target M31         │
│ ○ jpl_sbdb        target M31         │
└──────────────────────────────────────┘
```

## Mechanics

| Action | Input | Result |
|--------|-------|--------|
| Object search | SIMBAD TAP `main_id LIKE '%query%'` | presence jump to ra/dec |
| Source search | local source index from `phi/sources.φ` | focus/activate source |
| Force search | filter by force type | show matching sources |

## Implementation

- **Fuse.js** or similar for fuzzy matching
- **SIMBAD TAP** for object name → coordinates
- **Source index** served by the relay as `/sources` (name + body from the loaded register — one truth, no client copy)
- **Keyboard**: ⌘K or Ctrl+Shift+P

## Phases

1. SIMBAD-only object search → presence jump
2. Static source name list → autocomplete
3. Fuzzy matching + force-type filter
