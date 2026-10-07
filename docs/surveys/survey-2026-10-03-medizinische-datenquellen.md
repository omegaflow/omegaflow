<!--
  title: Survey — Medizinische/Life-Science-Datenquellen
  class: survey
  date: 2026-10-03
  sha256: feb2807818faf2b6bd66f009f0c0e7fd0c1f949e7230dc80b9cc1e05ca9a1c5e
  status: live
  see-also: docs/handover/archiv/handover-2026-10-03-sensory-folge227.md phi/sources.φ
-->
# Survey — Medizinische/Life-Science-Datenquellen (2026-10-03)

## Zweck

Operator-Wort 2026-10-03: welche medizinischen Datenquellen es gibt, um wirklich
forschen zu können — Somatik, Psychosomatik, Biologie, Chemie, Psychologie,
Neurologie und die noch unbekannten Gebiete. Dies ist die gemessene erste
Landschaft: eine Erreichbarkeits-/Zugangs-Messung (`archive_search --verdict`,
Datum 2026-10-03), kein Lizenz- oder Größennachweis. `[V: 200/206]` = direkt
erreichbar; `[V: 403/404]` = direkt blockiert/fehlt; `ungemessen` = nicht
verifiziert. Lizenz-/Größenangaben sind Betreiberangaben, so markiert. Jede
Quelle, die nicht per `--verdict` belegt ist, ist als `ungemessen` geführt —
nicht als offen.

Der Bestand (`phi/sources.φ`) trägt heute nur zwei dieser Quellen: **OpenNeuro**
(`:3313–3407`, PD-EEG-Subset) und **PhysioNet** (`:3409–3417`, `bidsleep`).
**NeuroVault** ist `declined` (`phi/declined_sources.φ:2686`, `no-physical-force`,
kein Zugangsgrund). Alles andere ist unregistriert; die meisten stehen nur als
Kandidaten im Katalog-Pool (`phi/pipeline/catalog/{re3data,zenodo,dryad}_catalog.φ`).

## A. Multimodale Plattformen & offene Archive

- **OpenNeuro** (Stanford/CCN) — https://openneuro.org [206]; GraphQL `https://openneuro.org/crn/graphql` (POST; bare GET 400). Offen (anonym), CC0/CC-BY. BIDS: EEG/MEG/fMRI/iEEG, klinische + gesunde Populationen. **Bereits registriert** (`phi/sources.φ:3313`).
- **PhysioNet** (MIT/NIH) — https://physionet.org [200]; Bestände mitdb/sleep-edfx/chbmit/mimiciv/eicu-crd [200]; `/api/` [404]. Gemischt: MIT-BIH/Sleep-EDF/CHB-MIT offen, MIMIC-IV/eICU nur Credentialing+DUA. WFDB/EDF/CSV. EKG/EEG/EMG/Schlaf/Intensiv. **Bereits registriert** (`:3409`, `bidsleep`).
- **NITRC** (NIH/BRAIN) — https://www.nitrc.org [200]; fcon_1000 [206]. Registrierung; je Projekt CC-BY/eigen. NIfTI/DICOM/BIDS (ABIDE, ADHD-200, NKI).
- **EBRAINS** (HBP) — https://www.ebrains.eu [200]. Registrierung; je Datensatz. BIDS/NIfTI/HDF5, Atlanten + Modelle.
- **Synapse** (Sage Bionetworks) — https://www.synapse.org [200]. Registrierung + DUA; je Projekt. AD Knowledge Portal, PsychENCODE.
- **Harvard Dataverse** — https://dataverse.harvard.edu [202]; API `/api`. Offen, CC0. CSV/u. a., Sozial-/Gesundheitswissen.
- **OSF** (Center for Open Science) — https://osf.io; API `https://api.osf.io/v2/` [200]. Offen. Psychologie, Präregistrierungen, Rohdaten.

## B. Klinische Daten / Intensiv / Epidemiologie

- **NHANES** (CDC) — https://wwwn.cdc.gov/nchs/nhanes/ [206]. Offen, Public Domain. XPT/CSV, Anthropometrie/Labore/Aktigraphie.
- **UK Biobank** — https://www.ukbiobank.ac.uk [403 direkt, Wayback]. Strikt DUA, eigene Lizenz. ~500 000 TN, Genomik/MRI/Wearable.
- **All of Us** (NIH) — https://www.researchallofus.org [200]; allofus.nih.gov [403]. Workbench + DUA. >400 000 TN, WGS/EHR.
- **BioLINCC** (NHLBI) — https://biolincc.nhlbi.nih.gov [200]. DUA. Framingham/ARIC/CARDIA.
- **WHO GHO** — https://www.who.int/data/gho; API `https://ghoapi.azureedge.net/api/` [200]. Offen, CC-BY-NC-SA. Mortalität/Morbidität.
- **Eurostat Gesundheit** — https://ec.europa.eu/eurostat [200]. Offen, CC-BY 4.0. SDMX/CSV.
- **healthdata.gov** (HHS) — https://healthdata.gov [200]. Offen. Medicare/Public-Health.
- **IHME GHDx** — https://ghdx.healthdata.org [200]. Offen/teils Registrierung. Global Burden of Disease.
- **GSS** (NORC) — https://gss.norc.org [206]. Registrierung. US-Sozial-/Einstellungsdaten seit 1972.
- **UK Data Service** — https://ukdataservice.ac.uk [200]. Registrierung/End User Licence.
- **NBER** — https://www.nber.org [200]. Working Papers offen.
- **OECD Health Statistics** — oecd.org [403 direkt, Wayback]. Offen, CC-BY 4.0.

## C. Bildgebung — MRI / fMRI / MEG / EEG / iEEG

- **Human Connectome Project** — https://humanconnectome.org [200]; ConnectomeDB https://db.humanconnectome.org [200]. Open Access Terms. NIfTI/CIFTI/MEG.
- **ABCD Study** — https://abcdstudy.org [200]; Daten via NIMH Data Archive. DUC. ~11 900 Kinder longitudinal.
- **NIMH Data Archive** — https://nda.nih.gov [200]; NDA-API. Registrierung + DUA. Psychiatrie-Kohorten.
- **NeuroVault** — https://neurovault.org [200]; API `/api/collections/` [200]. Offen, CC0. Statistische Hirnkarten. **`declined`** (`phi/declined_sources.φ:2686`, `no-physical-force`).
- **OASIS-3** — https://www.oasis-brains.org [200]. Registrierung + DUA. Altern/Demenz MRI/PET.
- **PPMI** (Michael J. Fox) — https://www.ppmi-info.org [200]. Registrierung + DUA. Parkinson-Längsschnitt.
- **Cam-CAN** — https://camcan-archive.mrc-cbu.cam.ac.uk [200]. Data Share Agreement. Lebensspannen-MEG/MRI.
- **ieeg.org** (UPenn) — https://www.ieeg.org [206]. Registrierung + User Agreement. Klinisches iEEG (Epilepsie).
- **TUH EEG Corpus** (Temple) — https://isip.piconepress.com/projects/tuh_eeg/ [200]. Registrierung + DUA. >30 000 klinische EEG-Sitzungen.
- **Allen Brain Map** — https://portal.brain-map.org [200]; RMA-API. Offen, CC-BY-NC. Genexpression/Zelltypen/Konnektivität.
- **NSRR** — https://sleepdata.org [pending; kein brauchbarer Snapshot] → **ungemessen**. Registrierung + DUA. Schlaf-EEG (SHHS/MESA).
- **MOUS** — https://www.mousdataset.org [pending] → **ungemessen**.

## D. Genetik / Genomik / Biobanken

- **dbGaP** (NCBI) — https://www.ncbi.nlm.nih.gov/gap/ [200]. DUA. VCF/BAM/CRAM + Phänotyp.
- **GDC / TCGA** (NCI) — https://portal.gdc.cancer.gov [200]; API `https://api.gdc.cancer.gov/status` [200]. Offene Schicht + DUA. ~11 000 TCGA-Fälle / >80 000 GDC.
- **EGA** (EBI/CRG) — https://ega-archive.org [200]; Metadaten-API `https://metadata.ega-archive.org/` [200]. Metadaten offen, Rohdaten DUA.
- **ENCODE** (NHGRI) — https://www.encodeproject.org [200]; API `…/search/?format=json` [200]. Offen, CC0/CC-BY. Funktionelle Genomik.
- **GTEx** (Broad) — https://gtexportal.org [206]. Portal offen; Rohsequenz dbGaP/AnVIL. ~950 Spender, 54 Gewebe. (API-Pfad `/api/v2/…` **ungemessen**, `/api/v2/dataset` [404].)
- **GWAS Catalog** (EBI) — https://www.ebi.ac.uk/gwas/ [200]; REST `…/rest/api/studies?size=1` [200]. Offen, CC-BY. Kuratierte GWAS-Assoziationen.
- **IEU OpenGWAS** (Bristol) — https://gwas.mrcieu.ac.uk; API `https://api.opengwas.io/api/` [200], `/status` [200]. Token nötig. >50 000 Summary-Statistiken.
- **FinnGen** — https://www.finngen.fi [206]. Summary offen, individual-level DUA. ~500 000 Finnen.
- **ClinVar** (NCBI) — https://www.ncbi.nlm.nih.gov/clinvar [200]. Offen, Public Domain. Variantenklassifikation.
- **OMIM** — https://www.omim.org [403 direkt, Wayback]. Registrierung/Subskription. Mendelsche Gene.
- **SRA / ENA** — https://www.ncbi.nlm.nih.gov/sra [200]; https://www.ebi.ac.uk/ena/browser/ [206]. Offen (je Studie). Rohsequenz.
- **GEO** (NCBI) — https://www.ncbi.nlm.nih.gov/geo/ [200]; E-utils [200]. Offen, Public Domain. Expression.
- **Human Cell Atlas** — https://data.humancellatlas.org [206]. Offen, CC-BY. Einzelzell-Transkriptomik (Loom/H5AD).
- **CZ CELLxGENE** — https://cellxgene.cziscience.com [206]. Offen, CC-BY. H5AD Single-Cell. (API-Pfad `/curation/v1/` [404], **ungemessen**.)
- **Expression Atlas** (EBI) — https://www.ebi.ac.uk/gxa/ [200]. Offen, CC-BY. Differentielle Expression.
- **ArrayExpress / BioStudies** (EBI) — https://www.ebi.ac.uk/arrayexpress/ [200]; https://www.ebi.ac.uk/biostudies/ [200]. Offen.
- **ProteomeXchange / PRIDE** — https://www.proteomexchange.org [206]; PRIDE-API `…/pride/ws/archive/v2/projects?pageSize=1` [200]. Offen, CC0/CC-BY. mzML.
- **MetaboLights** (EBI) — https://www.ebi.ac.uk/metabolights/ [206]; API `…/ws/studies` [200]. Offen, CC0.
- **Open Targets Platform** — https://platform.opentargets.org [206]; GraphQL `https://api.platform.opentargets.org/api/v4/graphql` (POST; GET 400). Offen, CC-BY. Target-Disease-Associationen.
- **EBI OLS4** — https://www.ebi.ac.uk/ols4 [206]. Offen, CC0. Ontologien (HPO/DOID/MONDO).

## E. Psychologie / Psychiatrie / Psychosomatik / Verhaltensdaten

- **Open Psychometrics** — https://openpsychometrics.org [206]. Offen (CC-BY-NC, Betreiberangabe). Psychometrische Selbstberichte.
- **PsychData** (ZPID) — https://www.psychdata.de [200]. Offen/teils Registrierung. DACH-Psychologie.
- **GESIS** — https://www.gesis.org [403 direkt, Wayback]. Registrierung/teils kostenpflichtig.
- **ICPSR / openICPSR** — https://www.icpsr.umich.edu [403]; https://www.openicpsr.org [403]. Mitgliedschaft/DUA.
- **Add Health** (UNC) — https://addhealth.cpc.unc.edu [206]. Registrierung + DUA (Kosten). ~20 000 Jugendliche longitudinal.
- **HRS** — https://hrs.isr.umich.edu [403 direkt, Wayback]. Registrierung + DUA. >40 000 über 50.
- **SHARE** (SHARE-ERIC) — https://www.share-project.org [200]. Registrierung + DUA. ~140 000 in 28 Ländern.
- **SOEP** (DIW) — https://www.diw.de/soep [200]. DUA-Vertrag. ~30 000 jährlich seit 1984.
- **Psychiatric Genomics Consortium** — https://www.med.unc.edu/pgc/ [200]. Summaries offen, individual-level DUA. Schizophrenie/Bipolar/MDD/ADHS/Autismus-GWAS.
- **medRxiv** — https://www.medrxiv.org [206]. Offen. Preprints (Literatur, kein Rohdatenregister).
- **Europe PMC** — REST `https://www.ebi.ac.uk/europepmc/webservices/rest/search?query=…&format=json` [200]. Offen. Auffindungs-/Verlinkungsschicht.

## F. Register / klinische Studien / Regulatorik

- **ClinicalTrials.gov** — API `https://clinicaltrials.gov/api/v2/studies` [200]. Offen, Public Domain. >500 000 Studien.
- **ISRCTN Registry** — https://www.isrctn.com [200]. Offen, CC-BY.
- **WHO ICTRP** — https://www.who.int/clinical-trials-registry-platform [200]. Offen. Metasuche.
- **openFDA** — API `https://api.fda.gov/` (`/drug/event.json?limit=1` [200]). Offen, Public Domain. Adverse Events/Labels/Recalls.
- **BfArM / EUDAMED / EMA** — **ungemessen**.

## G. Weitere open-access Archive

- **Zenodo** (CERN/EU) — https://zenodo.org; API `/api/…`. Offen, je Record CC.
- **Dryad** — https://datadryad.org. Offen, CC0.

## H. Biologie — Genomik/Proteomik/Transkriptomik/Metabolomik (Life-Science-Block)

- **NCBI (GenBank/RefSeq/Entrez/GEO/SRA)** — https://www.ncbi.nlm.nih.gov/; E-utils `https://eutils.ncbi.nlm.nih.gov/entrez/eutils/` (`--entrez db=nuccore` → count 68268, gemessen). Offen; dbGaP DUA. FASTA/FASTQ/SRA/XML.
- **Ensembl** (EBI/Sanger) — https://www.ensembl.org/ [206]; REST `https://rest.ensembl.org/`. Offen. FASTA/GFF3/VCF. **Hinweis:** `archive_search --ensembl` lieferte HTTP 500 — Modus defekt, Quelle erreichbar.
- **IGSR / 1000 Genomes** — https://www.internationalgenome.org/ [206]. Offen. VCF/BAM/CRAM.
- **gnomAD** (Broad) — https://gnomad.broadinstitute.org/ [206]. Aggregat offen, Rohdaten teils DUA.
- **UniProt** — https://www.uniprot.org/ [206]; REST `https://rest.uniprot.org/` (`--uniprot p53 human` → P04637). Offen, CC-BY 4.0.
- **PRIDE** (EBI) — https://www.ebi.ac.uk/pride/ [206]; API `…/pride/ws/archive/v2/`. Offen. mzML.
- **PDC** (NCI) — https://datacommons.cancer.gov/repository/proteomic-data-commons (nur `--tavily`, **URL nicht per verdict gemessen**). Offen laut Text. **ungemessen**.
- **Human Protein Atlas** — https://www.proteinatlas.org/ [200]. Offen, CC-BY-SA 4.0.
- **Metabolomics Workbench / NMDR** (NIH/UCSD) — https://www.metabolomicsworkbench.org/ [200]. Offen. mzML/CSV.
- **MassBank** — https://massbank.eu/ [200]. Offen (je Instanz). Referenz-Massenspektren.
- **BMRB** — https://bmrb.io/ [200]. Offen (CC0/CC-BY). NMR-Spektren.
- **Modellorganismen** (alle FASTA/GFF/JSON, offen): MGI https://www.informatics.jax.org/ [200] · ZFIN https://zfin.org/ [200] · FlyBase https://flybase.org/ [206] · WormBase https://wormbase.org/ **[403, nur  Proton/Browser]** · SGD https://www.yeastgenome.org/ [200] · TAIR https://www.arabidopsis.org/ **[403, Registrierung/teils lizenzpflichtig]**.

## I. Chemie

- **PubChem** (NCBI) — https://pubchem.ncbi.nlm.nih.gov/ [206]; PUG-REST `…/rest/pug/` (`--pubchem caffeine` → CID 2519). Offen, Public Domain. ~110 Mio. Verbindungen.
- **ChEMBL** (EBI) — https://www.ebi.ac.uk/chembl/ [200]; API `…/chembl/api/data/` (`--chembl aspirin` → CHEMBL25). Offen, CC-BY-SA 3.0. ~2,4 Mio. Verbindungen.
- **ZINC** (UCSF) — https://zinc.docking.org/ [200]. Offen. >1 Mrd. andockbare Moleküle.
- **Crystallography Open Database** — https://www.crystallography.net/ (`--cod` lieferte Records). Offen, CC0. ~500 k CIF.
- **CCDC / CSD** — https://www.ccdc.cam.ac.uk/ [206]. **Lizenzpflichtig** (akademische Lizenz). >1,2 Mio. Strukturen.
- **DrugBank** — https://go.drugbank.com/ **[403 direkt+Proton, nur Wayback]**. Kommerziell lizenziert.
- **NIST WebBook** — https://webbook.nist.gov/ (`--exa`-Treffer, **nicht per verdict gemessen**). Offen (US-Regierung). **ungemessen**.

## J. Pharmakologie / Toxikologie

- **OpenFDA** — https://open.fda.gov/ [206]; API `https://api.fda.gov/`. Offen, Public Domain.
- **BindingDB** (UCSD) — https://www.bindingdb.org/ [206]. Offen, CC-BY 4.0. ~2,8 Mio. Affinitäten.
- **Tox21** — https://tripod.nih.gov/tox21/ **[404 direkt+Proton, nur Wayback]**. Daten offen, aber über PubChem. Die gemessene URL ist nicht erreichbar.

## K. Strukturbiologie

- **PDB / PDBe / RCSB** — https://www.rcsb.org/ [200], https://www.ebi.ac.uk/pdbe/ (`--pdb` 3F6Z). Offen, CC0. >220 k Strukturen.
- **EMDB** — https://www.ebi.ac.uk/pdbe/emdb/ [200]. Offen, CC0. Kryo-EM-Dichtekarten.
- **AlphaFold DB** (DeepMind/EBI) — https://alphafold.ebi.ac.uk/ (Modell-Datei `AF-P04637-F1-model_v6.pdb` gemessen); API `…/api/prediction/{accession}`. Offen, CC-BY 4.0. ~214 Mio. Vorhersagen.

## L. Bildgebung / Mikroskopie

- **IDR** (EBI/OME) — https://idr.openmicroscopy.org/ [200]. Offen, CC-BY 4.0. OME-Zarr/OME-TIFF.
- **Cell Painting** (Broad) — https://www.broadinstitute.org/cell-painting **[404, kein Snapshot]**. Daten liegen auf `cellpainting.org`/`registry.opendata.aws` (**ungemessen**).

## M. Querschnitt / Ontologien / Sekundär (Modus-belegt)

Reactome (`--reactome p53` → R-HSA-69541; [206]; CC0) · Gene Ontology/QuickGO (`--go apoptosis` → GO:0097194; CC-BY 4.0) · InterPro (`--interpro kinase` → IPR000023; CC0) · BioModels (`--biomodels p53`; CC0) · BioStudies [200] · ENA (`--ena result=read_run` → ERR10006159; [206]).

## Zusammenfassung der Zugangszustände (gemessen 2026-10-03)

- **Offen, HTTP 200/206 + API:** NCBI/GenBank/Entrez/GEO/SRA, Ensembl (Seite), IGSR, gnomAD, GDC/TCGA-Offenschicht, UniProt, PRIDE, HPA, GXA, GTEx-Portal, ENCODE, MetaboLights, Metabolomics Workbench, MassBank, BMRB, MGI/ZFIN/FlyBase/SGD, PubChem, ChEMBL, ZINC, COD, OpenFDA, BindingDB, PDB/PDBe, EMDB, AlphaFold, IDR, Allen Brain, Reactome/GO/InterPro/BioModels, ENA.
- **DUA / kontrolliert:** dbGaP, EGA, UK Biobank, All of Us, BioLINCC, NDA, ABCD, HCP (Open-Access-Terms), OASIS, PPMI, NITRC (Registrierung), Add Health, HRS, SHARE, SOEP, PSG/TCGA-Rohdaten, GTEx-Rohdaten.
- **Lizenzpflichtig/kommerziell:** CCDC/CSD, DrugBank, OMIM (Bulk), TAIR.
- **Direkt blockiert (403), nur Browser/Proton/Wayback:** UK Biobank, All of Us-Programmseite, ICPSR/openICPSR, HRS, GESIS, OECD, OMIM, WormBase, TAIR, DrugBank.
- **Gemessen nicht erreichbar (404):** `tripod.nih.gov/tox21/`, `broadinstitute.org/cell-painting`.
- **Behoben 2026-10-03 (`46427ceea`):** `archive_search --ensembl` → EBI Search bricht für die Domäne `ensembl` serverseitig ab (Pfad-Form HTTP 500; uniprot/chembl/reactome 200); der Modus läuft jetzt über die Ensembl-REST-API `rest.ensembl.org/xrefs/symbol/homo_sapiens/…` + `lookup/id/…` (gemessen: p53/TP53/ENSG/nonsense).
- **Ungemessen:** NIST WebBook, korrekte Cell-Painting-/Tox21-Heimaturls, GTEx-API-Pfad, CELLxGENE-API, BfArM/EUDAMED/EMA, verschiedene Bulk-Endpunkte.

## Offen / Braucht

Die Liste ist ein **Kandidaten-Pool**, kein Register. Jede Quelle braucht Mountains
Verdikt (Zulassung/Force-Gate) und eine `phi/sources.φ`-Zeile (Origin/Compiler/
Felder) — die Register-Schreibung ist Mountains Recht. Ein Teil der Quellen
verlangt Operator-gebundene Zugänge (DUA/Registrierung/Kosten): UK Biobank, All of
Us, dbGaP, EGA, MIMIC-IV, NDA/ABCD, OASIS, PPMI, Add Health, HRS, SHARE, SOEP,
ICPSR — diese gehören in die Future-Operator-Queue, nicht in eine Linien-Übergabe.
