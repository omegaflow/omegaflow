<!--
  title: Handover — Ernte-Folge 70 (Stand 2026-09-17)
  session: Ernte-Folge 70
  class: handover
  date: 2026-09-17
  sha256: ca554bfa7f2d10bc89f18ccf746454f1e97e207204c7fc92f778126c1130167f
  status: live
-->
# Handover — Ernte-Folge 70 (2026-09-17)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht.
Der Planungs-Pass nennt die offenen Punkte als nummerierte Auswahl (der erste ist
der härteste undatiert); die Session arbeitet so viele ab wie möglich.
Wartestellungen (`wartend`) sind kein Auswahlpunkt — sie nennen nur ihren Auslöser
und werden nie als Handlungsschritt geführt. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Harvest-Architektur — Dispatch-Beweis rosetta_odf (härtester undatiert)

Der rosetta_odf-Block (`asset fehlt`, `timeout 350`) steht; der Push `dcbba8ac`
hat `harvest-dispatch` **`35268354408`** getriggert (gemessen `queued` am HEAD).
Er triggert `harvest.yml -f format=rosetta_odf -f timeout=350`; das Gate liest
`asset fehlt` → Compile bis 350 min.

- **Dispatch-Beweis `termin`** — nach Abschluss `ci_manage view 35268354408`
  einmal; bei success den rosetta-Block-note auf `asset present` + Run-Id/Bytes/
  sha256 setzen (`phi/harvest.φ`). Nicht pollen.
- **Ungelaufene Pfade, `pending`** — das Idempotenz-Gate, der `force`-Lauf, der
  `timeout`-Input-Pfad, der zweite Zeuge mit Shard-Vollständigkeit, `--check`
  Feldeindeutigkeit + timeout-Validierung (`harvest_reg.rs`) — erst im ersten
  rosetta-Lauf gemessen.
- **Parameterisierte Familien** — `gedi_l2a`/`icesat2_atl03`/`swot_l2_lr_ssh`
  (`--day`/`--prefix`), `dl3_skymap` (`--telescope`), `juno_ocru_odf` (`--volume`)
  brauchen einen Argument-Punkt im Register; `text`/`volume` sind generische Keys
  mit mehreren Quellen unter einem Format — ein Block je Format braucht eine
  Entscheidung. (Schritt: Argument-Punkt entwerfen — `grind-pro`.)
- **`auto-dispatch`** zuletzt falten (kein Löschen vor Parität).

## Quellen-Routen

- **`magic_dl3` `termin`** — `dl3-skymap-cdn.yml` dispatched 2026-09-17
  (run **`35268770926`**, `queued`; FITS-URL HTTP 200, 328320 B). Schritt: nach
  Abschluss `ci_manage view 35268770926`; bei success `phi/sources.φ:7882`-note
  auf present + Bytes/sha256.
- **gedi/icesat2/swot protected-Bucket-403 `operator-gebunden`** — `research-max`
  gemessen: der EDL-Token wird an den drei `/s3credentials`-Endpunkten akzeptiert
  (HTTP 200); ein SigV4-`ListObjectsV2` gibt je Bucket **403 `AccessDenied` mit
  explizitem Deny auf `s3:ListBucket`** in der identitätsbasierten NGAP-Policy
  (SWOT: `s3_same_region_access_bucket_limit_policy`). Signatur/Region/Host sind
  korrekt (`src/archivar/range.rs:14,15,482,489`) — das Listing ist
  policy-geschlossen, kein Signing-Bug. Die Quellen-notes (`phi/sources.φ:1495,
  6475,6481`) tragen den Befund. Schritt: CMR-Granule → direktes `GetObject`
  bauen, oder Operator-Datenabkommen (SWOT-EULA).
- **`ephemeris_epm` Format-Routing-Lücke** — die zehn EPM-Assets sind CDN-present,
  aber `src/archivar` routet nur `format == "ephemeris_binary"`
  (`extract.rs:1769`, `main_flow.rs:83,113`); `phi/sources.φ` trägt
  `format ephemeris_epm`. Der Parser `parse_ephemeris_binary` (`motion.rs:517`)
  existiert, die Route fehlt. Schritt: EPM-Quellen auf `ephemeris_binary` setzen
  oder eine `ephemeris_epm`-Route ergänzen — `bau`-Linie.
- **`catalog_gaia_sso` ohne cdn-Workflow** — `gaia_sso_tno.bin` (99160 B) ist
  present (created 2026-09-08), aber kein benannter `gaia-sso-cdn.yml` im Baum.
  Schritt: Workflow bauen oder als descoped messen — `grind-pro`.

## Stehender Pass (gemessen 2026-09-17)

- **HEAD** `dcbba8ac`; `origin/main == HEAD` (Fast-Forward, der rosetta-Push steht).
- **CI-Status** — Watchdog-Snapshot 21:21:56+02:00 war älter als der Push
  (20:01:32Z); nachgemessen via `ci_manage list`: `harvest-dispatch` `35268354408`
  queued, `ci-check` `35268354412` pending; Present-Kette aus folge68
  (`harvest-dispatch` 35264838482, `harvest` 35264854799 `-f format=maven_tnf`)
  success. Der CI-Status-Eintrag `docs/zustand/external-state.md` ist durch den
  HEAD-Wechsel fällig — **fremd-modifiziert**, nicht angefasst (besitzende Linie
  faltet).
- **Postfach** — leer; kein neuer externer Eingang (`state/mail/` am Baum absent).
  `docs/zustand/external-state.md` (Postfach-Zeile) fällig bei neuem
  Ledger-Eingang oder 2⁶ min — fremd-modifiziert, nicht angefasst.
- `git_safety --snapshot` → `refs/safety/1789675337`.

## Wartend (kein Auswahlpunkt)

- `planetary-odf-cdn` `35231817955` (rosetta+mro) — bei success wird
  `rosetta_odf.bin` present; dann Block-note auf present.
- `demeter-cdn` `35228716483`; Failed (attempt 1): `swot-cdn` `35251359490`,
  `gedi-cdn` `35250788545`, `ci-check` `35250778775`.

## Benchmark

- `grind-flash` (flash) — 13 `.secrets.local`-Reader gehärtet (exakter Key,
  letzte Zeile), 2 Tests, `cargo check` 0/0; `magic_dl3` dispatched. Routine
  bleibt flash.
- `grind-pro` (pro) — drei `arm≠format`-Blöcke geschrieben + `ned-cdn.yml`
  resumierbar. **A=A-Korrektur:** die Übergabe-Vorgabe „fehlt" war ungemessen;
  `gh release view` misst alle drei Familien **present**, die Blöcke stehen auf
  `asset present` (kein Dispatch). Dieselbe Klasse wie `fermi_4fgl`: die
  Registrierungs-Bedingung UND die Messung entscheiden.
- `research-max` (max) — EDL-Route mehrstufig gemessen (Credentials → SigV4 →
  XML-Körper) und klassifiziert: `AccessDenied` (Policy), operator-gebunden.
  Der harte Route-Atom, max gerechtfertigt.
- Burn (`session_burn`, aggregiert): grind-flash $0.0122, grind-pro $0.2784,
  research-max $0.0492 — flash für Routine, max für den Route-Atom.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `phi/harvest.φ`, `phi/sources.φ`,
  `.github/workflows/ned-cdn.yml`, die 13
  `tools/harvest/src/bin/*.rs` (`.secrets.local`-Reader),
  `docs/handover/handover-2026-09-17-ernte-folge70.md`
  (+ archiviertes `handover-2026-09-17-ernte-folge69.md`).
- **Fremd (nicht anfassen):** `docs/zustand/external-state.md` (modifiziert), die
  gestagten Handover-Archiv-Renames (`handover-2026-09-16-*`),
  `handover-2026-09-17-bau-folge68.md`,
  `handover-2026-09-17-entscheid-folge37.md`,
  `handover-2026-09-17-forschung-folge66.md`; **während dieser Session
  aufgetaucht (fremd, parallel):** `src/archivar/atdf.rs`,
  `src/archivar/extract.rs`, `src/archivar/main_flow.rs`,
  `tools/harvest/src/bin/ulysses_atdf_compiler.rs` (untracked) — nicht anfassen,
  nicht mitcommitten. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
