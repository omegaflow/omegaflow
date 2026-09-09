# vo-tap

A TAP client for the Virtual Observatory. Rust `std` only, `curl` for the wire.
One binary, the two TAP submission paths, the response formats.

## What it is

- `query_sync(root, adql, Format)` — posts `REQUEST=doQuery`, returns the raw
  body (`csv`, `json`, `text`, `votable`, `votable/td`).
- `submit_async(root, adql, Format)` — creates a UWS job, returns a `Job` with
  `phase()`, `run()`, `wait()`, `result()`, `delete()`.
- `parse_csv`, `parse_text`, `parse_votable` (TABLEDATA), `parse_json_rows` —
  each returns `(column names, rows)` as strings.
- `tables(root)` — reads `tap_schema.tables`.

## The UWS flow

`REQUEST=doQuery` submitted to `/tap/async` creates the job in `PENDING` on
DaCHS (the GAVO data centre stack, and others). The job runs after an explicit
`POST PHASE=RUN` to `{job}/phase`. This is TAP-1.1. `Job::wait` posts
`PHASE=RUN` when it observes `PENDING`, then polls to `COMPLETED`.

For the async result format, use `Format::VotableTd` (`votable/td`): DaCHS
emits the default `votable` as a `BINARY` table; `votable/td` emits `TABLEDATA`,
which `parse_votable` reads.

## CLI

```
vo-tap sync  <root> <adql> [--format csv|json|text|votable|votable/td]
vo-tap async <root> <adql> [--format …] [--poll N] [--timeout N]
vo-tap tables <root>
```

```
vo-tap sync https://dc.g-vo.org/tap/sync "SELECT TOP 5 * FROM ivoa.obscore" --format csv
vo-tap async https://dc.g-vo.org/tap/sync "SELECT COUNT(*) FROM gdr3spec.spectra" --format votable/td
```

## Measured against

GAVO `dc.g-vo.org`, VizieR `tapvizier`, Gaia ESDC, NED/IPAC, NOIRLab Astro Data
Lab, HEASARC. Anonymous UWS jobs bind to the client's IP address — the machine
that creates a job is the machine that runs it.

## License

BSD-3-Clause (like pyVO). Copyright (c) 2026 Johannes Tyroller.
