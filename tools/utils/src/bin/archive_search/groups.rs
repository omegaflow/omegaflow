#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Group {
    All,
    Astro,
    Literature,
    Web,
    Data,
}

const ASTRO: &[&str] = &[
    "ads",
    "alerce",
    "antares",
    "dust",
    "exoplanet-archive",
    "fink",
    "frbcat",
    "gaia",
    "heasarc",
    "horizons",
    "lasair",
    "mast",
    "ned",
    "nssdc",
    "nssdc-auth",
    "nssdca",
    "simbad",
    "ssdc-auth",
    "supermag",
    "vizier",
];

const LITERATURE: &[&str] = &[
    "ads",
    "alphaxiv",
    "alphaxiv-call",
    "alphaxiv-researchers",
    "alphaxiv-tools",
    "arxiv",
    "awmf",
    "base",
    "cochrane",
    "consensus",
    "core",
    "crossref",
    "dblp",
    "doaj",
    "europepmc",
    "gemini",
    "hal",
    "mendeley",
    "ntrs",
    "oapen",
    "openaire",
    "openalex",
    "opencitations",
    "orcid",
    "perplexity",
    "psychporta",
    "pubmed",
    "ror",
    "scholexplorer",
    "scopus",
    "semanticscholar",
    "unpaywall",
];

const WEB: &[&str] = &[
    "4get",
    "brave",
    "cc",
    "duckduckgo-instant",
    "exa",
    "firecrawl",
    "gbooks",
    "googlecse",
    "grepapp",
    "hn",
    "ia-search",
    "jina",
    "kiwix",
    "linkup",
    "marginalia",
    "mwmbl",
    "openverse",
    "rss-bridge",
    "scrape",
    "searchapi",
    "searxng",
    "serpapi",
    "serper",
    "sourcegraph",
    "stackexchange",
    "tavily",
    "wayback",
    "wiki",
    "wiby",
];

const DATA: &[&str] = &[
    "alphafold",
    "arcgis",
    "biomodels",
    "chembl",
    "ckan",
    "clinicaltrials",
    "cmr",
    "cod",
    "copernicus",
    "dataeuropa",
    "datacite",
    "dataone",
    "dataverse",
    "dryad",
    "ena",
    "ensembl",
    "entrez",
    "erddap",
    "fdsn",
    "figshare",
    "gbif",
    "geonames",
    "gosat",
    "goes-live",
    "hapi",
    "huggingface",
    "interpro",
    "lpf",
    "materialsproject",
    "nasa-images",
    "oai-pmh",
    "open-meteo",
    "openfda",
    "opensn",
    "osf",
    "overpass",
    "pangaea",
    "pdb",
    "pubchem",
    "reactome",
    "re3data",
    "seanoe",
    "sparql",
    "stac",
    "tap",
    "uniprot",
    "usgs",
    "vires",
    "zenodo",
];

pub fn parse(name: &str) -> Option<Group> {
    match name {
        "all" => Some(Group::All),
        "astro" => Some(Group::Astro),
        "literature" => Some(Group::Literature),
        "web" => Some(Group::Web),
        "data" => Some(Group::Data),
        _ => None,
    }
}

pub fn valid(name: &str) -> bool {
    parse(name).is_some()
}

fn members(group: Group) -> &'static [&'static str] {
    match group {
        Group::All => &[],
        Group::Astro => ASTRO,
        Group::Literature => LITERATURE,
        Group::Web => WEB,
        Group::Data => DATA,
    }
}

pub fn allows(mode: &str, name: &str) -> bool {
    match parse(name) {
        Some(Group::All) => true,
        Some(group) => members(group).contains(&mode),
        None => name.is_empty(),
    }
}

pub fn filter<'a>(modes: impl IntoIterator<Item = &'a str>, name: &str) -> Vec<&'a str> {
    modes
        .into_iter()
        .filter(|mode| allows(mode, name))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn astro_group_holds_astro_arms_only() {
        for mode in [
            "alerce", "fink", "dust", "antares", "simbad", "vizier", "ned", "mast", "gaia",
            "horizons", "ads", "heasarc",
        ] {
            assert!(allows(mode, "astro"), "{mode}");
        }
        assert!(!allows("openalex", "astro"));
        assert!(!allows("searxng", "astro"));
        assert!(!allows("zenodo", "astro"));
    }

    #[test]
    fn literature_group_holds_the_literature_arms() {
        for mode in [
            "openalex",
            "crossref",
            "semanticscholar",
            "europepmc",
            "pubmed",
            "ads",
            "base",
            "core",
            "doaj",
            "arxiv",
            "openaire",
        ] {
            assert!(allows(mode, "literature"), "{mode}");
        }
        assert!(!allows("alerce", "literature"));
        assert!(!allows("zenodo", "literature"));
    }

    #[test]
    fn web_group_and_data_group_are_split() {
        for mode in [
            "4get",
            "searxng",
            "mwmbl",
            "marginalia",
            "serper",
            "brave",
            "exa",
            "tavily",
            "searchapi",
            "serpapi",
            "googlecse",
        ] {
            assert!(allows(mode, "web"), "{mode}");
        }
        for mode in [
            "zenodo",
            "datacite",
            "figshare",
            "pangaea",
            "dataone",
            "dataverse",
            "seanoe",
            "ckan",
            "cmr",
        ] {
            assert!(allows(mode, "data"), "{mode}");
        }
        assert!(!allows("openalex", "data"));
        assert!(!allows("simbad", "web"));
    }

    #[test]
    fn all_and_empty_name_keep_every_arm() {
        assert!(allows("anything", "all"));
        assert!(allows("anything", ""));
        assert!(!allows("anything", "unknown"));
        assert!(valid("astro"));
        assert!(!valid("unknown"));
    }

    #[test]
    fn filter_keeps_rank_order_and_drops_foreign_arms() {
        let modes = ["openalex", "alerce", "crossref", "simbad", "zenodo"];
        assert_eq!(filter(modes, "astro"), vec!["alerce", "simbad"]);
        assert_eq!(filter(modes, "literature"), vec!["openalex", "crossref"]);
        assert_eq!(filter(modes, "all").len(), modes.len());
    }
}
