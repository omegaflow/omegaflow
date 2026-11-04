# Documentation

Four places, nothing else.

- `specs/` — the contracts and the baselines the CDN workflows read.
- `reference/` — external references kept as text: NIST SP 330 and SP 811, the
  ephemeris source index, the Mariner chain, the search API registration.
- `state/` — the publish log of the CDN.
- `LICENSE` — the documentation licence.

The code documents itself: `src/` carries each contract in a `*_CONVENTION`
constant beside the code that holds it, and `tools/` carries the readers.

A relative link in this tree must resolve. `.github/workflows/docs-links.yml`
fails the run when one does not.
