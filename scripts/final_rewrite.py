#!/usr/bin/env python3
"""Final comprehensive rewrite of sources.φ:
1. omegaflow/catalogs → omegaflow/sources
2. catalogs-{domain} → {domain} (drop prefix)
3. github.com-omegaflow-catalogs → correct API domain (born-CDN astro)
Usage: python3 scripts/final_rewrite.py [--dry-run]
"""
import re, sys

SOURCE_PHI = "phi/sources.φ"
DRY_RUN = "--dry-run" in sys.argv

# Domain mappings from git archaeology (363 sphere/kp/fcc/vizier fixes)
SPHERE_FIX = {}
with open("/tmp/opencode/final_domain_map.tsv") as f:
    for line in f:
        parts = line.strip().split("\t")
        if len(parts) >= 2:
            SPHERE_FIX[parts[0]] = parts[1]

# Born-CDN astro catalogs → API domain
# Based on data origin: file names in commit 8dad557 + TAP_MARKERS + build comments
BORN_CDN_DOMAINS = {
    # HEASARC TAP → heasarc.gsfc.nasa.gov
    "astro_batse_grb": "heasarc.gsfc.nasa.gov",
    "astro_batse_pulsar": "heasarc.gsfc.nasa.gov",
    "astro_chandra_master": "heasarc.gsfc.nasa.gov",
    "astro_fermi_4fgl": "heasarc.gsfc.nasa.gov",
    "astro_fermi_4lac": "heasarc.gsfc.nasa.gov",
    "astro_heasarc_icecube": "heasarc.gsfc.nasa.gov",
    "astro_integral_agn": "heasarc.gsfc.nasa.gov",
    "astro_integral_bsc": "heasarc.gsfc.nasa.gov",
    "astro_integral_isgri4yr": "heasarc.gsfc.nasa.gov",
    "astro_integral_var": "heasarc.gsfc.nasa.gov",
    "astro_maxi_gsc7yr": "heasarc.gsfc.nasa.gov",
    "astro_maxi_master": "heasarc.gsfc.nasa.gov",
    "astro_maxi_ssc": "heasarc.gsfc.nasa.gov",
    "astro_nicer_master": "heasarc.gsfc.nasa.gov",
    "astro_rxte_master": "heasarc.gsfc.nasa.gov",
    "astro_rxte_slew": "heasarc.gsfc.nasa.gov",
    "astro_swift_bat": "heasarc.gsfc.nasa.gov",
    "astro_swift_grb": "heasarc.gsfc.nasa.gov",
    "astro_tevcat": "heasarc.gsfc.nasa.gov",
    "astro_xmm_slew": "heasarc.gsfc.nasa.gov",
    "astro_xmm_sources": "heasarc.gsfc.nasa.gov",
    # Gaia TAP → gea.esac.esa.int
    "astro_gaia_binaries": "gea.esac.esa.int",
    "astro_gaia_blue_supergiants": "gea.esac.esa.int",
    "astro_gaia_bright_vmag7": "gea.esac.esa.int",
    "astro_gaia_bright_vmag9": "gea.esac.esa.int",
    "astro_gaia_giants": "gea.esac.esa.int",
    "astro_gaia_high_pm": "gea.esac.esa.int",
    "astro_gaia_hypervelocity": "gea.esac.esa.int",
    "astro_gaia_metal_poor": "gea.esac.esa.int",
    "astro_gaia_nearby_plx10": "gea.esac.esa.int",
    "astro_gaia_nearby_plx50": "gea.esac.esa.int",
    "astro_gaia_white_dwarfs": "gea.esac.esa.int",
    # VizieR TAP → tapvizier.cds.unistra.fr
    "astro_vizier_2qz": "tapvizier.cds.unistra.fr",
    "astro_vizier_6dfgs": "tapvizier.cds.unistra.fr",
    "astro_vizier_apass": "tapvizier.cds.unistra.fr",
    "astro_vizier_carmenes": "tapvizier.cds.unistra.fr",
    "astro_vizier_cosmic_voids": "tapvizier.cds.unistra.fr",
    "astro_vizier_distant_halo": "tapvizier.cds.unistra.fr",
    "astro_vizier_dla": "tapvizier.cds.unistra.fr",
    "astro_vizier_first": "tapvizier.cds.unistra.fr",
    "astro_vizier_gcvs_variables": "tapvizier.cds.unistra.fr",
    "astro_vizier_glade": "tapvizier.cds.unistra.fr",
    "astro_vizier_glade2": "tapvizier.cds.unistra.fr",
    "astro_vizier_gleam": "tapvizier.cds.unistra.fr",
    "astro_vizier_hecate": "tapvizier.cds.unistra.fr",
    "astro_vizier_hi4pi": "tapvizier.cds.unistra.fr",
    "astro_vizier_lotss_dr3": "tapvizier.cds.unistra.fr",
    "astro_vizier_nvss": "tapvizier.cds.unistra.fr",
    "astro_vizier_orion_region": "tapvizier.cds.unistra.fr",
    "astro_vizier_prigozhin_pulsars": "tapvizier.cds.unistra.fr",
    "astro_vizier_qso_variable": "tapvizier.cds.unistra.fr",
    "astro_vizier_sdss_qso": "tapvizier.cds.unistra.fr",
    "astro_vizier_snrs": "tapvizier.cds.unistra.fr",
    "astro_vizier_tycho2": "tapvizier.cds.unistra.fr",
    "astro_vizier_ucac5": "tapvizier.cds.unistra.fr",
    "astro_vizier_variable_stars": "tapvizier.cds.unistra.fr",
    "astro_vizier_vsx_variables": "tapvizier.cds.unistra.fr",
    "astro_vizier_xmm4d13s": "tapvizier.cds.unistra.fr",
    "astro_vizier_young_stars": "tapvizier.cds.unistra.fr",
    "astro_frb_chime": "tapvizier.cds.unistra.fr",
    "astro_frb_catalog_aa": "tapvizier.cds.unistra.fr",
    "astro_frb_catalog_pasa": "tapvizier.cds.unistra.fr",
    # SDSS → skyserver.sdss.org
    "astro_sdss_galaxies": "skyserver.sdss.org",
    "astro_sdss_stars": "skyserver.sdss.org",
    # IRSA → irsa.ipac.caltech.edu
    "astro_wise_allsky": "irsa.ipac.caltech.edu",
    "astro_twomass_psc": "irsa.ipac.caltech.edu",
    # NMDB → nmdb.eu
    "astro_nmdb_stations": "nmdb.eu",
    # JPL → ssd-api.jpl.nasa.gov
    "astro_jpl_asteroids": "ssd-api.jpl.nasa.gov",
    # SIMBAD→Gaia remaps (file contents are Gaia data)
    "astro_simbad": "gea.esac.esa.int",
    "astro_simbad_bright_stars": "gea.esac.esa.int",
    "astro_simbad_nearby_stars": "gea.esac.esa.int",
    "astro_simbad_barium_stars": "gea.esac.esa.int",
    "astro_simbad_blue_giants": "gea.esac.esa.int",
    "astro_simbad_blue_supergiants": "gea.esac.esa.int",
    "astro_simbad_brown_dwarfs": "gea.esac.esa.int",
    "astro_simbad_carbon_stars": "gea.esac.esa.int",
    "astro_simbad_carbon_stars_var": "gea.esac.esa.int",
    "astro_simbad_ch_stars": "gea.esac.esa.int",
    "astro_simbad_emission_stars": "gea.esac.esa.int",
    "astro_simbad_giant_stars": "gea.esac.esa.int",
    "astro_simbad_herbig_haro": "gea.esac.esa.int",
    "astro_simbad_pre_ms_stars": "gea.esac.esa.int",
    "astro_simbad_proper_motion": "gea.esac.esa.int",
    "astro_simbad_pulsating_white_dwarfs": "gea.esac.esa.int",
    "astro_simbad_red_giants": "gea.esac.esa.int",
    "astro_simbad_red_supergiants": "gea.esac.esa.int",
    "astro_simbad_s_stars": "gea.esac.esa.int",
    "astro_simbad_stellar_associations": "gea.esac.esa.int",
    "astro_simbad_ttauri_stars": "gea.esac.esa.int",
    "astro_simbad_white_dwarfs": "gea.esac.esa.int",
    "astro_simbad_wolf_rayet_stars": "gea.esac.esa.int",
    "astro_simbad_yellow_supergiants": "gea.esac.esa.int",
    "astro_simbad_young_stellar_objects": "gea.esac.esa.int",
    "astro_simbad_yso": "gea.esac.esa.int",
    # SIMBAD→VizieR remaps (file contents are VizieR data)
    "astro_simbad_agn": "tapvizier.cds.unistra.fr",
    "astro_simbad_blazars": "tapvizier.cds.unistra.fr",
    "astro_simbad_blue_compact_dwarfs": "tapvizier.cds.unistra.fr",
    "astro_simbad_cepheids": "tapvizier.cds.unistra.fr",
    "astro_simbad_dark_nebulae": "tapvizier.cds.unistra.fr",
    "astro_simbad_dwarf_galaxies": "tapvizier.cds.unistra.fr",
    "astro_simbad_dwarf_novae": "tapvizier.cds.unistra.fr",
    "astro_simbad_eclipsing_binaries": "tapvizier.cds.unistra.fr",
    "astro_simbad_flare_stars": "tapvizier.cds.unistra.fr",
    "astro_simbad_fu_oris": "tapvizier.cds.unistra.fr",
    "astro_simbad_galaxies": "tapvizier.cds.unistra.fr",
    "astro_simbad_galaxy_clusters": "tapvizier.cds.unistra.fr",
    "astro_simbad_gamma_bursts": "tapvizier.cds.unistra.fr",
    "astro_simbad_globular_clusters": "tapvizier.cds.unistra.fr",
    "astro_simbad_gravitational_lenses": "tapvizier.cds.unistra.fr",
    "astro_simbad_hii_regions": "tapvizier.cds.unistra.fr",
    "astro_simbad_hvc": "tapvizier.cds.unistra.fr",
    "astro_simbad_interstellar_medium": "tapvizier.cds.unistra.fr",
    "astro_simbad_ir_sources": "tapvizier.cds.unistra.fr",
    "astro_simbad_ism": "tapvizier.cds.unistra.fr",
    "astro_simbad_long_period": "tapvizier.cds.unistra.fr",
    "astro_simbad_lpv_mira": "tapvizier.cds.unistra.fr",
    "astro_simbad_masers": "tapvizier.cds.unistra.fr",
    "astro_simbad_microquasars": "tapvizier.cds.unistra.fr",
    "astro_simbad_millisecond_pulsars": "tapvizier.cds.unistra.fr",
    "astro_simbad_mira_variables": "tapvizier.cds.unistra.fr",
    "astro_simbad_molecule_clouds": "tapvizier.cds.unistra.fr",
    "astro_simbad_nebulae": "tapvizier.cds.unistra.fr",
    "astro_simbad_novae": "tapvizier.cds.unistra.fr",
    "astro_simbad_oh_maser": "tapvizier.cds.unistra.fr",
    "astro_simbad_oh_masers": "tapvizier.cds.unistra.fr",
    "astro_simbad_planetary_nebulae": "tapvizier.cds.unistra.fr",
    "astro_simbad_post_agb_stars": "tapvizier.cds.unistra.fr",
    "astro_simbad_pulsars": "tapvizier.cds.unistra.fr",
    "astro_simbad_quasars": "tapvizier.cds.unistra.fr",
    "astro_simbad_rcrb_variables": "tapvizier.cds.unistra.fr",
    "astro_simbad_reflection_nebulae": "tapvizier.cds.unistra.fr",
    "astro_simbad_rr_lyrae": "tapvizier.cds.unistra.fr",
    "astro_simbad_supernova_remnants": "tapvizier.cds.unistra.fr",
    "astro_simbad_supernovae": "tapvizier.cds.unistra.fr",
    "astro_simbad_symbiotic_stars": "tapvizier.cds.unistra.fr",
    "astro_simbad_variable_stars": "tapvizier.cds.unistra.fr",
    "astro_simbad_xray_sources": "tapvizier.cds.unistra.fr",
    # Exosphere SIMBAD remaps
    "exosphere_simbad_brown_dwarfs": "gea.esac.esa.int",
    "exosphere_simbad_carbon_stars": "gea.esac.esa.int",
    "exosphere_simbad_white_dwarfs": "gea.esac.esa.int",
    "exosphere_simbad_wolf_rayet": "gea.esac.esa.int",
    "exosphere_simbad_young_stellar_objects": "gea.esac.esa.int",
    "exosphere_simbad_eclipsing_binaries": "tapvizier.cds.unistra.fr",
    "exosphere_simbad_galaxies": "tapvizier.cds.unistra.fr",
    "exosphere_simbad_galaxy_clusters": "tapvizier.cds.unistra.fr",
    "exosphere_simbad_high_z_galaxies": "tapvizier.cds.unistra.fr",
    "exosphere_simbad_highest_redshift_quasar": "tapvizier.cds.unistra.fr",
    "exosphere_simbad_millisecond_pulsars": "tapvizier.cds.unistra.fr",
    "exosphere_simbad_novae": "tapvizier.cds.unistra.fr",
    "exosphere_simbad_pulsars": "tapvizier.cds.unistra.fr",
    "exosphere_simbad_quasars": "tapvizier.cds.unistra.fr",
    "exosphere_simbad_supernovae": "tapvizier.cds.unistra.fr",
    "exosphere_simbad_symbiotic_stars": "tapvizier.cds.unistra.fr",
    # Legacy sources still on bare 'catalogs' release (pre-domain migration)
    "geosphere_noaa_coops_water_levels_usa": "tidesandcurrents.noaa.gov",
    "geosphere_esa_cci_soil_moisture_erddap": "erddap.emodnet-physics.eu",
}


def domain_to_tag(domain):
    return domain.replace("/", "-")


def build_full_domain_map(sources_phi_path):
    """Build a complete {source_name: correct_bare_domain_tag} from current sources.φ.
    Derives the correct domain for every source by checking the sphere fix map,
    born-CDN map, and the current domain tag."""
    full_map = {}
    lines = open(sources_phi_path).readlines()
    cur_source = None
    
    for line in lines:
        if line.startswith("source "):
            cur_source = line.strip().split()[1]
            continue
        
        if cur_source and line.startswith("url ") and "releases/download/" in line:
            m = re.search(r"catalogs-([^/]+)/", line)
            m_bare = re.search(r"releases/download/catalogs/", line)
            if m:
                current_tag = m.group(1)
            elif m_bare:
                current_tag = "catalogs"  # legacy bare release
            else:
                cur_source = None
                continue
            
            # Priority: sphere_fix > born_cdn > current_tag
            if cur_source in SPHERE_FIX:
                correct_domain = domain_to_tag(SPHERE_FIX[cur_source])
            elif cur_source in BORN_CDN_DOMAINS:
                correct_domain = domain_to_tag(BORN_CDN_DOMAINS[cur_source])
            else:
                # Keep current domain but drop the catalogs- prefix (which we drop anyway)
                # For now, the "correct" domain is the current tag minus catalogs-
                # Wait — for already-correct sources, current tag IS the domain
                # After the prefix drop, the tag becomes bare {domain}
                # But the tag already has the domain. Just use current tag as-is.
                # The prefix stripping happens during URL rewrite.
                correct_domain = current_tag  # will become bare domain
                # BUT: for github.com variants, keep as-is
                # For regular domains, keep as-is
            
            full_map[cur_source] = correct_domain
            cur_source = None
    
    return full_map


def rewrite_sources(domain_map, dry=False):
    """Rewrite sources.φ line-by-line.
    Transformations in ONE pass:
    1. omegaflow/catalogs → omegaflow/sources
    2. releases/download/catalogs-{tag}/ → releases/download/{domain}/
    3. releases/download/catalogs/ → releases/download/{domain}/ (legacy bare release)
    4. raw.githubusercontent.com/omegaflow/catalogs → omegaflow/sources
    """
    lines = open(SOURCE_PHI).readlines()
    count = 0
    current_source = None
    
    for i in range(len(lines)):
        line = lines[i]
        
        if line.startswith("source "):
            current_source = line.strip().split()[1]
            continue
        
        if current_source is None:
            continue
        
        if not line.startswith("url "):
            continue
        
        # Handle raw.githubusercontent.com URLs (just repo rename)
        if "raw.githubusercontent.com/omegaflow/catalogs" in line:
            new_line = line.replace("omegaflow/catalogs", "omegaflow/sources")
            if new_line != line:
                if dry:
                    print(f"  {current_source}: raw CDN repo rename")
                    count += 1
                else:
                    lines[i] = new_line
                    count += 1
            continue
        
        if "releases/download/" not in line:
            continue
        
        # Transformation 1: repo name
        new_line = line.replace("omegaflow/catalogs", "omegaflow/sources")
        
        # Determine correct domain tag
        if current_source in domain_map:
            correct_tag = domain_map[current_source]
        else:
            # Keep current tag, stripping catalogs- prefix
            m = re.search(r"catalogs-?([^/]*)/", new_line)
            if m:
                old_tag = m.group(1)
                correct_tag = old_tag if old_tag else "catalogs"
            else:
                # No catalogs match — probably already bare
                lines[i] = new_line
                continue
        
        # Handle bare 'catalogs' release → replace with domain
        if "releases/download/catalogs/" in new_line:
            new_line = new_line.replace("releases/download/catalogs/", f"releases/download/{correct_tag}/", 1)
        # Handle catalogs-{tag} → {tag}
        elif "releases/download/catalogs-" in new_line:
            m = re.search(r"catalogs-([^/]+)/", new_line)
            if m:
                old_tag = m.group(1)
                new_line = new_line.replace(f"catalogs-{old_tag}/", f"{correct_tag}/", 1)
        
        if new_line == line:
            continue
        
        if dry:
            old_ref = line.strip()[4:90]
            new_ref = new_line.strip()[4:90]
            print(f"  {current_source}: {old_ref} → {new_ref}")
            count += 1
            continue
        
        lines[i] = new_line
        count += 1
    
    # Update comments referencing old repo
    for i in range(len(lines)):
        if "# All serve from raw.githubusercontent.com/omegaflow/catalogs/main/" in lines[i]:
            lines[i] = lines[i].replace("omegaflow/catalogs", "omegaflow/sources")
    
    if not dry and count > 0:
        open(SOURCE_PHI, "w").writelines(lines)
    
    return count
    
    if not dry and count > 0:
        open(SOURCE_PHI, "w").writelines(lines)
    
    return count


def main():
    # Build domain map
    print("Building domain map...", file=sys.stderr)
    domain_map = build_full_domain_map(SOURCE_PHI)
    
    # Count unique domains
    unique_domains = set(domain_map.values())
    print(f"Sources mapped: {len(domain_map)}, unique domains: {len(unique_domains)}", file=sys.stderr)
    
    # Verify no catalogs- remnants in map values
    catalogs_values = [v for v in domain_map.values() if v.startswith("catalogs-")]
    if catalogs_values:
        print(f"WARNING: {len(catalogs_values)} catalogs- prefixes in map!", file=sys.stderr)
    
    # Rewrite
    count = rewrite_sources(domain_map, dry=DRY_RUN)
    action = "Would rewrite" if DRY_RUN else "Rewrote"
    print(f"{action} {count} URLs", file=sys.stderr)
    
    if DRY_RUN:
        return
    
    # Verify final state
    content = open(SOURCE_PHI).read()
    cats_prefix = len(re.findall(r"catalogs-[a-z]", content))
    cats_release = len(re.findall(r"releases/download/catalogs/", content))
    cats_repo = len(re.findall(r"omegaflow/catalogs", content))
    
    if cats_prefix:
        print(f"WARNING: {cats_prefix} catalogs- prefixes remain in URLs", file=sys.stderr)
    if cats_release:
        print(f"WARNING: {cats_release} bare catalogs release refs remain", file=sys.stderr)
    if cats_repo:
        print(f"WARNING: {cats_repo} omegaflow/catalogs references remain", file=sys.stderr)
    
    if not cats_prefix and not cats_release and not cats_repo:
        print("ALL CLEAN: no catalogs- prefix, no catalogs repo refs", file=sys.stderr)


if __name__ == "__main__":
    main()
