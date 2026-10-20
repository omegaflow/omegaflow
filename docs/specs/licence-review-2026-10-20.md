<!--
  title: Lizenz-Review — fünf Attributions-Quellen (2026-10-20)
  class: spec
  date: 2026-10-20
  status: live
  sha256: 20f617ce4ac1116bce47e225c155f86f327ce7b1609a33e315a5a2173924a5b3
-->
# Lizenz-Review — fünf Attributions-Quellen

Die CDN-Allowlist (`src/archivar/cdn.rs`) lässt nur PD, CC0, CC-BY, CC-BY-NC,
CC-BY-SA, OGL, Etalab, PDL, JP und own-work zu. Die folgenden fünf Quellen
tragen kein SPDX-Token, verlangen aber nur Attribution/Zitation. Sie werden
unter dem Review-Token `reviewed-2026-10-20` geführt; der Beleg steht jeweils
auf dem Datenhost.

| Quelle | Block | Beleg | Zitat | Entscheidung |
|---|---|---|---|---|
| WSO | `wso_polar.bin` | <http://wso.stanford.edu/DataPolicy.html> | “we ask only three things: that you notify us; that you acknowledge the source of the data; that you send us a copy of resulting research reports and papers.” | **admitted**, Attribution; keine kommerzielle Weiterverbreitung |
| Kitt Peak KPVT | `kpvt_polar.bin` | <https://nispdata.nso.edu/ftp/kpvt/synoptic/README> | “These data are freely available with one restriction: If you use the data … in a published paper, we ask that you include an acknowledgement as follows: NSO/Kitt Peak data used here are produced cooperatively by NSF/NOAO, NASA/GSFC, and NOAA/SEL.” | **admitted**, Attribution |
| AMANDA-II | `amanda.s2e1` | <https://icecube.wisc.edu/data-releases/2008/09/amanda-7-year-data/> | “Please cite use of this data as R. Abbasi, et al., arXiv:0809.1646. Phys. Rev. D 79, 062001 (2009).” | **admitted**, Attribution |
| EDGES | `edges_790.bin`, `edges_792.bin`, `edges_raw_spectrum_2016-250.bin` | <https://galileo.sese.asu.edu/edges-data/Readme.md> | “Usage of this data in any published work must be accompanied by citations of both Bowman et al (2018) and Murray et al (2026)…” | **admitted**, Attribution |
| HIZOA / VizieR | `hizoa.json` | <https://cds.unistra.fr/vizier-org/licences_vizier.html> | “VizieR are free of usage in a scientific context; however, as it is the usage … the original authors and publication references including the publisher have to be explicitly cited … The commercial usage of the data is subject to rules depending of the origin.” | **admitted**, wissenschaftliche Nutzung; kommerzielle Nutzung ausgeschlossen |

## Grenze

`reviewed-2026-10-20` ist kein SPDX-Token und keine allgemeine Freigabe. Es gilt
nur für die fünf oben genannten Blöcke, mit dem jeweiligen Beleg als `terms`-URL.
Neue Quellen mit `free-open`, `other-attribution` oder `vizier-scientific-free`
bleiben gesperrt, bis ein eigener Review-Eintrag vorliegt.
