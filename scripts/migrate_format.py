#!/usr/bin/env python3
"""Migrate phi/sources.φ to canonical format.

field_in -> field
cmap -> map
tail -> rows last
lat_key -> lat, lon_key -> lon, alt_key -> alt
ra_key -> ra, dec_key -> dec, plx_key -> plx
pmra_key -> pmra, pmdec_key -> pmdec
force moves inline: field <path> <force> <unit>

Usage: python3 scripts/migrate_format.py [--write]
"""
import re
import json
import sys
import time
import collections
import urllib.parse
import urllib.request

SRC = "phi/sources.φ"
OUT = "phi/sources_new_format.φ"
UA = "omegaflow/1.0"
CACHE = "phi/recovery/unit_cache.json"

import os

_URI_UNIT = {
    "deg": "deg", "deg.": "deg", "degree": "deg", "degrees": "deg",
    "degT": "deg", "rad": "rad", "mas": "mas", "mas/yr": "mas_yr",
    "mas/y": "mas_yr", "arcsec": "arcsec",
    "m": "m", "meters": "m", "km": "km", "ft": "ft", "nmi": "nmi",
    "cm": "cm", "mm": "mm", "m/s": "m/s", "m/s2": "m/s2", "mgal": "mGal",
    "kn": "kn", "kn": "kn", "km/s": "km/s", "km/h": "km/h",
    "km/h": "km/h", "mph": "mph",
    "hPa": "hPa", "mb": "hPa", "mbar": "hPa", "pa": "Pa", "mmHg": "mmHg",
    "inHg": "inHg",
    "K": "K", "C": "C", "C": "C", "F": "F", "F": "F",
    "nT": "nT", "uT": "µT", "G": "G",
    "mag": "mag", "Jy": "Jy", "mJy": "mJy", "W/m2": "W/m2",
    "W/m^2": "W/m2", "W": "W", "MW": "MW", "kW": "kW",
    "eV": "eV", "keV": "keV", "MeV": "MeV", "GeV": "GeV", "J": "J",
    "erg/s": "erg/s", "s": "s", "sec": "s", "min": "min", "h": "h",
    "d": "d", "day": "d", "yr": "yr",
    "Hz": "Hz", "kHz": "kHz", "MHz": "MHz", "GHz": "GHz",
    "pc": "pc", "kpc": "kpc", "Mpc": "Mpc", "au": "au", "ly": "ly",
    "M_earth": "M_earth", "R_earth": "R_earth", "M_jup": "M_jup",
    "R_jup": "R_jup", "p/cm3": "p/cm3", "cm-2": "cm-2",
    "kg/m3": "kg/m3", "mg/m3": "mg/m3", "ug/m3": "µg/m3", "µg/m3": "µg/m3",
    "ppm": "ppm", "ppb": "ppb", "pct": "pct", "%": "pct",
    "psu": "psu", "ntu": "ntu", "pH": "pH",
    "km/h": "km/h", "m/yr": "m/yr", "mm/yr": "mm/yr",
    "m3/s": "m3/s", "ft3/s": "ft3/s", "L/s": "L/s",
    "count": "count", "index": "index", "sigma": "sigma",
    "1/yr": "1/yr", "sfu": "sfu", "z": "z", "pc/cm3": "pc/cm3",
    "mjd": "mjd", "iso8601": "iso8601", "unix_s": "unix_s",
    "detection": "detection", "statute_mile": "statute_mile",
    "nm": "nm", "micron": "µm", "um": "µm",
}


def uri_to_unit(uri):
    """Map an API-declared unit string (from TAP metadata / NDBC header) to our
    canonical keyword."""
    if not uri:
        return None
    key = str(uri).strip()
    if key in _URI_UNIT:
        return _URI_UNIT[key]
    low = key.lower()
    # handle parenthesized / compound
    for pat, canon in (
        ("m s-1", "m/s"), ("m.s-1", "m/s"), ("m s**-1", "m/s"),
        ("km s-1", "km/s"), ("kpc", "kpc"), ("deg yr-1", "deg/yr"),
        ("mas yr-1", "mas_yr"), ("mas/yr", "mas_yr"),
        ("w m-2", "W/m2"), ("w m-2 hz-1", "W/m2/Hz"),
        ("j s-1", "W"), ("kg m-3", "kg/m3"), ("g cm-3", "g/cm3"),
        ("erg s-1", "erg/s"), ("10-11 erg/cm2/s", "erg/cm2/s"),
    ):
        if pat in low:
            return canon
    return None


def load_cache():
    try:
        return json.load(open(CACHE))
    except Exception:
        return {}


UNIT_CACHE = load_cache()

try:
    IDX_NAMES = json.load(open("/tmp/idx_names.json"))
except Exception:
    IDX_NAMES = {}

# Native column maps for known-format text/CSV APIs (from actual responses).
# {url_substring: {column_index: (native_name, unit, force)}}
NATIVE_COLS = {
    "firms.modaps.eosdis.nasa.gov": {
        0: ("latitude", "deg", "gravity"), 1: ("longitude", "deg", "gravity"),
        2: ("bright_ti4", "K", "thermal"), 3: ("scan", "km", "em"),
        4: ("track", "km", "em"), 5: ("acq_date", None, None),
        6: ("acq_time", None, None), 7: ("satellite", None, None),
        8: ("confidence", "index", "em"), 9: ("version", None, None),
        10: ("bright_ti5", "K", "thermal"), 11: ("frp", "MW", "thermal"),
        12: ("daynight", None, None),
    },
    "dataserver-coids.inpe.br": {
        0: ("focos", None, None), 1: ("lat", "deg", "gravity"),
        2: ("lon", "deg", "gravity"), 3: ("pais", None, None),
        4: ("estado", None, None), 5: ("municipio", None, None),
        6: ("bioma", None, None), 7: ("deflagrada", None, None),
        8: ("data", None, None), 9: ("tempo", None, None),
        10: ("satelite", None, None), 11: ("satelite_precisao", None, None),
        12: ("frp", "MW", "thermal"),
    },
    "ourairports-data": {
        0: ("id", None, None), 1: ("ident", None, None),
        2: ("type", None, None), 3: ("name", None, None),
        4: ("latitude_deg", "deg", "gravity"),
        5: ("longitude_deg", "deg", "gravity"),
        6: ("elevation_ft", "ft", "gravity"), 7: ("continent", None, None),
        8: ("iso_country", None, None), 9: ("iso_region", None, None),
        10: ("municipality", None, None), 11: ("scheduled_service", None, None),
        12: ("icao_code", None, None), 13: ("iata_code", None, None),
        14: ("gps_code", None, None), 15: ("local_code", None, None),
    },
    "hpiers.obspm.fr/iers/bul/bulb_new/bulletinb.dat": {
        0: ("date", None, None), 1: ("mjd", "mjd", "em"),
        2: ("x_pole", "mas", "em"), 3: ("y_pole", "mas", "em"),
        4: ("ut1_utc", "ms", "em"), 5: ("dx", "mas", "em"),
        6: ("dy", "mas", "em"), 7: ("x_err", "mas", "em"),
    },
    "globalcmt.org": {
        0: ("date", None, None), 1: ("time", None, None),
        2: ("lat", "deg", "gravity"), 3: ("lon", "deg", "gravity"),
        4: ("depth_km", "km", "gravity"), 5: ("mb", "mag", "seismic-body"),
        6: ("ms", "mag", "seismic-surface"), 7: ("mw", "mag", "seismic-surface"),
    },
}

NCEI = {
    "EMXP": "mm", "EMXT": "C", "EMNT": "C",
    "PRCP": "mm", "SNOW": "mm", "SNWD": "mm",
    "TAVG": "C", "TMAX": "C", "TMIN": "C", "TOBS": "C",
    "AWND": "m/s", "WDF2": "deg", "WDF5": "deg", "WSF2": "m/s", "WSF5": "m/s",
    "DX32": "d", "DT32": "d", "DX70": "d", "DX90": "d",
    "DP01": "d", "DP05": "d", "DP10": "d",
    "DYSD": "d", "DYSN": "d", "DYTS": "d", "DYTG": "d",
    "CDSD": "d", "CLDD": "d", "HDSD": "d", "HTDD": "d",
    "TSUN": "min", "PSUN": "pct", "EVAP": "mm",
}

NDBC = {
    "WDIR": "deg", "WSPD": "m/s", "GST": "m/s", "WVHT": "m",
    "DPD": "s", "APD": "s", "MWD": "deg", "PRES": "hPa",
    "ATMP": "C", "WTMP": "C", "DEWP": "C",
    "VIS": "nmi", "PTDY": "hPa", "TIDE": "ft",
    "SwH": "m", "SwP": "s", "WWH": "m", "WWP": "s",
    "SwD": "deg", "WWD": "deg", "BAR": "hPa",
}

SWPC = {
    "proton_speed": "km/s", "proton_temperature": "K", "proton_density": "p/cm3",
    "proton_vx_gse": "km/s", "proton_vy_gse": "km/s", "proton_vz_gse": "km/s",
    "proton_vx_gsm": "km/s", "proton_vy_gsm": "km/s", "proton_vz_gsm": "km/s",
    "bt": "nT", "bz_gsm": "nT", "by_gsm": "nT", "bx_gsm": "nT",
    "flux": "W/m2", "observed_flux": "W/m2",
    "ssn": "count", "smoothed_ssn": "count", "observed_swpc_ssn": "count",
    "f10.7": "sfu", "smoothed_f10.7": "sfu",
}

STRUCTURAL = {
    "hex", "flight", "name", "type", "id", "title", "designation", "obsid",
    "station", "status", "place", "country", "date", "time", "time_tag",
    "timestamp_utc", "local_date_time", "local_date_time_full", "utc_offset", "recno",
    "code", "symbol", "href", "link", "ref", "reference", "url", "text", "summary",
    "description", "label", "category", "emergency", "spi", "sil_type", "locale",
    "timezone", "location", "objectid", "sitename", "site_name", "station_name",
    "network", "channel", "instrument", "units", "source_type", "assoc_name", "state",
    "region", "city", "iso", "fips", "gender", "species", "genus", "family", "order",
    "class", "phylum", "kingdom", "scientific_name", "common_name",
    "vernacular_name", "scientificname", "basis_of_record", "occurrence_status",
    "event_date", "recorded_by", "day", "month", "year", "hour", "minute", "second",
    "version", "revision", "geometry_type", "mag_type", "alert", "tsunami",
    "event_type", "event_id", "detail", "felt", "cdi", "mmi", "rms", "nst", "dmin",
    "gap", "net", "updated", "horizontal_error", "vertical_error", "depth_error",
    "magnitude_error", "line1", "line2", "line3", "line4", "sat", "satellite",
    "qualifier", "count", "number", "total", "index", "variable",
    "magtype",
    "province", "district", "town", "village", "municipality", "filter", "detector",
    "telescope", "observatory", "config", "mode", "submode", "resolution", "quality",
    "flag", "flags", "warning", "caution", "notes", "comment", "schedule", "route",
    "carrier", "operator", "language", "currency", "continent", "dataset", "doi",
    "citation", "author", "publisher", "phone", "email", "address", "website",
    "contact", "origintime", "iscancel", "isfinal", "publicid", "locality",
    "flynn_region", "evid", "target_name", "target_name_2", "color", "spectral_type",
    "geometry", "target", "identifier", "handle", "accession", "contributor",
    "creator", "instrument_name", "facility", "method", "technique", "protocol",
    "annotation", "remark", "comment_text", "seq_num", "row_index", "time_end",
    "time_start", "start_time", "end_time", "data_release", "release_version",
    "release", "survey", "project", "mission", "alt_baro", "baro_rate", "track",
    "icaoid", "icao_id", "airport_id", "airport", "altim", "radialvelocityerr",
    "magrms", "distmod", "decl", "equinox", "analysis_flags", "earliestyearbp",
    "mostrecentyearbp", "totalresults", "disc_year", "oid", "class1", "filterid",
    "field", "gwgc", "pgc", "ugc", "volcano_number", "volcano", "sitedescription",
    "n_events", "event", "orbit", "majaxis", "receipttime", "reporttime", "metartype",
    "rawob", "qcfield", "begin_datetime", "end_datetime", "begin_quality",
    "max_quality", "end_quality", "observatory", "max_datetime", "format",
    "coded_type", "electron_contaminaton", "active", "source", "comments", "iso3",
    "shortname", "wxstring", "observationid", "observation_id",
        "energy", "satellite",
        "hyperleda", "maj", "max", "min", "source_number",
        "energy_bandpassname", "e_raj2000", "e_dej2000",
        "name", "2mass", "jname", "seq", "trigger_num", "qflg",
        "otype_txt", "parallax", "parallax_error", "plx_err",
        "ra_error", "dec_error", "pmra_error", "pmdec_error",
        "grating", "cycle", "proposal", "sequence_number", "data_mode",
        "mean", "far", "semi_major_axis_68", "semi_minor_axis_68",
        "position_angle_68", "semi_major_axis_95", "semi_minor_axis_95",
        "position_angle_95", "source_name", "object", "target",
        "alt_gammaray_name_1", "alt_gammaray_name_2", "alt_gammaray_name_3",
        "alt_gammaray_name_4", "alt_gammaray_name_5", "alt_gammaray_name_6",
        "npred", "lp_epeak", "lp_epeak_error", "plec_epeak", "plec_epeak_error",
        "plec_exp_factor_s", "plec_exp_factor_s_error", "time_peak",
        "time_peak_error", "tevcat_assoc", "source_type_alt", "assoc_name_alt",
        "assoc_catalog", "sed_class", "he_peak", "he_peak_error",
        "he_nufnu_peak", "he_nufnu_peak_error", "specobjid", "zerr",
        "widthfitb", "spind", "excess_variance", "plate", "fiberid",
        "ctrpart_class", "slew_info", "bat_detection", "bat_dettype",
        "redshift_err", "redshift_line", "redshift_from", "followup",
        "radio_detection", "infra_detection", "opt_detection",
        "radio_ref", "infra_ref", "opt_ref", "ot_pos_ref",
        "other_obs", "other_obs2", "other_obs3", "other_obs4",
        "other_obs_ref", "other_obs2_ref", "other_obs3_ref",
        "other_obs4_ref", "web_page", "srcid", "pps_srcnum",
        "n_obs", "n_contrib", "n_exp", "ccdpn", "ccdm1", "ccdm2",
        "xmm_revolution", "classopt_class", "classx_class",
        "redshift_tpz_flag_zconf", "redshift_lph_flag_zconf",
        "redshift_from", "variable_name", "variablename", "starttime",
        "redshift_type", "redshift_ref", "redshift_line", "level",
        "stormname", "basin", "lastupdated", "severity", "date_",
        "anomaly", "uncertainty", "variable_name", "seq",
        "time_trigger", "node", "2qz", "intno", "intname", "nobs",
        "minaxis", "maxaxis", "s1_4", "incl", "first", "sdss-dr12",
        "flag1", "flag2", "flag3", "mobs", "p(s)", "t_0", "q", "s",
        "ned", "tyc1", "tyc2", "tyc3", "pflag", "num", "xpos", "ypos",
        "epram", "epdem", "eq", "recno", "max", "min",
        "prox", "tyc", "hip", "ccdm", "epra-1990", "epde-1990", "posflg",
        "corr", "srcidgaia", "org", "nu", "epucac", "sdss", "sptype",
        "mwsc", "bckwide", "lrmswide", "epos", "ctot", "filename", "sname",
        "investigator", "speciescode", "varnum", "m_varnum", "gcvs",
        "processid", "bin", "stateprovince", "occurrencedate", "rflg",
        "bflg", "cflg", "aflg", "pubdate", "georss:point", "volcanoname",
        "stationidentifier", "message", "stormtype", "advdate", "advisnum",
        "stormnum", "fcstprd", "county", "incidentname", "hail_size",
        "validtimefrom", "validtimeto", "utc", "labels", "bns", "created",
        "t_scid", "assoc1", "revolution", "angstrom_exponent_440-870nm",
        "obs_collection", "filters", "calib_level", "site", "collection",
        "eo:cloud_cover", "landsat:collection_number", "wnd", "fatalities",
        "observed_on", "startdate", "enddate", "s_dec", "neo",
        "classification", "ext", "ph_qual", "sqf_3_6", "fall",
        "peri", "spinper", "ssi", "toi", "zone", "scan", "s/g", "var_flg",
        "z1", "hr1", "brightness", "vartype", "min1", "alpha", "pa", "va",
        "iphas", "r'-ha", "logt", "r2", "logg", "rapmdeg", "depmdeg",
        "vlsr", "reg", "simbadname", "msz", "y5r500", "mcxc", "star",
        "deaths", "s15ghz", "stot", "ptot", "injuries", "eruption_number",
        "eruption_month", "eruption_day", "tectonic_setting", "vei",
        "date_start", "date_end", "sid", "site_no", "net_slip_rate",
        "aseismic_slip_factor", "v_mean", "observations",
        "stac_eo_cloud_cover", "hist_min", "hist_max", "capacity",
        "primary_fuel", "yn_snr", "peak_dt", "peak_va", "gage_ht",
        "speed_radius", "rigidity_gv", "capacity_mw", "yn_mass",
        "theta500", "i1_f_ap", "i2_f_ap", "i4_f_ap", "f3p6tot", "f4p5tot",
        "f5p8tot", "f8p0tot",         "morph", "extended", "mangrove_area_km2", "pi", "self",
        "source_sample", "6dfgs", "nm", "nz", "p-value", "ugos", "vgos",
        "analysed_sst", "headline", "areadesc", "volcanoid",
        "aviationcolorcode", "total_count", "area_m2", "loss_m2",
        "maxscale", "kp_index", "estimated_kp", "a_running",
        "station_count", "generated", "stid", "currentconditions",
        "multiplicity", "kmdepth", "nidheight", "damheight", "icao",
        "group", "pipeline", "wilayah", "waktu", "dirasakan", "unit",
        "kedalaman", "dateofocc", "subreg", "hostility_d", "victim_d",
        "hostilitytype_l", "location_name_en", "location_name_fr",
        "stationid", "stationurl", "graphurl", "bright_t31", "dtg", "hhmm",
        "tau", "incidenttypecategory", "eventtype", "hoursold", "irwinid",
        "sum_p0010001", "sum_h0010001", "sum_estimatedunder18pop",
        "dailyacres", "sum_estimated18to64pop", "sum_estimated65pluspop",
        "sum_estimated0_14pop", "unitcode", "kenn", "nuclide", "tec",
        "vtec_assimilated_tecu", "vtec_rms_tecu", "movementdir",
        "movementspeed", "rawtaf", "issuetime", "hazard",
        "textdescription", "massgap", "band", "bbh", "ext525", "ext1020",
        "f107", "f107_adj", "tsi", "stdev", "carrington_rotation",
        "coverage",         "satelliteid", "landsat:wrs_path", "landsat:wrs_row", "map",
        "cloud_modification", "observation_uuid", "user_login",
        "observed_on_month", "observation", "alert_level", "threat",
        "stn", "yyyy", "mm", "volcanotitle", "activity", "slug",
        "subregion", "datacoverage", "polygonacres", "cwa",
        "forecastoffice", "forecasthourly", "gridid", "gridx", "gridy",
        "radarstation", "generatedat", "area_km", "uf", "alertlevel",
        "poly_gisacres", "poly_polygondatetime", "floodclass",
        "countrycode", "owner", "pgm", "met", "area", "return_period",
        "doxy", "area_ha", "area_km2", "tle0", "tle1", "tle2",
        "tle_source", "cloudcover", "mass_donor", "mass_bh",
        "assoc_pulsar", "host_galaxy", "sc_hr4", "spin", "subhalo_count",
        "gal_contam", "meanvaluesandf", "maxvaluesandf", "minvaluesandf",
        "descriptionsandf", "fullname", "spkid", "s/n", "average",
        "minimum", "maximum",         "pc:count", "tic", "sectors", "lastburst", "boxes", "claimedtype",
        "objid", "allwise", "assoc", "binary", "f0",         "dr3name",         "oname", "opt", "wise", "cl", "rad", "hiiname",
        "country_name_en", "totaldeath", "totalaffected", "numberinjured",
        "numberhomeless", "zipcode", "stat", "stat_st", "evidence_method",
        "evidence_category", "auth", "drought_class",
        "susceptibility_class", "studyid", "abstract", "season",
        "studyname", "eqmagunk", "damageamountorder",
        "damagemillionsdollars", "melt", "round", "respondent-name",
        "aphiaid", "hab_category",         "cellcount_units", "daynight", "establishmentmeans",
        "mediacount", "recclass", "des", "date_detected", "energy_range",
        "association", "model_cohort", "nominal_resolution", "nsbh",
        "ngoodobsrel",         "lev", "otype", "gdacs:alertlevel", "gdacs:country", "subbasin",
        "eqid", "morphology", "eruptionnumber", "lastknowneruption",
        "occurrencestatus", "scntfcn", "detectn", "cllct_d", "turtleid",
        "loc", "unit_of_measure", "eur", "gbp", "jpy", "features", "imp",
        "activitycategory", "vnum", "dsci", "dmaj",
        "s1ghz", "preferredname", "howmany", "obsdt", "damage_usd",
        "timestamp", "commonname", "chaetoceros", "alexandriu",
        "cochlodini", "country_iso", "gain_m2",
    }

def is_structural(name):
    if name.isdigit():
        return False
    n = name.lower()
    if n in STRUCTURAL:
        return True
    if n.startswith(("nb_", "has_", "is_")):
        return True
    for s in ("_flag", "_code", "_id", "_type", "_status", "_key", "_text",
              "_name", "_date", "_time", "_url", "_link", "_ref", "_year",
              "_datetime", "_version", "_release", "_quality"):
        if n.endswith(s):
            return True
    if re.match(r"^WT\d{2}$", name):
        return True
    return False


# Physical units only — a field survives migration only if it carries a real
# physical quantity. Counts, indices, scalars, codes, flags, timestamps are
# NOT physics: they propagate nothing, occupy no field volume. Stripped.
# Council SI matrix (bindend): allowed units per force
PHYSICAL_UNITS = {
    # em (0)
    "W/m2", "Wm-2", "nT", "uT", "T", "sfu", "mag", "eV", "TeV", "Jy", "mJy",
    "nm", "Hz", "kHz", "MHz", "GHz", "TECU", "pct",
    # gravity (1)
    "M_sun", "M_jup", "M_earth", "kg", "g", "m", "km", "ft", "nmi", "mi",
    "au", "AU", "pc", "kpc", "Mpc", "m2", "km2", "deg", "rad", "mas",
    # acoustic (2)
    "dB", "s",
    # seismic-body (3)
    "mm/yr", "m/yr", "arcsec", "arcmin",
    # seismic-surface (4)
    "°",
    # thermal (5)
    "K", "C", "F",
    # diffusion (6)
    "p/cm3", "cm-3", "1/cm3", "p/m3", "m-3", "kg/m3", "µg/m3", "ug/m3",
    "ppm", "ppb", "psu", "cm-2",
    # advective (7)
    "m/s", "km/s", "km/h", "mph", "kn", "hPa", "Pa", "mb", "mbar",
    "mmHg", "inHg", "d", "yr", "mjd", "deg/yr", "mas_yr",
    # biotic (8)
    "1/min",
}

def physical_unit(name, path, force, url):
    """Return the physical unit for a field, or None (stripped) if the field
    is not a real physical quantity."""
    u = infer_unit(name, path, force, url)
    if u in PHYSICAL_UNITS:
        return u
    return None


# Timestamp/duration metadata that is NOT a physical field quantity: how long
# a sensor integrated, when an observation happened, epoch of a catalogue row.
# These propagate nothing. Stripped. (True physical periods survive.)
def position_directive(tgt, col_name, src, unit, url):
    """If a field is a position coordinate (lat/lon/alt/ra/dec/plx/pm), return
    the 3-token position directive; else None."""
    n = (col_name or tgt).lower()
    src_l = src.lower()
    # only PRIMARY position coordinates become position directives; derived
    # positions (solar_lat, grid_lat, ...) are scalar fields, not the frame.
    if n not in ("latitude", "lat", "longitude", "lon", "lng", "altitude",
                 "elevation", "height", "ra", "dec", "plx", "parallax",
                 "pmra", "pmdec", "pm_ra", "pm_dec", "declination"):
        return None
    # celestial position
    if n in ("ra", "right_ascension") and unit in ("deg", "hms", "rad"):
        return f"ra {src} deg"
    if n in ("dec", "declination") and unit in ("deg", "rad"):
        return f"dec {src} deg"
    if n in ("plx", "parallax") and unit in ("mas", "arcsec"):
        return f"plx {src} mas"
    if n in ("pmra", "pm_ra") and unit == "mas_yr":
        return f"pmra {src} mas_yr"
    if n in ("pmdec", "pm_dec") and unit == "mas_yr":
        return f"pmdec {src} mas_yr"
    # terrestrial position
    if n in ("latitude", "lat") and unit in ("deg", "rad"):
        return f"lat {src} deg"
    if n in ("longitude", "lon", "lng") and unit in ("deg", "rad"):
        return f"lon {src} deg"
    if n in ("altitude", "elevation", "height") and unit in ("m", "km", "ft"):
        u = "km" if ("iss" in url or "km" in src_l) else unit
        return f"alt {src} {u}"
    return None


def domain_physical_lines(url, frame_lines):
    """For known physical sources whose declared fields were too generic or
    whose cache parsing failed, return the correct physical extractor lines.
    These restore wrongly-dropped blocks."""
    lines = []
    if "imag-data.bgs.ac.uk" in url and "/xyzf" in url:
        # BGS magnetometer: HAPI columns X, Y, Z, F in nT
        lines += ["path 1.0 em nT", "path 1.1 em nT", "path 1.2 em nT",
                  "path 2 em nT"]
    elif "api.tidesandcurrents.noaa.gov" in url:
        if "product=water_level" in url:
            lines += ["last data.v gravity m"]
        elif "product=water_temperature" in url:
            lines += ["last data.v thermal C"]
        elif "product=air_temperature" in url:
            lines += ["last data.v thermal C"]
        elif "product=conductivity" in url or "product=specific_conductance" in url:
            lines += ["last data.v diffusion psu"]
        elif "product=salinity" in url:
            lines += ["last data.v diffusion psu"]
        elif "product=currents" in url or "product=wind" in url:
            lines += ["last data.v advective m/s"]
        elif "product=water_level" in url:
            lines += ["last data.v gravity m"]
    elif "waterservices.usgs.gov" in url:
        if "parameterCd=00060" in url:
            lines += ["last value.timeSeries.0.values.0.value.0.value advective m3/s"]
        elif "parameterCd=00065" in url:
            lines += ["last value.timeSeries.0.values.0.value.0.value gravity m"]
        elif "parameterCd=00010" in url:
            lines += ["last value.timeSeries.0.values.0.value.0.value thermal C"]
        elif "parameterCd=00095" in url:
            lines += ["last value.timeSeries.0.values.0.value.0.value diffusion psu"]
        elif "parameterCd=00045" in url:
            lines += ["last value.timeSeries.0.values.0.value.0.value gravity m"]
    elif "xray-flares" in url and "swpc.noaa.gov" in url:
        lines += ["last max_xrlong em W/m2", "last current_int_xrlong em W/m2"]
    elif "pegelonline.wsv.de" in url:
        lines += ["last value gravity cm"]
    elif "geomag.usgs.gov" in url or "gis.ngdc.noaa.gov" in url:
        lines += ["field value em nT"]
    elif "earthquake.usgs.gov" in url and "geojson" in url:
        # USGS quake feeds: magnitude + depth from GeoJSON event properties
        lines += ["geojson events mag 0 seismic_magnitude seismic_depth_m"]
    elif "ovation_aurora" in url:
        lines += ["map coordinates", "lat 0 deg", "lon 1 deg",
                  "field 2 em W/m2"]
    elif "differential-protons" in url:
        lines += ["last flux em p/cm2/s"]
    elif "blitzortung.org" in url:
        lines += ["map .", "lat lat deg", "lon lon deg"]
    elif "heasarc" in url or "xamin" in url:
        # X-ray detector photon rates are frequencies (Hz), not abstract counts
        if "count_rate" in url or "counts" in url or "avg_rate" in url or "rate_err" in url:
            lines += ["field count_rate em Hz"]
        if "isgri_soft_rate" in url or "intbsc" in url:
            lines += ["field isgri_soft_rate em Hz", "field isgri_hard_rate em Hz"]
    elif "smartbay_obs_acoustic" in url:
        # underwater acoustic sensor: sound pressure level in dB
        lines += ["last_row TotalSPL acoustic dB"]
    elif "mars.nasa.gov/rss/api/?feed=weather" in url:
        # Mars rover weather = surface measurements on Mars
        lines += ["on mars 4.59 137.44",
                  "path soles.0.min_temp thermal C",
                  "path soles.0.max_temp thermal C",
                  "path soles.0.pressure acoustic hPa"]
    elif "kp.gfz.de" in url or "superdarn.ca" in url:
        lines += ["field value em scalar"]  # replaced by real inference below
    return lines


def is_time_metadata(name):
    n = name.lower()
    if n.endswith("_count") or n.endswith("_total") or n.endswith("_num") or \
       "count" in n or "total" in n:
        return True
    if n in ("recordedby", "country", "value", "val", "q",
             "superevents", "country.value"):
        return True
    if n in ("mjd", "epoch", "obstime", "t_min", "t_max", "t_start",
             "t_stop", "date", "time", "duration", "exptime", "exposure",
             "t_exptime", "elapsed", "interval", "t0", "t1", "time_start",
             "time_end", "start_time", "end_time", "t_trigger",
             "peak_time", "time_peak", "t_transit", "pl_tranmid",
             "validtimefrom", "validtimeto", "lastupdated", "gpstime",
             "time_tag", "timestamp_utc", "datetime", "date_time",
             "start", "stop", "tmin", "tmax", "tmid", "tstart", "tstop"):
        return True
    if n.startswith(("t_", "start_", "end_")) and (
            n.endswith("time") or n.endswith("date") or n.endswith("s")
            or n.endswith("start") or n.endswith("stop") or n.endswith("t")):
        return True
    if n.endswith(("_duration", "_exposure", "_exptime", "_elapsed",
                   "_interval", "_time", "_date", "_epoch", "_mjd")):
        return True
    return False


# Domain-specific ambiguous field resolution: same name, different unit per API.
DOMAIN_FIELD_UNITS = {
    # JPL SSD asteroid orbital elements
    "ssd-api.jpl.nasa.gov": {
        "a": ("au", "gravity"), "e": ("index", "gravity"), "i": ("deg", "gravity"),
        "H": ("mag", "em"), "M": ("deg", "gravity"), "q": ("au", "gravity"),
        "n": ("deg/yr", "gravity"), "tp": ("mjd", "gravity"), "P": ("yr", "gravity"),
        "vx": ("km/s", "gravity"), "vy": ("km/s", "gravity"), "vz": ("km/s", "gravity"),
        "ma": ("deg", "gravity"), "s": ("index", "gravity"),
    },
    # NASA Exoplanet Archive
    "exoplanetarchive.ipac.caltech.edu": {
        "pl_name": ("?", "structural"), "sy_dist": ("pc", "em"), "pl_orbper": ("d", "gravity"),
        "pl_rade": ("R_earth", "gravity"), "pl_bmasse": ("M_earth", "gravity"),
        "pl_eqt": ("K", "thermal"), "st_teff": ("K", "thermal"), "ra": ("deg", "em"),
        "dec": ("deg", "em"), "pl_orbsmax": ("au", "gravity"), "pl_bmassj": ("M_jup", "gravity"),
        "pl_rvamp": ("m/s", "advective"), "pl_trandep": ("pct", "em"),
        "pl_tranmid": ("mjd", "em"), "pl_trandur": ("s", "em"),
        "pl_orbeccen": ("index", "gravity"), "pl_orbincl": ("deg", "gravity"),
    },
    # IRSA / IPAC cross-match metadata
    "irsa.ipac.caltech.edu": {
        "assoc_ra": ("deg", "em"), "assoc_dec": ("deg", "em"),
        "assoc_error_radius": ("arcsec", "em"), "assoc_prob_bay": ("pct", "em"),
        "assoc_prob_lr": ("pct", "em"), "j_m": ("mag", "em"), "h_m": ("mag", "em"),
        "k_m": ("mag", "em"),
    },
    # SDSS photometric bands -> magnitudes
    "skyserver.sdss.org": {
        "u": ("mag", "em"), "g": ("mag", "em"), "r": ("mag", "em"),
        "i": ("mag", "em"), "z": ("mag", "em"), "a": ("deg", "em"),
        "ra": ("deg", "em"), "dec": ("deg", "em"),
    },
    # NOAA tides & currents datagetter: t=time, v=value(unit by product), f=flag
    "api.tidesandcurrents.noaa.gov": {
        "t": ("iso8601", "structural"), "v": ("???", "???"), "f": ("index", "structural"),
        "d": ("index", "structural"),
    },
    # USGS water services: value depends on the parameter/unit in URL
    "waterservices.usgs.gov": {
        "value": ("???", "???"),
    },
    # SWPC magnetosphere + solar
    "services.swpc.noaa.gov": {
        "bt": ("nT", "em"), "bz": ("nT", "em"), "by": ("nT", "em"), "bx": ("nT", "em"),
        "kp": ("index", "em"), "Kp": ("index", "em"), "extent": ("index", "em"),
        "pi": ("index", "em"), "snr": ("sigma", "em"), "DM": ("pc/cm3", "em"),
        "g": ("index", "em"),
    },
    # BGS magnetometer
    "imag-data.bgs.ac.uk": {
        "X": ("nT", "em"), "Y": ("nT", "em"), "Z": ("nT", "em"),
    },
    # CMEMS ocean model
    "my.cmems-du.eu": {
        "uo": ("m/s", "advective"), "vo": ("m/s", "advective"),
    },
    # sensor.community air quality
    "data.sensor.community": {
        "CO": ("ppm", "diffusion"), "P1": ("µg/m3", "diffusion"), "P2": ("µg/m3", "diffusion"),
    },
    # NASA opendap ozone/atmos
    "opendap.nccs.nasa.gov": {
        "O3": ("ppb", "diffusion"), "RH": ("pct", "diffusion"), "T": ("K", "thermal"),
        "U": ("m/s", "advective"), "QV": ("kg/m3", "diffusion"),
    },
    # ICE velocity (ITS-LIVE)
    "its-live-data.jpl.nasa.gov": {
        "v0": ("m/yr", "advective"),
    },
    # GLOBALCMT moment magnitude
    "www.globalcmt.org": {
        "mw": ("mag", "seismic-surface"), "M": ("mag", "seismic-surface"),
    },
    # NGD C solar/mag
    "www.ngdc.noaa.gov": {
        "bx": ("nT", "em"), "by": ("nT", "em"), "bz": ("nT", "em"), "bt": ("nT", "em"),
        "dy": ("d", "structural"), "mo": ("d", "structural"),
    },
    # SERVIR soil moisture
    "gis1.servirglobal.net": {
        "sm": ("pct", "diffusion"),
    },
    # NASA POWER meteorology parameters
    "power.larc.nasa.gov": {
        "T2M": ("C", "thermal"), "T2M_MAX": ("C", "thermal"),
        "T2M_MIN": ("C", "thermal"), "T2MDEW": ("C", "thermal"),
        "RH2M": ("pct", "diffusion"), "PS": ("kPa", "acoustic"),
        "PRECTOTCORR": ("mm", "diffusion"), "PRECTOT": ("mm", "diffusion"),
        "WS10M": ("m/s", "advective"), "WS10M_MAX": ("m/s", "advective"),
        "WS50M": ("m/s", "advective"), "WD10M": ("deg", "advective"),
        "WD50M": ("deg", "advective"),
        "ALLSKY_SFC_SW_DWN": ("W/m2", "em"), "ALLSKY_SFC_LW_DWN": ("W/m2", "em"),
        "ALLSKY_SFC_SW_UP": ("W/m2", "em"), "ALLSKY_SFC_LW_UP": ("W/m2", "em"),
        "ALLSKY_SRF_ALB": ("pct", "em"), "CLRSKY_SFC_SW_DWN": ("W/m2", "em"),
        "GWETPROF": ("pct", "diffusion"), "GWETTOP": ("pct", "diffusion"),
    },
    # Thai earthquake
    "earthquake.tmd.go.th": {
        "tm": ("mag", "seismic-surface"),
    },
    # Heasarc x-ray
    "heasarc.gsfc.nasa.gov": {
        "lx": ("W", "em"), "LX": ("W", "em"), "pi": ("index", "em"),
        "snr": ("sigma", "em"), "ts": ("s", "em"), "extent": ("index", "em"),
        "DM": ("pc/cm3", "em"),
    },
    # ADS-B ground speed
    "api.adsb.lol": {
        "gs": ("m/s", "advective"),
    },
    # ArcGIS wave buoy
    "services9.arcgis.com": {
        "hs": ("m", "acoustic"),
    },
    # Astrocat
    "api.astrocats.space": {
        "ra": ("deg", "em"),
    },
}


def value_based_unit(name, value, force):
    """Infer unit from the actual API value magnitude + field name."""
    if not isinstance(value, (int, float)):
        return None
    n = name.lower()
    a = abs(value)
    # temperature
    if any(w in n for w in ("temp", "sst", "teff", "dewp", "atmp", "wtmp",
            "bright_ti", "tavg", "tmax", "tmin")):
        return "K" if a > 100 else "C"
    # solar wind speed
    if "proton_speed" in n or "solar_wind" in n:
        return "km/s" if a > 50 else "m/s"
    # speed
    if any(w in n for w in ("speed", "velocity", "spd", "wspd", "wdsp")):
        if a > 1000: return "km/h"   # orbital / aviation km/h
        if a > 100: return "km/s"   # solar wind km/s
        return "m/s"
    # pressure
    if any(w in n for w in ("pres", "baro", "slp", "altimeter", "press")):
        return "Pa" if a > 10000 else "hPa"
    # magnetic
    if any(w in n for w in ("_bt", "bz", "by", "bx", "mag_field", "nT")):
        return "nT"
    # flux / irradiance
    if any(w in n for w in ("flux", "irrad", "radiation", "fpeak", "fint",
            "xray")):
        return "W/m2"
    # magnitude
    if any(w in n for w in ("mag", "magnitude")):
        return "mag"
    # altitude/elevation
    if any(w in n for w in ("alt", "elev", "height", "depth", "altitude")):
        return "m"
    # density
    if "density" in n:
        return "p/cm3" if a < 1000 else "kg/m3"
    # frequency
    if any(w in n for w in ("hz", "freq", "frequency")):
        return "Hz"
    # period
    if any(w in n for w in ("period", "orbper")):
        return "d"
    # angles
    if any(w in n for w in ("dir", "bearing", "azimuth", "wdir")):
        return "deg"
    # percentage
    if any(w in n for w in ("humidity", "pct", "percent", "rh", "prob")):
        return "pct"
    # power
    if any(w in n for w in ("power", "watt", "mw")):
        return "MW" if a < 10000 else "W"
    return None


def cache_lookup(url, field_name):
    """Look up field unit/force from the API cache (ground truth)."""
    entry = UNIT_CACHE.get(url)
    if not isinstance(entry, dict) or entry.get("error"):
        return None, None
    # header_units from NDBC text
    if "header_units" in entry:
        hu = entry["header_units"]
        if field_name.upper() in hu:
            u = uri_to_unit(hu[field_name.upper()])
            if u:
                return u, None
    if "csv_columns" in entry:
        cols = entry["csv_columns"]
        if field_name.upper() in cols:
            return None, None
    # named field with unit metadata
    if field_name in entry and isinstance(entry[field_name], dict):
        meta = entry[field_name]
        if meta.get("unit"):
            u = uri_to_unit(meta["unit"])
            if u:
                return u, None
        if meta.get("value") is not None:
            return value_based_unit(field_name, meta["value"], None), None
    return None, None


def cache_col_unit(url, col_name):
    """For TAP responses cached by field name, resolve a column's unit."""
    entry = UNIT_CACHE.get(url)
    if not isinstance(entry, dict) or entry.get("error"):
        return None
    if col_name in entry and isinstance(entry[col_name], dict):
        meta = entry[col_name]
        if meta.get("unit"):
            return uri_to_unit(meta["unit"])
        if meta.get("value") is not None:
            return value_based_unit(col_name, meta["value"], None)
    return None


def cache_col_by_index(url, idx):
    """Map a column index to the nth native column name from the cached
    API response (cache preserves the response's key order)."""
    entry = UNIT_CACHE.get(url)
    if not isinstance(entry, dict) or entry.get("error"):
        return None
    keys = [k for k in entry if k not in ("error", "header_units", "csv_columns")]
    if not keys:
        return None
    if idx < len(keys):
        return keys[idx]
    return None


def infer_unit(name, path, force, url):
    """Determine the source unit from field meaning. Physical truth only.
    Returns a canonical unit keyword or None."""
    n = name.lower()
    c = (n + "_" + path).lower()
    domain = url.split("/")[2] if url else ""

    # VizieR error prefix: e_X has the same unit as X
    if n.startswith("e_") and len(n) > 2 and n[2:] in (
            "raj2000", "dej2000", "dist", "mag", "z", "parallax", "pmra",
            "pmdec", "lii", "bii", "b-v", "plx"):
        return infer_unit(n[2:], path, force, url)

    # Domain-specific ambiguous resolution
    for dom_key, table in DOMAIN_FIELD_UNITS.items():
        if dom_key in domain and name in table:
            return table[name][0]

    # magnitudes — any band magnitude is em mag (Kmag, BTmag, Vmag, Jmag,
    # phot_g_mean_mag, w1mpro, ...)
    if n.endswith("mag") or n in ("mag", "magnitude", "vmag", "bmag", "rmag",
            "gmag", "imag", "zmag", "jmag", "hmag", "kmag", "bp_rp",
            "bp_mag", "rp_mag", "phot_g_mean", "w1mpro", "w2mpro",
            "w3mpro", "w4mpro", "pmag", "apparent_mag", "absolute_mag",
            "magrms", "phot_g_mean_mag"):
        return "mag"
    if re.match(r"^[a-z][a-z]?mag$", n) or re.match(r"^e_[a-z][a-z]?mag$", n):
        return "mag"

    # redshift — a column named z in an astronomy/TAP context is redshift
    if n in ("z", "zabs", "zphot", "zspec", "redshift", "rvz", "rvz_redshift"):
        if "sky" in domain or "tap" in url or "sim" in domain or \
           "irsa" in domain or "ned" in domain or "heasarc" in domain or \
           "mast" in domain or "simbad" in domain:
            return "z"
    if n in ("zabs", "zphot", "redshift", "rvz", "rvz_redshift"):
        return "z"

    # magnetic field — nT
    if any(w in c for w in ("field_magnitude", "_bt", "bz_", "by_", "bx_",
            "mag_field", "magnetic", "geomagnetic", "rtsw_mag", "_b_",
            "magnetometer")):
        if "mag" not in n.split("_")[-1] or "field_magnitude" in n:
            return "nT"
    if n == "bt" or n == "field_magnitude":
        return "nT"

    # temperature — thermal C (matrix keyword is C)
    if any(w in c for w in ("temp", "sst", "teff", "dewp", "dew_point",
            "atmp", "wtmp", "heat_index", "wind_chill", "tmax", "tmin",
            "tavg", "tobs", "emxt", "emnt", "t2m", "bright_ti",
            "pl_eqt", "avg_max_monthly", "bleaching", "analysed_sst",
            "st_teff", "sea_surface_temperature", "air_temperature",
            "water_temperature", "2m_temperature")):
        if any(w in c for w in ("teff", "st_teff", "pl_eqt", "bright_ti", "proton_temperature")):
            return "K"
        return "C"

    # pressure — acoustic hPa
    if any(w in c for w in ("_hpa", "_mbar", "altimeter", "baro", "press",
            "pres", "slp", "ptdy", "mb", "pa")):
        if "pa" in n or n.endswith("_pa"):
            return "Pa"
        return "hPa"

    # velocity — advective (m/s default, km/s for solar/orbital)
    if any(w in c for w in ("proton_speed", "solar_wind", "radial_vel",
            "vhelio", "hrv", "vlsr", "rv", "cz")):
        return "km/s"
    if any(w in c for w in ("wspd", "wind_spd", "wind_speed", "gust",
            "speed", "velocity", "current", "water_speed", "flow",
            "vx", "vy", "vz", "motion", "movement", "v_mean", "vel",
            "spd", "m_s", "ms")):
        return "m/s"
    if n in ("gs", "ground_speed"):
        return "m/s"

    # wind/wave direction — deg
    if any(w in c for w in ("wdir", "wind_dir", "winddir", "drct", "wdird",
            "mwd", "wvdir", "swd", "wwd", "wdf2", "wdf5", "direction",
            "heading", "bearing", "azimuth", "winddirection")):
        return "deg"

    # length / position — gravity m (or km for altitude-style)
    if any(w in c for w in ("_km", "alt_km", "elev_km", "height_km",
            "depth_km", "km_depth", "altitude_km", "sy_dist")):
        return "km" if "dist" not in n else "pc"
    if n in ("sy_dist", "dist", "distance", "majaxis", "mdist"):
        return "pc" if force in ("em", "gravity") else "m"
    if any(w in c for w in ("_ft", "_feet", "elev_ft", "alt_ft", "tide")):
        return "ft"
    if any(w in c for w in ("_nmi", "nmile", "nautical")):
        return "nmi"
    if any(w in c for w in ("depth", "elevation", "altitude", "height",
            "diameter", "diam", "radius", "alt", "elev", "gauge_height",
            "water_level", "sea_level", "wave_height", "swh", "wvht",
            "vhm0", "swell", "sig_ht", "hmax", "hs", "thickness",
            "pl_radj", "pl_rade", "maj", "footprint", "gage_ht",
            "nidheight", "damheight", "active_layer", "depth__m_")):
        return "m"
    if n in ("lat", "lon", "latitude", "longitude", "lng", "solar_lat",
            "solar_lon"):
        return "deg"

    # celestial position
    if n in ("ra", "dec", "declination", "right_ascension", "ragaia",
             "degaia", "ctrpart_ra", "ctrpart_dec", "assoc_ra", "assoc_dec"):
        return "deg"
    if n in ("plx", "parallax"):
        return "mas"
    if n in ("pmra", "pmdec", "pm_ra", "pm_dec", "pm", "pmde", "e_pmra",
             "e_pmde", "q_pmra", "q_pmde"):
        return "mas_yr"
    if n in ("error_radius", "pos_err", "assoc_error_radius", "e_raj2000",
             "e_dej2000", "error_ell_major", "error_ell_minor"):
        return "deg"
    if n in ("lii", "bii", "glon", "glat", "elon", "elat"):
        return "deg"

    # flux / irradiance — em W/m2
    if any(w in c for w in ("flux", "irrad", "radiance", "xray", "x-ray",
            "solar_flux", "radio_flux", "fpeak", "fint", "fluence",
            "allsky_sfc", "sw_down", "lw_down", "sw_net", "lw_net",
            "radiation", "insolation", "par", "photosynthetic",
            "count_rate", "obs_flux", "current_int", "max_xrlong",
            "bckwide", "lrmswide", "speak", "sint")):
        return "W/m2"
    if any(w in c for w in ("f10.7", "sfu")):
        return "sfu"
    if any(w in c for w in ("jy", "s15ghz", "stot", "ptot", "s1400",
            "f12um", "f25um", "f60um", "f100um", "fnu_", "f24",
            "local_rms", "stotal", "speaktot")):
        return "Jy"
    if n == "frp" or n == "fpr":
        return "MW"

    # spectral / energy — em eV
    if any(w in c for w in ("_kev", "_mev", "_gev", "_ev", "pivot_energy",
            "em_min", "em_max", "lp_epeak", "plec_epeak", "highest_energy",
            "epeak", "ezero", "fluence_e", "he_peak", "nu_fnu")):
        return "eV"
    if n.endswith("_tev"):
        return "TeV"
    if any(w in c for w in ("_hz", "frequency", "freq", "nu_syn", "nu_eff")):
        return "Hz"

    # density / concentration — diffusion
    if any(w in c for w in ("_ppm", "ppm_")):
        return "ppm"
    if any(w in c for w in ("_ppb", "ppb_", "nh3")):
        return "ppb"
    if any(w in c for w in ("_ug_m3", "ugm3", "µg_m3", "pm2_5", "pm10",
            "tvoc", "particulate", "aerosol", "pm25")):
        return "µg/m3"
    if any(w in c for w in ("salinity", "psal", "conductivity", "sal",
            "so")):
        return "psu"
    if "turbidity" in c:
        return "ntu"
    if "density" in c or "biomass" in c or "chlorophyll" in c or "chl_" in c:
        return "kg/m3"
    if n in ("proton_density", "electron_density"):
        return "p/cm3"
    if "nhi" in c or "column_density" in c or "col_density" in c:
        return "cm-2"
    if n in ("doxy",):
        return "µmol/kg"
    if "precip" in c or "rain" in c or "snow" in c or "_prcp" in c or \
       n == "prcp" or "evap" in c or "snow_depth" in c:
        return "mm"

    # humidity / percentage
    if any(w in c for w in ("humidity", "rhum", "rel_hum", "rh2m", "rh",
            "relhum", "soil_moisture", "cloud_cover", "sky_cover", "albedo",
            "reflect", "fraction", "probability", "pct", "percent",
            "psun", "sea_ice_fraction", "single_scattering")):
        return "pct"

    # seismic
    if any(w in c for w in ("pga", "pgv", "peak_ground", "shakemap",
            "spectral_accel", "sa_", "mgal", "acceleration")):
        return "m/s2" if "mgal" not in c else "mGal"

    # mass
    if n in ("pl_bmasse", "pl_masse", "m_earth"):
        return "M_earth"
    if n in ("pl_bmassj", "pl_massj", "mjup"):
        return "M_jup"
    if any(w in c for w in ("m_sun", "st_mass", "mass_log", "ez_mass",
            "yn_mass", "mass_bh", "mass_donor")):
        return "M_sun"

    # orbital
    if n in ("pl_orbsmax", "a", "semi_major_axis", "dist_min", "q"):
        return "au"
    if n in ("pl_orbper", "orbital_period", "period", "p0", "porb", "per"):
        return "d"
    if n in ("pl_tranmid",):
        return "mjd"

    # redshift-distance column
    if n in ("z", "zabs", "zphot"):
        return "z"

    # AOD / optical depth (dimensionless but physical)
    if "optical_depth" in c or "angstrom" in c or "aod" in c:
        return "scalar"

    return None

def infer_force(name, path, block_force, url=""):
    """Rat der 5 Stimmen: physikalische Force nach Bedeutung.
    Druck (hPa, Pa) ist Strömung → advective. Welle (m) ist Schall → acoustic.
    Windrichtung ist Oberflächenwirkung → seismic-surface. Temperatur immer
    thermal. Konzentration immer diffusion. Masse/Distanz → gravity."""
    n = name.lower()
    c = (n + "_" + path).lower()
    up = name.upper()
    domain = url.split("/")[2] if url else ""

    # ---- STRIPPED: infrastructure, abstract counts, metadata ----
    if n.endswith(("_capacity_mw", "_count", "_total")) or \
       n in ("capacity_mw", "numrows", "number", "total", "quota",
             "population", "npred", "ssn", "sunspot_number", "people",
             "fatalities", "deaths_total", "totaldeaths", "deaths",
             "numberinjured", "totalaffected", "n_obs", "n_contrib",
             "n_exp", "nobs", "mobs", "num", "count", "mediacount"):
        return None

    # ---- thermal (5): temperature ----
    if any(w in c for w in ("temp", "sst", "teff", "dewp", "dew_point",
            "atmp", "wtmp", "heat_index", "wind_chill", "tmax", "tmin",
            "ttavg", "tobs", "emxt", "emnt", "t2m", "bright_ti", "pl_eqt",
            "avg_max_monthly", "bleaching", "analysed_sst", "st_teff",
            "sea_surface_temperature", "air_temperature", "water_temperature",
            "2m_temperature", "t31", "ssta", "dhw", "heating_week")):
        return "thermal"
    if n in ("frp", "fpr"):
        return "thermal"

    # ---- diffusion (6): concentration, density, humidity, chemistry ----
    if any(w in c for w in ("density", "ppm", "ppb", "precip", "rain",
            "snow", "moisture", "humidity", "rhum", "rel_hum", "rh2m",
            "salinity", "psal", "conductivity", "turbidity", "aod",
            "aerosol", "particulate", "pm2_5", "pm10", "pm25", "chlorophyll",
            "chl_", "biomass", "tvoc", "oxygen", "nitrate", "phosphate",
            "silicate", "co2", "ch4", "n2o", "col_density", "nhi",
            "optical_depth", "angstrom", "doxy", "psu", "sal", "so2",
            "no2", "co_total", "extinction", "ug_m3", "µg_m3",
            "mg_m3", "mg/l", "µmol", "mmol", "napi", "flux_density")):
        return "diffusion"

    # ---- advective (7): velocity, pressure, time, flow ----
    # (wind direction is seismic-surface, not advective)
    if any(w in c for w in ("wdir", "wind_dir", "winddir", "drct", "wdird",
            "wdf2", "wdf5", "winddirection")):
        return "seismic-surface"
    if any(w in c for w in ("speed", "velocity", "wspd", "wind", "gust",
            "current", "vx", "vy", "vz", "flow", "motion", "movement",
            "v_mean", "discharge", "streamflow", "runoff", "hrv", "vhelio",
            "cz", "radial_vel", "vel", "spd", "gs", "proton_speed",
            "component_of_current", "ugos", "vgos", "curin", "curout",
            "m/s", "m_s")):
        return "advective"
    if any(w in c for w in ("press", "pres", "baro", "slp", "altimeter",
            "altim", "ptdy", "hpa", "mb", "pa")):
        return "advective"

    # ---- acoustic (2): wave height, sound, wave periods ----
    if any(w in c for w in ("wave", "swh", "wvht", "vhm0", "swell",
            "wvhgt", "dpd", "apd", "mwd", "wave_height", "sig_ht", "hmax",
            "hs", "sea_state", "sound", "db", "spl", "total_spl",
            "surf", "breaker", "surge", "whitenoise", "wvper")):
        return "acoustic"
    if any(w in c for w in ("wdir", "wind_dir", "winddir", "drct", "wdird",
            "swd", "wwd", "wdf2", "wdf5", "direction", "heading",
            "bearing", "azimuth", "winddirection")):
        return "seismic-surface"

    # ---- seismic-surface (4): seismic waves, wind effects ----
    if any(w in c for w in ("pga", "pgv", "shakemap", "peak_ground",
            "spectral_accel", "seismic", "intensity", "mmi", "dmin", "gap")):
        return "seismic-surface"
    if n in ("mw", "mb", "ms", "ml", "magnitude", "mag", "tm", "eqmagunk"):
        return "seismic-surface"

    # ---- seismic-body (3): tectonics, depth, structure ----
    if any(w in c for w in ("depth", "fault", "slip", "creep", "rupture",
            "dmin")):
        return "seismic-body"
    if n in ("depth", "kmdepth"):
        return "seismic-body"

    # ---- gravity (1): mass, distance, position, size ----
    if any(w in c for w in ("lat", "lon", "latitude", "longitude", "alt",
            "elev", "elevation", "altitude", "height", "diameter",
            "radius", "dist", "distance", "sy_dist", "maj", "pl_radj",
            "pl_rade", "pl_bmasse", "pl_bmassj", "pl_mass", "m_sun",
            "st_mass", "pl_orbsmax", "pl_orbper", "orbital", "semi_major",
            "inclination", "eccen", "footprint", "solar_lat", "solar_lon",
            "gauge_height", "water_level", "sea_level", "tide", "gage_ht",
            "mass_log", "ez_mass", "yn_mass", "mass_bh", "mass_donor",
            "nidheight", "damheight", "dist_min", "v_rel",
            "active_layer", "depth__m_", "visibility", "radius_e",
            "pl_orbper", "pl_orbsmax", "a_au", "pl_rade", "pl_massj",
            "R_earth", "R_jup", "m_earth", "m_jup")):
        return "gravity"
    if n in ("lat", "lon", "latitude", "longitude", "alt", "elev", "decl",
             "height", "ft", "nmi", "mi", "au", "pc", "kpc", "Mpc",
             "m2", "km2", "deg", "mas", "M_sun", "M_jup", "M_earth",
             "kg", "g", "rad", "°"):
        return "gravity"

    # ---- em (0): radiation, magnitudes, magnetic, redshift, spectral, energy ----
    if any(w in c for w in ("flux", "irrad", "radiance", "xray", "mag",
            "magnitude", "bt", "bz", "by", "bx", "mag_field", "magnetic",
            "geomagnetic", "redshift", "z_phot", "zabs", "rvz", "energy",
            "_ev", "hz", "freq", "pmra", "pmdec", "photon", "luminos",
            "radio", "uv", "solar", "sfu", "fpeak", "fint", "fluence",
            "wavelength", "pivot_energy", "tec", "rigidity", "speak",
            "sint", "sp-index", "stot", "ptot", "s15ghz", "f12um",
            "f25um", "f60um", "f100um", "fnu_", "f24", "snr", "sig",
            "signif_avg", "fe_h", "alpha_fe", "vrad", "rv", "fwhm",
            "lii", "bii", "glon", "glat", "dm", "count_rate", "tec",
            "b_surf", "nu_syn", "pulse", "light", "albedo", "reflect",
            "sunspot", "albedo", "reflect", "fraction")):
        return "em"
    if n in "iurgzvbyhjkb" and len(n) == 1:
        return "em"
    if n.endswith("mag") or n == "mag":
        return "em"
    if n in ("z", "redshift", "zabs", "zphot", "zspec"):
        return "em"
    if n in ("hrv", "vrad", "vlsr", "cz"):
        return "em"
    if re.match(r"^[a-z][a-z]?mag$", n):
        return "em"

    # ---- domain-specific ambiguous resolution ----
    for dom_key, table in DOMAIN_FIELD_UNITS.items():
        if dom_key in url and name in table:
            return table[name][1]

    # time periods / durations → advective (time flow)
    if n in ("period", "p0", "d", "yr", "mjd", "duration", "t_exptime",
             "exptime", "exposure", "t_min", "t_max", "t90", "t50",
             "elapsed", "interval"):
        return "advective"

    # rates → advective
    if n in ("mas_yr", "deg_yr", "deg/yr"):
        return "advective"

    return block_force


def parse_tap_cols(url):
    if not any(m in url.lower() for m in ("query=", "query=")):
        return {}
    try:
        q = re.search(r"[Qq][Uu][Ee][Rr][Yy]=([^&]+)", url)
        if not q:
            return {}
        qs = urllib.parse.unquote_plus(q.group(1))
        m = re.search(r"(?i)select\s+(.+?)\s+from\s+", qs)
        if not m:
            return {}
        cs = m.group(1)
        cs = re.sub(r"(?i)top\s+\d+\s+", "", cs)
        cs = re.sub(r"(?i)count\(.*?\)\s+as\s+\w+", "", cs)
        cs = re.sub(r"\w+\.", "", cs)
        cols = [c.strip().strip('"').lower() for c in cs.split(",") if c.strip()]
        cols = [re.sub(r"\([^)]*\)", "", c).strip() for c in cols]
        return {i: c for i, c in enumerate(cols) if c and c != "distinct"}
    except Exception:
        return {}

def main():
    write = "--write" in sys.argv
    blocks = open(SRC).read().strip().split("\n\n")
    new_blocks = []
    stats = {"total": 0, "structural": 0, "units": collections.Counter(),
             "forces": collections.Counter(), "unknown": []}

    for b in blocks:
        b = b.strip()
        if not b:
            continue
        lines = b.split("\n")
        force = ""
        url = ""
        frames = []
        for line in lines:
            ls = line.strip()
            if ls.startswith("url "):
                url = ls.replace("url ", "")
            if ls.startswith("force "):
                force = ls.split("force ")[1]
        tap_cols = parse_tap_cols(url)
        new_lines = []

        for line in lines:
            ls = line.strip()
            if not ls:
                continue
            parts = ls.split()
            key = parts[0]

            if key in ("url", "ttl"):
                new_lines.append(ls)
                continue
            if key in ("on", "at", "body") or (key == "pos" and " " in ls):
                if key == "pos":
                    # legacy data-carried position with TARGET names; fields now
                    # carry explicit position units -> drop the redundant line
                    continue
                # "at <body> 1" is the barycentric frame of a body; a source
                # measuring that body's SURFACE (weather, seismic, sound) must
                # be "on <body> <lat> <lon>" — a fixed geodetic point.
                if key == "at" and len(parts) >= 2 and parts[1] != "sun":
                    if "mars.nasa.gov/rss/api/?feed=weather" in url:
                        frames.append("on mars 4.59 137.44")
                        continue
                # a template block (URL has {lat}/{lon}) must not carry a
                # hardcoded on-earth point — its position is the substituted
                # presence window, declared by the same template variables
                if key == "on" and len(parts) == 4:
                    try:
                        float(parts[2]); float(parts[3])
                        if ("{lat}" in url or "{lon}" in url) and \
                           parts[1] == "earth":
                            frames.append(f"on earth {{lat}} {{lon}}")
                            continue
                    except ValueError:
                        pass
                # canonical barycentric/inertial frame: "at <body>" — the
                # legacy scale parameter ("at sun 1.0") is always 1 and dead
                if key == "at" and len(parts) >= 2:
                    frames.append(f"at {parts[1]}")
                    continue
                frames.append(ls)
                continue
            if key == "format":
                new_lines.append(ls)
                continue
            if key == "force":
                force = parts[1]
                continue

            if key == "map" and len(parts) >= 2:
                new_lines.append(ls)
                continue
            if key == "cmap" and len(parts) >= 2:
                new_lines.append(f"map {parts[1]}")
                continue
            if key == "rows":
                new_lines.append("rows")
                continue
            if key == "tail":
                new_lines.append("rows last")
                continue
            if key in ("geojson", "flatten", "cmr", "kepler", "hapi"):
                new_lines.append(ls)
                continue

            if key in ("lat_key", "lon_key") and len(parts) >= 2:
                v = parts[1]
                vl = v.lower()
                if key == "lon_key":
                    # a longitude key whose column name carries lon/long is a
                    # longitude ("dec_" is often a decimal-format prefix, e.g.
                    # dec_long_va). Only a column that is clearly declination
                    # (no longitude hint) is a mislabeled celestial dec.
                    if "lon" in vl or "lng" in vl or "long" in vl:
                        new_lines.append(f"lon {v} deg")
                    elif "dec" in vl or "decl" in vl:
                        new_lines.append(f"dec {v} deg")
                    else:
                        new_lines.append(f"lon {v} deg")
                else:
                    new_lines.append(f"lat {v} deg")
                continue
            # bare position directives (lat/lon/alt/ra/dec/plx/pmra/pmdec):
            # always 3 tokens: <dir> <path|literal> <unit>
            if key in ("lat", "lon") and len(parts) >= 2:
                new_lines.append(f"{key} {parts[1]} deg")
                continue
            if key == "alt" and len(parts) >= 2:
                p = parts[1].lower()
                u = "ft" if ("_ft" in p or p.endswith("ft")) else ("km" if ("_km" in p or p.endswith("km")) else "m")
                new_lines.append(f"alt {parts[1]} {u}")
                continue
            if key in ("ra", "dec") and len(parts) >= 2:
                v = parts[1]
                if key == "dec" and any(s in v.lower() for s in ("lon", "lng",
                        "longitude", "long", "lo_")):
                    # source mislabeled a longitude column as declination
                    new_lines.append(f"lon {v} deg")
                else:
                    new_lines.append(f"{key} {v} deg")
                continue
            if key == "plx" and len(parts) >= 2:
                new_lines.append(f"plx {parts[1]} mas")
                continue
            if key in ("pmra", "pmdec") and len(parts) >= 2:
                new_lines.append(f"{key} {parts[1]} mas_yr")
                continue
            if key == "alt_key" and len(parts) >= 2:
                p = parts[1].lower()
                u = "ft" if "_ft" in p else ("km" if "_km" in p else "m")
                new_lines.append(f"alt {parts[1]} {u}")
                continue
            if key == "ra_key" and len(parts) >= 2:
                new_lines.append(f"ra {parts[1]} deg")
                continue
            if key == "dec_key" and len(parts) >= 2:
                v = parts[1]
                if any(s in v.lower() for s in ("lon", "lng", "longitude",
                        "long", "lo_")):
                    # source mislabeled a longitude column as declination
                    new_lines.append(f"lon {v} deg")
                else:
                    new_lines.append(f"dec {v} deg")
                continue
            if key == "plx_key" and len(parts) >= 2:
                new_lines.append(f"plx {parts[1]} mas")
                continue
            if key == "pmra_key" and len(parts) >= 2:
                new_lines.append(f"pmra {parts[1]} mas_yr")
                continue
            if key == "pmdec_key" and len(parts) >= 2:
                new_lines.append(f"pmdec {parts[1]} mas_yr")
                continue

            if key in ("field", "field_in") and len(parts) >= 3:
                src, tgt = parts[1], parts[2]
                stats["total"] += 1
                unit = infer_unit(tgt, src, force, url)
                col_name = None
                f = None
                native = None
                idx_of = tgt
                mcol = re.match(r"^col(\d+)$", tgt)
                if mcol:
                    idx_of = mcol.group(1)
                for dom, tmap in NATIVE_COLS.items():
                    if dom in url and idx_of.isdigit() and int(idx_of) in tmap:
                        native = tmap[int(idx_of)]
                        break
                if native:
                    # the native column map is authoritative — do NOT fall
                    # through to other inference that could re-label it
                    col_name = native[0]
                    unit = native[1]
                    f = native[2]
                elif unit is None and tgt.isdigit():
                    col_name = IDX_NAMES.get(url, {}).get(tgt) or tap_cols.get(int(tgt))
                    if not col_name:
                        col_name = cache_col_by_index(url, int(tgt))
                    if col_name:
                        unit = infer_unit(col_name, col_name, force, url)
                        if unit is None:
                            unit = cache_col_unit(url, col_name)
                    else:
                        unit = cache_col_unit(url, tgt)
                if unit is None:
                    unit, _ = cache_lookup(url, tgt)
                if f is None:
                    f = infer_force(col_name or tgt, src, force, url)
                # a temperature unit is always thermal — no force may override
                if unit in ("C", "F", "K"):
                    f = "thermal"
                pos_dir = position_directive(tgt, col_name, src, unit, url)
                if pos_dir:
                    new_lines.append(pos_dir)
                    stats["units"][unit] += 1
                elif unit in PHYSICAL_UNITS and f and not is_time_metadata(tgt):
                    new_lines.append(f"field {src} {f} {unit}")
                    stats["units"][unit] += 1
                    stats["forces"][f] += 1
                else:
                    # not a physical quantity -> stripped (never an oscillator)
                    stats["structural"] += 1
                continue

            if key in ("last", "first", "count", "path", "deep", "last_row",
                       "obj_last"):
                if key == "count":
                    # counting records/lines/events is a derived statistic,
                    # not a propagating field quantity -> stripped
                    stats["structural"] += 1
                    continue
                if len(parts) >= 3:
                    src = parts[1]
                    tgt = parts[2] if len(parts) > 2 else src
                    unit = infer_unit(tgt, src, force, url)
                    col_name = None
                    native = None
                    for dom, tmap in NATIVE_COLS.items():
                        if dom in url and tgt.isdigit() and int(tgt) in tmap:
                            native = tmap[int(tgt)]
                            break
                    if native:
                        unit = native[1]
                        f = native[2]
                        col_name = native[0]
                    else:
                        if unit is None and tgt.isdigit():
                            col_name = tap_cols.get(int(tgt)) or cache_col_by_index(url, int(tgt))
                            if col_name:
                                unit = infer_unit(col_name, col_name, force, url)
                                if unit is None:
                                    unit = cache_col_unit(url, col_name)
                            else:
                                unit = cache_col_unit(url, tgt)
                        if unit is None:
                            unit, _ = cache_lookup(url, tgt)
                        f = infer_force(col_name or tgt, src, force, url)
                    if unit in PHYSICAL_UNITS and f and not is_time_metadata(tgt):
                        new_lines.append(f"{key} {src} {f} {unit}")
                        stats["units"][unit] += 1
                        stats["forces"][f] += 1
                    else:
                        stats["structural"] += 1
                continue

            if key in ("last_obj", "last_line", "regex", "xml", "ephemeris",
                       "vectors", "alt_sign", "lon_sign", "lat_sign",
                       "dist_scale", "rv_scale"):
                new_lines.append(ls)
                continue
            # canonical coordinate extractors: no _key suffix. The directive
            # always extracts from data; the suffix carried no information.
            if key in ("epoch_key", "val_key", "vel_key", "trk_key", "vr_key",
                       "dist_key", "z_key", "tau_key", "rv_key", "depth_key",
                       "radvel_key"):
                bare = key[:-4]
                new_lines.append(f"{bare} {' '.join(parts[1:])}")
                continue
            new_lines.append(ls)

        # resolve the frame by the data's nature.
        # celestial (ra/dec/plx directives) -> at sun 1.0
        # terrestrial (lat/lon directives, or {lat}/{lon} templates, or a
        # terrestrial domain) -> on earth (or "on earth {lat} {lon}" when the
        # URL carries position templates)
        has_celestial_pos = any(
            l.split() and l.split()[0] in ("ra", "dec", "plx", "pmra", "pmdec")
            for l in new_lines) or ("{ra}" in url and "{dec}" in url)
        has_terrestrial_pos = any(
            l.split() and l.split()[0] in ("lat", "lon") for l in new_lines)
        has_terr_template = "{lat}" in url and "{lon}" in url
        if has_celestial_pos:
            frames = ["at sun"]
        elif has_terrestrial_pos or has_terr_template:
            # lat/lon directives are geodetic coordinates of a BODY — they can
            # never live in a barycentric (at sun) frame
            if any(f.startswith("at ") for f in frames):
                if has_terr_template:
                    frames = ["on earth {lat} {lon}"]
                else:
                    frames = ["on earth 0 0"]
        if len(frames) > 1:
            frames = frames[-1:]
        # order: url, ttl, frame, then the rest
        head = [l for l in new_lines if l.split() and l.split()[0] in ("url", "ttl")]
        body = [l for l in new_lines
                if not (l.split() and l.split()[0] in ("url", "ttl", "on",
                                                       "at", "body"))]
        new_lines = head + frames + body

        # enrich from the API cache: physical fields present in the response
        # but not declared in the block (e.g. ISS velocity, solar_lat/lon,
        # footprint). Declared source keys/paths track what we already emitted.
        declared = set()
        for l in new_lines:
            p = l.split()
            if p and p[0] in ("field", "last", "first", "path", "deep",
                              "last_row", "obj_last", "lat", "lon", "alt",
                              "ra", "dec", "plx", "pmra", "pmdec"):
                if len(p) >= 2:
                    declared.add(p[1].lower())
        cache_entry = UNIT_CACHE.get(url)
        if isinstance(cache_entry, dict) and not cache_entry.get("error"):
            for fname, meta in cache_entry.items():
                if fname in ("error", "header_units", "csv_columns"):
                    continue
                if not isinstance(meta, dict) or meta.get("value") is None:
                    continue
                key = url.replace("/", "_").replace("?", "_")
                if fname.lower() in declared:
                    continue
                u = cache_col_unit(url, fname)
                if not u:
                    u = infer_unit(fname, fname, force, url)
                f = infer_force(fname, fname, force, url)
                if u in ("C", "F", "K"):
                    f = "thermal"
                pos = position_directive(fname, fname, fname, u, url)
                if pos:
                    new_lines.append(pos)
                    declared.add(fname.lower())
                    stats["units"][u] += 1
                elif u in PHYSICAL_UNITS and f and not is_time_metadata(fname):
                    new_lines.append(f"field {fname} {f} {u}")
                    declared.add(fname.lower())
                    stats["units"][u] += 1
                    stats["forces"][f] += 1

        # a block is complete only when it declares a measurement extractor
        # (field/last/path/...) or a container that carries measurements inline
        # (geojson events, hapi, kepler, ephemeris). Position alone is not.
        has_measurement = False
        for l in new_lines:
            p = l.split()
            if p and (p[0] in ("field", "last", "first", "count", "path",
                               "deep", "last_row", "obj_last") or
                      (p[0] == "geojson" and len(p) > 2) or
                      p[0] in ("hapi", "kepler", "ephemeris", "vectors")):
                has_measurement = True
                break
        if not has_measurement:
            restored = domain_physical_lines(url, new_lines)
            if restored:
                # keep the block header (url/ttl/frame/position) and re-add
                # the physical measurement extractors — without duplicating
                # lines that are already present
                frame_ok = [l for l in new_lines if l.split() and l.split()[0] in
                            ("url", "ttl", "on", "at", "body", "format",
                             "header", "map", "rows", "lat", "lon", "alt",
                             "ra", "dec", "plx", "pmra", "pmdec")]
                existing = set(frame_ok)
                add = [l for l in restored if l not in existing]
                new_lines = frame_ok + add
            else:
                # pure metadata/catalog block with no physical measurement
                new_lines = []

        # deduplicate within the block (cache enrichment may re-add a line)
        seen = set()
        uniq = []
        for l in new_lines:
            if l not in seen:
                seen.add(l)
                uniq.append(l)
        new_lines = uniq

        new_blocks.append("\n".join(new_lines))

    # join only non-empty blocks — a single blank line separates blocks
    result = "\n\n".join(b for b in new_blocks if b.strip())
    if not result.endswith("\n"):
        result += "\n"

    if write:
        open(OUT, "w").write(result)

    print(f"Fields: {stats['total']}  Structural: {stats['structural']}  "
          f"Units: {sum(stats['units'].values())}  ???: {len(stats['unknown'])}")

    ni = [x for x in stats["unknown"] if not x[0].isdigit()]
    idx = [x for x in stats["unknown"] if x[0].isdigit()]
    print(f"Non-index ???: {len(ni)}  Index ???: {len(idx)}")

    if ni:
        nf = collections.Counter(t for t, _, _, _ in ni)
        print("Non-index names:")
        for n, c in nf.most_common(15):
            print(f"  {c:4d} {n}")

    dom = collections.Counter()
    for t, s, u, f in stats["unknown"]:
        d = u.split("/")[2] if u else "?"
        dom[d] += 1
    print("Top ??? domains:")
    for d, c in dom.most_common(10):
        print(f"  {c:5d} {d}")


if __name__ == "__main__":
    main()
