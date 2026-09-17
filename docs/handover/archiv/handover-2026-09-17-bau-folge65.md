<!--
  title: Handover — Bau-Folge 65 (Stand 2026-09-17)
  session: Bau-Folge 65
  class: handover
  date: 2026-09-17
  sha256: 4f33139a090943511bb8ee6fbda34dda9ecb1342be61615cdc00bcf530d355d6
  status: live
-->
# Handover — Bau-Folge 65 (2026-09-17)

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
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt.

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung.

## Stehender Pass (2026-09-17, HEAD 84489abb)

- Postfach: **neuer externer Eingang** — Thomas Keller (TRISP/MLZ/FRM-II) antwortet
  auf die NSE/Haug-Anfrage (Wiebke Lohstroh weitergeleitet): er sendet die
  I(q,t)-Daten „in einigen Tagen". Letzte `state/mail/mail_ledger.φ`-Zeile
  `1789650050`. Die übrigen Sonden-Anfragen (Rubin, Sotgiu) unverändert. Der
  `docs/zustand/external-state.md`-Postfach-Eintrag trägt den Eingang noch nicht —
  die Datei ist fremd-uncommittet, nicht angefasst.
- CI am HEAD `84489abb`: die drei Mirror-Workflows wurden per Push-Auto-Dispatch
  gestartet — vires-hapi `35242487205` success, swpc-mirror `35242483603` success,
  quake-feeds `35241740107` success (Folger `35242480713`). Watchdog-Snapshot
  17:05 trägt die übrigen aktiven (demeter/ps1/physionet/gaia-xp/planetary-odf);
  failed pii-exposure `35230489744`, paper-check `35228278279`/`35224760511`.
- Zustand-Ledger: `docs/zustand/external-state.md` trägt weiter fremde
  uncommittete Zeilen (Postfach/PII/CI @ `bc9d6a0b`) — nicht überschrieben.

## Fast-ttl-Mirror — verifiziert und die Positions-Kante entschieden

- **Verifiziert:** alle 25 Assets present (7 vires-hapi + 9 swpc-mirror + 9
  quake-feeds). Anomalie: `api.geonet.org.nz/quake-MMI-4.json` fehlte nach Lauf
  `35241740107` (dessen Log „mirrored 9, voids 0" meldete — der behauptete Upload
  materialisierte nicht), der Folger `35242480713` erzeugte es um 16:00:32Z. Der
  quake-Assetname wechselt täglich (`{week_ago}`/`{today}`).
- **Positions-Klasse:** `{lat}`/`{lon}` sind konstant `0.000000` (Frame-Anker
  `on earth 0 0 0`, `phi/sources.φ:98`), nicht die Presence; instabil ist nur
  `{hour_ago}` (minuten-granular). Ein Mirror kann für diese Route nie greifen.
  Gebaut: das Quell-Direktiv `live` in `src/archivar/types.rs`/`parse.rs`/
  `fetch.rs`/`main_flow.rs`; Gate-Tests `test_parse_live_directive_marks_live_only`
  + `test_live_only_source_skips_the_cdn_fallback_when_live_voids`.

## PDF-Bild-Extraktor (Gremium-Votum A)

- Gebaut: `pdf_images()` in `tools/utils/src/bin/archive_search/pdf.rs` +
  Modus `archive_search --pdf-image <file|url> [--out <dir>]`. `DCTDecode`→`.jpg`
  (Magic `FF D8`…`FF D9`), `FlateDecode`-Raster→PNG (stored DEFLATE + CRC32),
  `JPX` signaturgeprüft; `CCITT`/`JBIG2`/`LZW` pending. 0 honored: fehlende
  `/Width`/`/Height`/`/ColorSpace` → skip. Tests in `pdf.rs`; `tools-map.md`
  nachgezogen (PDF→Bild→`vision`).

## Offen

- **`phi/sources.φ` — die `live`-Zeile** steht im Arbeitsbaum (Z. 98), aber die
  Datei trägt einen **fremden uncommitteten Delta** (uscrn/noe4/cosmic_ro/isc
  sha256 + maven/mariner-Blöcke) → nicht mitcommittet. (Schritt: beim nächsten
  sauberen Stand die eine Zeile mitnehmen; bis dahin ist das `live`-Direktiv
  gebaut, aber von keiner Quelle benutzt.)
- **`live_markers()` des Mirror-Compilers ist inkonsistent zur Laufzeit-Render:**
  `{hour_ago}`/`{now}` als `hour_str` (Sekunden + `Z`) gegen `render.rs` (Minuten,
  kein `Z`); `{lat}`/`{lon}` hartcodiert `29.5`/`-95.0` (Houston). Betrifft
  SWPC/quake-Mirror, die `{now}` tragen. (Schritt: prüfen, welche registrierten
  Mirror-URLs `{now}` tragen; Treffer → eigener Atom.)
- **`auto-dispatch.yml`** (aus `post.md`, An bau/ernte): koppelt „Workflow-Code
  geändert" an „Daten neu ernten"; `cancel-in-progress: false` stapelt Folger
  (gaia-xp-full 3 queued, physionet 3 Läufe), ps1 feuert stündlich per Cron; 87
  Workflow-Dateien uncommittet. (Schritt: Dispatch an „Asset fehlt" koppeln oder
  `cancel-in-progress: true` für die idempotenten Harvests.)
- **Linien-Kommandos:** nur `start.md` nennt `omega_sh sha`/`git_safety --close`;
  `bau.md`/`ernte.md`/`forschung.md`/`entscheid.md` noch nicht. (Schritt: die
  Phase-2-Preamble in den vier Kommandos angleichen.)
- **`vision`-Lesetest des Extraktors ungemessen:** ob `vision` eine extrahierte
  `.jpg` per `read` liest. (Schritt: `archive_search --pdf-image
  docs/reference/dsn_trk-2-18.1988-10-15.pdf`, dann den Pfad an `vision` reichen.)

## Benchmark (gemessen, session_burn)

Drei getrennte Atome, kein Doppel-Lauf: Mirror-Verifikation `grind-flash`
**$0.0301**; Positions-Klasse + `live`-Direktiv `grind-pro` **$0.1334**
(Urteil+Bau in einem Kontext — 4,4× der flash-Läufe); PDF-Extraktor
`grind-flash` **$0.0271**. Flash-first bestätigt für die mechanischen Atome; der
Urteil-Lauf blieb bei `grind-pro` (kein `max`-Doppellauf).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
