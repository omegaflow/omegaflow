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
    "knots": "knots", "kn": "knots", "km/s": "km/s", "km/h": "kmh",
    "kmh": "kmh", "mph": "mph",
    "hPa": "hPa", "mb": "hPa", "mbar": "hPa", "pa": "Pa", "mmHg": "mmHg",
    "inHg": "inHg",
    "K": "K", "degC": "degC", "C": "degC", "degF": "degF", "F": "degF",
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
    "km/h": "kmh", "m/yr": "m/yr", "mm/yr": "mm/yr",
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
    "EMXP": "mm", "EMXT": "degC", "EMNT": "degC",
    "PRCP": "mm", "SNOW": "mm", "SNWD": "mm",
    "TAVG": "degC", "TMAX": "degC", "TMIN": "degC", "TOBS": "degC",
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
    "ATMP": "degC", "WTMP": "degC", "DEWP": "degC",
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
PHYSICAL_UNITS = {
    # position / angle
    "deg", "rad", "mas", "arcsec", "arcmin", "mas_yr", "deg/yr",
    # length
    "m", "km", "ft", "nmi", "cm", "mm", "µm", "nm",
    "pc", "kpc", "Mpc", "au", "ly", "R_earth", "R_jup",
    # velocity / acceleration
    "m/s", "km/s", "knots", "kmh", "mph", "m/s2", "mGal", "m/yr", "mm/yr",
    # temperature
    "K", "degC", "degF",
    # pressure
    "Pa", "hPa", "mmHg", "inHg",
    # EM / radiation
    "nT", "µT", "T", "G", "W/m2", "W/m2/Hz", "Jy", "mJy", "W", "MW",
    "sfu", "eV", "keV", "MeV", "GeV", "TeV", "J", "erg/s",
    "Hz", "kHz", "MHz", "GHz", "mag", "z", "TECU", "GV",
    # concentration / diffusion
    "kg/m3", "g/cm3", "mg/m3", "µg/m3", "ppm", "ppb", "mg/l", "mol/m3",
    "µmol/kg", "psu", "ntu", "pH", "p/cm3", "cm-2", "kg/m2",
    "mm", "m3/s", "ft3/s", "L/s",
    # time (epoch coordinate)
    "s", "min", "h", "d", "yr", "ms", "mjd",
    # area / volume / mass
    "m2", "km2", "m3", "km3", "M_earth", "M_jup", "M_sun", "u",
    "pc/cm3",
}


def physical_unit(name, path, force, url):
    """Return the physical unit for a field, or None (stripped) if the field
    is not a real physical quantity."""
    u = infer_unit(name, path, force, url)
    if u in PHYSICAL_UNITS:
        return u
    return None


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
        "T2M": ("degC", "thermal"), "T2M_MAX": ("degC", "thermal"),
        "T2M_MIN": ("degC", "thermal"), "T2MDEW": ("degC", "thermal"),
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
        return "K" if a > 100 else "degC"
    # solar wind speed
    if "proton_speed" in n or "solar_wind" in n:
        return "km/s" if a > 50 else "m/s"
    # speed
    if any(w in n for w in ("speed", "velocity", "spd", "wspd", "wdsp")):
        if a > 200: return "km/s"
        if a > 40: return "m/s"
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
    n = name.lower()
    combined = (n + "_" + path).lower()
    domain = url.split("/")[2] if url else ""

    # VizieR error prefix: e_X has the same unit as X
    if n.startswith("e_") and len(n) > 2:
        base = n[2:]
        if base in ("b-v", "raj2000", "dej2000", "dist", "mag", "z",
                    "parallax", "pmra", "pmdec", "lii", "bii", "dist",
                    "plx"):
            return infer_unit(base, path, force, url)

    # Domain-specific ambiguous resolution
    for dom_key, table in DOMAIN_FIELD_UNITS.items():
        if dom_key in url and name in table:
            return table[name][0]

    # Explicit unit suffixes in field names
    if n.endswith("_au"):
        return "au"
    if n.endswith("_deg") or n.endswith("_degre"):
        return "deg"
    if n.endswith("_kev") or n.endswith("_mev") or n.endswith("_gev") or n.endswith("_ev"):
        return "eV"
    if n.endswith("_tev"):
        return "TeV"
    if n.endswith("_hz"):
        return "Hz"
    if n.endswith("_km_s") or n.endswith("_kmps"):
        return "km/s"
    if n.endswith("_ms") or n.endswith("_m_s"):
        return "m/s"
    if n.endswith("_nm"):
        return "nm"
    if n.endswith("_m") and "_" in n:
        return "m"
    if n.endswith("_kg_m3"):
        return "kg/m3"
    if n.endswith("_seconds") or n.endswith("_inseconds") or n.endswith("_sec"):
        return "s"
    if n.endswith("seconds"):
        return "s"
    if n == "detected_duration":
        return "s"
    if n == "detected_energy":
        return "eV"
    if n == "aqi":
        return "index"
    if n == "alt":
        return "m"
    if n == "hail_size":
        return "index"
    if n == "elev":
        return "m"
    if n in ("visibility", "vis"):
        return "m"
    if n == "s_ra":
        return "deg"
    if n == "s_dec":
        return "deg"
    if n == "angstrom_exponent_440-870nm":
        return "index"
    if n == "psal":
        return "psu"
    if "component_of_current" in n:
        return "m/s"
    if n == "albedo":
        return "pct"
    if n == "optical_depth":
        return "index"
    if n in ("logg", "logt"):
        return "index"
    if n in ("rapmdeg", "depmdeg"):
        return "deg"
    if n == "vlsr":
        return "km/s"
    if n in ("s15ghz", "stot", "ptot"):
        return "Jy"
    if n in ("zd", "zl", "zs"):
        return "z"
    if n == "net_slip_rate":
        return "mm/yr"
    if n == "aseismic_slip_factor":
        return "index"
    if n == "v_mean":
        return "m/s"
    if n == "peak_va":
        return "m/s"
    if n == "gage_ht":
        return "m"
    if n == "speed_radius":
        return "km"
    if n == "rigidity_gv":
        return "GV"
    if n == "capacity_mw":
        return "MW"
    if n.endswith("_km2"):
        return "km2"
    if n.endswith("_f_ap") or n.endswith("_tot") or n.endswith("tot"):
        return "mag"
    if n == "theta500":
        return "deg"
    if n == "yn_mass":
        return "M_sun"
    if n in ("ugos", "vgos"):
        return "m/s"
    if n == "analysed_sst":
        return "degC"
    if n in ("x_size", "y_size"):
        return "arcsec"
    if n == "amplitude":
        return "index"
    if n == "vel":
        return "m/s"
    if n in ("area_m2", "loss_m2"):
        return "m2"
    if n in ("dst",):
        return "nT"
    if n == "total_count":
        return "count"
    if n == "kmdepth":
        return "km"
    if n in ("nidheight", "damheight"):
        return "m"
    if n == "inclination":
        return "deg"
    if n in ("f4", "f5", "f6"):
        return "index"
    if n == "kedalaman":
        return "m"
    if n == "number_spots":
        return "count"
    if n == "gpstime":
        return "unix_s"
    if n == "p_astro":
        return "pct"
    if n == "bright_t31":
        return "K"
    if n == "dailyacres":
        return "m2"
    if n == "hoursold":
        return "s"
    if n.endswith("_tecu") or n == "tec":
        return "TECU"
    if n == "movementdir":
        return "deg"
    if n == "movementspeed":
        return "m/s"
    if n in ("f107", "f107_adj"):
        return "sfu"
    if n in ("ext525", "ext1020"):
        return "index"
    if n == "bbh":
        return "count"
    if n == "tsi":
        return "W/m2"
    if "angstrom" in n or "refractive_index" in n or "asymmetry" in n:
        return "index"
    if "single_scattering" in n:
        return "pct"
    if n.startswith("ssa_") and n.endswith("nm"):
        return "pct"
    if "optical_depth" in n or "total_optical_depth" in n:
        return "index"
    if n == "dens":
        return "kg/m3"
    if n == "ssta":
        return "degC"
    if n == "ts_fig":
        return "index"
    if n in ("avg_max_monthly_mean", "bleaching_threshold"):
        return "degC"
    if n == "polygonacres":
        return "m2"
    if n == "area_km":
        return "km2"
    if n in ("area", "area_ha", "area_km2", "poly_gisacres"):
        return "m2"
    if n == "return_period":
        return "yr"
    if n == "doxy":
        return "µmol/kg"
    if n in ("currents",):
        return "m/s"
    if n == "bottom_of_ocean_mixed_layer":
        return "m"
    if n == "cloudcover":
        return "pct"
    if n == "z_phot":
        return "z"
    if n in ("mass_donor", "mass_bh"):
        return "M_sun"
    if n == "age":
        return "yr"
    if n in ("e_mean",):
        return "index"
    if n == "p_value":
        return "index"
    if n.startswith("hardness_ratio") or n == "hr2":
        return "index"
    if n.startswith("co_"):
        return "ppm"
    if n.startswith("luminosity"):
        return "W"
    if n == "size_arcmin":
        return "arcmin"
    if n == "avg_rate":
        return "count/s"
    if n in ("rate_err",):
        return "count/s"
    if n == "mass_log_msun":
        return "M_sun"
    if n == "vel_disp":
        return "km/s"
    if n in ("classtar", "sharp"):
        return "index"
    if n == "per":
        return "d"
    if n == "tmag_unc":
        return "mag"
    if n in ("p_dot", "pdot"):
        return "s/s"
    if n in ("b_surf",):
        return "G"
    if n in ("lx", "tfopwg_disp"):
        return "W"
    if n in ("t_min", "t_max"):
        return "mjd"
    if n == "pl_masse":
        return "M_earth"
    if n == "st_mass":
        return "M_sun"
    if n in ("vx", "vy"):
        return "m/s"
    if n in ("cr",):
        return "count/s"
    if n in ("mbcorr", "psfmag_g"):
        return "mag"
    if n in ("dec",):
        return "deg"
    if n == "vhelio_avg":
        return "km/s"
    if n.startswith("s") and n[1:].isdigit():
        return "mJy"
    if n == "f2-8":
        return "mJy"
    if n in ("ex", "m_h", "alpha_m", "sini", "n"):
        return "index"
    if n == "reff":
        return "arcsec"
    if n in ("hr0", "asc", "ruwe"):
        return "index"
    if n == "mt":
        return "mag"
    if n == "tspan":
        return "s"
    if n == "plx":
        return "mas"
    if n == "pm":
        return "mas_yr"
    if n == "porb":
        return "d"
    if n in ("fx", "mx"):
        return "count/s"
    if n == "mdist":
        return "pc"
    if n in ("fe_h", "alpha_fe", "e_alpha"):
        return "dex"
    if n in ("vrad", "rv"):
        return "km/s"
    if n == "age_bstep":
        return "yr"
    if n in ("peak", "fpwide"):
        return "count/s"
    if n.startswith("s(") or n == "sp-index":
        return "mJy"
    if n in ("vhb", "mvt"):
        return "mag"
    if n in ("hrv",):
        return "km/s"
    if n in ("fwhm", "fwhma", "fwhmb", "wpeak"):
        return "km/s"
    if n in ("r'-i'",):
        return "mag"
    if n == "speaktot":
        return "mJy"
    if n in ("d", "mu-max", "beta-max"):
        return "index"
    if n == "s15" or (n.startswith("s") and n[1:].isdigit() and len(n) <= 3):
        return "mJy"
    if n.startswith("f") and n.endswith("um") and len(n) >= 4:
        return "Jy"
    if n == "loglir":
        return "W"
    if n == "mu_app":
        return "mag"
    if n in ("local_rms", "nx", "nep", "blambda"):
        return "index"
    if n == "logvarpa":
        return "deg"
    if n == "stotal":
        return "mJy"
    if n == "polfrac":
        return "pct"
    if n == "polangle":
        return "deg"
    if n == "active_layer_thickness":
        return "m"
    if n in ("totaldeath", "totalaffected", "numberinjured"):
        return "count"
    if n in ("stat_lat", "stat_long"):
        return "deg"
    if n == "daily_flood_likelihood":
        return "pct"
    if n in ("err", "explosivity_index"):
        return "index"
    if n == "ndvi_anomaly":
        return "index"
    if n == "deaths_total":
        return "count"
    if n == "sal_surface":
        return "psu"
    if n == "deaths_total":
        return "count"
    if n == "so":
        return "psu"
    if n == "chlor_a":
        return "mg/m3"
    if n in ("anom", "success_rate", "rank"):
        return "index"
    if n == "slev":
        return "m"
    if n == "db_area":
        return "km2"
    if n == "petromag_g":
        return "mag"
    if n == "pop_max":
        return "count"
    if n == "atomic_mass":
        return "u"
    if n in ("deaths_total", "deaths_total_".rstrip("_"), "deathtotal",
             "cellcount"):
        return "count"
    if n.endswith("_mw"):
        return "MW"
    if n.startswith("fnu_") or n.startswith("fnu-"):
        return "Jy"
    if n in ("undulate", "ls_size"):
        return "index"
    if n in ("deathtotal", "deathstotal"):
        return "count"
    if n.startswith("fnu_") or n == "f24" or n == "e_f24":
        return "Jy"
    if n.startswith("snr") or n.endswith("snr"):
        return "index"
    if n == "bleaching_alert_area":
        return "km2"
    if n.endswith("_ugm3") or n.endswith("_ug/m3"):
        return "µg/m3"
    if n in ("x_pole", "y_pole"):
        return "arcsec"
    if n in ("ut1-utc", "lod"):
        return "s"
    if n in ("dpsi", "depsilon"):
        return "mas"
    if n == "population":
        return "count"
    if n == "dist_min":
        return "au"
    if n == "v_rel":
        return "km/s"
    if n.endswith("(m)") or n.endswith("_meters") or n == "elevation":
        return "m"
    if n in ("tvoc",):
        return "µg/m3"
    if n in ("nh3",):
        return "ppb"
    if n.endswith("_index") or "index" in n:
        return "index"
    if n.startswith("z_phot") or n.startswith("ez_z"):
        return "z"
    if n == "ez_mass":
        return "M_sun"
    if n in ("ugosa",):
        return "m/s"
    if n in ("vgosa",):
        return "m/s"
    if n == "radius_e":
        return "km"
    if n == "sea_ice_fraction":
        return "pct"
    if n in ("peak_tm", "analysis_error"):
        return "K"
    if n in ("dusmass", "ducmass"):
        return "kg/m2"
    if n in ("totexttau", "totsctau"):
        return "index"
    if n == "rvz_redshift":
        return "z"
    if n == "gdacs:alertscore":
        return "index"
    if n in ("strike1", "dip1", "rake1"):
        return "deg"
    if n == "dist2land":
        return "km"
    if n in ("totaldeaths", "totaldamage", "totalinjuries"):
        return "count"
    if n.endswith("inmeters") or n.endswith("m") and ("elevation" in n or "uncertainty" in n):
        return "m"
    if n == "wteq":
        return "kg/m2"
    if n == "salinty" or n == "salinity":
        return "psu"
    if n in ("curin", "curout"):
        return "m/s"
    if n == "dep":
        return "m"
    if n == "pm25" or n == "pm10":
        return "µg/m3"
    if re.match(r"^col\d+$", n):
        return "index"
    if n == "emsc:depth":
        return "km"
    if n == "sst_analysed":
        return "degC"
    if n == "sst_uncertainty":
        return "K"
    if n == "feh":
        return "dex"
    if n == "r'-i" or n == "r'-i'":
        return "mag"
    if n in ("strike", "dip", "rake"):
        return "deg"
    if n == "gain_m2":
        return "m2"
    if n in ("deathstotal", "people"):
        return "count"
    if n in ("totscatau", "totexttau", "totsctau"):
        return "index"
    if n in ("deathstotal", "people"):
        return "count"
    if re.match(r"^col\d+$", n):
        return "index"
    if n == "uf":
        return "index"
    if n.startswith("ssa_") and n.endswith("nm"):
        return "pct"

    # GRB / X-ray instrument families: spectral params -> index/eV,
    # position -> deg, timing -> s, flux/rate -> count/s
    if any(n.startswith(p) for p in ("bat_", "lp_", "plec_", "xrt_", "uvot_",
            "pn_", "m1_", "m2_", "ep_", "om_", "spec_", "stack_", "classx_",
            "galaxy_", "wise_", "gaiadr3_")):
        if "chi2" in n or "dof" in n or "slope" in n or "plsl" in n or \
           "ctslope" in n or "index" in n or "exp_factor" in n or \
           "prob" in n or "flag" in n or "mode" in n or "filter" in n or \
           "quality" in n or "dettype" in n or "detection" in n or \
           "pileup" in n or "submode" in n or "bg" in n or "ecf" in n or \
           "vig" in n or "maskfrac" in n or "offax" in n or "revolution" in n or \
           n.endswith("_s"):
            return "index"
        if "epeak" in n or "ezero" in n or "fluence" in n or "nu_fnu" in n or \
           "energy" in n or "e_" in n or "hardness" in n or "hr" in n or \
           n.endswith("_energy"):
            return "eV"
        if "flux" in n or "rate" in n or "cts" in n or "count" in n:
            return "count/s"
        if "ra" in n or "dec" in n or "theta" in n or "phi" in n or \
           "pos_err" in n or "pixel" in n or "offset" in n or "diam" in n:
            return "deg"
        if "t90" in n or "t50" in n or "start" in n or "stop" in n or \
           "expo" in n or "exposure" in n or "ontime" in n or "date" in n or \
           n.endswith("_t") or "time" in n:
            return "s"
        if "mag" in n:
            return "mag"
        if "nh" in n:
            return "cm-2"
        if "parallax" in n or "pm" in n:
            return "mas_yr"
        return "index"

    # pl_index / lp_beta / variability_index / detection_significance families
    if any(w in combined for w in ("pl_index", "photon_index", "lp_index",
            "plec_index", "variability_index", "spectral_index", "lp_beta",
            "frac_variability", "detection_significance", "significance",
            "pl_index_error", "lp_index_error", "plec_index_error",
            "lp_beta_error", "plec_exp_index", "frac_variability_error")):
        return "index"
    if n in ("wdsp", "wspd"):
        return "m/s"
    if n == "frp":
        return "MW"

    # Domain-specific
    if "swpc.noaa.gov" in domain and n in SWPC:
        return SWPC[n]
    if "ndbc.noaa.gov" in domain and n.upper() in NDBC:
        return NDBC[n.upper()]
    if "ncei.noaa.gov" in domain and name.upper() in NCEI:
        return NCEI[name.upper()]
    if "gracedb.ligo.org" in domain and n in ("numrows", "far"):
        return "count" if n == "numrows" else "1/yr"

    # Position/angle
    if n in ("assoc_ra", "assoc_dec", "assoc_error_radius", "error_radius",
             "e_raj2000", "e_dej2000", "ra", "dec", "majdiam",
             "mindiam", "ctrpart_ra", "ctrpart_dec", "pos_err",
             "bat_ra", "bat_dec", "bat_pos_err", "bat_theta", "bat_phi",
             "error_ell_major", "error_ell_minor", "error_ell_pa",
             "ramdeg", "demdeg", "q_ramdeg", "q_demdeg", "ra(icrs)",
             "de(icrs)", "ragaia", "degaia", "e_ragaia",
             "latitude", "longitude", "lat", "lon"):
        return "deg"
    if n in ("bat_t90", "bat_t50", "bat_start", "bat_stop",
             "bat_t100_start", "bat_t100_stop"):
        return "s"
    if n in ("ot_ra", "ot_dec", "ot_pos_err"):
        return "deg"
    if n in ("pmra", "pmdec", "pm_ra", "pm_dec", "pm", "e_pmra", "e_pmde",
             "pmde"):
        return "mas_yr"
    if n in ("lii", "bii", "glon", "glat", "elon", "elat"):
        return "deg"

    # Temperature
    if any(w in combined for w in ("_degr", "_degc", "temp_c", "sst_c", "tmax",
            "tmin", "tavg", "air_temp", "water_temp", "sea_surface_temp",
            "sea_surface_temperature", "air_temperature", "water_temperature",
            "dew_point", "dewpoint", "dewp", "temp", "atmp", "wtmp",
            "heat_index", "wind_chill", "emxt", "emnt", "tobs", "slp",
            "dhw", "degree_heating_week")):
        return "degC"
    if any(w in combined for w in ("teff_gspphot", "teff", "st_teff",
            "bright_ti4", "bright_ti5")):
        return "K"
    if any(w in combined for w in ("_degf", "fahrenheit", "temp_degf")):
        return "degF"
    if n == "pl_eqt":
        return "K"
    if "temp" in combined:
        return "degC"

    # Pressure
    if any(w in combined for w in ("_hpa", "_mbar", "altimeter", "barometric",
            "baro_press", "pres", "slp", "baro", "press")):
        return "hPa"
    if any(w in combined for w in ("_inhg", "in_hg")):
        return "inHg"
    if any(w in combined for w in ("_mmhg", "mm_hg")):
        return "mmHg"

    # Speed
    if any(w in combined for w in ("_knot", "_kn_", "kts", "knots")):
        return "knots"
    if any(w in combined for w in ("_kmh", "km_h", "kmph")):
        return "kmh"
    if any(w in combined for w in ("_mph", "mi_h")):
        return "mph"
    if "proton_speed" in n:
        return "km/s"
    if any(w in combined for w in ("wspd", "wind_spd", "wind_speed", "wdsp",
            "awnd", "wsf2", "wsf5", "sea_water_speed", "current_speed",
            "current_velocity", "water_speed", "velocity", "_ms", "m_s")):
        return "m/s"
    if n in ("gs", "ground_speed"):
        return "m/s"

    # Direction
    if any(w in combined for w in ("wdir", "winddir", "wind_dir", "drct",
            "wdird", "mwd", "swd", "wwd", "deg", "decl", "wdf2", "wdf5",
            "bearing", "heading")):
        return "deg"

    # Distance/length
    if any(w in combined for w in ("_ft", "_feet", "alt_ft", "elev_ft", "tide")):
        return "ft"
    if any(w in combined for w in ("_nmi", "nmile", "nautical_mile", "vis")):
        return "nmi"
    if any(w in combined for w in ("_km", "alt_km", "elev_km")):
        return "km"
    if any(w in combined for w in ("depth__m_", "depth_m", "_alt_m", "elev_m",
            "height_m")):
        return "m"
    if n in ("elevation", "altitude", "height", "depth", "diameter", "diam",
             "radius", "rad", "pl_radj", "maj", "wvht", "swh", "vhm0",
             "swell"):
        return "m"
    if n in ("latitude", "longitude", "lat", "lon"):
        return "deg"
    if n in ("distance", "dist", "sy_dist"):
        return "pc" if force in ("em", "em gravity") else "m"

    # Magnetic
    if any(w in combined for w in ("_nt", "_bt", "bz_", "by_", "bx_",
            "rtsw_mag", "geomagnetic", "magnetic_field")):
        return "nT"

    # Wave
    if any(w in combined for w in ("wave_height", "wave_ht", "sig_ht", "hmax",
            "hs", "wvhgt")):
        return "m"

    # Precipitation
    if any(w in combined for w in ("precip", "rain", "snow", "_prcp", "precp",
            "precipitation", "rainfall", "snowfall", "snow_water", "emxp",
            "mnpn", "mxpn", "evap", "snwd")):
        return "mm"

    # Humidity/percentage
    if any(w in combined for w in ("_pct", "_percent", "rhum", "humidity",
            "rel_hum", "relhum", "soil_moisture", "cloud_cover", "sky_cover",
            "ice_cover", "albedo", "reflect", "assoc_prob_bay",
            "assoc_prob_lr", "frac_variability", "psun", "prob",
            "confidence")):
        return "pct"

    # Concentration
    if any(w in combined for w in ("_ppm", "ppm_")):
        return "ppm"
    if any(w in combined for w in ("_ppb", "ppb_")):
        return "ppb"
    if any(w in combined for w in ("_ug_m3", "ugm3", "pm2_5", "pm10",
            "particulate", "aerosol", "aod")):
        return "µg/m3"
    if "salinity" in combined:
        return "psu"
    if "conductivity" in combined:
        return "psu"
    if "turbidity" in combined:
        return "ntu"
    if n == "ph":
        return "pH"

    # Magnitude
    if any(w in combined for w in ("_mag", "magnitude", "vmag", "bmag", "rmag",
            "gmag", "imag", "zmag", "jmag", "hmag", "kmag", "w1mpro",
            "w2mpro", "w3mpro", "w4mpro", "mag3_6", "mag4_5", "mag8_0",
            "h_m", "k_m", "psfmag_r", "phot_g_mean", "bp_rp", "pmag",
            "apparent_mag", "absolute_mag", "b-v")):
        return "mag"
    if n in ("mag", "magrms"):
        return "mag"
    if n.endswith("mag") and len(n) > 3:
        return "mag"
    if n == "b-v":
        return "mag"
    if n in "iurgzvbyhjk" and len(n) == 1:
        return "mag"

    # Flux/radiation
    if any(w in combined for w in ("flux", "irradiance", "allsky", "shortwave",
            "longwave", "sw_down", "lw_down", "sw_net", "lw_net", "radiance",
            "radiation", "insolation", "par", "photosynthetic", "xray",
            "x-ray", "x_ray", "solar_radiation", "solar_flux", "radio_flux",
            "fpeak", "fint", "fluence", "intensity", "count_rate")):
        return "W/m2"
    if any(w in combined for w in ("sfu", "f10.7")):
        return "sfu"
    if n in ("frp", "fire_radiative_power"):
        return "MW"

    # Orbital
    if n in ("pl_orbsmax", "a", "semi_major_axis"):
        return "au"
    if n == "pl_bmasse":
        return "M_earth"
    if n == "pl_rade":
        return "R_earth"
    if n in ("pl_orbper", "p0", "period"):
        return "d"

    # Velocity/redshift
    if any(w in combined for w in ("radial_velocity", "radial_vel", "cz",
            "recessional")):
        return "km/s"
    if n == "rv":
        return "km/s"
    if n in ("redshift", "z", "zabs", "zphot"):
        return "z"
    if n == "parallax":
        return "mas"
    if n == "mass":
        return "M_sun"
    if n == "galactic_nh":
        return "cm-2"
    if n in ("pl_massj",):
        return "M_jup"
    if n == "pl_orbeccen":
        return "index"
    if n in ("sst_min", "sst_max"):
        return "degC"
    if n in ("redshift_lph_z_best", "redshift_info_distance"):
        return "pc"
    if n == "approx_source_var":
        return "index"
    if n == "wavelength":
        return "nm"
    if n == "sst":
        return "degC"
    if n == "signif_avg":
        return "sigma"
    if n == "baa_7day_max":
        return "degC"
    if n in ("he_peak", "he_peak_error", "he_nufnu_peak",
             "he_nufnu_peak_error", "lp_epeak", "lp_epeak_error",
             "plec_epeak", "plec_epeak_error", "plec_exp_factor_s",
             "plec_exp_factor_s_error"):
        return "eV"
    if n in ("time_peak", "time_peak_error"):
        return "s"
    if n == "npred":
        return "count"
    if n == "highest_energy_photon":
        return "eV"
    if n in ("isgri_soft_rate", "isgri_hard_rate",
             "isgri_soft_rate_error", "isgri_hard_rate_error"):
        return "count/s"
    if n in ("widthfitb", "spind", "var_20_40kev", "excess_variance"):
        return "index"
    if n == "zerr":
        return "z"
    if n in ("ctrpart_ra", "ctrpart_dec", "pos_err"):
        return "deg"
    if n == "log_lx":
        return "W"
    if n == "chi_squared":
        return "index"
    if n in ("ramdeg", "demdeg", "xpos", "ypos"):
        return "deg"
    if n in ("q_ramdeg", "q_demdeg", "ra(icrs)", "de(icrs)"):
        return "deg"
    if n in ("ragaia", "degaia", "e_ragaia"):
        return "deg"
    if n in ("speak", "sint"):
        return "Jy"
    if n in ("bckwide", "lrmswide"):
        return "Jy"
    if n == "epos":
        return "deg"
    if n in ("q_pmra", "q_pmde"):
        return "mas_yr"
    if "pixel" in n or "rawx" in n or "rawy" in n or "rawxy" in n:
        return "pixel"
    # XMM variability / detection statistics -> index
    if any(w in n for w in ("chi2", "prob", "fvar", "fratio", "fluxvar",
            "det_ml", "extent_ml", "ml", "dist_nn", "n_blend", "sig",
            "counts", "cts")):
        if "dist_nn" in n:
            return "arcsec"
        return "index"
    if n in ("extent", "extent_error", "extent_neg_err", "extent_pos_err"):
        return "index"

    # Water/tide
    if any(w in combined for w in ("tide", "water_level", "sea_level", "stage",
            "gauge_height", "sla", "adt", "ssh")):
        return "m"
    if n in ("msl", "mhhw", "mllw"):
        return "m"

    # Discharge
    if any(w in combined for w in ("discharge", "streamflow", "cfs",
            "cubic_feet")):
        return "m3/s"

    # Gravity
    if any(w in combined for w in ("mgal", "milligal", "free_air", "bouguer")):
        return "mGal"
    if any(w in combined for w in ("pga", "pgv", "peak_ground", "shakemap",
            "spectral_accel")):
        return "m/s2"

    # Frequency
    if any(w in combined for w in ("_hz", "frequency", "freq", "_khz", "_mhz",
            "_ghz", "nu_syn", "nu_eff")):
        return "Hz"

    # Energy
    if any(w in combined for w in ("_mev", "_gev", "_kev", "_ev",
            "electron_volt", "pivot_energy")):
        return "eV"
    if n in ("em_min", "em_max"):
        return "eV"
    if any(w in combined for w in ("_joule", "_j_")):
        return "J"

    # Duration
    if any(w in combined for w in ("_sec", "duration", "elapsed", "interval",
            "exptime", "exposure", "t_exptime", "dpd", "apd", "swp", "wwp",
            "tsun")):
        return "s"

    # Days
    if any(w in combined for w in ("dx32", "dt32", "dx70", "dx90", "dp01",
            "dp05", "dp10", "dysd", "dysn", "dyts", "dytg", "cdsd", "cldd",
            "hdsd", "htdd")):
        return "d"

    # Density
    if any(w in combined for w in ("density", "chlorophyll", "chl_", "biomass",
            "concentration", "conc_", "content", "abundance", "o2", "no3",
            "po4", "sio4", "oxygen", "nitrate", "phosphate", "silicate",
            "doc", "poc", "dic", "alkalinity")):
        return "kg/m3"
    if n == "nhi":
        return "cm-2"
    if n == "proton_density":
        return "p/cm3"

    # Power/electrical
    if any(w in combined for w in ("power", "watt", "_mw")):
        return "W"
    if any(w in combined for w in ("_volt", "voltage", "_amp")):
        return "V"

    # Time
    if n in ("mjd", "epoch"):
        return "mjd"
    if "datetime" in n:
        return "iso8601"
    if n == "obstime":
        return "unix_s"

    # Visibility
    if "vis" in n or "visib" in n:
        return "nmi"

    # Dimensionless with unit
    if any(w in combined for w in ("_index", "index_", "pl_index",
            "photon_index", "lp_index", "plec_index", "variability_index",
            "spectral_index", "pl_index_error", "lp_index_error",
            "plec_index_error", "lp_beta", "lp_beta_error", "plec_exp_index",
            "frac_variability_error")):
        return "index"
    if n in ("sig", "snr", "s_n", "detection_significance", "significance",
             "sigma", "confidence"):
        return "sigma"
    if n in ("mh_gspphot", "distmod", "radialvelocityerr", "limitingmag"):
        return "index"
    if n in ("scalerank", "steepness", "extent", "scale_rank"):
        return "index"
    if n in ("ssn", "smoothed_ssn", "sunspot_number", "numrows"):
        return "count"
    if n == "far":
        return "1/yr"
    if n == "dm":
        return "pc/cm3"
    if n in ("duration", "t90"):
        return "s"
    if n in ("a", "semi_major_axis", "pl_orbsmax"):
        return "au"
    if n == "pl_rade":
        return "R_earth"
    if n == "pl_bmasse":
        return "M_earth"
    if n in ("pl_orbper", "period", "p0"):
        return "d"
    if n == "frp":
        return "MW"

    # Generic
    if "wind" in combined or "gust" in combined:
        return "m/s"
    if n in ("value", "val", "v"):
        if force == "thermal":
            return "degC"
        if force in ("em", "em gravity"):
            return "mag"
        if force == "acoustic":
            return "hPa"
        if force in ("seismic-body", "seismic-surface"):
            return "mag"
        if force == "gravity":
            return "m"
        if force == "advective":
            return "m/s"
        if force == "diffusion":
            return "kg/m3"
        if force == "biotic":
            return "detection"

    return None


def infer_force(name, path, block_force, url=""):
    """Determine the physical force per field from field meaning."""
    n = name.lower()
    c = (n + "_" + path).lower()
    up = name.upper()

    # Domain-specific ambiguous resolution
    for dom_key, table in DOMAIN_FIELD_UNITS.items():
        if dom_key in url and name in table:
            return table[name][1]

    # unit suffix -> em/gravity
    if n.endswith(("_au", "_deg", "_degre", "_kev", "_mev", "_gev", "_ev",
                   "_hz", "_km_s", "_nm", "_kg_m3", "_ms", "_m_s")):
        return "gravity" if n.endswith(("_au", "_deg", "_degre")) else "em"

    # Unambiguous time/exposure/statistical -> em
    if n in ("duration", "exptime", "exposure", "t_exptime", "mjd", "epoch",
             "dateTime", "datetime", "t90", "obstime"):
        return "em"
    if n in ("confidence", "assoc_prob_bay", "assoc_prob_lr", "snr", "sig",
             "detection_significance", "significance", "sigma"):
        return "em"
    if n in ("P0", "p0"):
        return "gravity"
    if n in ("e_pmde", "e_pmra", "pmde", "e_raj2000", "e_dej2000"):
        return "em"
    if n in ("em_min", "em_max", "energy_bandpassname"):
        return "em"
    if n in ("b-v", "zabs", "zphot", "wavelength", "parallax", "2mass",
             "jname", "seq", "trigger_num", "qflg", "otype_txt",
             "majdiam", "mindiam", "mass", "redshift_lph_z_best",
             "redshift_info_distance", "approx_source_var"):
        return "em"
    if n in ("sst", "baa_7day_max", "sst_min", "sst_max"):
        return "thermal"
    if n == "alt":
        return "gravity"
    if n == "elev":
        return "gravity"
    if n in ("visibility", "vis", "s_ra", "angstrom_exponent_440-870nm",
             "validtimefrom", "validtimeto", "utc", "labels", "bns",
             "created", "t_scid", "assoc1", "revolution", "s_dec",
             "obs_collection", "filters", "calib_level", "site",
             "collection", "eo:cloud_cover", "landsat:collection_number",
             "wnd", "fatalities", "observed_on", "startdate", "enddate"):
        return "em"
    if n == "psal":
        return "diffusion"
    if "angstrom" in n or "refractive_index" in n or "asymmetry" in n or \
       "optical_depth" in n or        "single_scattering" in n or \
       (n.startswith("ssa_") and n.endswith("nm")):
        return "diffusion"
    if n in ("dens", "ssta", "ts_fig"):
        return "diffusion"
    if n in ("map", "cloud_modification", "observation_uuid", "user_login",
             "observed_on_month", "observation", "alert_level", "threat"):
        return "em"
    if n in ("stn", "yyyy", "mm", "volcanotitle", "activity", "slug",
             "subregion", "datacoverage", "polygonacres",
             "avg_max_monthly_mean", "bleaching_threshold"):
        return "em"
    if n in ("cwa", "forecastoffice", "forecasthourly", "gridid", "gridx",
             "gridy", "radarstation", "generatedat", "area_km", "uf",
             "alertlevel"):
        return "em"
    if n in ("poly_gisacres", "poly_polygondatetime", "floodclass",
             "countrycode", "owner", "pgm", "met", "area", "return_period",
             "doxy", "area_ha", "area_km2"):
        return "em"
    if n == "doxy":
        return "diffusion"
    if n in ("currents", "bottom_of_ocean_mixed_layer"):
        return "advective"
    if n in ("tle0", "tle1", "tle2", "tle_source", "cloudcover", "z_phot",
             "mass_donor", "mass_bh", "age", "e_mean"):
        return "em"
    if n in ("mass_donor", "mass_bh"):
        return "gravity"
    if n in ("p_value", "hardness_ratio_1", "hr2", "co_total_column",
             "co_surface", "co_500hpa", "co_300hpa", "luminosity_x",
             "luminosity_0_1_2_4", "size_arcmin", "avg_rate",
             "assoc_pulsar"):
        return "em"
    if n in ("rate_err", "mass_log_msun", "vel_disp", "host_galaxy",
             "sc_hr4", "spin", "subhalo_count", "gal_contam",
             "meanvaluesandf", "maxvaluesandf", "minvaluesandf",
             "descriptionsandf"):
        return "em"
    if n in ("mass_log_msun", "vel_disp"):
        return "gravity"
    if n in ("classtar", "sharp", "fullname", "spkid", "per", "s/n",
             "average", "minimum", "maximum", "pc:count"):
        return "em"
    if n in ("tic", "sectors", "lastburst", "tmag_unc", "p_dot", "pdot",
             "b_surf", "lx",              "tfopwg_disp", "t_min", "t_max", "pl_masse"):
        return "em"
    if n in ("pl_masse", "st_mass"):
        return "gravity"
    if n in ("boxes", "cr", "vx", "vy", "dec", "claimedtype", "mbcorr",
             "objid", "psfmag_g", "allwise"):
        return "em"
    if n in ("vx", "vy"):
        return "advective"
    if n in ("ex", "m_h", "alpha_m", "vhelio_avg", "sini", "reff", "assoc",
             "binary", "f0",              "s1400", "f2-8", "n"):
        return "em"
    if n in ("hr0", "asc", "mt", "tspan", "ruwe", "dr3name", "plx", "pm",
             "porb", "fx", "mx", "mdist"):
        return "em"
    if n in ("fe_h", "alpha_fe", "vrad", "age_bstep", "peak", "fpwide",
             "s(1ghz)", "sp-index", "vhb", "mvt", "rv", "e_alpha"):
        return "em"
    if n in ("hrv", "fwhm", "fwhma", "fwhmb", "wpeak", "r'-i'",
             "speaktot", "d", "mu-max", "beta-max", "oname", "opt"):
        return "em"
    if n in ("nep", "logvarpa", "s15", "mu_app", "loglir", "f12um",
             "f25um", "f60um",              "f100um", "local_rms", "nx", "blambda"):
        return "em"
    if n in ("stotal", "polfrac", "polangle", "wise", "cl", "rad",
             "hiiname", "country_name_en", "active_layer_thickness",
             "totaldeath", "totalaffected", "numberinjured"):
        return "em"
    if n in ("numberhomeless", "zipcode", "stat", "stat_st", "stat_lat",
             "stat_long", "evidence_method", "explosivity_index",
             "evidence_category",              "auth", "err", "daily_flood_likelihood"):
        return "em"
    if n in ("ndvi_anomaly", "drought_class", "susceptibility_class",
             "studyid", "abstract", "season", "deaths_total", "sal_surface",
             "eqmagunk", "damageamountorder", "damagemillionsdollars",
             "studyname"):
        return "em"
    if n == "sal_surface":
        return "diffusion"
    if n in ("deaths_total", "so", "chlor_a", "anom", "slev",
             "success_rate", "rank", "db_area", "petromag_g", "pop_max",
             "atomic_mass", "melt"):
        return "em"
    if n in ("so", "chlor_a"):
        return "diffusion"
    if n in ("deaths_total", "undulate", "round", "respondent-name",
             "aphiaid", "cellcount", "cellcount_units", "hab_category",
             "nuclear_capacity_mw", "ls_size", "fnu_12", "fnu_25"):
        return "em"
    if n in ("deaths_total", "fnu_60", "fnu_100", "f24", "e_f24",
             "snr24", "w3snr", "w4snr", "bleaching_alert_area",
             "daynight"):
        return "em"
    if n in ("establishmentmeans", "mediacount", "x_pole", "y_pole",
             "ut1-utc", "lod", "dpsi", "depsilon", "recclass",
             "pm25_ugm3"):
        return "em"
    if n in ("population", "des", "dist_min", "v_rel", "date_detected",
             "energy_range", "association", "elevation(m)", "tvoc", "nh3",
             "deaths_total", "i3_f_ap"):
        return "em"
    if n in ("nox_index", "voc_index", "z_phot_median", "model_cohort",
             "nominal_resolution", "nsbh", "ngoodobsrel", "ez_z_68l",
             "ez_z_68u", "ez_mass", "ugosa"):
        return "em"
    if n in ("vgosa", "radius_e", "sea_ice_fraction", "analysis_error",
             "peak_tm", "lev", "dusmass", "ducmass", "totexttau",
             "totsctau", "otype"):
        return "em"
    if n in ("vgosa", "radius_e"):
        return "advective"
    if n in ("totsctau", "rvz_redshift", "gdacs:alertlevel",
             "gdacs:country", "gdacs:alertscore", "strike1", "dip1",
             "rake1", "dist2land", "subbasin", "eqid"):
        return "em"
    if n in ("totaldeaths", "totaldamage", "totalinjuries", "totsctau",
             "elevationm", "morphology", "eruptionnumber",
             "lastknowneruption", "wteq", "occurrencestatus",
             "coordinateuncertaintyinmeters"):
        return "em"
    if n in ("scntfcn", "detectn", "cllct_d", "salinty", "turtleid",
             "curin", "curout", "dep", "loc", "pm25"):
        return "em"
    if n in ("curin", "curout"):
        return "advective"
    if n in ("salinty",):
        return "diffusion"
    if n in ("emsc:depth", "unit_of_measure", "eur", "gbp", "jpy",
             "features", "imp", "activitycategory"):
        return "em"
    if re.match(r"^col\d+$", n):
        return "em"
    if n in ("value_2", "totscatau", "totsctau", "deathstotal"):
        return "em"
    if n in ("sst_analysed", "sst_uncertainty", "dsci", "vnum", "dmaj",
             "s1ghz", "totsctau", "deaths_total", "value_2"):
        return "em"
    if n in ("feh", "r'-i", "strike", "dip", "rake", "preferredname",
             "howmany", "obsdt", "damage_usd", "timestamp"):
        return "em"
    if n in ("commonname", "chaetoceros", "alexandriu", "cochlodini",
             "country_iso", "gain_m2"):
        return "em"
    if n == "gain_m2":
        return "biotic"
    if "component_of_current" in n:
        return "advective"
    if n == "albedo":
        return "em"
    if n in ("logg", "logt", "rapmdeg", "depmdeg", "vlsr", "vartype",
             "min1", "alpha", "pa", "va", "iphas", "r'-ha", "r2"):
        return "em"
    if n in ("s15ghz", "stot", "ptot", "zd", "zl", "zs", "reg",
             "simbadname", "msz", "y5r500", "mcxc", "star", "deaths"):
        return "em"
    if n in ("injuries", "eruption_number", "eruption_month", "eruption_day",
             "tectonic_setting", "vei", "date_start", "date_end", "sid",
             "site_no", "net_slip_rate", "aseismic_slip_factor", "v_mean"):
        return "em"
    if n in ("observations", "stac_eo_cloud_cover", "hist_min", "hist_max",
             "capacity", "primary_fuel", "yn_snr", "peak_dt", "peak_va",
             "gage_ht",              "speed_radius", "rigidity_gv", "capacity_mw"):
        return "em"
    if n in ("yn_mass", "theta500", "i1_f_ap", "i2_f_ap", "i4_f_ap",
             "f3p6tot", "f4p5tot", "f5p8tot", "f8p0tot", "morph",
             "extended"):
        return "em"
    if n == "mangrove_area_km2":
        return "biotic"
    if n in ("pi", "self", "source_sample", "6dfgs", "nm", "nz", "p-value",
             "x_size", "y_size", "amplitude", "ugos", "vgos",
             "analysed_sst"):
        return "em"
    if n in ("headline", "areadesc", "volcanoid", "aviationcolorcode",
             "total_count", "maxscale", "kp_index", "estimated_kp",
             "a_running", "dst", "vel"):
        return "em"
    if n in ("area_m2", "loss_m2"):
        return "biotic"
    if n in ("station_count", "generated", "stid", "currentconditions",
             "multiplicity", "kmdepth", "nidheight", "damheight",
             "inclination", "f4", "f5", "f6"):
        return "em"
    if n in ("icao", "group", "pipeline", "wilayah", "waktu", "dirasakan",
             "unit", "kedalaman", "number_spots", "gpstime", "p_astro"):
        return "em"
    if n in ("dateofocc", "subreg", "hostility_d", "victim_d",
             "hostilitytype_l", "location_name_en", "location_name_fr",
             "stationid", "stationurl", "graphurl", "bright_t31"):
        return "em"
    if n in ("dtg", "hhmm", "tau", "incidenttypecategory", "eventtype",
             "hoursold", "irwinid", "sum_p0010001", "sum_h0010001",
             "sum_estimatedunder18pop", "dailyacres"):
        return "em"
    if n in ("sum_estimated18to64pop", "sum_estimated65pluspop",
             "sum_estimated0_14pop", "unitcode", "kenn", "nuclide", "tec",
             "vtec_assimilated_tecu", "vtec_rms_tecu", "movementdir",
             "movementspeed"):
        return "em"
    if n in ("rawtaf", "issuetime", "hazard", "textdescription", "massgap",
             "band", "bbh", "ext525", "ext1020", "f107", "f107_adj"):
        return "em"
    if n in ("tsi", "stdev", "carrington_rotation", "coverage",
             "satelliteid", "landsat:wrs_path", "landsat:wrs_row"):
        return "em"
    if n in ("detected_energy", "totalTimeInSeconds".lower(),
             "detected_duration", "hail_size"):
        return "em"
    if n in ("pl_massj", "pl_orbeccen"):
        return "gravity"
    if n == "visib":
        return "em"
    if n == "signif_avg":
        return "em"
    if n in ("ramdeg", "demdeg", "xpos", "ypos", "flag3", "ned", "tyc1",
             "tyc2", "tyc3", "pflag", "num", "epram", "epdem"):
        return "em"
    if n in ("q_ramdeg", "q_demdeg", "ra(icrs)", "de(icrs)", "q_pmra",
             "q_pmde", "prox", "tyc", "hip", "ccdm", "epra-1990",
             "epde-1990", "posflg"):
        return "em"
    if n in ("ragaia", "degaia", "e_ragaia", "speak", "sint", "corr",
             "srcidgaia", "org", "nu", "epucac", "sdss", "sptype", "mwsc"):
        return "em"
    if n in ("bckwide", "lrmswide", "epos", "ctot", "filename", "sname",
             "investigator", "speciescode", "varnum", "m_varnum", "gcvs",
             "processid", "bin"):
        return "em"
    if n in ("stateprovince", "occurrencedate", "rflg", "bflg", "cflg",
             "aflg", "pubdate", "georss:point", "volcanoname", "aqi",
             "totaltimeinseconds", "detected_duration", "detected_energy",
             "hail_size"):
        return "em"
    if "pixel" in n or "rawx" in n or "rawy" in n:
        return "em"
    # XMM variability/detection statistics -> em
    if any(w in n for w in ("chi2", "prob", "fvar", "fratio", "fluxvar",
            "det_ml", "extent_ml", "ml", "dist_nn", "n_blend", "sig",
            "counts", "cts", "extent")):
        return "em"
    if n in ("he_peak", "he_peak_error", "he_nufnu_peak",
             "he_nufnu_peak_error", "lp_epeak", "lp_epeak_error",
             "plec_epeak", "plec_epeak_error", "plec_exp_factor_s",
             "plec_exp_factor_s_error"):
        return "em"
    if n in ("time_peak", "time_peak_error"):
        return "em"
    if n == "npred":
        return "em"
    if "bat_" in n or n.startswith("lp_") or n.startswith("plec_"):
        return "em"
    if n in ("highest_energy_photon", "widthfitb", "spind",
             "isgri_soft_rate", "isgri_hard_rate", "var_20_40kev",
             "excess_variance", "zerr", "isgri_soft_rate_error",
             "isgri_hard_rate_error", "log_lx", "chi_squared",
             "ctrpart_ra", "ctrpart_dec", "pos_err",
             "bat_ra", "bat_dec", "bat_pos_err", "bat_theta", "bat_phi",
             "bat_t90", "bat_t50", "bat_start", "bat_stop",
             "bat_t100_start", "bat_t100_stop", "ot_ra", "ot_dec",
             "ot_pos_err", "galactic_nh", "redshift_err",
             "error_ell_major", "error_ell_minor", "error_ell_pa",
             "other_obs", "other_obs2", "other_obs3", "other_obs4",
             "srcid", "pps_srcnum"):
        return "em"
    if n == "pl_eqt":
        return "thermal"
    if n.endswith("_m") and len(n) == 3:
        return "em"  # J_m, H_m, K_m -> magnitudes
    # wdir / period disambiguation by domain
    if n == "wdir":
        if "ndbc.noaa.gov" in url or "arcgis.com" in url or "buoy" in url:
            return "acoustic"  # wave direction
        return "advective"  # wind direction
    if n == "period":
        if any(d in url for d in ("exoplanet", "ssd", "ssd-api", "sbdb",
                                  "minorplanet", "kepler", "orbit")):
            return "gravity"
        return "em"  # variability / pulsation

    # pl_index / lp_beta / variability_index / detection_significance families -> em
    if any(w in c for w in ("pl_index", "photon_index", "lp_index",
            "plec_index", "variability_index", "spectral_index", "lp_beta",
            "frac_variability", "detection_significance", "significance",
            "pl_index_error", "lp_index_error", "plec_index_error",
            "lp_beta_error", "plec_exp_index", "frac_variability_error",
            "snr", "s_n")):
        return "em"
    if n in ("wdsp", "wspd"):
        return "advective"
    if n == "frp":
        return "thermal"
    if n in ("mh_gspphot", "distmod", "limitingmag"):
        return "em"

    # GRB / X-ray instrument families -> em
    if any(n.startswith(p) for p in ("bat_", "lp_", "plec_", "xrt_", "uvot_",
            "pn_", "m1_", "m2_", "ep_", "om_", "spec_", "stack_", "classx_",
            "galaxy_", "wise_", "gaiadr3_")):
        return "em"

    # VizieR error prefix e_X -> same force as X (em)
    if n.startswith("e_") and len(n) > 2:
        return "em"
    if n in ("assoc_ra", "assoc_dec", "assoc_error_radius"):
        return "em"
    # NCEI climate codes
    if up in ("EMXT", "EMNT", "TAVG", "TMAX", "TMIN", "TOBS", "DX32", "DT32",
              "DX70", "DX90", "DP01", "DP05", "DP10", "DYSD", "DYSN", "DYTS",
              "DYTG", "CDSD", "CLDD", "HDSD", "HTDD", "PSUN"):
        return "thermal"
    if up in ("PRCP", "SNOW", "SNWD", "EVAP", "MNPN", "MXPN"):
        return "diffusion"
    if up in ("AWND", "WSF2", "WSF5"):
        return "advective"
    if up in ("WDF2", "WDF5", "TSUN"):
        return "acoustic"

    # thermal
    if any(w in c for w in ("temp", "sst", "teff", "dewp", "dew_point", "atmp",
            "wtmp", "heat_index", "wind_chill", "heating_week", "bright_ti",
            "tmax", "tmin", "tavg", "tobs", "emxt", "emnt", "dhw")):
        return "thermal"
    # diffusion
    if any(w in c for w in ("density", "o2", "no3", "po4", "chl", "biomass",
            "salinity", "conductivity", "turbidity", "pm2_5", "pm10", "aod",
            "aerosol", "particulate", "conc", "ppm", "ppb", "precip", "rain",
            "snow", "moisture", "humidity", "rhum", "rel_hum", "chlorophyll",
            "turbid", "oxygen", "nitrate", "phosphate", "silicate", "doc",
            "poc", "dic", "alkalinity", "co2", "ch4", "n2o", "prcp", "evap",
            "nhi", "abundance", "content", "col_density", "optical_depth")):
        return "diffusion"
    # advective
    if any(w in c for w in ("speed", "velocity", "spd", "wspd", "wind", "gust",
            "flow", "current", "vx", "vy", "vz", "proton_speed", "discharge",
            "streamflow", "runoff", "cfs", "track", "heading", "course",
            "awnd", "wsf2", "wsf5")):
        return "advective"
    # acoustic
    if any(w in c for w in ("pres", "baro", "slp", "altimeter", "press",
            "altim", "wave", "swh", "wvht", "vhm0", "swell", "wvhgt", "dpd",
            "apd", "mwd", "sound", "db", "wdf2", "wdf5")):
        return "acoustic"
    # em
    if any(w in c for w in ("_bt", "bz_", "by_", "bx_", "mag_field", "magnetic",
            "geomagnetic", "nt", "flux", "xray", "irrad", "radian", "energy",
            "_ev", "hz", "freq", "redshift", "vmag", "bmag", "rmag", "gmag",
            "imag", "zmag", "jmag", "hmag", "kmag", "phot_g", "bp_rp",
            "w1mpro", "w2mpro", "w3mpro", "w4mpro", "f10.7", "sfu", "mag",
            "magnitude", "pmra", "pmdec", "radial_vel", "cz", "photon",
            "luminos", "radio", "uv", "solar", "sunspot", "sr", "fpeak",
            "fint", "fluence", "count_rate", "spectral_index", "lii", "bii",
            "glon", "glat", "elon", "elat", "dm", "exptime", "exposure",
            "wavelength", "nu_syn", "pivot_energy", "fluence", "sfr")):
        return "em"
    # gravity
    if any(w in c for w in ("tide", "water_level", "sea_level", "sla", "adt",
            "ssh", "gauge_height", "stage", "msl", "mgal", "free_air",
            "bouguer", "grav", "orb", "pl_orbsmax", "pl_orbper", "pl_rade",
            "pl_bmasse", "semi_major", "eccen", "inclination", "depth",
            "elevation", "altitude", "height", "diameter", "radius",
            "sy_dist", "dist", "distance", "pl_radj", "maj", "mhhw", "mllw",
            "latitude", "longitude", "lat", "lon")):
        return "gravity"
    # seismic
    if any(w in c for w in ("pga", "pgv", "shakemap", "peak_ground",
            "spectral_accel", "seismic", "intensity", "mmi", "dmin", "gap")):
        return "seismic-surface"
    # single-letter photometric bands -> em
    if n in "iurgzvbyhjkb" and len(n) == 1:
        return "em"
    # error/sigma fields -> em (statistical on em measurements)
    if n in ("sig", "snr", "s_n", "sigma", "error", "err", "e", "t", "r",
             "q", "n", "max", "min", "mean", "extent", "scale_rank",
             "scalerank", "steepness"):
        return block_force
    # fallback to block force only for generic names
    if n in ("value", "val", "v"):
        return block_force
    return None


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
            new_blocks.append("")
            continue
        lines = b.split("\n")
        force = ""
        url = ""
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
                new_lines.append(ls)
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
                base = "lat" if "lat" in key else "lon"
                new_lines.append(f"{base} {parts[1]} deg")
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
                new_lines.append(f"dec {parts[1]} deg")
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
                    col_name = native[0]
                    unit = native[1]
                    f = native[2]
                if unit is None and tgt.isdigit():
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
                if unit in PHYSICAL_UNITS and f:
                    new_lines.append(f"field {src} {f} {unit}")
                    stats["units"][unit] += 1
                    stats["forces"][f] += 1
                else:
                    # not a physical quantity -> stripped (never an oscillator)
                    stats["structural"] += 1
                continue

            if key in ("last", "first", "count", "path", "deep", "last_row",
                       "obj_last"):
                if len(parts) >= 3:
                    src = parts[1]
                    tgt = parts[2] if len(parts) > 2 else src
                    unit = infer_unit(tgt, src, force, url)
                    col_name = None
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
                    if unit in PHYSICAL_UNITS and f:
                        new_lines.append(f"{key} {src} {f} {unit}")
                        stats["units"][unit] += 1
                        stats["forces"][f] += 1
                    else:
                        stats["structural"] += 1
                continue

            if key in ("last_obj", "last_line", "regex", "xml", "ephemeris",
                       "vectors", "epoch_key", "val_key", "vel_key",
                       "trk_key", "vr_key", "dist_key", "z_key", "tau_key",
                       "alt_sign", "lon_sign", "lat_sign", "rv_key",
                       "dist_scale", "rv_scale"):
                new_lines.append(ls)
                continue
            new_lines.append(ls)

        new_blocks.append("\n".join(new_lines))

    result = "\n\n".join(new_blocks)
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
