# Changelog

Format nach Keep a Changelog 1.1, Versionen nach SemVer 2.0.0.

## [Unreleased]

### Added

- `omegaflow-membrane` — das Feld als eigenes Paket: ω-Loop, die zwei
  GPU-Arbeiter, die Radiatoren und ihre zwei Sonden. Der Kern hängt an keinem
  Werkzeug mehr; `wgpu` liegt nur dort.
- `ci_matrix_scan` — jedes Workspace-Mitglied braucht einen `nextest`-Schritt,
  kein `--bin`-Filter.
- `test_shape_scan` — keine `allow`s, keine Tests, die nur die Existenz prüfen.

### Changed

- Die CI-Matrix testet `measure`, `gate`, `science`, `service`, `app` und
  `harvest` vollständig; vorher liefen 320 Sonden und 507 der 513
  Harvest-Binaries nie.
- Der Clippy-Job lintet mit `cache-targets: false` — ein Cache-Treffer meldete
  vorher „grün", ohne zu übersetzen.
- `eeglab::eeg_from_bin` weist f64-Proben ab, statt sie still nach f32 zu
  verengen (wie `brainvision`).

### Fixed

- `decode_etopo1` holt den Körperradius nicht mehr aus dem CDN; er ist ein
  Parameter. Der Decoder war von einem privaten Credential abhängig.
