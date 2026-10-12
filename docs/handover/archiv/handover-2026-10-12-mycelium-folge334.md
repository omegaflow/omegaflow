<!--
  title: Handover — Mycelium-Folge 334 (2026-10-12)
  session: Mycelium-Linie in einem Pass — zwei rote CI-Läufe aus dem Log gemessen (tools-build: browser_relay-Syntaxfehler src/archivar/relay.rs:850; register-coverage: Transport-Gate), tools-map.md auf die eine Spur fortgeschrieben (xfer migrate, kein legacy.log), die adressierten Blöcke gefaltet, Stehender Pass
  class: handover
  date: 2026-10-12
  sha256: b694aaa145788430ad8b5e7418cf4ee0afff782a1251aa94605fb29cd3a8a8c0
  status: live
-->
# Handover — Mycelium-Folge 334 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Der **Stehende Pass** wird zitiert, nie kopiert (`state/zustand/standing-pass.md`;
die Session schreibt ihn als Schlussakt am HEAD neu). Diese Session konsumierte
`handover-2026-10-12-mycelium-folge333.md` (→ `archiv/`).

## Burn: open 0.01 · close 0.04 · cap 0.40 — Grund: Mycelium-Folge 334 in einem Atom — zwei rote Läufe aus dem Log gemessen, `tools-map.md` auf die eine Spur fortgeschrieben, adressierte mountain/river-Blöcke gefaltet (Erledigtes), Stehender Pass; deepseek-flash, kein pro/max (gemessen `session_burn`, line-Bucket).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „die letzte runde schreibt bitte schicke agenten lis und fixe den riss" | 2026-10-12 | Operator (Session 330) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „bitte setze alles um nicht nur klein und begrenzt und halb" | 2026-10-12 | Operator (Session 333) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „ich möchte das umgestellt wird und nicht zweigleisig gefahren wird" | 2026-10-12 | Operator (Session 333) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-12 | Operator (Session 334) · session-weiter Delegations-Consent `/consent`; `/commit` ist das Commit-Wort |
| „kannst du das bitte fixen" (der getragene Riss: PATH-Frische hängt am roten `tools-build`) | 2026-10-12 | Operator (Session 334) · `state/operator-gespraeche/2026-10-12-mycelium.md` |

## Erledigt in diesem Atom (gemessen)

- **`tools-map.md` auf die eine Spur fortgeschrieben** (der Träger-Punkt aus folge333 war noch nicht nachgezogen): Zeile 295 trug weiter „Legacy-Token … `legacy.log`" und listete `migrate`/`manifest` nicht; Zeile 316 ebenso. Jetzt: `xfer mark/list/fold/close/manifest/migrate/coverage`, `migrate --line <L> [--handover <p>] [--write]` ersetzt alte Tokens, „eine Spur, kein `legacy.log`". Header-sha256 nachgezogen (`omega_sh sha docs/concepts/tools-map.md`).
- **Adressierte Blöcke (`mountain-folge333`, `river-folge200`) gefaltet — Erledigtes gemessen:** PDS POS-6 registriert (`phi:33069-33078`, `POS_M1_ENC_42SEC.DAT`/`.LBL`); ISIS-Container gebaut (`isis-env.yml`, Lauf `38280076049` success, `isis 10.0.0`); M10-Ephemeride regeneriert (`38278478615` success, `phi:32031` sha `67949cb6…`); 8-Ziele: PVO-ORSE (`phi:12473-12477`), ssd-jpl-Familie (`phi:32721+`, 48 Blöcke), BC430/P10/P11/Venus-Radar (folge327/328, `phi:32402-32495`); THEMIS-ASI (folge330, `phi:21027`). Die Echos reiten auf der Sender-Umstellung (unten).
- **`src/archivar/relay.rs:850` geheilt — der Riss der PATH-Frische (Operator-Wort 2026-10-12).** `channel_ref: Some(channel_ref),` stand in einem **Tupel**-Pattern (`for &(…) in &records`, `SampleRecord` = 26 × f64) — unter `#[cfg(feature = "browser_relay")]` (`src/archivar/mod.rs:258`) verborgen, darum von `cargo check` (ohne Feature) unsichtbar. Jetzt `channel_ref,` (der Record-Slot ist f64; `channel_ref.to_le_bytes()` schreibt ihn). Der Push auf `src/**` triggert `tools-build` neu; nach grün refresht `bin/.tools_ensure` `register_lookup`/`xfer` aus `tools-latest`.

## Offen — eigen

### tools-map.md — Träger (Werkzeug-Karte)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste `--orphan-docs`-Pass
- **Lage:** (gemessen 2026-10-12/334) `docs/concepts/tools-map.md` ist die Werkzeug-Karte (Mycelium-owned); in diesem Atom auf die eine Spur fortgeschrieben (`xfer migrate`, kein `legacy.log`).
- **Blockade:** keine.
- **Braucht:** bei Werkzeug-Änderungen fortschreiben; sonst ruht der Träger.

### Transport-Echo der adressierten Legacy-Tokens — wartet auf Sender-`xfer migrate`
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** die Sender (mountain/river) haben `xfer migrate --line <L> --write` auf ihre `## An mycelium`-Blöcke angewandt
- **Lage:** (gemessen 2026-10-12/334, `register_lookup --tokens`, Baum-Bin) die eigenen ausgehenden verankerten Tokens `xfer:mycelium.ses_eceb3d78fffeTNxaWJuGfFG211.5~6c4ffa0e` (→ mountain) und `…6~3ed6b902` (→ river) sind `SENT`; die Legacy-Tokens der eingehenden Blöcke (`xfer:mountain329-1/-2/-4`, `xfer:river197-6`, `xfer:future-244-2`) sind mit dem Tool nicht faltbar (`xfer fold` → `token malformed`) — die Sender müssen sie zuerst umstellen.
- **Blockade:** der `xfer migrate`-Pfad erreicht das PATH erst mit dem nächsten grünen `tools-build` (nach dem `relay.rs:850`-Fix, oben erledigt); die Blöcke gehören den Sendern.
- **Braucht:** nach dem Release-Refresh: `xfer migrate --line mountain --write` / `--line river --write` durch die Sender; danach faltet Mycelium die verankerten Tokens (`xfer fold`/`close`).

## An mountain

Origin: mycelium-334 (2026-10-12):

- `xfer:mycelium.ses_eceb3d78fffeTNxaWJuGfFG211.5~6c4ffa0e` **Umstellung auf verankerte Tokens** — die 11 adressierten Einträge aus `mountain-folge331` sind gemessen-erledigt (POS-6 `phi:33069-33078`; SPDF/NTRS/DSC `phi:31656-31693`; 8-Ziele `phi:32827`/`32849`/`32856`/`32863`; M10-Attitude `phi:31558`/`31639`/`31645`; ISIS `38280076049` success; M10-Ephemeride `phi:32028`). Bitte `xfer migrate --line mountain --write` auf den eigenen `## An mycelium`-Block anwenden, sobald das Release frisch ist; danach faltet die Owner-Session.
[manifest n=1 root=3399cb36]

## An river

Origin: mycelium-334 (2026-10-12):

- `xfer:mycelium.ses_eceb3d78fffeTNxaWJuGfFG211.6~3ed6b902` **THEMIS-ASI Keogramm** — Compiler-Asset registriert (`phi:21027`); Lauf `38258632187` **success**. Bitte `xfer migrate --line river --write` auf den eigenen Block anwenden, sobald das Release frisch ist.
[manifest n=1 root=d58815b4]

- **`src/archivar/relay.rs:850` — geheilt (Mycelium, Operator-Wort 2026-10-12).** Der Syntaxfehler (`channel_ref: Some(channel_ref),` in einem Tupel-Pattern) stand unter `#[cfg(feature = "browser_relay")]` und blockte `tools-build`; jetzt `channel_ref,`. Achtung River: `relay.rs` ist dein Pfad — der Eingriff war das ausdrückliche Operator-Wort (der Riss der PATH-Frische); bitte beim nächsten Pass übernehmen/gegenprüfen.

## An sensory

Origin: mycelium-334 (2026-10-12):

- **`sensory-folge255` — vier `## An river`-Einträge ohne Token → `XFER-MISSING`.** Das `register-coverage`-Transport-Gate (`register_lookup --tokens --noecho-fail`) fällt auch auf tokenlosen Transport-Einträgen; die vier Einträge (`docs/concepts/der-paradigmenwechsel.md`, `docs/paper/flyby-path-2-falsification-metric-addendum.md`, `docs/paper/flyby-path-2-preregistration.md`, `docs/auftrag/auftrag-flyby2-kette.md`) tragen keinen `xfer:`-Marker. **Braucht:** je Eintrag `xfer mark --line sensory --to river "…"` und den Token in den Block schreiben, oder die Einträge messen und schließen.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
