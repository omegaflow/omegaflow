# Mycelium — Kuprat: die vier Kanäle sind belegt (2026-10-19)

**Linie:** mycelium · **Datum:** 2026-10-19 · **HEAD vor dem Pass:** `17f84a7f8`
**Gemessen, nicht bewertet.** Anlass: `post/in/2026-10-18-sensory-an-mycelium-kuprat.md`
(Token `.19`), `post/in/2026-10-18-future-an-mycelium-kuprat.md`.

## 0. Die Abnahme-Zeile des Dispatch ist ein Artefakt

Der Dispatch verlangt:

```sh
for t in 7286412 15179114 EELS SRD62; do
  bin/register_lookup "$t" | grep -q 'live.*sha256' || { echo "unvollstaendig: $t"; exit 1; }
done
```

Gemessen: **alle vier** melden `FEHLT`. Ursache ist nicht der Block, sondern das
Instrument — `register_lookup` indexiert `sha256` nicht als Schlüssel
(`tools/register/src/bin/register_lookup.rs` führt keine `sha256`-Zeile), also kann
`grep 'live.*sha256'` nie treffen. Dasselbe Muster wie der AIA-Doppelgänger: die
Prüfung misst den Prüfer.

## 1. Die Blöcke tragen Compiler und sha256 (4/4)

| Kanal | Block | Compiler | sha256 (im Block) |
|---|---|---|---|
| RIXS spin | `phi/sources.φ:14217` | `crystal_compiler.rs` | `e4b3335d38666344389417c11b322119efad819d27f1fb7f27eab6ac2f992b54` |
| RIXS charge | `phi/sources.φ:14207` | `crystal_compiler.rs` | `3846da527aa21caf9b7b4cf6de8189fb9d03211685709bf5d07e30e8fd4e4038` |
| EELS | `phi/sources.φ:14195` | `crystal_compiler.rs` | `eaee21811b21b9b9f11ba98b33fd573f17858cd0b24a100e56643b4966842ec0` |
| SRD62 | `phi/sources.φ:31278` | `srd62_compiler.rs` | `7a001d65192acd5a790b33a0d4a7ff03ad91ecb815b950d05b2263d3a7d53672` |

`bin/register_lookup --compilers` bindet `crystal_compiler` an `rixs_spin`, `rixs_charge`
und `eels_acoustic`; `srd62_compiler` an den SRD62-Block. Die zweite Dispatch-Bedingung
ist damit **erfüllt** (nicht `rixs_cuprate_probe`, aber `srd62_compiler` ist der genannte
Alternativ-Zweig).

## 2. Die drei CDN-Assets sind echt (geladen, gehasht)

| Asset | HTTP | Bytes | sha256 (gemessen) | = Block |
|---|---|---|---|---|
| `crystallography.net/rixs_spin.bin` | 200 | 15 025 | `e4b3335d38666344…` | ✓ |
| `crystallography.net/rixs_charge.bin` | 200 | 5 080 689 | `3846da527aa21caf…` | ✓ |
| `crystallography.net/eels_acoustic.bin` | 200 | 481 834 | `eaee21811b21b9b9…` | ✓ |

## 3. SRD62 ist bewusst nicht auf dem CDN

Der Block trägt `terms no-license` (`nist.gov/srd/public-law`); nach der
CDN-Regel (`upload_release`: *no-license-stated … stay off the CDN*) bleibt die `url`
der Portal-Endpunkt. Gemessen 2026-10-19: `…/Hts/DoSearch?Properties=44` = HTTP 200,
32 452 B HTML, sha256 `5361ccb2dcafdc46…` — **nicht** der Block-Hash `7a001d65…`.
Der Block-Hash adressiert den Compiler-Ausgang, nicht den URL-Body; das ist bei
`no-license` die einzige mögliche Form.

## 4. Ergebnis

- **Kuprat ist materiell geschlossen:** 4/4 Kanäle mit `url`/`origin`/`compiler`/`sha256`,
  drei davon am CDN byte-verifiziert.
- **Offen:** nur die Abnahme-Zeile (Instrument). Ersatzprüfung, die den Block misst:

```sh
awk '/^url /{u=$2} /^sha256 /{print u" "$2}' phi/sources.φ \
  | grep -E 'rixs_spin|rixs_charge|eels_acoustic|srdata.nist.gov' | wc -l   # 4
```

- **Nicht getan:** `witness kuprat` — der Vertrag ist vier Zeugenklassen
  (`witnesses.φ:1`), die Kanäle sind `witness substance` (`witnesses.φ:120-142`).
  Kein neuer Zeuge wird erfunden.
