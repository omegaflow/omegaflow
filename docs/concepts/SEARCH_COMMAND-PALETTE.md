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

Client-side only. No server changes.

- **Fuse.js** or similar for fuzzy matching
- **SIMBAD TAP** for object name → coordinates
- **Source index** built from existing `load_sources()` → served as JSON endpoint
- **Keyboard**: ⌘K or Ctrl+Shift+P

## Phases

1. SIMBAD-only object search → presence jump
2. Static source name list → autocomplete
3. Fuzzy matching + force-type filter
