<!--
  title: Stimmen — River P7.1 coupling-Direktive Grammatik (Rat + Forschung + Roster, 2026-10-12)
  class: stimmen
  date: 2026-10-12
  sha256: -
  status: live
-->
# `coupling`-Register-/Parser-Direktive — Rat + Recherche + Roster (2026-10-12)

Volle Kette (Operator-Wort: „wenn du council befragst auch mit research und roster"):
Forschungs-Schicht (`archive_search --all`, Digest `state/research/2026-10-12-river-p71-coupling-grammar-recherche.md`);
Rat (fünf Stimmen); Roster (`river-ui` Duck · Claude · Qwen · Z.ai/GLM Deep Think Max + geteilt
`open-weight-ui` DeepSeek V4 Pro, 5/5).

**Frage:** konkrete Grammatik + Kanal-Verweis-Form + Platzierung der `coupling`-Direktive (sparse
Paar-Kopplung) — nach dem Form-Kern (role-keyed typisierte `density_refs`, K deklariert, `quasi_neutral`
als Regime-Constraint).

## Forschungs-Schicht (gemessen)

Identität = **immutabel undurchsichtige ID**; **Alias = reassignbarer Name**; **Ordinal/Index ist nie
Identität**. Belege: Confluent Schema Registry (immutable Schemas, Subject-Aliase), TR-181 `Alias`
(immutable once set), YANG SID (stabiler SID), IVOA Identifiers, Venti/Software Heritage
(content-signiert), Registry-Kette `manifest_id`+`previous_manifest_id`. Grammatik-Konvention für
sparse Paar-Matrizen trennt `pattern` (Struktur) von Werten, Index-Basis/Symmetrie explizit;
Regime-Constraints als eigenes `constraints:`/`<constraintDecl>`-Objekt.

## Rat (fünf Stimmen) — Entwurf

```
coupling <kind> from <ref> to <ref> rate <K|pending>
         density_from <number|mass>:<ref>[:<m_c>]
         density_to   <number|mass>:<ref>[:<m_c>]
         [region <name>] [conserves <token>] [quasi-neutral <constraint|pending>]
constraint quasi-neutral <name> region <name> [(alias <ref> [charge <Z>])|pending]
```
Verdikt: `<ref>` = **Alias-Name auf der `channel`-Zeile**; `pending` als legaler Token-Wert;
from/to rollenschlüssig; K deklariert; Kopplungen im Quellblock (`SourceConfig.couplings`);
`rate`/`from`/`to`/`density_*` Pflicht, `region`/`conserves` Pflicht für Reibung; ein Eintrag pro
ungeordnetem Paar.

## Roster (5/5) — ratifiziert die Richtung, reißt den Entwurf an vier Stellen

| Seat | Verdikt | Kern |
|---|---|---|
| Duck.ai (GPT-6 Luna) | trägt, drei Schärfungen | Alias richtig (Hash unhandlich, URI nur dokumentübergreifend); **schwächste Stelle = `constraint quasi-neutral`** — die Einzel-Alias-Form drückt die Summenbedingung nicht aus; Teilnehmerliste nötig; `pending` typisiert |
| Claude (Sonnet 5.5 Extra hoch) | trägt mit Riss | **Alias im Source UND Hash im aufgelösten Artefakt** (Alias versteckt Retargeting); Alias braucht Vertrag (Scope, Eindeutigkeit, reservierte Wörter); **`SourceConfig.couplings` falsch** — Kopplung ist ein Zwei-Kanal-Objekt mit Erhaltungsanspruch, gehört auf Register-Ebene; K ohne Einheit; symmetrische kinds brauchen `(kind, ungeordnetes Paar, region)`-Schlüssel; `pending` braucht Run-Gate; `kind` als geschlossene Menge mit Validierungsmatrix |
| Qwen (Qwen3.7-Plus) | trägt | `pending` nur als **strikt getyptes Deferred-Object**, vor dem Zeitintegrator zwingend resolved; `density_*`-Syntax „cluttered" → `<mode> [<value|profile_ref>] [mass_factor <m_c>]`; `quasi-neutral` ist ein **Regime-Wechsel** (ändert die Gleichungsnatur), nicht ein Constraint; **`SourceConfig.couplings` über Bord** → `TopologyConfig`/`InteractionConfig` |
| Z.ai (GLM-5.3 Deep Think Max) | trägt mit drei Verträgen | Alias richtig, aber **Pflicht auf der channel-Zeile**, flacher globaler Namespace, Uniqueness hart (Fehler nennt beide Stellen), All-Errors-Diagnostik + Edit-Distance, Alias→interner epoch-skopierter ID sofort nach Parse; **schwächste Stelle = die Density-Klausel** (positionale Mikro-Grammatik, m_c-Bedingtheit in `[...]` nicht ausdrückbar, Refs auf dritte/vierte Kanäle → generischer Resolver); `pending`-Termination als harter Gate; Dual-Ladung vermeiden; `conserves` aus `kind` ableiten; **Platzierung: top-level/`couplings`-Block**, nicht SourceConfig |
| DeepSeek V4 Pro (1.7T) | trägt mit Riss | Alias richtig, aber nur mit harten Regeln (mandatory, Scope/Eindeutigkeit, Duplikat=Load-Fehler, Rename-Story); Hash gehört in den kompilierten IR, nie in die Grammatik; **`constraint quasi-neutral` reißt grammatisch** — `charge <Z>` dupliziert die Ladung (zweite Source of Truth, driftet) → Ladung auf die `channel`-Zeile, der Constraint referenziert nur die Menge |

## Konvergenz (5/5 + Rat-Richtung)

1. **Alias-Name ist die richtige Referenz-Form** für die handgeschriebene Quelle — **Hash ist der
   abgeleitete kanonische Identitäts-Träger im kompilierten IR** (Pipeline: DSL-Namen → Resolve
   (Namen→IDs, pending-Check) → IR (IDs) → Numerik sieht nie Namen). Alias braucht einen harten
   Vertrag: Pflicht auf der `channel`-Zeile, Scope/Eindeutigkeit, Duplikat = Load-Fehler,
   Rename-Story (Tooling, nicht Content-Addressing).
2. **Die Kopplung lebt auf Register-/Relations-Ebene, nicht in `SourceConfig.couplings`** — ein
   top-level `coupling`-Block (beide Endpunkte benennbar; im Block von `from` unsichtbar beim Lesen
   von `to`; Spiegel-Deklaration → stilles Double-Counting). In `SourceConfig.couplings` gehört nur
   das **aufgelöste pro-Kanal-Beitrags-Artefakt**.
3. **`charge` gehört auf die `channel`-Zeile**, nicht doppelt in den Constraint (zweite Source of
   Truth driftet); der `quasi-neutral`-Constraint referenziert die Menge (Summenbedingung
   `Σ_s Z_s n_s = n_e`), Enforcement per Ref-Alias.
4. **`pending` braucht ein hartes Termination-Gate** (finaler Sweep: kein `pending` überlebt die
   Config-Resolution; typisiertes Deferred-Object, nie stiller Default).
5. **Density-Klausel entzerren**: `density_from number:<ref>` | `density_from mass:<ref>:<m_c>` als
   Alternativen statt positionaler `[...]`; die Refs können auf Kanäle zeigen, die weder from noch to
   sind → generischer Namespace-Resolver über **alle** Ref-Positionen.
6. **`kind` als geschlossene Menge mit Validierungsmatrix** (Pflichtfelder, K-Einheit, erlaubte
   Dichte-Arten, Default für `conserves`); `conserves` aus `kind` ableiten/prüfen, nicht frei
   annotieren.

## Korrigierte Direktive (Vorschlag, Rat+UI)

```
channel <alias> <conserved>:<role>:<op>:<pde-type>:<medium>:<?>:<?> <unit> [charge <Z>]
coupling <kind> from <alias> to <alias> rate <K|pending>
         density_from number:<alias> | mass:<alias>:<m_c>
         density_to   number:<alias> | mass:<alias>:<m_c>
         [region <name>] [conserves <token>] [quasi-neutral <constraint-name>]
constraint quasi-neutral <name> region <name>
```

## Gehaltener Riss

- **Platzierung:** der Rat sagte `SourceConfig.couplings`, der Roster 5/5 sagt **Register-/Relations-Ebene**
  (nur das aufgelöste Beitrags-Artefakt gehört in `SourceConfig`). Der Baum folgt dem Roster — der
  Rat-Entwurf ist hier überstimmt, nicht geglättet.
- **`charge` doppelt** (Constraint vs. Kanal): Roster einhellig → auf die `channel`-Zeile.

## Nächster Bau (bounded, Schritt-für-Schritt; `cargo check` 0/0 je Schritt)

1. `channel`-Zeile um **Alias-Name** (+ optional `charge <Z>`) erweitern; Parser-Arm + `ChannelRegistry::alias`-Verdrahtung.
2. **Top-level `coupling`-Block** (eigener Rückgabewert von `parse_sources`, nicht `SourceConfig`);
   `CouplingRegistry` daraus bauen.
3. `constraint quasi-neutral`-Block (Register-Ebene).
4. Gate-Fixtures (`coupling-pending-zero`, `coupling-ref-unresolved`, `coupling-no-region`,
   `channel-alias-reassign`, `coupling-dupe`) im selben Atom.
5. **Kein `coupling`-Datenwert wird geschrieben, solange IRI-2020/NRLMSISE-00/IGRF-Alias und K nicht
   live auflösen** — die Zeile bleibt `pending`.
