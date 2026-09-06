<!--
  title: Befund — Ded-31-Zweiter-Zeuge verbreitert (Same-Day-Phase): die GO-J/GO-JS-ODR-Fenster der Floor-Ära phasenaufgelöst — wo ein Fenster auf einen laut-registrierten Tag fällt, trägt der geschlossene Regelkreis laut/ruhig nur als kurze Sub-Phasen (300-s), und der unabhängige open-loop Trägerlinien-Ton bricht in den lauten Sub-Phasen nicht ein (st14 1997-02-26: laut 128.2 gegen ruhig 110.4 im selben Fenster, n 3/7) — kein Same-Day-laut/ruhig-Signal über die erreichbaren laut-Tage, Verbreiterung des gemessenen Nein zur Open-Loop-Lautheits-Signatur
  class: befund
  date: 2026-09-06
  sha256: b67b7a8d11fbe292a4452214c8dc21b5d403d1e843d8079d8c556397ad1aa9c4
  status: draft
  antwortet-auf: docs/befund/befund-galileo-odr-goj-beschaffung2.md docs/befund/befund-galileo-gwe-odr-zweiter-zeuge.md
  see-also: docs/befund/befund-galileo-ops-aera-floor.md docs/befund/befund-galileo-floor-stufen-te.md
-->
# Befund: Ded-31-Zweiter-Zeuge verbreitert — die GO-J/GO-JS-ODR-Windows der Floor-Ära phasenaufgelöst (Same-Day-laut/ruhig-Splits bei 300-s-Auflösung)

## Frage & Bindung

Der Ded-31-Zweiter-Zeuge fragt, ob die station-gebundene laut/ruhig-Floor-Lautheit
der Galileo-Gipfel upstream-real ist — ein unabhängiger (open-loop) Receiver-Record
müsste auf den laut-Phasen ein anderes/lauteres Spektral-Verhalten tragen — oder ob
sie ein Reduktions-/Schleifen-Artefakt der geschlossenen Schleife ist. Der
Vorgänger (`befund-galileo-odr-goj-beschaffung2.md`) hat den zeitgleichen
GO-J/GO-JS-ODR-Record beschafft und auf Fenster-Ebene gemessen: die drei
ODR-Fenster, die eine wirklich laute geschlossene Phase decken, tragen schmale
starke Linien innerhalb der ruhigen Spanne (erstes Nein, n = 1 Same-Day-Split).
Dieser Lauf verbreitert den Zeugen über **mehr laut-registrierte Tage** und löst
die Fenster **phasenaufgelöst** (300-s) auf: statt das ganze ODR-Fenster als eine
Phase zu klassieren, wird jede 300-s-Sub-Phase durch die zeitgleiche
geschlossene-Scheife-Lautheit markiert (Floor-class resid RMS an derselben
Station und demselben Tag). Die Frage ist dieselbe: bricht der open-loop
Trägerlinien-Ton (segsnr) in den laut-markierten Sub-Phasen ein?

Bindung wie die Vorlagen: Register-Zellen (Modus, Tag, Station) mit Boden
(Stärke exakt −2560), laut = RMS ≥ 1 Hz, Lock (|resid| > 1000 Hz) vor dem
Rauschen getrennt; Tag-Schlüssel kanonisch rund. Neu: die ODR-Segmente eines
Fensters werden in 300-s-Bins geteilt; jeder Bin wird durch die Floor-class
resid-RMS derselben Station und desselben Tages **in diesem Bin** als
laut-Phase (≥ 1 Hz) oder ruhig-Phase markiert (≥ 8 Floor-Samples im Bin), der
segsnr-Median der 8192-Sample-Segmente im Bin ist der open-loop Ton der Phase.
Probe `tools/measure/src/bin/galileo_odr_sameday_phase_probe.rs` (neu,
`cargo check` 0/0, RUSTFLAGS `-D warnings`), Report
`/tmp/opencode/galileo_odr_sameday_phase_report.txt`.

## n zuerst (0 geehrt)

| Maß | n | Beleg |
|---|---|---|
| GO-J/GO-JS-ODR-Windows in der Floor-Ära (INDEX-Vollinventar, 23 + 25 Dateien) | 12 Fenster über 8 UTC-Tage | Volumen-INDEX + Annex-Listing, frisch gemessen |
| davon lokal (die frühere Ernte + dieses Blatt) | 10 | `data/galileo_goj_odr/` |
| laut-registrierte (Modus-1-Boden-)Tage an den ODR-Stationen innerhalb der ODR-Fenster | 5 | 1996-11-08 st43 (24.5 Hz), 1996-12-19 st43 (4.81), 1996-12-21 st14 (4.10), 1997-02-26 st14 (29.1), 1997-02-27 st63 (38.5) |
| Same-Day-laut/ruhig-Split (ein ODR-Fenster enthält laut- UND ruhig-markierte 300-s-Bins) | 3 Fenster | st14 1997-02-26 GOJS (n 3/7), st14 1996-12-21 GOJ (n 1/1), st43 1996-12-19 GOJS (n 1/1) |
| ODR-Fenster mit nur einer Phase | 5 | 0 geehrt je fehlender Seite |
| zusätzlich geholte ODR-Dateien | 0 | 0 geehrt — siehe unten |
| Opposition-Anker 1996-06-26..30 | 0 | in KEINEM GO-J/GO-JS/GO-SUN-ODR (Volumen-INDEX) |

## Messung 1 — welche laut-Tage liegen in der GO-J/GO-JS-Abdeckung

Die GO-J/GO-JS-ODR-Volumina tragen in der Floor-Ära (bis 1997-02-28) genau 12
Fenster an den Trio-Stationen; die Register-Zellen (Modus 1, Boden) dieser Tage
sind oben gezählt: **5 laut-registrierte Tage** an den ODR-Stationen
(1996-11-08 st43, 1996-12-19 st43, 1996-12-21 st14, 1997-02-26 st14,
1997-02-27 st63). Die lautesten Floor-Anker der Ära — die Opposition-Tage
1996-06-26..30 an st63 — liegen in **keinem** der drei ODR-Volumina
(GO-J/GO-JS/GO-SUN INDEX: kein Eintrag im Juni 1996): 0 geehrt, sie sind durch
den ODR-Record nicht zeugen-fähig. Zusätzliche ODR-Dateien auf laut-Tagen
innerhalb der Abdeckung existieren nicht über die 10 lokalen hinaus: das einzige
nicht lokale Floor-Ära-Fenster (GOJ 53420905, 1995-12-08 st63) trägt eine dünne
Register-Zelle (n 25 < 30, nicht robust-laut) und wurde bereits in der früheren
Ernte ausgeschlossen — benannt, nicht geholt.

## Messung 2 — Phasenaufgelöste Same-Day-Splits (Kernmessung)

Die Probe markiert in jedem ODR-Fenster die 300-s-Bins durch die zeitgleiche
geschlossene-Scheife-Lautheit. Ergebnis je Fenster (Ton = segsnr-Median der
8192-Sample-Segmente im Bin):

| ODR-Window | Zelle (Tag, Modus, Station) | laut-Bins (segsnr) | ruhig-Bins (segsnr) | Split |
|---|---|---|---|---|
| GOJS 70571407 st14 1997-02-26 | m1 laut 29.1 Hz | n 3, Ton 128.2 | n 7, Ton 110.4 | **Same-Day** |
| GOJ 63561707 st14 1996-12-21 | m1 laut 4.10 Hz | n 1, Ton 62.4 | n 1, Ton 69.3 | **Same-Day** (dünn) |
| GOJS 63540659 st43 1996-12-19 | m1 laut 4.81 Hz | n 1, Ton 81.8 | n 1, Ton 104.1 | **Same-Day** (dünn) |
| GOJ 70571807 st14 1997-02-26 | m1 laut 29.1 Hz, Fenster in ruhiger Phase | n 0 | n 31, Ton 132.8 | 0 geehrt (nur ruhig) |
| GOJ 70580900 st63 1997-02-27 | m1 laut 38.5 Hz, Fenster weitgehend ohne Floor-Samples | n 1, Ton 26.0 | n 0 | 0 geehrt (dünn) |
| GOJ 63131033 st43 1996-11-08 | m2 laut, Fenster 4 min | n 0 | n 0 | 0 geehrt (kein Bin ≥ 8 Floor) |
| übrige ruhig-Tage (st43 1996-11-08/12-22/1997-02-26, st63 1996-11-08, st14 1997-02-25) | ruhig | n 0 | alle ruhig (Ton 74.9–132.8) | 0 geehrt |

Die drei Fenster mit beiden Phasen lesen keinen systematischen Einbruch der
open-loop Tons in der laut-markierten Sub-Phase: st14 1997-02-26 laut 128.2
gegen ruhig 110.4 (laut-Phase leicht höher, tragfähig n 3/7), st14 1996-12-21
laut 62.4 gegen ruhig 69.3 und st43 1996-12-19 laut 81.8 gegen ruhig 104.1
(je n 1/1, dünn; hier liest die laut-Phase leicht niedriger — die Richtung ist
je Split einzeln benannt, nicht zu einem „laut = stärker" geglättet). Die
laut-Phasen-Töne aller drei Splits liegen vollständig innerhalb der Spanne der
ruhigen Phasen desselben Tages (kein Einbruch um Größenordnungen). Die frühere
Fenster-Ebene-Klassierung („GOJS-Window = laut-Phase, 84 Hz") ist damit
präzisiert: der geschlossene Regelkreis ist auch in diesem Fenster nur in kurzen
Sub-Phasen laut (die 84-Hz-Window-RMS des Vorgängers war über n 3576 in-window
Samples gemittelt; phasenaufgelöst liegen die lauten Samples in einem
Schluss-Burst von ~15 min), und der unabhängige open-loop Träger bricht dort
nicht ein.

## Verdikt (Verbreiterung, ehrlich benannt)

Der Same-Day-laut/ruhig-Split ist jetzt über **drei** laut-registrierte Tage
gezogen (Vorgänger: ein Tag, Fenster-Ebene) — bei 300-s-Auflösung innerhalb
derselben ODR-Fenster, derselben Station und desselben Tages: st14 1997-02-26
(n 3/7), st14 1996-12-21 (n 1/1), st43 1996-12-19 (n 1/1). In allen drei liegt der open-loop Trägerlinien-Ton der laut-Phase innerhalb der
Spanne der ruhigen Phase desselben Tages (kein systematischer Einbruch in der
laut-Phase) — kein Same-Day-laut/ruhig-Signal über die erreichbaren laut-Tage. Die Floor-Lautheit erscheint damit auch bei phasenaufgelöster
Betrachtung nicht als empfangene Spektral-Störung im unabhängigen open-loop
Record; die Richtung des Ded-31-Zweiter-Zeugen-Verdikts ist über mehr laut-Tage
verbreitert und bleibt ein gemessenes Nein zur Open-Loop-Lautheits-Signatur. Die
n sind je Split ehrlich klein (3 Fenster, je 300-s-Bins; zwei Splits dünn n 1/1),
die Opposition-Anker 1996-06-26..30 sind durch keinen ODR-Record zeugen-fähig
(0 geehrt); ein endgültiges Verdikt über die volle Spektral-Reduktion der langen
ODR-Serien bleibt wie im Vorgänger als nächster Schritt benannt.

## Grenzen

- Phasen-Auflösung 300-s: die geschlossene-Scheife-Lautheit ist innerhalb der
  ODR-Fenster kurz-burstig; viele Bins tragen < 8 Floor-Samples (keine Phase,
  0 geehrt), zwei der drei Same-Day-Splits sind dünn (n 1/1). Der segsnr ist
  eine erste unabhängige Größe; eine volle Spektral-Reduktion (Linienform,
  Phasenrauschen) ist nicht gezogen.
- n der laut-markierten Sub-Phasen ist klein; die zwei dünnen Splits stützen die
  Richtung, tragen das Verdikt aber nicht allein — das tragende Fenster ist
  st14 1997-02-26 GOJS (n 3/7).
- Die Opposition-Anker 1996-06-26..30 und alle laut-Tage ausserhalb der 8
  ODR-Tage sind durch den GO-J/GO-JS-Record nicht zeugen-fähig (keine ODR-Datei
  an diesen Tagen in den Volumina) — 0 geehrt, keine Erfindung.
- Daten maschinenlokal unter `data/galileo_goj_odr/` (Provenienz + sha256 dort
  und im Beschaffung-2-Blatt); dauerhaftes Zuhause ist der PDS-Annex.

## Registrierung (Session-Duty, CDN-Manifestation)

Die ODR-Ernte ist ein Ausschnitt bestehender öffentlicher PDS3-Volumina
(GO-J-RSS-1-ODR-V1.0, GO-JS-RSS-1-ODR-V1.0 am PDS-PPI-Annex; dauerhaftes
öffentliches Zuhause = der Annex, HTTP 200 gemessen). Nach der Resid-Praezedenz
des Repos (Commit 630a793, Operator-Wort 2026-09-05: „Abgeleitete
Galileo-Assets leben nicht in phi/sources.φ; Registrierung = Workflow +
CDN-Release") wird für diesen abgeleiteten Galileo-Datensatz **keine** neue
`phi/sources.φ`-Zeile angelegt (die Galileo-CD-Assets resid/receiver stehen
dort ebenfalls nicht); die CI-Manifestation der Galileo-Daten läuft über die
dedizierten CD-Workflows (`galileo-*-cdn.yml`), die die Assets aufs
`pds-ppi.igpp.ucla.edu`-Release manifestieren. Ein eigener ODR-Manifestator
(ein harvest-Compiler, der aus den rohen ODR ein nutzbares binäres Asset
reduziert, + eine `galileo-odr-cdn.yml`-Zeile mit `--ci-mode`) steht laut
SOURCE_PORT/der bestehenden Workflow-Struktur an keiner von diesem
Mess-/Befund-Agenten zu setzenden Stelle — er ist ein CI-Lauf (Netzabruf der
298 MB + Upload aufs CDN-Release), der hier nicht gefahren wird. Zustand
ehrlich registriert: **`pending` bis der CI-Lauf / die Haupt-Session den
ODR-Manifestator setzt** — `.github/workflows/` nicht angefasst, keine fremde
Datei berührt; was CI fahren würde, ist die `galileo-odr-cdn.yml`-Zeile
(`galileo_odr_compiler --ci-mode` → Asset auf `pds-ppi.igpp.ucla.edu`), benannt,
nicht fabriziert.

## Register-Satz

*Der Ded-31-Zweiter-Zeuge ist über mehr laut-Tage verbreitert und
phasenaufgelöst: innerhalb der GO-J/GO-JS-ODR-Abdeckung der Floor-Ära liegen
5 laut-registrierte (Modus-1-Boden-)Tage an den ODR-Stationen; die
phasenaufgelöste Same-Day-Messung (300-s-Bins, laut/ruhig durch die zeitgleiche
geschlossene-Scheife-Floor-RMS derselben Station und desselben Tages) zieht den
laut/ruhig-Split in drei ODR-Fenstern auf laut-Tagen (st14 1997-02-26 n 3/7,
st14 1996-12-21 n 1/1, st43 1996-12-19 n 1/1) — in allen dreien bricht der
open-loop Trägerlinien-Ton (segsnr) in der laut-Phase nicht ein (128.2 gegen
110.4; 62.4 gegen 69.3; 81.8 gegen 104.1), die Fenster-Ebene-Klassierung des
Vorgängers ist als kurze laut-Sub-Phasen präzisiert, das Nein zur
Open-Loop-Lautheits-Signatur bleibt gemessen und ist über mehr laut-Tage
verbreitert (Same-Day-Splits n = 3 laut-Tage, zwei dünn n 1/1; Opposition-Anker
1996-06-26..30 in keinem ODR-Volumen, 0 geehrt; zusätzliche ODR-Dateien auf
laut-Tagen innerhalb der Abdeckung existieren nicht über die 10 lokalen, 0
geehrt); keine phi/sources.φ-Registrierung (Resid-Praezedenz, Workflow +
CDN-Release), kein eigener ODR-Manifestator laut SOURCE_PORT (Zuständigkeit
Haupt-Session/CI benannt).*
