# omegaflow · ωφ

**A kybernetic field system — a free line in the 4D ICRS block.**

A = A.

Time is a coordinate: omegaflow rests as a free line in the 4D block (ICRS/J2000), with no past,
present, or future privileged. A std-only Rust core (the *Archivar*) loads stars and bodies from
catalogued ephemerides into that frame. A WebGPU/WGSL compute pipeline (the *Mathematikerin*)
evaluates the field. A binary φ(x,y,z,t) protocol (26 × f64 per record) carries it between them.
The membrane samples the block as a 2D surface (constant t, z) and renders the point cloud in the
browser — WebGPU/WGSL and vanilla JavaScript, not Rust. Sensors and open sources feed the same frame.

```
sources (APIs, ephemerides, connected sensors, CDN) ⇌ Archivar (Rust, std-only) ⇌ Mathematikerin (WebGPU WGSL) ⇌ silicon membranes
```

A measured zero is a value; an absent value stays absent and renders black. No default fills a
physical gap — and that discipline is enforced in code, not in prose: a commit gate refuses
unbacked state claims and speculation vocabulary (`src/gate/`).

![omegaflow — the sun](docs/assets/omegaflow_sun.png)

<sub>Motion: [`docs/assets/omegaflow_sun.mp4`](docs/assets/omegaflow_sun.mp4) · [omegaflow.space](https://omegaflow.space)</sub>

## Run it

```
cargo run                          # the membrane (ESC closes it)
cargo run --features browser_relay # + WebSocket on 127.0.0.1:1618, the browser sensor
```

Or open [omegaflow.space/membrane.html](https://omegaflow.space/membrane.html) in a WebGPU browser.

## What is in it

- **ICRS block universe** — ephemeris-driven bodies, force media, a lookup by enclosure and
  motion laws; the presence is a free line at rest, the operator tunes to the coordinate.
- **Transfer-entropy lens** — a probe with phase-randomized surrogate nulls; a broken null stays
  broken and visible ([`docs/paper/broken-null-control.md`](docs/paper/broken-null-control.md)).
- **A source commons** — sha256-pinned sources and a self-carrying port protocol
  ([`docs/SOURCE_PORT.md`](docs/SOURCE_PORT.md)).
- **The membrane** — the point cloud and the browser sensor ([`static/membrane.html`](static/membrane.html)).

## Documentation

- [`docs/concepts/archivar-mathematikerin.md`](docs/concepts/archivar-mathematikerin.md) — the wire/GPU/force data contract
- [`docs/concepts/kybernetische-astrophysik.md`](docs/concepts/kybernetische-astrophysik.md) — the physics
- [`docs/concepts/kybernaut-native-methodology.md`](docs/concepts/kybernaut-native-methodology.md) — the method
- [`docs/paper/`](docs/paper/) — preregistrations and results

## License

Dual — see NOTICE:
- Code (`src/`): PolyForm Noncommercial 1.0.0 with ShareAlike — see [LICENSE](LICENSE)
- Documentation, prose, working data (`docs/`, prose): Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International — see [docs/LICENSE](docs/LICENSE)

## Support

omegaflow is a personal, unfunded, non-commercial research project. If it is
useful to you and you would like to support it — no obligation, no deliverable:

- GitHub Sponsors: https://github.com/sponsors/omegaflow
- (Open Collective / Patreon — see [`.github/FUNDING.yml`](.github/FUNDING.yml) once active)
