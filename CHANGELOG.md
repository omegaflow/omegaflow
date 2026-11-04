# Changelog

Format follows Keep a Changelog 1.1, versions follow SemVer 2.0.0.

## [Unreleased]

### Added

- `omegaflow-membrane` — the field as its own package: the omega loop, the two
  GPU workers, the radiators and their two probes. The core depends on no tool;
  `wgpu` lives there and nowhere else.
- `ci_matrix_scan` — every workspace member needs a `nextest` step, no `--bin`
  filter.
- `test_shape_scan` — no `allow`s, no tests that only check existence.

### Changed

- The CI matrix tests `measure`, `science`, `service`, `app` and `harvest`
  completely; before, 320 probes and most harvest binaries never ran.
- The clippy job lints with `cache-targets: false`; a cache hit used to report
  green without compiling.
- `eeglab::eeg_from_bin` refuses f64 samples instead of narrowing them to f32
  without a word, as `brainvision` does.

### Fixed

- `decode_etopo1` no longer reads the body radius from the CDN; it is a
  parameter. The decoder depended on a private credential.
