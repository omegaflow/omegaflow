<!--
  title: Handover — Mountain-Folge 189 (Stand 2026-09-27)
  session: Mountain-Folge 189
  class: handover
  date: 2026-09-27
  sha256: 5380a5f0c0d25df749092d812e604d1f0b2912c6bd9142b976beeffdf3c2e3fe
  status: live
-->
# Handover — Mountain-Folge 189 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Der Stehende Pass wird zitiert, nie kopiert
(`state/zustand/standing-pass.md`).

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„ja aus dem register aufstellen" — die offene Liste streng aus dem Register, nicht aus der Vorgänger-Tafel | 2026-09-27 | Operator (Session, Mountain 187)
„erst messen" — pySPEDAS und die Kandidaten vor jedem Verdikt messen | 2026-09-27 | Operator (Session, Mountain 187)
„Listen zur Entscheidung dienen nicht — ich brauche Erklärungen" | 2026-09-27 | Operator (Session, Mountain 187)
„das war dein Vorgänger der wohl Mist gebaut hat" | 2026-09-27 | Operator (Session, Mountain 187)
„Du kannst. Führe den Plan aus — als line-Agent" | 2026-09-27 | Operator (Session, Mountain 187)
„Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt). Delegiere an die Taucher (alle Sub-Agenten), höre die Stimmen bei Architektur-/Abschluss-Entscheidungen. Eine Session ist ein abgeschlossenes Atom. Dispatch flash-first. Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort — Commit und Push trägt `/commit`." | 2026-09-27 | Operator (Mountain-Session 189)

## Offen (aufgeschlüsselt)

### ESA-CCI-SST — Leseseite funktional verifizieren
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `esacci-sst-cdn`-Lauf `36346238365` beendet → `esacci_sst.bin`
  manifestiert.
- **Lage:** (gemessen 2026-09-27, dieser Atom) Reader-Arme gebaut:
  `src/archivar/geo.rs:42` `MAGIC_ESACCI_SST = *b"ESS1"`, `:204`
  `COMP_ESACCI_SST = 1`, `:264` `magic_of`, `:309` `comp_max`;
  `src/archivar/main_flow.rs:3784` Dispatch; `src/archivar/extract.rs:727-730`
  Feldname `esacci_sst_l4_cdr3`; der Compiler
  (`tools/harvest/src/bin/esacci_sst_compiler.rs`) konsumiert die gemeinsamen
  Konstanten (keine Doppeldefinition). `cargo check` 0/0. Test
  `esacci_sst_geo_series_roundtrip_and_component_name` (`src/archivar/tests.rs`)
  übt `magic_of`/`comp_max`/`write_bin`/`parse_bin`/`geo_series_component_name`
  am Format — sein Lauf ist CI (lokal nicht ausführbar); die funktionale
  Lese-Verifikation des CDN-Bins steht aus (kein headless Reader-Bin in
  `tools/`).
- **Blockade:** keine.
- **Braucht:** `ci_manage log 36346224407` (führt den neuen Test) einmal lesen;
  nach `esacci-sst-cdn` `36346238365` den manifesten Bin-Stand prüfen.

### CI-check — archivar-Runde verifizieren
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check`-Lauf `36346224407` (HEAD `94ccbccf3`, enthält
  `b4106e69a`) beendet.
- **Lage:** (gemessen 2026-09-27 via `ci_manage view 36345426609`) der
  `b4106e69a`-eigene `ci-check` wurde gecancelt (0 Jobs, Push-Überholung durch
  `4783bbf5`); kein nicht-cancelled Lauf auf `b4106e69a`. Der nächste
  einschließende Lauf ist `36346224407` (pending).
- **Blockade:** keine.
- **Braucht:** sobald beendet `ci_manage log 36346224407` einmal lesen; grün →
  schließen; rot → Testname + erste Fehlerzeile heilen.

### NED ByParams — Token-Kanal
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** NED-Cook-Token (`NED_BYPARAMS_TIMEOUT_TOKEN`) trifft per Mail ein.
- **Lage:** (gemessen 2026-09-27 via
  `sread tools/harvest/src/bin/ned_byparams_compiler.rs`)
  `X-NED-Timeout-Token` nur am Form-POST (`http_post_form`, Z. 116-123);
  `poll_ticket`/`fetch_body` tragen keinen Header; `.secrets.local` und
  `state/mail/mail_ledger.φ` ohne Token (Postfach misst: kein NED-Eintrag).
- **Blockade:** Token fehlt.
- **Braucht:** mit dem Token den echten ByParams-Job fahren; scheitern Poll/Fetch,
  Token an `http_get`/`fetch_body` ergänzen (erst messen, dann ergänzen).

### ttl-Verdikt-Zeile (Rat 2026-09-27) + no-cadence-Sprachloch
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der fremde `AGENTS.md`-Hunk ist committet.
- **Lage:** (gemessen 2026-09-27 via `sgrep`/`git show`) Rat: `ttl` in
  `phi/sources.φ` ist eine Verdikt-Zeile → Mountain; `derive_ttl`
  (`src/archivar/port.rs:1237`) liefert für Binär-SPK `None` — die Datei-Zeile
  trägt; alle 81 `format ephemeris_binary`-Blöcke tragen `ttl 86400` (HEAD,
  gemessen). Flush-Gate `src/archivar/parse.rs:80`: `ttl 0` = inaktiv — kein
  eigener „no-cadence"-Zustand.
- **Blockade:** `AGENTS.md` trägt fremde uncommittete Hunks (pfad-eigener Commit
  würde fremde Arbeit sweepen).
- **Braucht:** die Präzisierung „ttl = Verdikt-Zeile" in `AGENTS.md` nachtragen,
  sobald der fremde Hunk committet ist; das no-cadence-Sprachloch als `pending`
  registrieren.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
