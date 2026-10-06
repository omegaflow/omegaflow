use super::*;

pub fn series_parse_bin(format: &str, bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    match format {
        "rpw_efield" => crate::rpw::parse_bin(bytes),
        "goes_xrs" => goes::parse_bin(bytes),
        "intermagnet_dbdt" => intermagnet::parse_bin(bytes),
        "omni2_serie" => omni2::parse_bin(bytes),
        "omni_hro" => omni_hro::parse_bin(bytes),
        "mitdb" => mitdb::parse_bin(bytes),
        "circor" => phonocardiogram::parse_bin(bytes),
        "ltmm" => movement_monitoring::parse_bin(bytes),
        "noaa_ccor" => ccor::parse_bin(bytes),
        "celestrak_eop" => celestrak_eop::parse_bin(bytes),
        "gk2a_ami" => gk2a_ami::parse_bin(bytes).map(|granules| {
            granules
                .into_iter()
                .map(|g| (g.t, g.rad_mean as f64, gk2a_ami::COMP_RADIANCE))
                .collect()
        }),
        "goes_abi" => goes_abi::parse_bin(bytes).map(|granules| {
            granules
                .into_iter()
                .map(|g| (g.t, g.rad_mean as f64, goes_abi::COMP_RADIANCE))
                .collect()
        }),
        "atdf" => atdf::parse_series(bytes),
        "ulysses_atdf" => atdf::parse_uly_series(bytes),
        "ulysses_atdf_x" => atdf::parse_uly_series_x(bytes),
        "gll_rss_atdf" => atdf::parse_gll_series(bytes),
        "gll_rss_atdf_x" => atdf::parse_gll_series_x(bytes),
        "lro_trk" => lro_utf::parse_series(bytes),
        "himawari_hsd" => hsd::parse_series(bytes),
        "maxi" => crate::maxi::parse_bin(bytes).map(|curves| {
            let mut out = Vec::new();
            for c in curves {
                for s in c.samples {
                    out.push((s.t_tdb, s.flux as f64, c.band));
                }
            }
            out
        }),
        "voyager_saturn" => voyager_saturn::parse_series(bytes),
        "mariner_occlt" => mariner_occlt::parse_series(bytes),
        "cors_rinex" => cors::parse_series(bytes),
        "drs_fits" => drs_fits::parse_series(bytes),
        "demeter_isl" => demeter::parse_series(bytes),
        "kcdc_kascade" => kcdc::parse_series(bytes),
        "juno_odf" => odf::parse_series(bytes),
        "juno_ocru_odf" => odf::parse_series(bytes),
        "magellan_odf" => odf::parse_series(bytes),
        "mgs_odf" => odf::parse_series(bytes),
        "mro_odf" => odf::parse_series(bytes),
        "odyssey_odf" => odf::parse_series(bytes),
        "messenger_odf" => odf::parse_series(bytes),
        "mars_express_odf" => odf::parse_series(bytes),
        "rosetta_odf" => ifms_agc::parse_series(bytes).map(|rows| {
            let mut out = Vec::with_capacity(rows.len() * 2);
            for (t, level, polar) in rows {
                out.push((t, level, ifms_agc::COMP_CARRIER_LEVEL));
                out.push((t, polar, ifms_agc::COMP_POLAR_ANGLE));
            }
            out
        }),
        "viking_text" => viking_text::parse_series(bytes),
        "voyager1_merged" => voyager_merged::parse_series(bytes),
        "voyager2_merged" => voyager_merged::parse_series(bytes),
        "vex_odf" => odf::parse_series(bytes),
        "galileo_odf" => odf::parse_series(bytes),
        "dawn_odf" => odf::parse_series(bytes),
        "cassini_odf" => odf::parse_series(bytes),
        "pioneer10_odf" => odf::parse_series(bytes),
        "pioneer11_odf" => odf::parse_series(bytes),
        "pathfinder_odf" => odf::parse_series(bytes),
        "odf_serie" => odf::odf_series(bytes),
        "cassini_tnf" | "maven_tnf" | "dart_tnf" | "messenger_tnf" => odf::tnf_parse_series(bytes),
        "ams02_spec" => tdat::ams02_series(bytes),
        "voyager_odr" => voyager_odr::parse_series(bytes),
        "voyager_occlt" => voyager_occlt::parse_series(bytes),
        "pds3_ring_occ" => pds3_ring_occ::parse_series(bytes),
        "pds3_fixed_width" => pds3_table::parse_table(bytes)
            .and_then(|t| fixed_width_rows_to_series(t.rows.iter().map(|r| r.values.as_slice()))),
        "pds3_binary" => pds3_binary::parse_table(bytes)
            .and_then(|t| fixed_width_rows_to_series(t.rows.iter().map(|r| r.values.as_slice()))),
        "pds3_img" => pds3_img::parse_series(bytes),
        "pds4_fixed_width" => pds4::parse_table(bytes)
            .and_then(|t| fixed_width_rows_to_series(t.rows.iter().map(|r| r.values.as_slice()))),
        "pds4_binary" => pds4_binary::parse_table(bytes)
            .and_then(|t| fixed_width_rows_to_series(t.rows.iter().map(|r| r.values.as_slice()))),
        "pds4_fits" => pds4_fits::parse_series(bytes),
        "pradan_ch2" => pradan_ch2::parse_series(bytes),
        "hips_png" => hips::parse_asset(bytes),
        "gras_2c" => gras_2c::parse_series(bytes),
        "pds4_acs_nir" => acs_nir::parse_series(bytes),
        "galileo_odr" => galileo_odr::parse_series(bytes),
        "galileo_ionocal" => ionocal::parse_series(bytes),
        "cassini_rsr" => cassini_rsr::parse_series(bytes),
        "flac" => flac::parse_series(bytes),
        "bidsleep" => bidsleep::parse_bin(bytes),
        "bison_velocity" => crate::bison_velocity::parse_bin(bytes)
            .map(|rs| rs.into_iter().map(|(t, v)| (t, v, 0)).collect()),
        "las" => crate::las::las_series::parse_series(bytes),
        "hamqsl_solar" => hamqsl::parse_bin(bytes),
        "usgs_comcat_m45" => usgs_comcat::parse_bin(bytes),
        "quake_ptevent" => quake_event::parse_series(bytes),
        "f107" => f107::parse_series(bytes),
        "goes_euvs" => euvs::parse_bin(bytes),
        "aia" => aia::parse_bin(bytes),
        "eve" => eve::parse_bin(bytes),
        "hapi_csv" => hapi_csv::parse_bin(bytes),
        "kasi" => kasi::parse_series(bytes),
        "wqp_result" => wqp_result::parse_series(bytes),
        "fink" => fink::parse_series(bytes),
        "eea_noise" => eea_noise::parse_series(bytes),
        "gaia_rrl" => gaia_rrl::parse_series(bytes),
        "viking_grav" => viking_grav::parse_series(bytes),
        "agrav" => agrav::parse_series(bytes),
        "emm_exi_l2a" => emm_exi::parse_series(bytes),
        _ => None,
    }
}

fn fixed_width_rows_to_series<'a, I>(rows: I) -> Option<Vec<(f64, f64, u32)>>
where
    I: IntoIterator<Item = &'a [Option<f64>]>,
{
    let mut out = Vec::new();
    for row in rows {
        let t = match row.first() {
            Some(Some(t)) => *t,
            _ => continue,
        };
        for (j, v) in row.iter().enumerate().skip(1) {
            if let Some(x) = v {
                out.push((t, *x, j as u32));
            }
        }
    }
    if out.is_empty() { None } else { Some(out) }
}

pub fn verify_records(format: &str, bytes: &[u8]) -> Option<usize> {
    match format {
        "catalog_mpcorb" => {
            let (chunks, _) = bytes.as_chunks::<{ crate::mpcorb::RECORD_STRIDE }>();
            Some(
                chunks
                    .iter()
                    .filter(|c| crate::mpcorb::parse_record(*c).is_some())
                    .count(),
            )
        }
        "catalog_dcom5" => {
            let (chunks, _) = bytes.as_chunks::<{ crate::dastcom::COMET_RECORD_BYTES }>();
            Some(
                chunks
                    .iter()
                    .filter(|c| crate::dastcom::parse_comet_record(*c).is_some())
                    .count(),
            )
        }
        "catalog_des_y6" => {
            let (chunks, _) = bytes.as_chunks::<{ crate::des_y6::DES_Y6_RECORD_STRIDE }>();
            Some(
                chunks
                    .iter()
                    .filter(|c| crate::des_y6::parse_record(*c).is_some())
                    .count(),
            )
        }
        "catalog_ossos" => {
            let (chunks, _) = bytes.as_chunks::<{ crate::ossos::OSSOS_RECORD_STRIDE }>();
            Some(
                chunks
                    .iter()
                    .filter(|c| crate::ossos::parse_record(*c).is_some())
                    .count(),
            )
        }
        "catalog_gaia_sso" => crate::gaia_sso::parse_bin(bytes).map(|bodies| bodies.len()),
        "mpcobs" => {
            if !bytes.len().is_multiple_of(MPCOBS_RECORD_STRIDE) {
                return None;
            }
            let mut n = 0usize;
            for c in bytes.as_chunks::<MPCOBS_RECORD_STRIDE>().0 {
                let epoch = f64::from_le_bytes(c[0..8].try_into().ok()?);
                let ra = f64::from_le_bytes(c[8..16].try_into().ok()?);
                let dec = f64::from_le_bytes(c[16..24].try_into().ok()?);
                if epoch.is_finite() && ra.is_finite() && dec.is_finite() {
                    n += 1;
                }
            }
            Some(n)
        }
        "apdb" => {
            if bytes.is_empty() {
                return None;
            }
            Some(
                String::from_utf8_lossy(bytes)
                    .lines()
                    .filter(|l| !l.trim().is_empty())
                    .count(),
            )
        }
        "solar_system_bodies" => {
            if bytes.len() < 8 || bytes[0..4] != SOLAR_BODIES_MAGIC {
                return None;
            }
            let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
            let stride = 64 + 64 + 8 + 8 + 11 * 8;
            if bytes.len() != 8 + n * stride {
                return None;
            }
            Some(n)
        }
        "hips_png" => hips::parse_bin(bytes).map(|_| 1),
        "astrometry_series" => astrometry_series_counts(bytes).map(|(_, samples)| samples),
        "viking_grav" => pds3_table::parse_table(bytes).map(|t| t.rows.len()),
        _ => None,
    }
}

pub fn astrometry_series_counts(bytes: &[u8]) -> Option<(usize, usize)> {
    let series = astrometry_series::parse_bin(bytes)?;
    let samples: usize = series.iter().map(|s| s.samples.len()).sum();
    Some((series.len(), samples))
}

const MPCOBS_RECORD_STRIDE: usize = 50;
const SOLAR_BODIES_MAGIC: [u8; 4] = [0xCF, 0x86, 0x0A, 0x00];

pub fn phase_series_parse_bin(format: &str, bytes: &[u8]) -> Option<Vec<odf::TnfPhaseRow>> {
    match format {
        "cassini_tnf" | "maven_tnf" | "dart_tnf" | "messenger_tnf" => odf::tnf_phase_series(bytes),
        "eht_uvfits" => uvfits::beat_rows(bytes),
        _ => None,
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SeriesRow {
    pub t: f64,
    pub value: f64,
    pub comp: u32,
    pub freq: f64,
    pub bin_width: f64,
}

pub fn series_rows(format: &str, bytes: &[u8]) -> Option<Vec<SeriesRow>> {
    if let Some(rows) = phase_series_parse_bin(format, bytes) {
        return Some(
            rows.into_iter()
                .map(|r| SeriesRow {
                    t: r.t,
                    value: r.value,
                    comp: r.comp,
                    freq: r.freq,
                    bin_width: r.bin_width,
                })
                .collect(),
        );
    }
    series_parse_bin(format, bytes).map(|recs| {
        let (freq, bin_width) = (spectral::SPECTRAL_NO_BAND, spectral::SPECTRAL_NO_BAND);
        recs.into_iter()
            .map(|(t, value, comp)| SeriesRow {
                t,
                value,
                comp,
                freq,
                bin_width,
            })
            .collect()
    })
}

#[derive(Clone, Debug)]
pub struct NamedSeries {
    pub rows: Vec<SeriesRow>,
    pub names: Vec<String>,
}

pub fn series_named(format: &str, bytes: &[u8]) -> Option<NamedSeries> {
    if format == "dsn_snapshot" {
        let (rows, names) = dsn::series_parse(bytes)?;
        let (freq, bin_width) = (spectral::SPECTRAL_NO_BAND, spectral::SPECTRAL_NO_BAND);
        return Some(NamedSeries {
            rows: rows
                .into_iter()
                .map(|(t, value, comp)| SeriesRow {
                    t,
                    value,
                    comp,
                    freq,
                    bin_width,
                })
                .collect(),
            names,
        });
    }
    let (names, raw_rows) = match format {
        "pds3_fixed_width" => {
            let table = pds3_table::parse_table(bytes)?;
            let names: Vec<String> = table.columns.into_iter().map(|c| c.name).collect();
            let rows = fixed_width_rows_to_series(table.rows.iter().map(|r| r.values.as_slice()))?;
            (names, rows)
        }
        "pds3_binary" => {
            let table = pds3_binary::parse_table(bytes)?;
            let names: Vec<String> = table.columns.into_iter().map(|c| c.name).collect();
            let rows = fixed_width_rows_to_series(table.rows.iter().map(|r| r.values.as_slice()))?;
            (names, rows)
        }
        "pds3_img" => {
            let raster = pds3_img::parse_image(bytes)?;
            let names: Vec<String> = raster.band_names.clone();
            let rows = pds3_img::band_means(&raster)?;
            (names, rows)
        }
        "pds4_fixed_width" => {
            let table = pds4::parse_table(bytes)?;
            let names: Vec<String> = table.columns.into_iter().map(|c| c.name).collect();
            let rows = fixed_width_rows_to_series(table.rows.iter().map(|r| r.values.as_slice()))?;
            (names, rows)
        }
        "pds4_binary" => {
            let table = pds4_binary::parse_table(bytes)?;
            let names: Vec<String> = table.columns.into_iter().map(|c| c.name).collect();
            let rows = fixed_width_rows_to_series(table.rows.iter().map(|r| r.values.as_slice()))?;
            (names, rows)
        }
        "pds4_fits" => {
            let (names, rows) = pds4_fits::parse_named_series(bytes)?;
            (names, rows)
        }
        "gras_2c" => {
            let recs = gras_2c::parse_series(bytes)?;
            let max_comp = recs.iter().map(|r| r.2).max()?;
            let names: Vec<String> = (0..=max_comp).map(gras_2c::gate_name).collect();
            (names, recs)
        }
        "pds4_acs_nir" => {
            let recs = acs_nir::parse_series(bytes)?;
            let names: Vec<String> = (0..acs_nir::ROW_DATA_COUNT as u32)
                .map(acs_nir::component_name)
                .collect();
            (names, recs)
        }
        _ => return None,
    };
    let (freq, bin_width) = (spectral::SPECTRAL_NO_BAND, spectral::SPECTRAL_NO_BAND);
    Some(NamedSeries {
        rows: raw_rows
            .into_iter()
            .map(|(t, value, comp)| SeriesRow {
                t,
                value,
                comp,
                freq,
                bin_width,
            })
            .collect(),
        names,
    })
}

pub fn series_declared_fields(format: &str, names: &[String], tau: f64) -> Vec<FieldConfig> {
    match format {
        "gras_2c" => names
            .iter()
            .enumerate()
            .filter_map(|(comp, _)| gras_2c::gate_field(comp as u32, tau))
            .collect(),
        "kasi" => kasi::declared_fields(tau),
        "wqp_result" => wqp_result::declared_fields(tau),
        "fink" => fink::declared_fields(tau),
        "eea_noise" => eea_noise::declared_fields(tau),
        "gaia_rrl" => gaia_rrl::declared_fields(tau),
        "viking_grav" => viking_grav::declared_fields(tau),
        "agrav" => agrav::declared_fields(tau),
        "pds4_acs_nir" => acs_nir::declared_fields(),
        _ => Vec::new(),
    }
}

pub fn sample_phase(channel: &Channel, sensor: &FieldConfig) -> Option<f64> {
    if sensor.key != "ul_phase_cycles" {
        return None;
    }
    odf::carrier_phase_rad(channel.value)
}

pub fn series_component_name(format: &str, comp: u32) -> Option<&'static str> {
    match format {
        "rpw_efield" => match comp {
            crate::rpw::COMP_EY => Some("rpw_e_y"),
            crate::rpw::COMP_EZ => Some("rpw_e_z"),
            _ => None,
        },
        "goes_xrs" => match comp {
            goes::COMP_XRSA => Some("goes_xrs_xrsa"),
            goes::COMP_XRSB => Some("goes_xrs_xrsb"),
            _ => None,
        },
        "intermagnet_dbdt" => match comp {
            intermagnet::COMP_DBDT => Some("intermagnet_dbdt"),
            _ => None,
        },
        "omni2_serie" => match comp {
            omni2::COMP_V1800 => Some("omni_solarwind_flow_speed_kms"),
            omni2::COMP_N1800 => Some("omni_solarwind_density_percc"),
            omni2::COMP_T1800 => Some("omni_solarwind_temp_k"),
            omni2::COMP_BX => Some("omni_imf_bx_gse_nt"),
            omni2::COMP_BY => Some("omni_imf_by_gsm_nt"),
            omni2::COMP_BZ => Some("omni_imf_bz_gsm_nt"),
            omni2::COMP_PRESSURE => Some("omni_solarwind_pressure_npa"),
            omni2::COMP_AE => Some("magnetosphere_ae_nt"),
            omni2::COMP_AL => Some("magnetosphere_al_nt"),
            omni2::COMP_AU => Some("magnetosphere_au_nt"),
            omni2::COMP_SYMH => Some("magnetosphere_symh_nt"),
            omni2::COMP_DST => Some("magnetosphere_dst_nt"),
            _ => None,
        },
        "omni_hro" => match comp {
            omni_hro::COMP_IMF_F => Some("omni_hro_imf_f_nt"),
            omni_hro::COMP_IMF_BX_GSE => Some("omni_hro_imf_bx_gse_nt"),
            omni_hro::COMP_IMF_BY_GSM => Some("omni_hro_imf_by_gsm_nt"),
            omni_hro::COMP_IMF_BZ_GSM => Some("omni_hro_imf_bz_gsm_nt"),
            omni_hro::COMP_SW_FLOW_SPEED => Some("omni_hro_solarwind_flow_speed_kms"),
            omni_hro::COMP_SW_DENSITY => Some("omni_hro_solarwind_density_percc"),
            omni_hro::COMP_SW_TEMP => Some("omni_hro_solarwind_temp_k"),
            omni_hro::COMP_SW_PRESSURE => Some("omni_hro_solarwind_pressure_npa"),
            _ => None,
        },
        "hamqsl_solar" => match comp {
            hamqsl::COMP_SOLARFLUX => Some("hamqsl_solarflux_sfu"),
            hamqsl::COMP_SOLARWIND => Some("hamqsl_solarwind_kms"),
            hamqsl::COMP_MAGFIELD => Some("hamqsl_magneticfield_nt"),
            _ => None,
        },
        "usgs_comcat_m45" => usgs_comcat::component_name(comp),
        "quake_ptevent" => quake_event::component_name(comp),
        "f107" => f107::component_name(comp),
        "goes_euvs" => euvs::component_name(comp),
        "mitdb" => match comp {
            mitdb::COMP_MLII => Some("mitdb_mlii"),
            mitdb::COMP_V1 => Some("mitdb_v1"),
            mitdb::COMP_V2 => Some("mitdb_v2"),
            mitdb::COMP_V4 => Some("mitdb_v4"),
            mitdb::COMP_V5 => Some("mitdb_v5"),
            _ => None,
        },
        "circor" => match comp {
            phonocardiogram::COMP_AV => Some("circor_pcg_av"),
            phonocardiogram::COMP_MV => Some("circor_pcg_mv"),
            phonocardiogram::COMP_PV => Some("circor_pcg_pv"),
            phonocardiogram::COMP_TV => Some("circor_pcg_tv"),
            phonocardiogram::COMP_PHC => Some("circor_pcg_phc"),
            _ => None,
        },
        "ltmm" => match comp {
            movement_monitoring::COMP_V => Some("ltmm_v_accel"),
            movement_monitoring::COMP_ML => Some("ltmm_ml_accel"),
            movement_monitoring::COMP_AP => Some("ltmm_ap_accel"),
            movement_monitoring::COMP_YAW => Some("ltmm_yaw_rate"),
            movement_monitoring::COMP_PITCH => Some("ltmm_pitch_rate"),
            movement_monitoring::COMP_ROLL => Some("ltmm_roll_rate"),
            _ => None,
        },
        "noaa_ccor" => match comp {
            ccor::COMP_INTENSITY => Some("noaa_ccor_intensity_dn"),
            _ => None,
        },
        "aia" => match comp {
            aia::COMP_94 => Some("aia_94_dn"),
            aia::COMP_131 => Some("aia_131_dn"),
            aia::COMP_171 => Some("aia_171_dn"),
            aia::COMP_193 => Some("aia_193_dn"),
            aia::COMP_211 => Some("aia_211_dn"),
            aia::COMP_304 => Some("aia_304_dn"),
            aia::COMP_335 => Some("aia_335_dn"),
            _ => None,
        },
        "eve" => match comp {
            eve::COMP_94 => Some("eve_94_line_irradiance"),
            eve::COMP_131 => Some("eve_131_line_irradiance"),
            eve::COMP_171 => Some("eve_171_line_irradiance"),
            eve::COMP_195 => Some("eve_195_line_irradiance"),
            eve::COMP_211 => Some("eve_211_line_irradiance"),
            eve::COMP_284 => Some("eve_284_line_irradiance"),
            eve::COMP_304 => Some("eve_304_line_irradiance"),
            eve::COMP_335 => Some("eve_335_line_irradiance"),
            eve::COMP_584 => Some("eve_584_line_irradiance"),
            eve::COMP_977 => Some("eve_977_line_irradiance"),
            eve::COMP_1032 => Some("eve_1032_line_irradiance"),
            _ => None,
        },
        "hapi_csv" => match comp {
            1 => Some("hapi_csv_magnitude_nt"),
            2 => Some("hapi_csv_x_nt"),
            3 => Some("hapi_csv_y_nt"),
            4 => Some("hapi_csv_z_nt"),
            _ => None,
        },
        "celestrak_eop" => match comp {
            celestrak_eop::COMP_UT1_UTC => Some("eop_iers_ut1_utc_s"),
            celestrak_eop::COMP_PMX => Some("eop_iers_polar_motion_x_arcsec"),
            celestrak_eop::COMP_PMY => Some("eop_iers_polar_motion_y_arcsec"),
            _ => None,
        },
        "gk2a_ami" => gk2a_ami::component_name(comp),
        "hips_png" => hips::component_name(comp),
        "goes_abi" => goes_abi::component_name(comp),
        "mtg_li" => mtg_li::component_name(comp),
        "emm_exi_l2a" => emm_exi::component_name(comp),
        "atdf" => atdf::component_name(comp),
        "ulysses_atdf" => atdf::uly_component_name(comp),
        "ulysses_atdf_x" => atdf::uly_component_name(comp),
        "gll_rss_atdf" => atdf::gll_component_name(comp),
        "gll_rss_atdf_x" => atdf::gll_component_name(comp),
        "lro_trk" => lro_utf::component_name(comp),
        "himawari_hsd" => hsd::component_name(comp),
        "maxi" => match comp {
            crate::maxi::BAND_2_20 => Some("maxi_2_20kev_flux_ph_s_cm2"),
            crate::maxi::BAND_2_4 => Some("maxi_2_4kev_flux_ph_s_cm2"),
            crate::maxi::BAND_4_10 => Some("maxi_4_10kev_flux_ph_s_cm2"),
            crate::maxi::BAND_10_20 => Some("maxi_10_20kev_flux_ph_s_cm2"),
            _ => None,
        },
        "voyager_saturn" => match comp {
            voyager_saturn::COMP_DOPPLER_HP => Some("voyager_saturn_doppler_count_hp"),
            voyager_saturn::COMP_DOPPLER_LP => Some("voyager_saturn_doppler_count_lp"),
            voyager_saturn::COMP_ANGLE_A => Some("voyager_saturn_angle_a"),
            voyager_saturn::COMP_ANGLE_B => Some("voyager_saturn_angle_b"),
            _ => None,
        },
        "mariner_occlt" => match comp {
            mariner_occlt::COMP_AMP_MIN => Some("mariner10_occlt_amp_min"),
            mariner_occlt::COMP_AMP_MAX => Some("mariner10_occlt_amp_max"),
            mariner_occlt::COMP_AMP_MEAN => Some("mariner10_occlt_amp_mean"),
            _ => None,
        },
        "voyager_occlt" => match comp {
            voyager_occlt::COMP_AMP_MIN => Some("voyager_occlt_amp_min"),
            voyager_occlt::COMP_AMP_MAX => Some("voyager_occlt_amp_max"),
            voyager_occlt::COMP_AMP_MEAN => Some("voyager_occlt_amp_mean"),
            _ => None,
        },
        "cors_rinex" => match comp {
            cors::OBS_L1 => Some("cors_rinex_l1_cycle"),
            cors::OBS_L2 => Some("cors_rinex_l2_cycle"),
            cors::OBS_L5 => Some("cors_rinex_l5_cycle"),
            cors::OBS_C1 => Some("cors_rinex_c1_m"),
            cors::OBS_P1 => Some("cors_rinex_p1_m"),
            cors::OBS_C2 => Some("cors_rinex_c2_m"),
            cors::OBS_P2 => Some("cors_rinex_p2_m"),
            cors::OBS_C5 => Some("cors_rinex_c5_m"),
            cors::OBS_S1 => Some("cors_rinex_s1_dbhz"),
            cors::OBS_S2 => Some("cors_rinex_s2_dbhz"),
            cors::OBS_S5 => Some("cors_rinex_s5_dbhz"),
            _ => None,
        },
        "drs_fits" => match comp {
            drs_fits::COMP_GX => Some("lpf_drs_dg_x_ms2"),
            drs_fits::COMP_GY => Some("lpf_drs_dg_y_ms2"),
            drs_fits::COMP_GZ => Some("lpf_drs_dg_z_ms2"),
            _ => None,
        },
        "pradan_ch2" => pradan_ch2::component_name(comp),
        "demeter_isl" => match comp {
            demeter::COMP_ORBIT => Some("demeter_isl_orbit_count"),
            demeter::COMP_NE => Some("demeter_isl_ne_cm3"),
            demeter::COMP_NI => Some("demeter_isl_ni_cm3"),
            demeter::COMP_TE => Some("demeter_isl_te_k"),
            demeter::COMP_VF => Some("demeter_isl_vf_v"),
            demeter::COMP_VI0 => Some("demeter_isl_vi0_ms"),
            _ => None,
        },
        "kcdc_kascade" => match comp {
            kcdc::COMP_ARRAY_E => Some("kcdc_kascade_energy_ev"),
            kcdc::COMP_ARRAY_XC => Some("kcdc_kascade_x_core_m"),
            kcdc::COMP_ARRAY_YC => Some("kcdc_kascade_y_core_m"),
            kcdc::COMP_ARRAY_ZE => Some("kcdc_kascade_zenith_deg"),
            kcdc::COMP_ARRAY_AZ => Some("kcdc_kascade_azimuth_deg"),
            kcdc::COMP_ARRAY_NE => Some("kcdc_kascade_ne_count"),
            kcdc::COMP_ARRAY_NMU => Some("kcdc_kascade_nmu_count"),
            kcdc::COMP_ARRAY_AGE => Some("kcdc_kascade_shower_age"),
            kcdc::COMP_CALO_NHAD => Some("kcdc_calorimeter_nhad_count"),
            kcdc::COMP_CALO_EHAD => Some("kcdc_calorimeter_ehad_ev"),
            kcdc::COMP_GRANDE_XC => Some("kcdc_grande_x_core_m"),
            kcdc::COMP_GRANDE_YC => Some("kcdc_grande_y_core_m"),
            kcdc::COMP_GRANDE_ZE => Some("kcdc_grande_zenith_deg"),
            kcdc::COMP_GRANDE_AZ => Some("kcdc_grande_azimuth_deg"),
            kcdc::COMP_GRANDE_NCH => Some("kcdc_grande_nch_count"),
            kcdc::COMP_GRANDE_NMU => Some("kcdc_grande_nmu_count"),
            kcdc::COMP_GRANDE_AGE => Some("kcdc_grande_shower_age"),
            kcdc::COMP_GEN_T => Some("kcdc_general_air_temp_c"),
            kcdc::COMP_GEN_P => Some("kcdc_general_air_pressure_hpa"),
            kcdc::COMP_LOPES_EFIELDMAX => Some("kcdc_lopes_efieldmax_v_m"),
            _ => None,
        },
        "juno_odf" => match comp {
            odf::COMP_OBSERVABLE => Some("juno_odf_observable_hz"),
            _ => None,
        },
        "juno_ocru_odf" => match comp {
            odf::COMP_OBSERVABLE => Some("juno_ocru_odf_observable_hz"),
            _ => None,
        },
        "magellan_odf" => match comp {
            odf::COMP_OBSERVABLE => Some("magellan_odf_observable_hz"),
            _ => None,
        },
        "mgs_odf" => match comp {
            odf::COMP_OBSERVABLE => Some("mgs_odf_observable_hz"),
            _ => None,
        },
        "mro_odf" => match comp {
            odf::COMP_OBSERVABLE => Some("mro_odf_observable_hz"),
            _ => None,
        },
        "odyssey_odf" => match comp {
            odf::COMP_OBSERVABLE => Some("odyssey_odf_observable_hz"),
            _ => None,
        },
        "messenger_odf" => match comp {
            odf::COMP_OBSERVABLE => Some("messenger_odf_observable_hz"),
            _ => None,
        },
        "mars_express_odf" => match comp {
            odf::COMP_OBSERVABLE => Some("mars_express_odf_observable_hz"),
            _ => None,
        },
        "rosetta_odf" => match comp {
            ifms_agc::COMP_CARRIER_LEVEL => Some("rosetta_odf_carrier_level_dbm"),
            ifms_agc::COMP_POLAR_ANGLE => Some("rosetta_odf_polar_angle_cycles"),
            _ => None,
        },
        "viking_text" => match comp {
            viking_text::COMP_RANGE_KM => Some("viking_lander_range_km"),
            viking_text::COMP_RANGE_RATE_KM_S => Some("viking_lander_range_rate_km_s"),
            _ => None,
        },
        "voyager2_merged" => match comp {
            voyager_merged::COMP_B_NT => Some("voyager2_b_nt"),
            voyager_merged::COMP_SPEED_KM_S => Some("voyager2_speed_km_s"),
            voyager_merged::COMP_DENSITY_N_CC => Some("voyager2_density_n_cc"),
            voyager_merged::COMP_TEMP_K => Some("voyager2_temp_k"),
            _ => None,
        },
        "voyager1_merged" => match comp {
            voyager_merged::COMP_B_NT => Some("voyager1_b_nt"),
            voyager_merged::COMP_SPEED_KM_S => Some("voyager1_speed_km_s"),
            voyager_merged::COMP_DENSITY_N_CC => Some("voyager1_density_n_cc"),
            voyager_merged::COMP_TEMP_K => Some("voyager1_temp_k"),
            _ => None,
        },
        "vex_odf" => match comp {
            odf::COMP_OBSERVABLE => Some("vex_odf_observable_hz"),
            _ => None,
        },
        "galileo_odf" => match comp {
            odf::COMP_OBSERVABLE => Some("galileo_odf_observable_hz"),
            _ => None,
        },
        "dawn_odf" => match comp {
            odf::COMP_OBSERVABLE => Some("dawn_odf_observable_hz"),
            _ => None,
        },
        "cassini_odf" => match comp {
            odf::COMP_OBSERVABLE => Some("cassini_odf_observable_hz"),
            _ => None,
        },
        "pioneer10_odf" => match comp {
            odf::COMP_OBSERVABLE => Some("pioneer10_odf_observable_hz"),
            _ => None,
        },
        "pioneer11_odf" => match comp {
            odf::COMP_OBSERVABLE => Some("pioneer11_odf_observable_hz"),
            _ => None,
        },
        "pathfinder_odf" => match comp {
            odf::COMP_OBSERVABLE => Some("pathfinder_odf_observable_hz"),
            _ => None,
        },
        "odf_serie" => match comp {
            odf::COMP_OBSERVABLE => Some("odf_serie_observable_hz"),
            _ => None,
        },
        "cassini_tnf" => match comp {
            odf::TNF_COMP_UL_PHASE => Some("cassini_tnf_ul_phase_cycles"),
            _ => None,
        },
        "maven_tnf" => match comp {
            odf::TNF_COMP_UL_PHASE => Some("maven_tnf_ul_phase_cycles"),
            _ => None,
        },
        "dart_tnf" => match comp {
            odf::TNF_COMP_UL_PHASE => Some("dart_tnf_ul_phase_cycles"),
            _ => None,
        },
        "messenger_tnf" => match comp {
            odf::TNF_COMP_UL_PHASE => Some("messenger_tnf_ul_phase_cycles"),
            _ => None,
        },
        "eht_uvfits" => match comp {
            uvfits::COMP_EHT_AA => Some("eht_uvfits_alma_aa_phase"),
            uvfits::COMP_EHT_AP => Some("eht_uvfits_apex_ap_phase"),
            _ => None,
        },
        "ams02_spec" => tdat::ams02_component_name(comp),
        "voyager_odr" => match comp {
            voyager_odr::COMP_SAMPLE => Some("voyager_odr_sample_count"),
            _ => None,
        },
        "pds3_ring_occ" => match comp {
            pds3_ring_occ::COMP_SIGNAL_RE => Some("pds3_ring_occ_signal_re"),
            pds3_ring_occ::COMP_SIGNAL_IM => Some("pds3_ring_occ_signal_im"),
            _ => None,
        },
        "galileo_odr" => match comp {
            galileo_odr::COMP_AD1 => Some("galileo_odr_ad1_count"),
            galileo_odr::COMP_AD2 => Some("galileo_odr_ad2_count"),
            galileo_odr::COMP_AD3 => Some("galileo_odr_ad3_count"),
            galileo_odr::COMP_AD4 => Some("galileo_odr_ad4_count"),
            _ => None,
        },
        "cassini_rsr" => match comp {
            cassini_rsr::COMP_I => Some("cassini_rsr_i_count"),
            cassini_rsr::COMP_Q => Some("cassini_rsr_q_count"),
            _ => None,
        },
        "flac" => match comp {
            flac::COMP_PCM => Some("nrs_hydrophone_pcm"),
            _ => None,
        },
        "bidsleep" => match comp {
            bidsleep::COMP_MX => Some("bidsleep_mx_ms2"),
            bidsleep::COMP_MY => Some("bidsleep_my_ms2"),
            bidsleep::COMP_MZ => Some("bidsleep_mz_ms2"),
            _ => None,
        },
        "bison_velocity" => match comp {
            0 => Some("bison_pmode_velocity_m_s"),
            _ => None,
        },
        "las" => match comp {
            crate::geo::COMP_LAS_X => Some("las_x_icrs_m"),
            crate::geo::COMP_LAS_Y => Some("las_y_icrs_m"),
            crate::geo::COMP_LAS_Z => Some("las_z_icrs_m"),
            crate::geo::COMP_LAS_INTENSITY => Some("las_intensity_dn"),
            crate::geo::COMP_LAS_CLASSIFICATION => Some("las_classification"),
            _ => None,
        },
        "kasi" => kasi::component_name(comp),
        "wqp_result" => wqp_result::component_name(comp),
        "fink" => fink::component_name(comp),
        "eea_noise" => eea_noise::component_name(comp),
        "gaia_rrl" => gaia_rrl::component_name(comp),
        "viking_grav" => viking_grav::component_name(comp),
        "agrav" => agrav::component_name(comp),
        _ => None,
    }
}

pub fn geo_series_parse_bin(format: &str, bytes: &[u8]) -> Option<Vec<crate::geo::GeoRec>> {
    let magic = crate::geo::magic_of(format)?;
    crate::geo::parse_bin(magic, bytes)
}

pub fn geo_series_component_name(format: &str, comp: u32) -> Option<&'static str> {
    match format {
        "gdp_drifter" => gdp_drifter::component_name(comp),
        "hfrnet_rtv" => hfrnet_rtv::component_name(comp),
        "emodnet_hfr" => emodnet_hfr::component_name(comp),
        "decaps_dr2_stars" => crate::decaps::component_name(comp),
        "toar_surface_o3" => match comp {
            crate::geo::COMP_TOAR_O3 => Some("toar_surface_o3_ppb"),
            _ => None,
        },
        "bgr_infrasound" => match comp {
            crate::geo::COMP_BGR_AZIM => Some("bgr_infrasound_back_azimuth_deg"),
            crate::geo::COMP_BGR_VAPP => Some("bgr_infrasound_apparent_velocity_ms"),
            crate::geo::COMP_BGR_RMS => Some("bgr_infrasound_rms_amplitude_pa"),
            crate::geo::COMP_BGR_FREQ => Some("bgr_infrasound_center_frequency_hz"),
            _ => None,
        },
        "noaa_nrs_psd" => match comp {
            crate::geo::COMP_NRS_PSD => Some("noaa_nrs_psd_db"),
            _ => None,
        },
        "onc_hydrophone_psd" => match comp {
            crate::geo::COMP_ONC_PSD => Some("onc_hydrophone_psd_db"),
            _ => None,
        },
        "superdarn_fitacf" => match comp {
            crate::geo::COMP_SDARN_V => Some("superdarn_fitacf_los_velocity_ms"),
            _ => None,
        },
        "superdarn_rawacf" => match comp {
            crate::geo::COMP_SDARN_POWER => Some("superdarn_rawacf_lag0_power_db"),
            _ => None,
        },
        "fdsn_waveform" => match comp {
            crate::geo::COMP_FDSN_BHZ => Some("fdsn_waveform_bhz_ms"),
            _ => None,
        },
        "ceein_infrasound" => match comp {
            crate::geo::COMP_CEIN_BDF => Some("ceein_infrasound_pressure_pa"),
            _ => None,
        },
        "fmi_gic" => match comp {
            crate::geo::COMP_GIC_A => Some("fmi_gic_a"),
            _ => None,
        },
        "fmi_gic_1min" => match comp {
            crate::geo::COMP_GIC_1MIN_A => Some("fmi_gic_1min_a"),
            _ => None,
        },
        "fmi_image_mag" => match comp {
            crate::geo::COMP_IMAGE_DXDT => Some("fmi_image_mag_dxdt"),
            _ => None,
        },
        "iers_eop_c04_lod" => match comp {
            crate::geo::COMP_IERS_LOD => Some("iers_eop_lod_s"),
            _ => None,
        },
        "black_marble_vnp46a3_nightlight" => match comp {
            crate::geo::COMP_VNP46A3_NIGHTLIGHT => Some("vnp46a3_nightlight_nw_cm2_sr"),
            _ => None,
        },
        "ghsl_built_s" => match comp {
            crate::geo::COMP_GHSL_BUILT => Some("ghsl_built_surface_m2"),
            _ => None,
        },
        "usda_fara_low_access" => match comp {
            crate::geo::COMP_USDA_FARA_SHARE => Some("usda_fara_low_access_share"),
            _ => None,
        },
        "epa_aqs_pm25" => match comp {
            crate::geo::COMP_EPA_AQS_PM25 => Some("epa_aqs_pm25_ug_m3"),
            _ => None,
        },
        "igets" => match comp {
            crate::geo::COMP_IGETS_G => Some("igets_gravity_nm_s2"),
            _ => None,
        },
        "hinet" => match comp {
            crate::geo::COMP_HINET_U => Some("hinet_velocity_u_ms"),
            crate::geo::COMP_HINET_E => Some("hinet_velocity_e_ms"),
            crate::geo::COMP_HINET_N => Some("hinet_velocity_n_ms"),
            _ => None,
        },
        "iss_lis" => match comp {
            crate::geo::COMP_ISSLIS_FLASH_RAD => Some("iss_lis_flash_radiance_uj_sr_m2_um"),
            _ => None,
        },
        "lis_otd" => match comp {
            crate::geo::COMP_LISOTD_FLASH_RAD => Some("lis_otd_flash_radiance_uj_sr"),
            _ => None,
        },
        "trmm_lis" => match comp {
            crate::geo::COMP_TRMMLIS_FLASH_RAD => Some("trmm_lis_flash_radiance_uj_sr_m2_um"),
            _ => None,
        },
        "glm_l1b" => match comp {
            crate::geo::COMP_GLML1B_FLASH_ENERGY => Some("glm_l1b_flash_radiant_energy_j"),
            _ => None,
        },
        "glm_l2" => match comp {
            crate::geo::COMP_GLML2_FLASH_ENERGY => Some("glm_l2_flash_radiant_energy_j"),
            _ => None,
        },
        "gosat_tanso3" => match comp {
            crate::geo::COMP_GOSAT_FOCUS_BAND1 => {
                Some("gosat_tanso3_radiance_focus_band1_w_m2_sr_um")
            }
            crate::geo::COMP_GOSAT_FOCUS_BAND2 => {
                Some("gosat_tanso3_radiance_focus_band2_w_m2_sr_um")
            }
            crate::geo::COMP_GOSAT_FOCUS_BAND3 => {
                Some("gosat_tanso3_radiance_focus_band3_w_m2_sr_um")
            }
            crate::geo::COMP_GOSAT_WIDE_BAND1 => {
                Some("gosat_tanso3_radiance_wide_band1_w_m2_sr_um")
            }
            crate::geo::COMP_GOSAT_WIDE_BAND2 => {
                Some("gosat_tanso3_radiance_wide_band2_w_m2_sr_um")
            }
            crate::geo::COMP_GOSAT_WIDE_BAND3 => {
                Some("gosat_tanso3_radiance_wide_band3_w_m2_sr_um")
            }
            _ => None,
        },
        "argo_bgc" => match comp {
            crate::geo::COMP_ARGO_DOXY => Some("argo_dac_bgc_doxy_umol_kg"),
            crate::geo::COMP_ARGO_NITRATE => Some("argo_dac_bgc_nitrate_umol_kg"),
            crate::geo::COMP_ARGO_CHLA => Some("argo_dac_bgc_chla_mg_m3"),
            crate::geo::COMP_ARGO_BBP700 => Some("argo_dac_bgc_bbp700_m1"),
            crate::geo::COMP_ARGO_PH_TOTAL => Some("argo_dac_bgc_ph_total"),
            _ => None,
        },
        "noaa_wod" => match comp {
            crate::geo::COMP_WOD_TEMP => Some("wod_temperature_degc"),
            crate::geo::COMP_WOD_PSAL => Some("wod_salinity_psu"),
            crate::geo::COMP_WOD_DOXY => Some("wod_oxygen_ml_l"),
            _ => None,
        },
        "supermag_1m" => match comp {
            crate::geo::COMP_SMG_N_NEZ => Some("supermag_n_nez_nt"),
            crate::geo::COMP_SMG_E_NEZ => Some("supermag_e_nez_nt"),
            crate::geo::COMP_SMG_Z_NEZ => Some("supermag_z_nez_nt"),
            crate::geo::COMP_SMG_N_GEO => Some("supermag_n_geo_nt"),
            crate::geo::COMP_SMG_E_GEO => Some("supermag_e_geo_nt"),
            crate::geo::COMP_SMG_Z_GEO => Some("supermag_z_geo_nt"),
            _ => None,
        },
        "iaga_text" => match comp {
            crate::geo::COMP_IAGA_X => Some("iaga_x_nt"),
            crate::geo::COMP_IAGA_Y => Some("iaga_y_nt"),
            crate::geo::COMP_IAGA_Z => Some("iaga_z_nt"),
            crate::geo::COMP_IAGA_F => Some("iaga_f_nt"),
            crate::geo::COMP_IAGA_H => Some("iaga_h_nt"),
            crate::geo::COMP_IAGA_D => Some("iaga_d_arcmin"),
            _ => None,
        },
        "noaa_ghcn_d" => match comp {
            crate::geo::COMP_GHCN_TMAX => Some("noaa_ghcn_d_tmax_c"),
            crate::geo::COMP_GHCN_TMIN => Some("noaa_ghcn_d_tmin_c"),
            crate::geo::COMP_GHCN_PRCP => Some("noaa_ghcn_d_prcp_mm"),
            crate::geo::COMP_GHCN_SNOW => Some("noaa_ghcn_d_snow_mm"),
            crate::geo::COMP_GHCN_SNWD => Some("noaa_ghcn_d_snwd_mm"),
            _ => None,
        },
        "noaa_gsod" => match comp {
            crate::geo::COMP_GSOD_TEMP => Some("noaa_gsod_temp_c"),
            crate::geo::COMP_GSOD_DEWP => Some("noaa_gsod_dewp_c"),
            crate::geo::COMP_GSOD_SLP => Some("noaa_gsod_slp_hpa"),
            crate::geo::COMP_GSOD_WDSP => Some("noaa_gsod_wdsp_ms"),
            crate::geo::COMP_GSOD_GUST => Some("noaa_gsod_gust_ms"),
            crate::geo::COMP_GSOD_TMAX => Some("noaa_gsod_max_c"),
            crate::geo::COMP_GSOD_TMIN => Some("noaa_gsod_min_c"),
            crate::geo::COMP_GSOD_PRCP => Some("noaa_gsod_prcp_mm"),
            _ => None,
        },
        "noaa_isd" => match comp {
            crate::geo::COMP_ISD_TEMP => Some("noaa_isd_temp_c"),
            crate::geo::COMP_ISD_DEWP => Some("noaa_isd_dewp_c"),
            crate::geo::COMP_ISD_WDIR => Some("noaa_isd_wdir_deg"),
            crate::geo::COMP_ISD_WSPD => Some("noaa_isd_wspd_ms"),
            crate::geo::COMP_ISD_SLP => Some("noaa_isd_slp_hpa"),
            _ => None,
        },
        "us_crn_hourly" => match comp {
            crate::geo::COMP_USCRN_TEMP => Some("us_crn_hourly_temp_c"),
            _ => None,
        },
        "opensensemap_temperatur" => match comp {
            crate::geo::COMP_OSM_TEMP => Some("opensensemap_temperature_c"),
            _ => None,
        },
        "copernicus_cdm_obs" => crate::copernicus::component_name(comp),
        "cosmic_ro" => match comp {
            crate::geo::COMP_COSMIC_REFRACT => Some("cosmic_ro_refractivity_n_units"),
            crate::geo::COMP_COSMIC_TEMP => Some("cosmic_ro_temperature_k"),
            crate::geo::COMP_COSMIC_PRES => Some("cosmic_ro_pressure_hpa"),
            _ => None,
        },
        "champ_plpt" => match comp {
            crate::geo::COMP_CHAMP_DENS => Some("champ_plpt_electron_density_cm3"),
            _ => None,
        },
        "ogimet_synop" => match comp {
            crate::geo::COMP_OGM_TEMP => Some("ogimet_synop_temp_c"),
            crate::geo::COMP_OGM_DEWP => Some("ogimet_synop_dewp_c"),
            crate::geo::COMP_OGM_WSPD => Some("ogimet_synop_wspd_ms"),
            crate::geo::COMP_OGM_SLP => Some("ogimet_synop_slp_hpa"),
            _ => None,
        },
        "nohrsc_snowfall" => match comp {
            crate::geo::COMP_NOHR_SNOWFALL => Some("nohrsc_snowfall_mm"),
            _ => None,
        },
        "esacci_sst_l4_cdr3" => match comp {
            crate::geo::COMP_ESACCI_SST => Some("esacci_sst_l4_cdr3"),
            _ => None,
        },
        "ersstv5_nino34" => match comp {
            crate::geo::COMP_ERSSTV5 => Some("ersstv5_nino34_ssta"),
            _ => None,
        },
        "ascat_wind" => match comp {
            crate::geo::COMP_ASCAT_WSPD => Some("ascat_wind_speed_m_s"),
            crate::geo::COMP_ASCAT_WDIR => Some("ascat_wind_direction_deg"),
            _ => None,
        },
        "cses_lap" => match comp {
            crate::geo::COMP_CSES_DENS => Some("cses_lap_electron_density_cm3"),
            crate::geo::COMP_CSES_TEMP => Some("cses_lap_electron_temp_k"),
            crate::geo::COMP_CSES_UP => Some("cses_lap_up_v"),
            crate::geo::COMP_CSES_UF => Some("cses_lap_uf_v"),
            _ => None,
        },
        "cses_efd" => match comp {
            crate::geo::COMP_CSES_EFD_ULF_EX => Some("cses_efd_ulf_ex_v_m"),
            crate::geo::COMP_CSES_EFD_ULF_EY => Some("cses_efd_ulf_ey_v_m"),
            crate::geo::COMP_CSES_EFD_ULF_EZ => Some("cses_efd_ulf_ez_v_m"),
            crate::geo::COMP_CSES_EFD_ELF_EX => Some("cses_efd_elf_ex_v_m"),
            crate::geo::COMP_CSES_EFD_ELF_EY => Some("cses_efd_elf_ey_v_m"),
            crate::geo::COMP_CSES_EFD_ELF_EZ => Some("cses_efd_elf_ez_v_m"),
            crate::geo::COMP_CSES_EFD_VLF_EX => Some("cses_efd_vlf_ex_v_m"),
            crate::geo::COMP_CSES_EFD_VLF_EY => Some("cses_efd_vlf_ey_v_m"),
            crate::geo::COMP_CSES_EFD_VLF_EZ => Some("cses_efd_vlf_ez_v_m"),
            _ => None,
        },
        "cses_hpm" => match comp {
            crate::geo::COMP_CSES_HPM_FG2_X => Some("cses_hpm_fg2_x_nt"),
            crate::geo::COMP_CSES_HPM_FG2_Y => Some("cses_hpm_fg2_y_nt"),
            crate::geo::COMP_CSES_HPM_FG2_Z => Some("cses_hpm_fg2_z_nt"),
            _ => None,
        },
        "cses_scm" => match comp {
            crate::geo::COMP_CSES_SCM_ULF_X => Some("cses_scm_ulf_x_nt"),
            crate::geo::COMP_CSES_SCM_ULF_Y => Some("cses_scm_ulf_y_nt"),
            crate::geo::COMP_CSES_SCM_ULF_Z => Some("cses_scm_ulf_z_nt"),
            _ => None,
        },
        _ => None,
    }
}

pub struct IscEvent {
    pub time: f64,
    pub lat: f64,
    pub lon: f64,
    pub depth_km: f64,
    pub magnitude: Option<f64>,
    pub mag_type: Option<String>,
}

pub fn parse_iscb_bin(bytes: &[u8]) -> Option<Vec<IscEvent>> {
    const MAGIC: [u8; 4] = *b"ISCB";
    const VERSION: u8 = 1;
    const HEADER_LEN: usize = 13;
    const REC_BYTES: usize = 56;
    const PRES_MAG: u8 = 0x01;
    const PRES_MAGTYPE: u8 = 0x02;
    const MAG_TYPE_LEN: usize = 8;
    if bytes.len() < HEADER_LEN || bytes[0..4] != MAGIC || bytes[4] != VERSION {
        return None;
    }
    let n = u64::from_le_bytes(bytes[5..13].try_into().ok()?) as usize;
    if bytes.len() != HEADER_LEN + n * REC_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let rec = bytes.get(HEADER_LEN + i * REC_BYTES..HEADER_LEN + (i + 1) * REC_BYTES)?;
        let f64_of = |r: std::ops::Range<usize>| {
            rec.get(r)
                .and_then(|x| x.try_into().ok())
                .map(f64::from_le_bytes)
        };
        let time = f64_of(8..16)?;
        let lat = f64_of(16..24)?;
        let lon = f64_of(24..32)?;
        let depth_km = f64_of(32..40)?;
        if !time.is_finite() || !lat.is_finite() || !lon.is_finite() || !depth_km.is_finite() {
            return None;
        }
        if !(-90.0..=90.0).contains(&lat) || !(-180.0..=180.0).contains(&lon) {
            return None;
        }
        let present = rec[0];
        let magnitude = if present & PRES_MAG != 0 {
            let v = f64_of(40..48)?;
            if v.is_finite() {
                Some(v)
            } else {
                return None;
            }
        } else {
            None
        };
        let mag_type = if present & PRES_MAGTYPE != 0 {
            let field = &rec[48..48 + MAG_TYPE_LEN];
            let end = field.iter().position(|&c| c == 0).unwrap_or(MAG_TYPE_LEN);
            let raw = &field[..end];
            if raw.is_empty() || !raw.iter().all(|c| c.is_ascii_graphic() || *c == b' ') {
                return None;
            }
            Some(String::from_utf8_lossy(raw).into_owned())
        } else {
            None
        };
        out.push(IscEvent {
            time,
            lat,
            lon,
            depth_km,
            magnitude,
            mag_type,
        });
    }
    Some(out)
}

pub struct NexradRadialSample {
    pub t: f64,
    pub az_deg: f64,
    pub el_deg: f64,
    pub range_km: f64,
    pub value: f64,
    pub kind: u32,
}

pub struct NexradLevel2 {
    pub site: Option<nexrad::NexradSite>,
    pub samples: Vec<NexradRadialSample>,
}

pub fn parse_nexrad_level2_bin(bytes: &[u8]) -> Option<NexradLevel2> {
    const REC_BYTES: usize = 44;
    const SITE_BYTES: usize = 1 + 4 + 8 + 8 + 8;
    const HEADER_BYTES: usize = 8 + SITE_BYTES;
    if bytes.len() < HEADER_BYTES || bytes[0..4] != crate::geo::MAGIC_NXR {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != HEADER_BYTES + n * REC_BYTES {
        return None;
    }
    let site = match bytes[8] {
        0 => None,
        1 => {
            let stid = bytes[9..13].try_into().ok()?;
            let lat_deg = f64::from_le_bytes(bytes[13..21].try_into().ok()?);
            let lon_deg = f64::from_le_bytes(bytes[21..29].try_into().ok()?);
            let alt_m = f64::from_le_bytes(bytes[29..37].try_into().ok()?);
            if !lat_deg.is_finite() || !lon_deg.is_finite() || !alt_m.is_finite() {
                return None;
            }
            Some(nexrad::NexradSite {
                stid,
                lat_deg,
                lon_deg,
                alt_m,
            })
        }
        _ => return None,
    };
    let mut out = Vec::with_capacity(n);
    let mut off = HEADER_BYTES;
    for _ in 0..n {
        let s = bytes.get(off..off + REC_BYTES)?;
        let f64_of = |r: std::ops::Range<usize>| {
            s.get(r)
                .and_then(|x| x.try_into().ok())
                .map(f64::from_le_bytes)
        };
        let t = f64_of(0..8)?;
        let az_deg = f64_of(8..16)?;
        let el_deg = f64_of(16..24)?;
        let range_km = f64_of(24..32)?;
        let value = f64_of(32..40)?;
        let kind = u32::from_le_bytes(s.get(40..44)?.try_into().ok()?);
        if !t.is_finite()
            || !az_deg.is_finite()
            || !el_deg.is_finite()
            || !range_km.is_finite()
            || !value.is_finite()
        {
            return None;
        }
        out.push(NexradRadialSample {
            t,
            az_deg,
            el_deg,
            range_km,
            value,
            kind,
        });
        off += REC_BYTES;
    }
    Some(NexradLevel2 { site, samples: out })
}

pub fn nexrad_component_name(kind: u32) -> Option<&'static str> {
    match kind {
        crate::geo::COMP_NXR_REF => Some("nexrad_level2_ref_dbz"),
        crate::geo::COMP_NXR_VEL => Some("nexrad_level2_vel_ms"),
        crate::geo::COMP_NXR_SW => Some("nexrad_level2_sw_ms"),
        _ => None,
    }
}

pub fn gbco_threads(body_name: &str, recs: &[crate::geo::GbcoRec]) -> Vec<StationThread> {
    recs.iter()
        .map(|r| StationThread {
            body_name: body_name.to_string(),
            lat: r.lat,
            lon: r.lon,
            alt: r.elev,
        })
        .collect()
}

pub fn station_view(threads: &[StationThread]) -> String {
    let mut out = String::new();
    for t in threads {
        if t.alt < 0.0 {
            out.push_str(&format!(
                "{} @ {},{}: surface elevation {} m — depth {} m below the datum\n",
                t.body_name, t.lat, t.lon, t.alt, -t.alt
            ));
        } else {
            out.push_str(&format!(
                "{} @ {},{}: surface elevation {} m\n",
                t.body_name, t.lat, t.lon, t.alt
            ));
        }
    }
    out
}

pub fn jlast(json: &JsonVal, key: &str) -> Option<f64> {
    if let Some((target_path, final_key)) = key.rsplit_once('.') {
        let parent = if target_path.is_empty() {
            json
        } else {
            jpath_val(json, target_path)?
        };
        if let JsonVal::Arr(arr) = parent {
            return arr.last().and_then(|v| {
                if let JsonVal::Obj(o) = v {
                    o.get(final_key).and_then(scalar_of)
                } else {
                    scalar_of(v)
                }
            });
        }
        return None;
    }
    match json {
        JsonVal::Arr(arr) => arr.last().and_then(|v| match v {
            JsonVal::Obj(o) => o.get(key).and_then(scalar_of),
            other => scalar_of(other),
        }),
        JsonVal::Obj(map) => map.get(key).and_then(|v| {
            if let JsonVal::Arr(a) = v {
                a.last().and_then(scalar_of)
            } else {
                None
            }
        }),
        _ => None,
    }
}

pub fn jfirst(json: &JsonVal, key: &str) -> Option<f64> {
    if let Some((prefix, final_key)) = key.rsplit_once('.') {
        let parent = if prefix.is_empty() {
            json
        } else {
            jpath_val(json, prefix)?
        };
        if let JsonVal::Arr(arr) = parent {
            return arr.first().and_then(|v| match v {
                JsonVal::Obj(o) => o.get(final_key).and_then(scalar_of),
                other => scalar_of(other),
            });
        }
        return None;
    }
    match json {
        JsonVal::Arr(arr) => arr.first().and_then(|v| match v {
            JsonVal::Obj(o) => o.get(key).and_then(scalar_of),
            other => scalar_of(other),
        }),
        JsonVal::Obj(map) => map.get(key).and_then(|v| {
            if let JsonVal::Arr(a) = v {
                a.first().and_then(scalar_of)
            } else {
                None
            }
        }),
        _ => None,
    }
}

pub fn row_matches(el: &JsonVal, fk: &str, fv: &str) -> bool {
    let JsonVal::Obj(map) = el else {
        return false;
    };
    match map.get(fk) {
        Some(JsonVal::Str(s)) => {
            if fk == "source" {
                s.eq_ignore_ascii_case(fv)
            } else {
                s == fv
            }
        }
        Some(JsonVal::Num(n)) => fv.parse::<f64>() == Ok(*n),
        _ => false,
    }
}

pub fn row_value(el: &JsonVal, key: &str) -> Option<f64> {
    match el {
        JsonVal::Obj(o) => o.get(key).and_then(scalar_of),
        other => scalar_of(other),
    }
}

pub fn jfirst_where(json: &JsonVal, key: &str, filter: Option<&(String, String)>) -> Option<f64> {
    let Some((fk, fv)) = filter else {
        return jfirst(json, key);
    };
    if let Some((prefix, final_key)) = key.rsplit_once('.') {
        let parent = if prefix.is_empty() {
            json
        } else {
            jpath_val(json, prefix)?
        };
        let JsonVal::Arr(arr) = parent else {
            return None;
        };
        return arr
            .iter()
            .find(|v| row_matches(v, fk, fv))
            .and_then(|v| row_value(v, final_key));
    }
    let JsonVal::Arr(arr) = json else {
        return None;
    };
    arr.iter()
        .find(|v| row_matches(v, fk, fv))
        .and_then(|v| row_value(v, key))
}

pub fn jlast_where(json: &JsonVal, key: &str, filter: Option<&(String, String)>) -> Option<f64> {
    let Some((fk, fv)) = filter else {
        return jlast(json, key);
    };
    if let Some((prefix, final_key)) = key.rsplit_once('.') {
        let parent = if prefix.is_empty() {
            json
        } else {
            jpath_val(json, prefix)?
        };
        let JsonVal::Arr(arr) = parent else {
            return None;
        };
        return arr
            .iter()
            .rev()
            .find(|v| row_matches(v, fk, fv))
            .and_then(|v| row_value(v, final_key));
    }
    let JsonVal::Arr(arr) = json else {
        return None;
    };
    arr.iter()
        .rev()
        .find(|v| row_matches(v, fk, fv))
        .and_then(|v| row_value(v, key))
}

pub fn kernel_id_of(name: &str) -> Option<u8> {
    match name {
        "inverse-square" => Some(0),
        "gaussian-inverse-square" => Some(1),
        "erfc" => Some(3),
        "exponential-decay" => Some(4),
        "patch-levy" => Some(5),
        "inverse-linear" => Some(6),
        _ => None,
    }
}

pub const EDF_CHANNEL_CAP: usize = 4096;

fn edf_force_unit(unit: &str) -> Option<(&'static str, &'static str, f64, f64)> {
    match unit {
        "V" => Some(("electric", "V", 1.0, 0.0)),
        "mV" => Some(("electric", "V", 1.0e-3, 0.0)),
        "uV" | "µV" | "μV" => Some(("electric", "V", 1.0e-6, 0.0)),
        "nV" => Some(("electric", "V", 1.0e-9, 0.0)),
        "degC" | "°C" => Some(("thermal", "K", 1.0, 273.15)),
        _ => None,
    }
}

pub fn edf_emit_channels(
    bytes: &[u8],
    position: &Position,
    epoch: f64,
    tau: f64,
    cap: usize,
) -> Vec<(Channel, FieldConfig)> {
    edf_emit_channels_with_electrodes(bytes, &[], None, position, epoch, tau, cap)
}

pub fn edf_emit_channels_with_electrodes(
    bytes: &[u8],
    electrodes: &[crate::archivar::electrodes::Electrode],
    coordsystem: Option<&crate::archivar::electrodes::CoordSystem>,
    position: &Position,
    epoch: f64,
    tau: f64,
    cap: usize,
) -> Vec<(Channel, FieldConfig)> {
    if cap == 0 {
        return Vec::new();
    }
    let Some(header) = crate::archivar::edf::parse_edf(bytes) else {
        return Vec::new();
    };
    let Some(kernel) = kernel_id_of("gaussian-inverse-square") else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for (idx, signal) in header.signals.iter().enumerate() {
        let Some((force_name, unit, scale, offset)) = edf_force_unit(&signal.unit) else {
            eprintln!(
                "edf: signal '{}' unit '{}' maps to no force — skipped",
                signal.label, signal.unit
            );
            continue;
        };
        let Some(force) = force_id_of(force_name) else {
            continue;
        };
        let samples = crate::archivar::edf::signal_samples(bytes, &header, idx);
        if samples.is_empty() {
            continue;
        }
        let name = if signal.label.is_empty() {
            format!("e{idx}")
        } else {
            signal.label.clone()
        };
        let fc = FieldConfig {
            key: name.clone(),
            name: name.clone(),
            kernel,
            force,
            tau,
            absorption: 0.0,
            advection: 0.0,
            unit: unit.to_string(),
            freq: crate::spectral::SPECTRAL_NO_BAND,
            bin_width: crate::spectral::SPECTRAL_NO_BAND,
            fold: None,
            aperture: Aperture::None,
        };
        let channel_position = match coordsystem {
            Some(cs) => match electrodes.iter().find(|e| e.name == signal.label) {
                Some(e) => Position::Electrode {
                    system: cs.system.clone(),
                    units: cs.units.clone(),
                    x: e.x,
                    y: e.y,
                    z: e.z,
                },
                None => position.clone(),
            },
            None => position.clone(),
        };
        let stride = samples.len().div_ceil(cap);
        for k in (0..samples.len()).step_by(stride) {
            let Some(raw) = samples[k] else {
                continue;
            };
            if !raw.is_finite() {
                continue;
            }
            let value = raw * scale + offset;
            if !value.is_finite() {
                continue;
            }
            out.push((
                Channel {
                    name: name.clone(),
                    value,
                    position: channel_position.clone(),
                    epoch,
                    z: 0.0,
                    freq: crate::spectral::SPECTRAL_NO_BAND,
                    bin_width: crate::spectral::SPECTRAL_NO_BAND,
                    station_code: Some(signal.label.clone()),
                },
                fc.clone(),
            ));
        }
    }
    out
}

pub fn extract_fields(ext: &Extract) -> Vec<FieldConfig> {
    match ext {
        Extract::Map { fields, .. }
        | Extract::CelestialMap { fields, .. }
        | Extract::Rows { fields, .. }
        | Extract::Flatten { fields, .. }
        | Extract::CmrPolygon { fields, .. }
        | Extract::CelestialPolygon { fields, .. }
        | Extract::KeplerMap { fields, .. }
        | Extract::ProfileMap { fields, .. }
        | Extract::EpnCore { fields, .. } => fields.clone(),
        Extract::Volume { .. } => Vec::new(),
        Extract::Field(fc)
        | Extract::First(fc, _)
        | Extract::Last(fc, _)
        | Extract::Count(fc)
        | Extract::LastRow(fc)
        | Extract::ObjLast(fc)
        | Extract::Path(fc)
        | Extract::Deep(fc)
        | Extract::Regex(fc) => vec![fc.clone()],
        Extract::GeojsonEvents {
            outputs,
            tau,
            absorption,
            advection,
            ..
        } => {
            if outputs.len() < 2 {
                return Vec::new();
            }
            vec![
                FieldConfig {
                    key: outputs[0].clone(),
                    name: outputs[0].clone(),
                    kernel: 0,
                    force: 3,
                    tau: *tau,
                    absorption: *absorption,
                    advection: *advection,
                    unit: "Mw".to_string(),
                    freq: 0.0,
                    bin_width: 0.0,
                    fold: None,
                    aperture: Aperture::None,
                },
                FieldConfig {
                    key: outputs[1].clone(),
                    name: outputs[1].clone(),
                    kernel: 0,
                    force: 3,
                    tau: *tau,
                    absorption: *absorption,
                    advection: *advection,
                    unit: String::new(),
                    freq: 0.0,
                    bin_width: 0.0,
                    fold: None,
                    aperture: Aperture::None,
                },
            ]
        }
        Extract::QuakeMlEvents {
            outputs,
            tau,
            absorption,
            advection,
        } => {
            if outputs.len() < 2 {
                return Vec::new();
            }
            vec![
                FieldConfig {
                    key: outputs[0].clone(),
                    name: outputs[0].clone(),
                    kernel: 1,
                    force: 3,
                    tau: *tau,
                    absorption: *absorption,
                    advection: *advection,
                    unit: "N m".to_string(),
                    freq: 0.0,
                    bin_width: 0.0,
                    fold: None,
                    aperture: Aperture::None,
                },
                FieldConfig {
                    key: outputs[1].clone(),
                    name: outputs[1].clone(),
                    kernel: 3,
                    force: 4,
                    tau: *tau,
                    absorption: *absorption,
                    advection: *advection,
                    unit: "Mw".to_string(),
                    freq: 0.0,
                    bin_width: 0.0,
                    fold: None,
                    aperture: Aperture::None,
                },
            ]
        }
        _ => Vec::new(),
    }
}

pub fn extract_header(s: &str, n: &str) -> Option<String> {
    for l in s.lines() {
        if let Some(c) = l.find(':')
            && l[..c].trim().eq_ignore_ascii_case(n)
        {
            return Some(l[c + 1..].trim().to_string());
        }
    }
    None
}

pub fn split_csv_line(line: &str) -> Vec<String> {
    if line.contains('\0') {
        split_delimited_line(line, '\0')
    } else {
        split_delimited_line(line, ',')
    }
}

fn split_delimited_line(line: &str, delimiter: char) -> Vec<String> {
    let mut fields = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if in_quotes {
            if c == '"' {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    cur.push('"');
                } else {
                    in_quotes = false;
                }
            } else {
                cur.push(c);
            }
        } else if c == '"' {
            in_quotes = true;
        } else if c == delimiter {
            fields.push(std::mem::take(&mut cur));
        } else {
            cur.push(c);
        }
    }
    fields.push(cur);
    fields
}

pub fn csv_to_json(text: &str) -> Option<JsonVal> {
    let mut lines = text
        .lines()
        .filter(|l| !l.trim().is_empty() && !l.trim().starts_with('#'));
    let header_line = lines.find(|l| l.contains(',') || l.contains('\0'))?;
    let headers = split_csv_line(header_line);
    if headers.len() < 2 {
        return None;
    }
    let mut rows = Vec::new();
    for line in lines {
        if !line.contains(',') && !line.contains('\0') {
            continue;
        }
        let fields = split_csv_line(line);
        if fields.len() != headers.len() {
            continue;
        }
        let mut obj = HashMap::new();
        for (h, f) in headers.iter().zip(fields.iter()) {
            obj.insert(h.clone(), JsonVal::Str(f.clone()));
        }
        rows.push(JsonVal::Obj(obj));
    }
    Some(JsonVal::Arr(rows))
}

fn key_or_constant(key: &str, row: &JsonVal) -> Option<f64> {
    if let Some(v) = jpath(row, key) {
        return Some(v);
    }
    match key.parse::<f64>() {
        Ok(c) if c.is_finite() => Some(c),
        _ => None,
    }
}

fn region_midpoint(
    row: &JsonVal,
    min_key: &str,
    max_key: &str,
    s_region_key: &str,
    axis: usize,
) -> Option<f64> {
    let mn = if min_key.is_empty() {
        None
    } else {
        jpath(row, min_key)
    };
    let mx = if max_key.is_empty() {
        None
    } else {
        jpath(row, max_key)
    };
    if let (Some(a), Some(b)) = (mn, mx)
        && a.is_finite()
        && b.is_finite()
    {
        if axis == 0 {
            return Some((0.5 * (a + unwrap_lon(b, a))).rem_euclid(360.0));
        }
        return Some(0.5 * (a + b));
    }
    if s_region_key.is_empty() || axis > 1 {
        return None;
    }
    let (lon, lat) = stc_s_polygon_centroid(&jstr(row, s_region_key)?)?;
    match axis {
        0 => Some(lon),
        1 => Some(lat),
        _ => None,
    }
}

fn unwrap_lon(b: f64, a: f64) -> f64 {
    let mut w = b;
    while w - a > 180.0 {
        w -= 360.0;
    }
    while w - a < -180.0 {
        w += 360.0;
    }
    w
}

pub fn stc_s_polygon_centroid(s_region: &str) -> Option<(f64, f64)> {
    let body = s_region.strip_prefix("Polygon")?.trim_start();
    let tokens: Vec<&str> = body.split_whitespace().skip(1).collect();
    if tokens.len() < 4 || !tokens.len().is_multiple_of(2) {
        return None;
    }
    let mut lat_sum = 0.0f64;
    let mut lons: Vec<f64> = Vec::with_capacity(tokens.len() / 2);
    for pair in tokens.chunks(2) {
        let lon: f64 = pair[0].parse().ok()?;
        let lat: f64 = pair[1].parse().ok()?;
        if !(lon.is_finite() && lat.is_finite() && (-90.0..=90.0).contains(&lat)) {
            return None;
        }
        lons.push(lon);
        lat_sum += lat;
    }
    let anchor = lons[0];
    let mut lon_sum_normalized = 0.0f64;
    for lon in &lons {
        lon_sum_normalized += unwrap_lon(*lon, anchor);
    }
    let n = lons.len() as f64;
    Some(((lon_sum_normalized / n).rem_euclid(360.0), lat_sum / n))
}

fn tsv_cell(field: &str) -> JsonVal {
    let trimmed = field.trim().trim_matches('"');
    match trimmed.parse::<f64>() {
        Ok(n) if n.is_finite() => JsonVal::Num(n),
        _ => JsonVal::Str(trimmed.to_string()),
    }
}

pub fn tsv_to_json(text: &str) -> Option<JsonVal> {
    let lines: Vec<&str> = text
        .lines()
        .filter(|l| {
            let t = l.trim();
            !t.is_empty() && !t.starts_with('#')
        })
        .collect();
    let header_line = lines.first()?;
    let headers: Vec<String> = header_line.split('\t').map(str::to_string).collect();
    if headers.len() < 2 {
        return None;
    }
    let data_start = match lines.iter().position(|l| {
        let cells: Vec<&str> = l.split('\t').collect();
        !cells.is_empty() && cells.iter().all(|c| c.chars().all(|ch| ch == '-'))
    }) {
        Some(p) => p + 1,
        None => 1,
    };
    let mut rows = Vec::new();
    for line in lines.iter().skip(data_start) {
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() != headers.len() {
            continue;
        }
        let mut obj = HashMap::new();
        for (h, f) in headers.iter().zip(fields.iter()) {
            obj.insert(h.clone(), tsv_cell(f));
        }
        rows.push(JsonVal::Obj(obj));
    }
    Some(JsonVal::Arr(rows))
}

pub fn universal_auto_detect(j: &JsonVal) -> Vec<Extract> {
    let arr = match jpath_val(j, "data").and_then(|v| {
        if let JsonVal::Arr(a) = v {
            Some(a)
        } else {
            None
        }
    }) {
        Some(a) => a,
        None => return vec![],
    };
    let first = match arr.first() {
        Some(JsonVal::Obj(m)) => m,
        _ => return vec![],
    };
    let has_ra = first.contains_key("ra");
    let has_dec = first.contains_key("dec");
    let has_lat = first.contains_key("lat");
    let has_lon = first.contains_key("lon");
    if has_ra && has_dec {
        let plx_key = if first.contains_key("plx") { "plx" } else { "" };
        let pmra_key = if first.contains_key("pmra") {
            "pmra"
        } else {
            ""
        };
        let pmdec_key = if first.contains_key("pmdec") {
            "pmdec"
        } else {
            ""
        };
        let rv_key = if first.contains_key("radvel") {
            "radvel"
        } else {
            ""
        };
        let dist_key = if first.contains_key("dist") {
            "dist"
        } else {
            ""
        };
        let z_key = if first.contains_key("z") { "z" } else { "" };
        let epoch_key = if first.contains_key("t") { "t" } else { "" };
        let mut fields = vec![];
        if first.contains_key("val") {
            fields.push(FieldConfig {
                key: "val".into(),
                name: "val".into(),
                kernel: 0,
                force: 0,
                tau: 0.0,
                absorption: 0.0,
                advection: 0.0,
                unit: String::new(),
                freq: 0.0,
                bin_width: 0.0,
                fold: None,
                aperture: Aperture::None,
            });
        }
        if first.contains_key("extent") {
            fields.push(FieldConfig {
                key: "extent".into(),
                name: "extent".into(),
                kernel: 0,
                force: 0,
                tau: 0.0,
                absorption: 0.0,
                advection: 0.0,
                unit: String::new(),
                freq: 0.0,
                bin_width: 0.0,
                fold: None,
                aperture: Aperture::None,
            });
        }
        if first.contains_key("tau") {
            fields.push(FieldConfig {
                key: "tau".into(),
                name: "tau".into(),
                kernel: 0,
                force: 0,
                tau: 0.0,
                absorption: 0.0,
                advection: 0.0,
                unit: String::new(),
                freq: 0.0,
                bin_width: 0.0,
                fold: None,
                aperture: Aperture::None,
            });
        }
        vec![Extract::CelestialMap {
            arr_path: "data".into(),
            ra_key: "ra".into(),
            dec_key: "dec".into(),
            dist_key: dist_key.into(),
            dist_scale: None,
            plx_key: plx_key.into(),
            z_key: z_key.into(),
            pmra_key: pmra_key.into(),
            pmdec_key: pmdec_key.into(),
            rv_key: rv_key.into(),
            rv_scale: None,
            epoch_key: epoch_key.into(),
            epoch_mjd: false,
            fields,
            tau_key: String::new(),
        }]
    } else if has_lat && has_lon {
        let alt_key = if first.contains_key("alt") { "alt" } else { "" };
        let epoch_key = if first.contains_key("t") { "t" } else { "" };
        let vel_key = if first.contains_key("vel") { "vel" } else { "" };
        let trk_key = if first.contains_key("trk") { "trk" } else { "" };
        let vr_key = if first.contains_key("vr") { "vr" } else { "" };
        let mut fields = vec![];
        if first.contains_key("val") {
            fields.push(FieldConfig {
                key: "val".into(),
                name: "val".into(),
                kernel: 0,
                force: 0,
                tau: 0.0,
                absorption: 0.0,
                advection: 0.0,
                unit: String::new(),
                freq: 0.0,
                bin_width: 0.0,
                fold: None,
                aperture: Aperture::None,
            });
        }
        if first.contains_key("extent") {
            fields.push(FieldConfig {
                key: "extent".into(),
                name: "extent".into(),
                kernel: 0,
                force: 0,
                tau: 0.0,
                absorption: 0.0,
                advection: 0.0,
                unit: String::new(),
                freq: 0.0,
                bin_width: 0.0,
                fold: None,
                aperture: Aperture::None,
            });
        }
        vec![Extract::Map {
            arr_path: "data".into(),
            lat_key: "lat".into(),
            lon_key: "lon".into(),
            alt_key: alt_key.into(),
            epoch_key: epoch_key.into(),
            val_key: String::new(),
            alt_scale: -1.0,
            vel_key: vel_key.into(),
            vel_scale: 1.0,
            trk_key: trk_key.into(),
            vr_key: vr_key.into(),
            fields,
            lat_sign: None,
            lon_sign: None,
            epoch_scale: 1.0,
            tau_key: String::new(),
            mag_type_key: String::new(),
        }]
    } else {
        vec![]
    }
}

pub fn jcount(json: &JsonVal, path: &str) -> Option<f64> {
    if path == "." || path.is_empty() {
        if let JsonVal::Arr(arr) = json {
            return Some(arr.len() as f64);
        }
        return None;
    }
    if path.contains('.') {
        let target = jpath_val(json, path)?;
        if let JsonVal::Arr(arr) = target {
            return Some(arr.len() as f64);
        }
        return None;
    }
    match json {
        JsonVal::Obj(map) => {
            if let Some(JsonVal::Arr(arr)) = map.get(path) {
                Some(arr.len() as f64)
            } else {
                None
            }
        }
        _ => None,
    }
}

pub fn jdeep_find_num(json: &JsonVal, key: &str) -> Option<f64> {
    match json {
        JsonVal::Obj(map) => {
            if let Some(v) = map.get(key)
                && let Some(n) = scalar_of(v)
            {
                return Some(n);
            }
            for v in map.values() {
                if let Some(n) = jdeep_find_num(v, key) {
                    return Some(n);
                }
            }
            None
        }
        JsonVal::Arr(arr) => {
            for v in arr {
                if let Some(n) = jdeep_find_num(v, key) {
                    return Some(n);
                }
            }
            None
        }
        _ => None,
    }
}

pub fn j2d_last_row(json: &JsonVal, col: &str) -> Option<f64> {
    if let JsonVal::Arr(arr) = json {
        if arr.len() < 2 {
            return None;
        }
        if let JsonVal::Arr(headers) = &arr[0] {
            let col_idx = headers.iter().position(|h| {
                if let JsonVal::Str(s) = h {
                    s.eq_ignore_ascii_case(col) || s.starts_with(col)
                } else {
                    false
                }
            })?;
            if let Some(JsonVal::Arr(last_row)) = arr.last() {
                return last_row.get(col_idx).and_then(scalar_of);
            }
        }
    }
    None
}

pub fn text_last_col(data: &str, col: &str) -> Option<f64> {
    if let Ok(idx) = col.parse::<usize>() {
        for line in data.lines().rev() {
            let trimmed = line.trim();
            if trimmed.is_empty()
                || trimmed.starts_with('#')
                || trimmed.chars().next().is_some_and(|c| c.is_alphabetic())
            {
                continue;
            }
            let cols = split_data_line(trimmed);
            if let Some(v) = cols.get(idx)
                && let Ok(f) = v.trim_matches('"').parse::<f64>()
            {
                return Some(f);
            }
        }
        return None;
    }
    let mut header_idx: Option<usize> = None;
    for line in data.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let stripped = (if let Some(s) = trimmed.strip_prefix('#') {
            s
        } else {
            trimmed
        })
        .trim();
        let cols = split_data_line(stripped);
        if header_idx.is_none() {
            if let Some(idx) = cols
                .iter()
                .position(|c| c.eq_ignore_ascii_case(col) || c.starts_with(col))
            {
                header_idx = Some(idx);
                break;
            }
            continue;
        }
    }
    let idx = header_idx?;
    for line in data.lines().rev() {
        let trimmed = line.trim();
        if trimmed.is_empty()
            || trimmed.starts_with('#')
            || trimmed.chars().next().is_some_and(|c| c.is_alphabetic())
        {
            continue;
        }
        let cols = split_data_line(trimmed);
        if let Some(v) = cols.get(idx)
            && let Ok(f) = v.trim_matches('"').parse::<f64>()
        {
            return Some(f);
        }
    }
    None
}

pub fn is_drop_key(key: &str) -> bool {
    let kl = key.to_lowercase();
    kl == "id"
        || kl == "hex"
        || kl == "flight"
        || kl == "callsign"
        || kl == "icao24"
        || kl == "origin_country"
        || kl == "evid"
        || kl == "publicid"
        || kl == "locality"
        || kl == "place"
        || kl == "region"
        || kl == "flynn_region"
        || kl == "satellite"
        || kl == "net"
        || kl == "source"
        || kl == "station"
        || kl == "name"
        || kl == "stid"
        || kl == "icao"
        || kl == "station_name"
        || kl == "country"
        || kl == "sitename"
        || kl == "variablename"
        || kl == "hypocenter"
        || kl == "code"
        || kl == "wmo"
        || kl == "wban"
        || kl == "usaf"
        || kl == "buoy_id"
        || kl == "platform"
        || kl == "sensor"
        || kl == "catalog"
        || is_time_key(key)
        || kl == "timestamp_utc"
        || kl == "observed_date"
        || kl == "generated"
        || kl == "local_date_time"
        || kl == "datetime"
        || kl == "timezone"
        || kl == "origintime"
        || kl == "obstime"
        || kl == "lastupdated"
        || kl == "begintime"
        || kl == "peaktime"
        || kl == "endtime"
        || kl == "announcedtime"
        || kl == "daynum"
        || kl == "type"
        || kl == "status"
        || kl == "alert"
        || kl == "magtype"
        || kl == "evtype"
        || kl == "auth"
        || kl == "iscancel"
        || kl == "isfinal"
        || kl == "domestictsunami"
        || kl == "issea"
        || kl == "istraining"
        || kl == "active"
        || kl == "count"
        || kl == "total"
        || kl == "number_spots"
        || kl == "station_count"
        || kl == "event_count"
        || kl == "multiplicity"
        || kl.starts_with("count_")
        || kl.starts_with("number_")
        || (kl.starts_with("n_") && kl.len() <= 5)
        || kl.ends_with("_index")
        || kl.ends_with("_scale")
        || kl.ends_with("_code")
        || kl.ends_with("_pct")
        || kl == "ssn"
        || kl == "kp_index"
        || kl == "estimated_kp"
        || kl == "kp"
        || kl == "a_running"
        || kl == "uv_index"
        || kl == "weather_code"
        || kl == "cdi"
        || kl == "mmi"
        || kl == "sig"
        || kl == "felt"
        || kl == "tsunami"
        || kl == "confidence"
        || kl == "dmin"
        || kl == "nst"
        || kl == "rms"
        || kl == "gap"
        || kl == "flare_index"
        || kl == "storm_level"
        || kl == "noaa_scale"
        || kl == "class"
        || kl == "classtype"
        || kl == "sample_size"
        || kl.ends_with("_size")
}

pub fn text_to_json(text: &str) -> Option<JsonVal> {
    let header = text.lines().find_map(|line| {
        let t = line.trim();
        if !t.starts_with('#') {
            return None;
        }
        let stripped = t.trim_start_matches('#').trim();
        if stripped.is_empty() {
            return None;
        }
        let cols: Vec<String> = stripped.split_whitespace().map(|s| s.to_string()).collect();
        if cols.len() > 5 { Some(cols) } else { None }
    })?;
    let data = text.lines().find_map(|line| {
        let t = line.trim();
        if t.starts_with('#') {
            return None;
        }
        let cols: Vec<String> = t.split_whitespace().map(|s| s.to_string()).collect();
        if cols.len() >= header.len() {
            Some(cols)
        } else {
            None
        }
    })?;
    let mut obj = HashMap::new();
    for (name, value) in header[5..].iter().zip(data[5..].iter()) {
        let lower = name.to_lowercase();
        if lower == "yy" || lower == "mm" || lower == "dd" || lower == "hh" || lower == "min" {
            continue;
        }
        if is_unit_name(&lower) || is_drop_key(&lower) {
            continue;
        }
        if let Ok(n) = value.parse::<f64>() {
            obj.insert(name.clone(), JsonVal::Num(n));
        }
    }
    if obj.is_empty() {
        None
    } else {
        Some(JsonVal::Obj(obj))
    }
}

fn tap_format(url: &str) -> Option<String> {
    let upper = url.to_ascii_uppercase();
    let pos = upper.find("FORMAT=")? + "FORMAT=".len();
    let rest = &url[pos..];
    let end = rest.find('&').unwrap_or(rest.len());
    let value = rest[..end].trim().to_ascii_lowercase();
    if value.is_empty() { None } else { Some(value) }
}

fn xml_unescape(s: &str) -> String {
    s.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&amp;", "&")
}

fn votable_attr(tag: &str, name: &str) -> Option<String> {
    let key = format!("{name}=");
    let pos = tag.find(&key)? + key.len();
    let rest = &tag[pos..];
    let quote = rest.chars().next()?;
    if quote != '"' && quote != '\'' {
        return None;
    }
    let end = rest[1..].find(quote)? + 1;
    Some(rest[1..end].to_string())
}

fn votable_cell(content: &str) -> JsonVal {
    let text = content.trim();
    if text.is_empty() {
        return JsonVal::Null;
    }
    match text.parse::<f64>() {
        Ok(v) if v.is_finite() => JsonVal::Num(v),
        _ => JsonVal::Str(xml_unescape(text)),
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum VotKind {
    Int,
    UInt,
    Float,
    Char,
    Unicode,
    Bool,
    Bit,
    Complex,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum VotShape {
    Scalar,
    Fixed(usize),
    Variable,
}

struct VotField {
    name: String,
    kind: VotKind,
    elem_bytes: usize,
    shape: VotShape,
}

struct VotCursor<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> VotCursor<'a> {
    fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        if self.pos + n > self.data.len() {
            return None;
        }
        let s = &self.data[self.pos..self.pos + n];
        self.pos += n;
        Some(s)
    }

    fn u32be(&mut self) -> Option<u32> {
        let b = self.take(4)?;
        Some(u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
    }
}

fn b64_val(b: u8) -> Option<u32> {
    match b {
        b'A'..=b'Z' => Some((b - b'A') as u32),
        b'a'..=b'z' => Some((b - b'a' + 26) as u32),
        b'0'..=b'9' => Some((b - b'0' + 52) as u32),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    }
}

fn base64_decode(input: &str) -> Option<Vec<u8>> {
    let mut sextets: Vec<u32> = Vec::with_capacity(input.len());
    let mut pad = 0usize;
    for b in input.bytes() {
        if b.is_ascii_whitespace() {
            continue;
        }
        if b == b'=' {
            pad += 1;
            if pad > 2 {
                return None;
            }
            continue;
        }
        if pad > 0 {
            return None;
        }
        sextets.push(b64_val(b)?);
    }
    let n = sextets.len();
    let rem = n % 4;
    let expect_pad = match rem {
        0 => 0,
        2 => 2,
        3 => 1,
        _ => return None,
    };
    if pad != expect_pad {
        return None;
    }
    let mut out = Vec::with_capacity(n / 4 * 3 + 3);
    let mut i = 0usize;
    while i + 4 <= n {
        let v0 = sextets[i];
        let v1 = sextets[i + 1];
        let v2 = sextets[i + 2];
        let v3 = sextets[i + 3];
        out.push(((v0 << 2) | (v1 >> 4)) as u8);
        out.push(((v1 << 4) | (v2 >> 2)) as u8);
        out.push(((v2 << 6) | v3) as u8);
        i += 4;
    }
    if rem == 2 {
        out.push(((sextets[i] << 2) | (sextets[i + 1] >> 4)) as u8);
    } else if rem == 3 {
        out.push(((sextets[i] << 2) | (sextets[i + 1] >> 4)) as u8);
        out.push(((sextets[i + 1] << 4) | (sextets[i + 2] >> 2)) as u8);
    }
    Some(out)
}

fn votable_field_plan(field_part: &str) -> Option<Vec<VotField>> {
    let mut fields = Vec::new();
    for chunk in field_part.split("<FIELD").skip(1) {
        let first = chunk.as_bytes()[0];
        if matches!(first, b'r' | b'R' | b's' | b'S') {
            continue;
        }
        let tag = &chunk[..chunk.find('>')?];
        let name = votable_attr(tag, "name").or_else(|| votable_attr(tag, "ID"))?;
        let datatype = votable_attr(tag, "datatype")?;
        let datatype = datatype.to_ascii_lowercase();
        let (kind, elem_bytes) = match datatype.as_str() {
            "boolean" | "logical" => (VotKind::Bool, 1),
            "bit" => (VotKind::Bit, 0),
            "unsignedbyte" => (VotKind::UInt, 1),
            "short" => (VotKind::Int, 2),
            "int" => (VotKind::Int, 4),
            "long" => (VotKind::Int, 8),
            "unsignedshort" => (VotKind::UInt, 2),
            "unsignedint" => (VotKind::UInt, 4),
            "char" => (VotKind::Char, 1),
            "unicodechar" => (VotKind::Unicode, 2),
            "float" => (VotKind::Float, 4),
            "double" => (VotKind::Float, 8),
            "floatcomplex" => (VotKind::Complex, 8),
            "doublecomplex" => (VotKind::Complex, 16),
            _ => return None,
        };
        let shape = match votable_attr(tag, "arraysize").as_deref() {
            None | Some("") => VotShape::Scalar,
            Some(a) if a.ends_with('*') => VotShape::Variable,
            Some(a) => {
                let mut total = 1usize;
                for dim in a.split('x') {
                    match dim.trim().parse::<usize>() {
                        Ok(n) if n > 0 => total = total.saturating_mul(n),
                        _ => return None,
                    }
                }
                if total == 1 {
                    VotShape::Scalar
                } else {
                    VotShape::Fixed(total)
                }
            }
        };
        fields.push(VotField {
            name: xml_unescape(&name),
            kind,
            elem_bytes,
            shape,
        });
    }
    if fields.is_empty() {
        None
    } else {
        Some(fields)
    }
}

fn votable_cell_bytes(cur: &mut VotCursor, f: &VotField) -> Option<(Vec<u8>, usize)> {
    if f.kind == VotKind::Bit {
        let nbits = match f.shape {
            VotShape::Scalar => 1,
            VotShape::Fixed(n) => n,
            VotShape::Variable => cur.u32be()? as usize,
        };
        return Some((cur.take(nbits.div_ceil(8))?.to_vec(), nbits));
    }
    let count = match f.shape {
        VotShape::Scalar => 1,
        VotShape::Fixed(n) => n,
        VotShape::Variable => cur.u32be()? as usize,
    };
    let nbytes = f.elem_bytes.saturating_mul(count);
    Some((cur.take(nbytes)?.to_vec(), count))
}

fn int_be(bytes: &[u8], n: usize) -> i64 {
    match n {
        1 => bytes[0] as i8 as i64,
        2 => i16::from_be_bytes([bytes[0], bytes[1]]) as i64,
        4 => i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as i64,
        _ => i64::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]),
    }
}

fn uint_be(bytes: &[u8], n: usize) -> u64 {
    match n {
        1 => bytes[0] as u64,
        2 => u16::from_be_bytes([bytes[0], bytes[1]]) as u64,
        4 => u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as u64,
        _ => u64::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]),
    }
}

fn char_string(bytes: &[u8]) -> String {
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..end]).trim().to_string()
}

fn votable_value(f: &VotField, bytes: &[u8], count: usize) -> Option<JsonVal> {
    let scalar = matches!(f.shape, VotShape::Scalar);
    match f.kind {
        VotKind::Char => Some(JsonVal::Str(char_string(bytes))),
        VotKind::Unicode => {
            let units: Vec<u16> = bytes
                .as_chunks::<2>()
                .0
                .iter()
                .map(|c| u16::from_be_bytes([c[0], c[1]]))
                .collect();
            let text = String::from_utf16_lossy(&units);
            let end = text.find('\0').unwrap_or(text.len());
            Some(JsonVal::Str(text[..end].trim().to_string()))
        }
        VotKind::Bool => {
            let vals: Vec<JsonVal> = bytes
                .iter()
                .map(|b| match b {
                    b'T' | b't' | 1 => JsonVal::Bool(true),
                    b'F' | b'f' | 0 => JsonVal::Bool(false),
                    _ => JsonVal::Null,
                })
                .collect();
            if scalar {
                match vals.first() {
                    Some(v @ JsonVal::Bool(_)) => Some(v.clone()),
                    _ => None,
                }
            } else {
                Some(JsonVal::Arr(vals))
            }
        }
        VotKind::Bit => {
            let mut vals = Vec::with_capacity(count);
            for i in 0..count {
                let bit = (bytes[i / 8] >> (7 - i % 8)) & 1 == 1;
                vals.push(JsonVal::Bool(bit));
            }
            if scalar {
                vals.pop()
            } else {
                Some(JsonVal::Arr(vals))
            }
        }
        VotKind::Int => {
            let mut vals = Vec::with_capacity(count);
            for chunk in bytes.chunks(f.elem_bytes).take(count) {
                vals.push(JsonVal::Num(int_be(chunk, f.elem_bytes) as f64));
            }
            if scalar {
                vals.pop()
            } else {
                Some(JsonVal::Arr(vals))
            }
        }
        VotKind::UInt => {
            let mut vals = Vec::with_capacity(count);
            for chunk in bytes.chunks(f.elem_bytes).take(count) {
                vals.push(JsonVal::Num(uint_be(chunk, f.elem_bytes) as f64));
            }
            if scalar {
                vals.pop()
            } else {
                Some(JsonVal::Arr(vals))
            }
        }
        VotKind::Float => {
            let mut vals = Vec::with_capacity(count);
            for chunk in bytes.chunks(f.elem_bytes).take(count) {
                let v = if f.elem_bytes == 4 {
                    f32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]) as f64
                } else {
                    f64::from_be_bytes([
                        chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6],
                        chunk[7],
                    ])
                };
                if v.is_finite() {
                    vals.push(JsonVal::Num(v));
                } else if scalar {
                    return None;
                } else {
                    vals.push(JsonVal::Null);
                }
            }
            if scalar {
                vals.pop()
            } else {
                Some(JsonVal::Arr(vals))
            }
        }
        VotKind::Complex => {
            let half = f.elem_bytes / 2;
            let mut vals = Vec::with_capacity(count);
            for chunk in bytes.chunks(f.elem_bytes).take(count) {
                let (r, i) = (&chunk[..half], &chunk[half..]);
                let (rv, iv) = if half == 4 {
                    (
                        f32::from_be_bytes([r[0], r[1], r[2], r[3]]) as f64,
                        f32::from_be_bytes([i[0], i[1], i[2], i[3]]) as f64,
                    )
                } else {
                    (
                        f64::from_be_bytes([r[0], r[1], r[2], r[3], r[4], r[5], r[6], r[7]]),
                        f64::from_be_bytes([i[0], i[1], i[2], i[3], i[4], i[5], i[6], i[7]]),
                    )
                };
                let re = if rv.is_finite() {
                    JsonVal::Num(rv)
                } else {
                    JsonVal::Null
                };
                let im = if iv.is_finite() {
                    JsonVal::Num(iv)
                } else {
                    JsonVal::Null
                };
                vals.push(JsonVal::Arr(vec![re, im]));
            }
            if scalar {
                vals.pop()
            } else {
                Some(JsonVal::Arr(vals))
            }
        }
    }
}

fn votable_binary_rows(data: &[u8], fields: &[VotField], binary2: bool) -> Option<JsonVal> {
    let nflag = if binary2 { fields.len().div_ceil(8) } else { 0 };
    let mut rows: Vec<JsonVal> = Vec::new();
    let mut cur = VotCursor { data, pos: 0 };
    while cur.pos < cur.data.len() {
        let mut flags: Vec<u8> = Vec::with_capacity(nflag);
        if nflag > 0 {
            let Some(fb) = cur.take(nflag) else { break };
            flags.extend_from_slice(fb);
        }
        let mut map = HashMap::new();
        let mut broken = false;
        for (i, f) in fields.iter().enumerate() {
            let null = binary2 && (flags[i / 8] >> (7 - i % 8)) & 1 == 1;
            let Some((bytes, count)) = votable_cell_bytes(&mut cur, f) else {
                broken = true;
                break;
            };
            if null {
                continue;
            }
            if let Some(v) = votable_value(f, &bytes, count) {
                map.insert(f.name.clone(), v);
            }
        }
        if broken {
            break;
        }
        rows.push(JsonVal::Obj(map));
    }
    if rows.is_empty() {
        None
    } else {
        Some(JsonVal::Arr(rows))
    }
}

fn votable_binary_from_parts(data_part: &str, field_part: &str) -> Option<JsonVal> {
    let (start, end_marker, binary2) = if let Some(p) = data_part.find("<BINARY2") {
        (p, "</BINARY2>", true)
    } else {
        let p = data_part.find("<BINARY")?;
        (p, "</BINARY>", false)
    };
    let region_end = data_part[start..]
        .find(end_marker)
        .map(|p| start + p)
        .unwrap_or(data_part.len());
    let stream_pos = data_part[start..region_end].find("<STREAM")? + start;
    let after = &data_part[stream_pos..];
    let tag_end = after.find('>')?;
    let tag = &after[..tag_end];
    if tag.ends_with('/') {
        return None;
    }
    let encoding = votable_attr(tag, "encoding").map(|e| e.to_ascii_lowercase());
    if encoding.as_deref() != Some("base64") {
        return None;
    }
    let content_start = stream_pos + tag_end + 1;
    let content_end = data_part[content_start..region_end]
        .find("</STREAM>")
        .map(|p| content_start + p)
        .unwrap_or(region_end);
    let bytes = base64_decode(&data_part[content_start..content_end])?;
    if bytes.is_empty() {
        return None;
    }
    let fields = votable_field_plan(field_part)?;
    votable_binary_rows(&bytes, &fields, binary2)
}

pub fn votable_to_json(body: &str) -> Option<JsonVal> {
    if !body.contains("<VOTABLE") {
        return None;
    }
    let data_pos = body.find("<DATA")?;
    let (field_part, data_part) = body.split_at(data_pos);
    let mut names: Vec<String> = Vec::new();
    for chunk in field_part.split("<FIELD").skip(1) {
        let tag = match chunk.find('>') {
            Some(p) => &chunk[..p],
            None => continue,
        };
        if let Some(n) = votable_attr(tag, "name").or_else(|| votable_attr(tag, "ID")) {
            names.push(xml_unescape(&n));
        }
    }
    if names.is_empty() {
        return None;
    }
    let Some(tab_start) = data_part.find("<TABLEDATA") else {
        return votable_binary_from_parts(data_part, field_part);
    };
    let table = &data_part[tab_start..];
    let mut rows: Vec<JsonVal> = Vec::new();
    for tr in table.split("<TR").skip(1) {
        let row_body = match tr.find('>') {
            Some(p) => &tr[p + 1..],
            None => continue,
        };
        let row_body = match row_body.find("</TR>") {
            Some(p) => &row_body[..p],
            None => row_body,
        };
        let mut obj = HashMap::new();
        for (i, cell) in row_body.split("<TD").skip(1).enumerate() {
            if i >= names.len() {
                break;
            }
            let val = if cell.starts_with('/') {
                JsonVal::Null
            } else {
                match cell.find('>') {
                    Some(p) => {
                        let after = &cell[p + 1..];
                        let content = match after.find("</TD>") {
                            Some(e) => &after[..e],
                            None => after,
                        };
                        votable_cell(content)
                    }
                    None => JsonVal::Null,
                }
            };
            obj.insert(names[i].clone(), val);
        }
        if !obj.is_empty() {
            rows.push(JsonVal::Obj(obj));
        }
    }
    if rows.is_empty() {
        None
    } else {
        Some(JsonVal::Arr(rows))
    }
}

pub fn tap_to_json(val: &JsonVal) -> Option<JsonVal> {
    let obj = match val {
        JsonVal::Obj(m) => m,
        _ => return None,
    };
    let data = match obj.get("data") {
        Some(JsonVal::Arr(a)) => a,
        _ => return None,
    };
    let meta = obj
        .get("metadata")
        .or_else(|| obj.get("columns"))
        .or_else(|| obj.get("parameters"));
    let Some(JsonVal::Arr(cols)) = meta else {
        return None;
    };
    let mut names: Vec<String> = Vec::new();
    for c in cols {
        if let JsonVal::Obj(mo) = c
            && let Some(JsonVal::Str(name)) = mo.get("name")
        {
            names.push(name.clone());
        }
    }
    if names.is_empty() {
        return None;
    }
    let mut rows: Vec<JsonVal> = Vec::new();
    for d in data {
        if let JsonVal::Arr(row) = d {
            let mut row_map = HashMap::new();
            for (name, cell) in names.iter().zip(row.iter()) {
                row_map.insert(name.clone(), cell.clone());
            }
            rows.push(JsonVal::Obj(row_map));
        }
    }
    if rows.is_empty() {
        None
    } else {
        Some(JsonVal::Arr(rows))
    }
}

pub fn tap_body_to_json(url: &str, body: &str) -> Option<JsonVal> {
    match tap_format(url).as_deref() {
        Some(v) if v.starts_with("votable") => votable_to_json(body),
        Some("csv") => csv_to_json(body),
        Some(v) if v.starts_with("json") => parse_json(body).and_then(|j| tap_to_json(&j)),
        _ => {
            let trimmed = body.trim_start_matches('\u{feff}').trim_start();
            if trimmed.starts_with('{') || trimmed.starts_with('[') {
                parse_json(body).and_then(|j| tap_to_json(&j))
            } else if body.contains("<VOTABLE") {
                votable_to_json(body)
            } else {
                None
            }
        }
    }
}

fn find_ci(hay: &str, needle: &str, from: usize) -> Option<usize> {
    let hb = hay.as_bytes();
    let nb = needle.as_bytes();
    if nb.is_empty() || from > hb.len().saturating_sub(nb.len()) {
        return None;
    }
    (from..=hb.len() - nb.len()).find(|&i| {
        hb[i..i + nb.len()]
            .iter()
            .zip(nb)
            .all(|(a, b)| a.to_ascii_lowercase() == *b)
    })
}

fn split_ci<'a>(hay: &'a str, needle: &str) -> Vec<&'a str> {
    let mut out = Vec::new();
    let mut rest = hay;
    while let Some(pos) = find_ci(rest, needle, 0) {
        out.push(&rest[..pos]);
        rest = &rest[pos + needle.len()..];
    }
    out.push(rest);
    out
}

fn html_cell_text(content: &str) -> String {
    let mut text = String::new();
    let mut i = 0usize;
    let bytes = content.as_bytes();
    while i < bytes.len() {
        if bytes[i] == b'<' {
            match content[i..].find('>') {
                Some(end) => {
                    let tag = &content[i + 1..i + end];
                    if tag.trim_end_matches('/').trim().eq_ignore_ascii_case("br") {
                        text.push('\n');
                    }
                    i += end + 1;
                }
                None => {
                    text.push_str(&content[i..]);
                    break;
                }
            }
        } else {
            let start = i;
            while i < bytes.len() && bytes[i] != b'<' {
                i += 1;
            }
            text.push_str(&content[start..i]);
        }
    }
    text.lines()
        .map(|l| l.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn html_header_key(text: &str) -> Option<String> {
    let mut key = String::new();
    let mut underscore = true;
    for c in xml_unescape(text).chars() {
        let mapped = if c.is_ascii_alphanumeric() {
            c.to_ascii_lowercase()
        } else {
            '_'
        };
        if mapped == '_' {
            if !underscore {
                key.push('_');
                underscore = true;
            }
        } else {
            underscore = false;
            key.push(mapped);
        }
    }
    while key.ends_with('_') {
        key.pop();
    }
    if key.is_empty() { None } else { Some(key) }
}

fn html_table_rows(table: &str) -> Option<JsonVal> {
    let mut names: Vec<String> = Vec::new();
    let mut seen: HashMap<String, usize> = HashMap::new();
    for chunk in split_ci(table, "<th").into_iter().skip(1) {
        let th_body = match chunk.find('>') {
            Some(p) => &chunk[p + 1..],
            None => continue,
        };
        let content = match find_ci(th_body, "</th", 0) {
            Some(e) => &th_body[..e],
            None => th_body,
        };
        let Some(base) = html_header_key(&html_cell_text(content)) else {
            continue;
        };
        let n = seen.entry(base.clone()).or_insert(0);
        *n += 1;
        names.push(if *n == 1 {
            base
        } else {
            format!("{base}_{}", *n)
        });
    }
    if names.is_empty() {
        return None;
    }
    let mut rows: Vec<JsonVal> = Vec::new();
    for tr in split_ci(table, "<tr").into_iter().skip(1) {
        let row_body = match tr.find('>') {
            Some(p) => &tr[p + 1..],
            None => continue,
        };
        let row_body = match find_ci(row_body, "</tr", 0) {
            Some(p) => &row_body[..p],
            None => row_body,
        };
        let mut obj = HashMap::new();
        for (i, cell) in split_ci(row_body, "<td").into_iter().skip(1).enumerate() {
            if i >= names.len() {
                break;
            }
            let val = if cell.starts_with('/') {
                JsonVal::Null
            } else {
                match cell.find('>') {
                    Some(p) => {
                        let after = &cell[p + 1..];
                        let content = match find_ci(after, "</td", 0) {
                            Some(e) => &after[..e],
                            None => after,
                        };
                        votable_cell(&html_cell_text(content))
                    }
                    None => JsonVal::Null,
                }
            };
            obj.insert(names[i].clone(), val);
        }
        if !obj.is_empty() {
            rows.push(JsonVal::Obj(obj));
        }
    }
    if rows.is_empty() {
        None
    } else {
        Some(JsonVal::Arr(rows))
    }
}

pub fn html_to_json(body: &str) -> Option<JsonVal> {
    let mut search_from = 0usize;
    while let Some(table_pos) = find_ci(body, "<table", search_from) {
        let table_body = &body[table_pos + "<table".len()..];
        let Some(table_end) = find_ci(table_body, "</table", 0) else {
            break;
        };
        let table = &table_body[..table_end];
        if let Some(rows) = html_table_rows(table) {
            return Some(rows);
        }
        search_from = table_pos + "<table".len() + table_end + "</table".len();
    }
    None
}

pub fn tdb_to_jd(tdb_secs: f64) -> f64 {
    tdb_secs / 86400.0 + J2000_EPOCH
}

pub fn flatten_geojson_coords(val: &[JsonVal]) -> Vec<(f64, f64, Option<f64>)> {
    if let Some(JsonVal::Num(_)) = val.first() {
        if val.len() >= 2
            && let (Some(lon), Some(lat)) = (scalar_of(&val[0]), scalar_of(&val[1]))
        {
            let z = if val.len() >= 3 {
                scalar_of(&val[2])
            } else {
                None
            };
            return vec![(lon, lat, z)];
        }
        return Vec::new();
    }
    let mut result = Vec::new();
    for v in val {
        if let JsonVal::Arr(inner) = v {
            result.extend(flatten_geojson_coords(inner));
        }
    }
    result
}

pub fn split_data_line(line: &str) -> Vec<&str> {
    if line.contains('|') && line.split('|').count() > 2 {
        line.split('|')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect()
    } else if line.contains('\t') && line.split('\t').count() > 2 {
        line.split('\t')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect()
    } else if line.contains(';') {
        line.split(';')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect()
    } else if line.contains(',') && line.split(',').count() > 2 {
        line.split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect()
    } else {
        line.split_whitespace().collect()
    }
}

pub enum ExtractResult {
    Measurements(Vec<(Channel, FieldConfig)>),
    WithEphemeris(Vec<(Channel, FieldConfig)>, Box<BodyEphemeris>),
}

const FITS_GCOUNT_DEFAULT: usize = 1;
const FITS_PCOUNT_DEFAULT: usize = 0;

fn fits_data_bytes(header: &crate::archivar::fits::FitsHeader) -> Option<usize> {
    let bitpix = header.int("BITPIX")?.unsigned_abs() as usize;
    let naxis = header.int("NAXIS")? as usize;
    let mut axes = if naxis == 0 { 0usize } else { 1usize };
    for i in 1..=naxis {
        axes *= header.int(&format!("NAXIS{i}"))? as usize;
    }
    let gcount = match header.int("GCOUNT") {
        Some(v) => v as usize,
        None => FITS_GCOUNT_DEFAULT,
    };
    let pcount = match header.int("PCOUNT") {
        Some(v) => v as usize,
        None => FITS_PCOUNT_DEFAULT,
    };
    Some((bitpix / 8) * axes * gcount + pcount)
}

fn tar_gz_yaml_rows(text: &str) -> Option<Vec<JsonVal>> {
    let (prolog, data) = text.split_once("# End of YAML header")?;
    let mut names: Vec<String> = Vec::new();
    let mut in_variables = false;
    let mut var_indent: Option<usize> = None;
    for line in prolog.lines() {
        let trimmed = line.trim_start();
        if !in_variables {
            if trimmed.starts_with("variables:") {
                in_variables = true;
            }
            continue;
        }
        let Some(rest) = trimmed.strip_prefix("- ") else {
            continue;
        };
        let indent = line.len() - trimmed.len();
        if *var_indent.get_or_insert(indent) != indent {
            continue;
        }
        let name = rest.trim_end().trim_end_matches(':');
        if name.is_empty() || name.contains(char::is_whitespace) {
            continue;
        }
        names.push(name.to_string());
    }
    if names.is_empty() {
        return None;
    }
    let mut rows = Vec::new();
    for line in data.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let cols: Vec<&str> = trimmed.split_whitespace().collect();
        if cols.len() < names.len() {
            continue;
        }
        let mut row = HashMap::new();
        for (name, token) in names.iter().zip(cols.iter()) {
            match token.parse::<f64>() {
                Ok(n) => row.insert(name.clone(), JsonVal::Num(n)),
                Err(_) => row.insert(name.clone(), JsonVal::Str((*token).to_string())),
            };
        }
        rows.push(JsonVal::Obj(row));
    }
    Some(rows)
}

fn tar_gz_yaml_member_key(name: &str) -> String {
    let base = name.rsplit('/').next().unwrap_or(name);
    let stem = base
        .strip_suffix(".txt")
        .or_else(|| base.strip_suffix(".rpt"))
        .unwrap_or(base);
    stem.split('_').next().unwrap_or(stem).to_string()
}

fn tar_gz_yaml_to_json(path: &str, wanted: &[String]) -> Option<JsonVal> {
    let file = std::fs::File::open(path).ok()?;
    let mut out: HashMap<String, JsonVal> = HashMap::new();
    let want = |name: &str| {
        if !name.ends_with(".txt") {
            return false;
        }
        let key = tar_gz_yaml_member_key(name);
        wanted.is_empty() || wanted.iter().any(|w| w == &key)
    };
    let scanned = crate::archivar::inflate::gunzip_tar_members(file, want, |name, data| {
        let key = tar_gz_yaml_member_key(name);
        if out.contains_key(&key) {
            return;
        }
        let text = String::from_utf8_lossy(data);
        let Some(rows) = tar_gz_yaml_rows(&text) else {
            return;
        };
        out.insert(key, JsonVal::Arr(rows));
    });
    if scanned.is_err() {
        return None;
    }
    if out.is_empty() {
        None
    } else {
        Some(JsonVal::Obj(out))
    }
}

fn fits_to_json(buf: &[u8]) -> Option<JsonVal> {
    let mut off = 0usize;
    for _ in 0..8 {
        let (header, data_start) = crate::archivar::fits::FitsHeader::parse(buf, off)?;
        if header.value("XTENSION") == Some("'BINTABLE'") {
            let (table, _next) = crate::archivar::fits::FitsTable::parse(buf, off)?;
            let mut rows = Vec::with_capacity(table.n_rows);
            for r in 0..table.n_rows {
                let mut obj = HashMap::new();
                for col in &table.columns {
                    if col.name.is_empty() {
                        continue;
                    }
                    let val = if col.code == 'A' {
                        table
                            .cell_str(buf, r, col)
                            .map(|s| JsonVal::Str(s.to_string()))
                    } else {
                        table.cell_f64(buf, r, col).map(JsonVal::Num)
                    };
                    if let Some(v) = val {
                        obj.insert(col.name.clone(), v);
                    }
                }
                rows.push(JsonVal::Obj(obj));
            }
            return Some(JsonVal::Arr(rows));
        }
        let data_bytes = fits_data_bytes(&header)?;
        let aligned = (data_start + data_bytes).div_ceil(2880) * 2880;
        if aligned <= off || aligned >= buf.len() {
            break;
        }
        off = aligned;
    }
    None
}

fn field_tau(src: &SourceConfig, key: &str, body: &str) -> Option<f64> {
    if let Some(secs) = derive_ttl(&src.url, body, &HashMap::new()) {
        return Some(secs as f64);
    }
    if let Some(tau) = src.extracts.iter().find_map(|ext| match ext {
        Extract::Field(fc) if fc.key == key && fc.tau.is_finite() && fc.tau > 0.0 => Some(fc.tau),
        _ => None,
    }) {
        return Some(tau);
    }
    let (_, _, tau) = probe_classify(key);
    if tau.is_finite() && tau > 0.0 {
        Some(tau)
    } else {
        None
    }
}

fn extract_raw(src: &SourceConfig, body: &str, now: f64, lsk: &LeapSeconds) -> ExtractResult {
    if src.format == "ephemeris_binary"
        || matches!(
            src.format.as_str(),
            "ephemeris_inpop"
                | "ephemeris_epm"
                | "ephemeris_de440"
                | "ephemeris_de442"
                | "ephemeris_noe4"
        )
    {
        let mut buf = Vec::new();
        if let Ok(mut f) = std::fs::File::open(body) {
            use std::io::Read;
            f.read_to_end(&mut buf).ok();
        }
        if let Some(eph) = parse_ephemeris_binary(&buf) {
            return ExtractResult::WithEphemeris(vec![], Box::new(eph));
        }
        return ExtractResult::Measurements(vec![]);
    }
    if src.format == "orbit_bin" {
        let mut buf = Vec::new();
        if let Ok(mut f) = std::fs::File::open(body) {
            use std::io::Read;
            f.read_to_end(&mut buf).ok();
        }
        if let Some(records) = crate::wind_orbit::parse_bin(&buf) {
            let rec = std::sync::Arc::new(crate::wind_orbit::orbit_rec(&records));
            return ExtractResult::WithEphemeris(
                vec![],
                Box::new(BodyEphemeris {
                    granules: Vec::new(),
                    rotation_matrices: Vec::new(),
                    props: None,
                    orbit: Some(rec),
                    granule_hint: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
                }),
            );
        }
        return ExtractResult::Measurements(vec![]);
    }
    if src.format == "sky1" {
        let mut buf = Vec::new();
        if let Ok(mut f) = std::fs::File::open(body) {
            use std::io::Read;
            f.read_to_end(&mut buf).ok();
        }
        let Some(rows) = crate::skymap::parse_header(&buf) else {
            return ExtractResult::Measurements(vec![]);
        };
        let Some(Extract::Field(fc)) = src.extracts.first() else {
            return ExtractResult::Measurements(vec![]);
        };
        let mut channels: Vec<(Channel, FieldConfig)> = Vec::new();
        let mut off = crate::skymap::HEADER_LEN;
        for _ in 0..rows {
            let Some(rec) = buf
                .get(off..off + crate::skymap::REC_BYTES)
                .and_then(crate::skymap::decode_rec)
            else {
                return ExtractResult::Measurements(vec![]);
            };
            off += crate::skymap::REC_BYTES;
            let ra = (rec.ra_deg as f64).to_radians();
            let dec = (rec.dec_deg as f64).to_radians();
            let (sa, ca) = ra.sin_cos();
            let (sd, cd) = dec.sin_cos();
            let p = [cd * ca, cd * sa, sd];
            channels.push((
                Channel {
                    z: 0.0,
                    freq: 0.0,
                    bin_width: 0.0,
                    epoch: now,
                    station_code: None,
                    position: Position::StateVector {
                        p,
                        v: [0.0, 0.0, 0.0],
                        track: false,
                    },
                    name: fc.name.clone(),
                    value: rec.value as f64,
                },
                fc.clone(),
            ));
        }
        if off != buf.len() {
            return ExtractResult::Measurements(vec![]);
        }
        return ExtractResult::Measurements(channels);
    }
    if src.format == "decaps_dr2_stars" {
        let Some(epoch) = src.catalog_epoch else {
            return ExtractResult::Measurements(vec![]);
        };
        if !epoch.is_finite() {
            return ExtractResult::Measurements(vec![]);
        }
        let mut buf = Vec::new();
        if let Ok(mut f) = std::fs::File::open(body) {
            use std::io::Read;
            f.read_to_end(&mut buf).ok();
        }
        let Some(stars) = crate::decaps::parse_bin(&buf) else {
            return ExtractResult::Measurements(vec![]);
        };
        let fields: Vec<&FieldConfig> = src
            .extracts
            .iter()
            .filter_map(|e| match e {
                Extract::Field(fc) => Some(fc),
                _ => None,
            })
            .collect();
        if fields.is_empty() {
            return ExtractResult::Measurements(vec![]);
        }
        let mut channels: Vec<(Channel, FieldConfig)> = Vec::new();
        for star in &stars {
            let ra = star.ra_deg.to_radians();
            let dec = star.dec_deg.to_radians();
            let (sa, ca) = ra.sin_cos();
            let (sd, cd) = dec.sin_cos();
            let p = [cd * ca, cd * sa, sd];
            for comp in 1..=crate::decaps::COMP_MAX {
                let Some(name) = crate::decaps::component_name(comp) else {
                    continue;
                };
                let Some(fc) = fields.iter().find(|fc| fc.name == name) else {
                    continue;
                };
                let Some(value) = crate::decaps::component_value(star, comp) else {
                    continue;
                };
                channels.push((
                    Channel {
                        z: 0.0,
                        freq: 0.0,
                        bin_width: 0.0,
                        epoch,
                        station_code: None,
                        position: Position::StateVector {
                            p,
                            v: [0.0, 0.0, 0.0],
                            track: false,
                        },
                        name: fc.name.clone(),
                        value,
                    },
                    (*fc).clone(),
                ));
            }
        }
        return ExtractResult::Measurements(channels);
    }
    if src.format == "catalog_allwise_psd" {
        let epoch = match src.catalog_epoch {
            Some(e) if e.is_finite() => e,
            _ => now,
        };
        let mut buf = Vec::new();
        if let Ok(mut f) = std::fs::File::open(body) {
            use std::io::Read;
            f.read_to_end(&mut buf).ok();
        }
        let Some(sources) = allwise::parse_bin(&buf) else {
            return ExtractResult::Measurements(vec![]);
        };
        let fields: Vec<FieldConfig> = src.extracts.iter().flat_map(extract_fields).collect();
        if fields.is_empty() {
            return ExtractResult::Measurements(vec![]);
        }
        let mut channels: Vec<(Channel, FieldConfig)> = Vec::new();
        for s in &sources {
            let ra = s.ra.to_radians();
            let dec = s.dec.to_radians();
            let (sa, ca) = ra.sin_cos();
            let (sd, cd) = dec.sin_cos();
            let p = [cd * ca, cd * sa, sd];
            for comp in 1..=allwise::COMP_MAX {
                let Some(name) = allwise::component_name(comp) else {
                    continue;
                };
                let Some(fc) = fields.iter().find(|fc| fc.name == name) else {
                    continue;
                };
                let Some(value) = allwise::component_value(s, comp) else {
                    continue;
                };
                channels.push((
                    Channel {
                        z: 0.0,
                        freq: 0.0,
                        bin_width: 0.0,
                        epoch,
                        station_code: None,
                        position: Position::StateVector {
                            p,
                            v: [0.0, 0.0, 0.0],
                            track: false,
                        },
                        name: fc.name.clone(),
                        value,
                    },
                    (*fc).clone(),
                ));
            }
        }
        return ExtractResult::Measurements(channels);
    }
    if src.format == "twomass_psc" {
        let epoch = match src.catalog_epoch {
            Some(e) if e.is_finite() => e,
            _ => now,
        };
        let mut buf = Vec::new();
        if let Ok(mut f) = std::fs::File::open(body) {
            use std::io::Read;
            f.read_to_end(&mut buf).ok();
        }
        let Some(rows) = twomass::read_bin(&buf) else {
            return ExtractResult::Measurements(vec![]);
        };
        let fields: Vec<FieldConfig> = src.extracts.iter().flat_map(extract_fields).collect();
        if fields.is_empty() {
            return ExtractResult::Measurements(vec![]);
        }
        let mut channels: Vec<(Channel, FieldConfig)> = Vec::new();
        for r in &rows {
            if !r[0].is_finite() || !r[1].is_finite() {
                continue;
            }
            let ra = r[0].to_radians();
            let dec = r[1].to_radians();
            let (sa, ca) = ra.sin_cos();
            let (sd, cd) = dec.sin_cos();
            let p = [cd * ca, cd * sa, sd];
            for comp in 1..=twomass::COMP_MAX {
                let Some(name) = twomass::component_name(comp) else {
                    continue;
                };
                let Some(fc) = fields.iter().find(|fc| fc.name == name) else {
                    continue;
                };
                let Some(value) = twomass::component_value(r, comp) else {
                    continue;
                };
                channels.push((
                    Channel {
                        z: 0.0,
                        freq: 0.0,
                        bin_width: 0.0,
                        epoch,
                        station_code: None,
                        position: Position::StateVector {
                            p,
                            v: [0.0, 0.0, 0.0],
                            track: false,
                        },
                        name: fc.name.clone(),
                        value,
                    },
                    (*fc).clone(),
                ));
            }
        }
        return ExtractResult::Measurements(channels);
    }
    if src.format == "swarm_tec" {
        const MAGIC: [u8; 4] = *b"SCTE";
        const FIELDS: usize = 5;
        const REC_BYTES: usize = FIELDS * 8;
        let mut buf = Vec::new();
        if let Ok(mut f) = std::fs::File::open(body) {
            use std::io::Read;
            f.read_to_end(&mut buf).ok();
        }
        if buf.len() < 8 || buf[0..4] != MAGIC {
            return ExtractResult::Measurements(vec![]);
        }
        let n = u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]) as usize;
        if buf.len() != 8 + n * REC_BYTES {
            return ExtractResult::Measurements(vec![]);
        }
        let fields: Vec<FieldConfig> = src.extracts.iter().flat_map(extract_fields).collect();
        if fields.is_empty() {
            return ExtractResult::Measurements(vec![]);
        }
        let mut channels: Vec<(Channel, FieldConfig)> = Vec::new();
        for i in 0..n {
            let base = 8 + i * REC_BYTES;
            let cell = |k: usize| -> Option<f64> {
                buf.get(base + k * 8..base + k * 8 + 8)
                    .and_then(|b| <[u8; 8]>::try_from(b).ok())
                    .map(f64::from_le_bytes)
            };
            let (Some(lat), Some(lon), Some(radius_km), Some(t), Some(vtec)) =
                (cell(0), cell(1), cell(2), cell(3), cell(4))
            else {
                continue;
            };
            if !lat.is_finite()
                || !lon.is_finite()
                || !radius_km.is_finite()
                || !t.is_finite()
                || !vtec.is_finite()
                || radius_km <= 0.0
            {
                continue;
            }
            let Some(epoch) = lsk.unix_to_tdb(t) else {
                continue;
            };
            let (sa, ca) = lon.to_radians().sin_cos();
            let (sd, cd) = lat.to_radians().sin_cos();
            let radius_m = radius_km * 1000.0;
            let p = [cd * ca * radius_m, cd * sa * radius_m, sd * radius_m];
            for fc in &fields {
                channels.push((
                    Channel {
                        z: 0.0,
                        freq: 0.0,
                        bin_width: 0.0,
                        epoch,
                        station_code: None,
                        position: Position::StateVector {
                            p,
                            v: [0.0, 0.0, 0.0],
                            track: false,
                        },
                        name: fc.name.clone(),
                        value: vtec,
                    },
                    fc.clone(),
                ));
            }
        }
        return ExtractResult::Measurements(channels);
    }
    if src.format == "catalog_charm2" {
        let mut buf = Vec::new();
        if let Ok(mut f) = std::fs::File::open(body) {
            use std::io::Read;
            f.read_to_end(&mut buf).ok();
        }
        let Some(stars) = charm2::parse_bin(&buf) else {
            return ExtractResult::Measurements(vec![]);
        };
        let fields: Vec<FieldConfig> = src.extracts.iter().flat_map(extract_fields).collect();
        if fields.is_empty() {
            return ExtractResult::Measurements(vec![]);
        }
        let mut channels: Vec<(Channel, FieldConfig)> = Vec::new();
        for s in &stars {
            let plx = match s.plx_mas {
                Some(p) if p.is_finite() && p > 0.0 => p,
                _ => continue,
            };
            let d = PARSEC_M * 1000.0 / plx;
            let ra = s.ra.to_radians();
            let dec = s.dec.to_radians();
            let (sa, ca) = ra.sin_cos();
            let (sd, cd) = dec.sin_cos();
            let p = [cd * ca * d, cd * sa * d, sd * d];
            for comp in 1..=charm2::COMP_MAX {
                let Some(name) = charm2::component_name(comp) else {
                    continue;
                };
                let Some(fc) = fields.iter().find(|fc| fc.name == name) else {
                    continue;
                };
                let Some(value) = charm2::component_value(s, comp) else {
                    continue;
                };
                channels.push((
                    Channel {
                        z: 0.0,
                        freq: 0.0,
                        bin_width: 0.0,
                        epoch: now,
                        station_code: None,
                        position: Position::StateVector {
                            p,
                            v: [0.0, 0.0, 0.0],
                            track: false,
                        },
                        name: fc.name.clone(),
                        value,
                    },
                    (*fc).clone(),
                ));
            }
        }
        return ExtractResult::Measurements(channels);
    }
    if src.format == "exofop_toi" {
        const MAGIC: [u8; 4] = *b"EXF1";
        const VERSION: u8 = 1;
        const HEADER_LEN: usize = 13;
        const REC_BYTES: usize = 44;
        let mut buf = Vec::new();
        if let Ok(mut f) = std::fs::File::open(body) {
            use std::io::Read;
            f.read_to_end(&mut buf).ok();
        }
        if buf.len() < HEADER_LEN || buf[0..4] != MAGIC || buf[4] != VERSION {
            return ExtractResult::Measurements(vec![]);
        }
        let n = u64::from_le_bytes([
            buf[5], buf[6], buf[7], buf[8], buf[9], buf[10], buf[11], buf[12],
        ]) as usize;
        if buf.len() != HEADER_LEN + n * REC_BYTES {
            return ExtractResult::Measurements(vec![]);
        }
        let Some(Extract::Field(fc)) = src.extracts.first() else {
            return ExtractResult::Measurements(vec![]);
        };
        let f64_at = |rec: &[u8], r: std::ops::Range<usize>| -> Option<f64> {
            rec.get(r)
                .and_then(|x| <[u8; 8]>::try_from(x).ok())
                .map(f64::from_le_bytes)
        };
        let f32_at = |rec: &[u8], r: std::ops::Range<usize>| -> Option<f32> {
            rec.get(r)
                .and_then(|x| <[u8; 4]>::try_from(x).ok())
                .map(f32::from_le_bytes)
        };
        let mut channels: Vec<(Channel, FieldConfig)> = Vec::with_capacity(n);
        let mut off = HEADER_LEN;
        for _ in 0..n {
            let Some(rec) = buf.get(off..off + REC_BYTES) else {
                return ExtractResult::Measurements(vec![]);
            };
            off += REC_BYTES;
            let ra_deg = match f64_at(rec, 16..24) {
                Some(v) if v.is_finite() && (0.0..360.0).contains(&v) => v,
                _ => continue,
            };
            let dec_deg = match f64_at(rec, 24..32) {
                Some(v) if v.is_finite() && v.abs() <= 90.0 => v,
                _ => continue,
            };
            let depth = match f32_at(rec, 40..44) {
                Some(v) if v.is_finite() && v > 0.0 => v as f64,
                _ => continue,
            };
            let ra = ra_deg.to_radians();
            let dec = dec_deg.to_radians();
            let (sa, ca) = ra.sin_cos();
            let (sd, cd) = dec.sin_cos();
            let p = [cd * ca, cd * sa, sd];
            channels.push((
                Channel {
                    z: 0.0,
                    freq: 0.0,
                    bin_width: 0.0,
                    epoch: now,
                    station_code: None,
                    position: Position::StateVector {
                        p,
                        v: [0.0, 0.0, 0.0],
                        track: false,
                    },
                    name: fc.name.clone(),
                    value: depth,
                },
                fc.clone(),
            ));
        }
        return ExtractResult::Measurements(channels);
    }
    if src.format == "arpansa" || src.format == "uvxml" {
        let Some(tau) = field_tau(src, "uv_index", body) else {
            return ExtractResult::Measurements(vec![]);
        };
        let fc = FieldConfig {
            key: "uv_index".to_string(),
            name: "uv_index".to_string(),
            kernel: 0,
            force: 0,
            tau,
            absorption: 0.0,
            advection: 0.0,
            unit: "UVI".to_string(),
            freq: 0.0,
            bin_width: 0.0,
            fold: None,
            aperture: Aperture::None,
        };
        let mut channels: Vec<(Channel, FieldConfig)> = Vec::new();
        let frame_name = frame_body_name(&src.frame);
        for r in crate::archivar::arpansa::parse_uv_xml(body) {
            let Some((lat, lon)) = crate::archivar::arpansa::station_coords(&r.id) else {
                continue;
            };
            channels.push((
                Channel {
                    z: 0.0,
                    freq: 0.0,
                    bin_width: 0.0,
                    epoch: now,
                    station_code: None,
                    position: Position::Surface {
                        body_name: frame_name.clone(),
                        lat,
                        lon,
                        alt: 0.0,
                    },
                    name: r.id,
                    value: r.index,
                },
                fc.clone(),
            ));
        }
        return ExtractResult::Measurements(channels);
    }
    if src.format == "igra_zip" {
        let Some(text) = std::fs::read(body)
            .ok()
            .and_then(|b| unzip(&b))
            .map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
        else {
            return ExtractResult::Measurements(vec![]);
        };
        let fcfg = |name: &str, kernel: u8, force: u8, unit: &str| -> Option<FieldConfig> {
            let tau = field_tau(src, name, &text)?;
            Some(FieldConfig {
                key: name.to_string(),
                name: name.to_string(),
                kernel,
                force,
                tau,
                absorption: 0.0,
                advection: 0.0,
                unit: unit.to_string(),
                freq: 0.0,
                bin_width: 0.0,
                fold: None,
                aperture: Aperture::None,
            })
        };
        let mut channels: Vec<(Channel, FieldConfig)> = Vec::new();
        for s in crate::archivar::igra::parse_igra(&text) {
            for lvl in &s.levels {
                let Some(alt) = lvl.gph_m else {
                    continue;
                };
                let position = Position::Surface {
                    body_name: frame_body_name(&src.frame),
                    lat: s.lat,
                    lon: s.lon,
                    alt,
                };
                let emitted: [(Option<f64>, Option<FieldConfig>); 6] = [
                    (
                        lvl.press_pa.map(|p| p / 100.0),
                        fcfg("igra_air_pressure_hpa", 5, 7, "hPa"),
                    ),
                    (lvl.temp_c, fcfg("igra_air_temp_c", 4, 5, "C")),
                    (lvl.wspd_ms, fcfg("igra_wind_speed_ms", 5, 7, "m/s")),
                    (lvl.wdir_deg, fcfg("igra_wind_direction_deg", 5, 7, "deg")),
                    (lvl.rh_pct, fcfg("igra_relative_humidity_pct", 1, 6, "%")),
                    (lvl.dpdp_c, fcfg("igra_dewpoint_c", 4, 5, "C")),
                ];
                for (value, fc) in emitted {
                    if let (Some(v), Some(fc)) = (value, fc) {
                        channels.push((
                            Channel {
                                z: 0.0,
                                freq: 0.0,
                                bin_width: 0.0,
                                epoch: now,
                                station_code: None,
                                position: position.clone(),
                                name: fc.name.clone(),
                                value: v,
                            },
                            fc,
                        ));
                    }
                }
            }
        }
        return ExtractResult::Measurements(channels);
    }
    if src.format == "fugin_cube" {
        let buf = match std::fs::read(body) {
            Ok(b) => b,
            Err(_) => return ExtractResult::Measurements(vec![]),
        };
        let Some(pixels) = crate::archivar::fugin::parse_fugin_cube(&buf) else {
            return ExtractResult::Measurements(vec![]);
        };
        let Some(tau) = field_tau(src, "fugin_moment0", body) else {
            return ExtractResult::Measurements(vec![]);
        };
        let fc = FieldConfig {
            key: "fugin_moment0".to_string(),
            name: "fugin_moment0".to_string(),
            kernel: 0,
            force: 0,
            tau,
            absorption: 0.0,
            advection: 0.0,
            unit: "K m/s".to_string(),
            freq: 0.0,
            bin_width: 0.0,
            fold: None,
            aperture: Aperture::None,
        };
        let mut channels: Vec<(Channel, FieldConfig)> = Vec::with_capacity(pixels.len());
        for p in &pixels {
            let ra = p.ra_deg.to_radians();
            let dec = p.dec_deg.to_radians();
            let (sa, ca) = ra.sin_cos();
            let (sd, cd) = dec.sin_cos();
            channels.push((
                Channel {
                    z: 0.0,
                    freq: 0.0,
                    bin_width: 0.0,
                    epoch: now,
                    station_code: None,
                    position: Position::StateVector {
                        p: [cd * ca, cd * sa, sd],
                        v: [0.0, 0.0, 0.0],
                        track: false,
                    },
                    name: fc.name.clone(),
                    value: p.moment0_k_ms,
                },
                fc.clone(),
            ));
        }
        return ExtractResult::Measurements(channels);
    }
    if src.format == "ndk" {
        let Some(Extract::Field(fc)) = src.extracts.first() else {
            return ExtractResult::Measurements(vec![]);
        };
        let m0_fc = fc.clone();
        let mut channels: Vec<(Channel, FieldConfig)> = Vec::new();
        for ev in crate::archivar::ndk::parse_ndk(body) {
            let m0_nm = ev.scalar_moment_dyne_cm() * 1e-7;
            if !m0_nm.is_finite() || m0_nm <= 0.0 {
                continue;
            }
            let position = Position::Surface {
                body_name: frame_body_name(&src.frame),
                lat: ev.centroid_lat,
                lon: ev.centroid_lon,
                alt: 0.0,
            };
            let Some(epoch) = ymd_to_days(ev.year as i64, ev.month, ev.day)
                .map(|d| d as f64 * 86400.0)
                .and_then(|unix| lsk.unix_to_tdb(unix))
            else {
                continue;
            };
            channels.push((
                Channel {
                    z: ev.centroid_depth_km,
                    freq: 0.0,
                    bin_width: 0.0,
                    epoch,
                    station_code: None,
                    position: position.clone(),
                    name: m0_fc.name.clone(),
                    value: m0_nm,
                },
                m0_fc.clone(),
            ));
        }
        return ExtractResult::Measurements(channels);
    }
    if src.format == "gistemp_aod550_axis_value_text"
        || src.format == "godas_pottmp_axis_value_text"
        || src.format == "openmeteo_pollen_axis_value_text"
        || src.format == "dhm_stage"
    {
        let Some(Extract::Field(fc)) = src.extracts.first() else {
            return ExtractResult::Measurements(vec![]);
        };
        let Frame::Surface { lat, lon, alt, .. } = &src.frame else {
            return ExtractResult::Measurements(vec![]);
        };
        let mut channels: Vec<(Channel, FieldConfig)> = Vec::new();
        for (unix, value) in crate::archivar::geo::parse_axis_value_text(body) {
            let Some(epoch) = lsk.unix_to_tdb(unix) else {
                continue;
            };
            channels.push((
                Channel {
                    z: 0.0,
                    freq: 0.0,
                    bin_width: 0.0,
                    epoch,
                    station_code: None,
                    position: Position::Surface {
                        body_name: frame_body_name(&src.frame),
                        lat: *lat,
                        lon: *lon,
                        alt: *alt,
                    },
                    name: fc.name.clone(),
                    value,
                },
                fc.clone(),
            ));
        }
        return ExtractResult::Measurements(channels);
    }
    if src.format == "trishuli_stage" {
        let Some(station_id) = src
            .station_code
            .as_deref()
            .and_then(|s| s.trim().parse::<u32>().ok())
        else {
            return ExtractResult::Measurements(vec![]);
        };
        let Some(station) = crate::archivar::geo::parse_dhm_stage(body, station_id) else {
            return ExtractResult::Measurements(vec![]);
        };
        let Some(Extract::Field(fc)) = src.extracts.first() else {
            return ExtractResult::Measurements(vec![]);
        };
        let Frame::Surface { lat, lon, alt, .. } = &src.frame else {
            return ExtractResult::Measurements(vec![]);
        };
        let (station_lat, station_lon, station_alt) = match (station.lat, station.lon) {
            (Some(station_lat), Some(station_lon)) => (station_lat, station_lon, *alt),
            _ => (*lat, *lon, *alt),
        };
        let mut channels: Vec<(Channel, FieldConfig)> = Vec::new();
        for (unix, value) in station.series {
            let Some(epoch) = lsk.unix_to_tdb(unix) else {
                continue;
            };
            channels.push((
                Channel {
                    z: 0.0,
                    freq: 0.0,
                    bin_width: 0.0,
                    epoch,
                    station_code: None,
                    position: Position::Surface {
                        body_name: frame_body_name(&src.frame),
                        lat: station_lat,
                        lon: station_lon,
                        alt: station_alt,
                    },
                    name: fc.name.clone(),
                    value,
                },
                fc.clone(),
            ));
        }
        return ExtractResult::Measurements(channels);
    }
    let mut channels: Vec<(Channel, FieldConfig)> = Vec::new();
    let mut extracted: HashMap<String, f64> = HashMap::new();
    let csv_zip_text: Option<String> = if src.format == "csv_zip" {
        std::fs::read(body)
            .ok()
            .and_then(|b| unzip(&b))
            .map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
    } else {
        None
    };
    let parsed_json = if src.format == "csv_zip" {
        let rows_only = src
            .extracts
            .iter()
            .all(|e| matches!(e, Extract::Rows { .. }));
        if rows_only {
            None
        } else {
            csv_zip_text.as_deref().and_then(csv_to_json)
        }
    } else if src.format == "csv" {
        if src.extracts.iter().any(|e| matches!(e, Extract::Hapi(_))) {
            hapi_csv_body(src, body).or_else(|| csv_to_json(body))
        } else {
            csv_to_json(body)
        }
    } else if src.format == "free text" {
        text_to_json(body)
    } else if src.format == "tap" {
        tap_body_to_json(&src.url, body)
    } else if src.format == "votable" {
        votable_to_json(body)
    } else if src.format == "asu-tsv" {
        tsv_to_json(body)
    } else if src.format == "html" {
        html_to_json(body)
    } else if src.format == "json" || src.format.is_empty() || src.format == "universal" {
        let body = body
            .strip_prefix("OK")
            .and_then(|r| r.strip_prefix('\n').or_else(|| r.strip_prefix("\r\n")))
            .unwrap_or(body);
        let parsed = parse_json(body);
        match parsed {
            Some(j)
                if src.format != "universal"
                    && !src.extracts.iter().any(|e| matches!(e, Extract::Hapi(_))) =>
            {
                tap_to_json(&j).or(Some(j))
            }
            other => other,
        }
    } else if src.format == "fits" {
        std::fs::read(body).ok().as_deref().and_then(fits_to_json)
    } else if src.format == "tar_gz_yaml" {
        let wanted: Vec<String> = src
            .extracts
            .iter()
            .flat_map(extract_fields)
            .filter_map(|fc| fc.key.split('.').next().map(str::to_string))
            .collect();
        tar_gz_yaml_to_json(body, &wanted)
    } else {
        None
    };
    let parsed_json = match parsed_json {
        Some(j) => Some(j),
        None if src.extracts.iter().any(|e| matches!(e, Extract::Hapi(_)))
            && matches!(src.format.as_str(), "" | "json" | "universal") =>
        {
            hapi_csv_body(src, body)
        }
        other => other,
    };
    let auto_extracts: Option<Vec<Extract>>;
    let effective_extracts: &[Extract] = if src.format == "universal" && src.extracts.is_empty() {
        if let Some(ref j) = parsed_json {
            auto_extracts = Some(universal_auto_detect(j));
            if let Some(ref auto) = auto_extracts {
                auto.as_slice()
            } else {
                return ExtractResult::Measurements(vec![]);
            }
        } else {
            return ExtractResult::Measurements(vec![]);
        }
    } else {
        &src.extracts
    };
    for ext in effective_extracts {
        match ext {
            Extract::Field(fc) => {
                if let Some(ref j) = parsed_json
                    && let Some(v) = jnum(j, &fc.key)
                {
                    extracted.insert(fc.name.clone(), v);
                }
            }
            Extract::First(fc, filter) => {
                if let Some(ref j) = parsed_json
                    && let Some(v) = jfirst_where(j, &fc.key, filter.as_ref())
                {
                    extracted.insert(fc.name.clone(), v);
                }
            }
            Extract::Last(fc, filter) => {
                if let Some(ref j) = parsed_json {
                    if let Some(v) = jlast_where(j, &fc.key, filter.as_ref()) {
                        extracted.insert(fc.name.clone(), v);
                    }
                } else if fc.key == "line"
                    && let Some(v) = body
                        .lines()
                        .rev()
                        .filter(|l| {
                            let t = l.trim();
                            !t.is_empty() && !t.starts_with('#')
                        })
                        .find_map(|l| {
                            split_data_line(l)
                                .last()
                                .and_then(|c| c.trim_matches('"').parse::<f64>().ok())
                        })
                {
                    extracted.insert(fc.name.clone(), v);
                }
            }
            Extract::Count(fc) => {
                let v = if src.format == "csv" || fc.key == "lines" {
                    Some(
                        body.lines()
                            .filter(|l| !l.trim().is_empty() && !l.trim().starts_with('#'))
                            .count() as f64,
                    )
                } else {
                    parsed_json.as_ref().and_then(|j| jcount(j, &fc.key))
                };
                if let Some(v) = v {
                    extracted.insert(fc.name.clone(), v);
                }
            }
            Extract::LastRow(fc) => {
                if src.format == "csv" {
                    if let Some(v) = text_last_col(body, &fc.key) {
                        extracted.insert(fc.name.clone(), v);
                    }
                } else if let Some(ref j) = parsed_json {
                    if let Some(v) = j2d_last_row(j, &fc.key) {
                        extracted.insert(fc.name.clone(), v);
                    }
                } else if let Some(v) = text_last_col(body, &fc.key) {
                    extracted.insert(fc.name.clone(), v);
                }
            }
            Extract::Path(fc) => {
                if let Some(ref j) = parsed_json
                    && let Some(v) = jpath(j, &fc.key)
                {
                    extracted.insert(fc.name.clone(), v);
                }
            }
            Extract::Deep(fc) => {
                if let Some(ref j) = parsed_json
                    && let Some(v) = jdeep_find_num(j, &fc.key)
                {
                    extracted.insert(fc.name.clone(), v);
                }
            }
            Extract::LastLine(n) => {
                if let Some(v) = body
                    .lines()
                    .rfind(|l| {
                        let t = l.trim();
                        !t.is_empty() && !t.starts_with('#')
                    })
                    .and_then(|line| {
                        split_data_line(line)
                            .into_iter()
                            .filter_map(|t| t.parse::<f64>().ok())
                            .next_back()
                    })
                {
                    extracted.insert(n.clone(), v);
                }
            }
            Extract::ObjLast(fc) => {
                if let Some(ref j) = parsed_json
                    && let Some(obj) = jpath_val(j, &fc.key)
                    && let JsonVal::Obj(m) = obj
                    && let Some(last_key) = m.keys().max_by(|a, b| {
                        if let (Ok(ka), Ok(kb)) = (a.parse::<i64>(), b.parse::<i64>()) {
                            ka.cmp(&kb)
                        } else {
                            a.cmp(b)
                        }
                    })
                    && let Some(val) = m.get(last_key).and_then(scalar_of)
                {
                    extracted.insert(fc.name.clone(), val);
                }
            }
            Extract::Regex(fc) => {
                if let Some(v) = extract_regex_val(body, &fc.key) {
                    extracted.insert(fc.name.clone(), v);
                }
            }
            Extract::XmlCount(tag, n) => {
                let count = body.matches(&format!("<{}>", tag)).count() as f64;
                extracted.insert(n.clone(), count);
            }
            Extract::LastObj(fk, fv, ek, n) => {
                if let Some(ref j) = parsed_json
                    && let JsonVal::Arr(arr) = j
                {
                    for v in arr.iter().rev() {
                        if let JsonVal::Obj(o) = v
                            && let Some(JsonVal::Str(s)) = o.get(fk)
                            && s == fv
                            && let Some(val) = jnum(v, ek)
                        {
                            extracted.insert(n.clone(), val);
                            break;
                        }
                    }
                }
            }
            Extract::Map {
                arr_path,
                lat_key,
                lon_key,
                alt_key,
                epoch_key,
                val_key,
                alt_scale,
                vel_key,
                vel_scale,
                trk_key,
                vr_key,
                fields,
                lat_sign,
                lon_sign,
                epoch_scale,
                tau_key,
                mag_type_key,
            } => {
                let eff_lat_key = lat_key.clone();
                let eff_lon_key = lon_key.clone();
                let eff_epoch_key = epoch_key.clone();
                if let Some(ref j) = parsed_json {
                    let rows: Vec<&JsonVal> = match jpath_val(j, arr_path) {
                        Some(JsonVal::Arr(arr)) => arr.iter().collect(),
                        Some(obj @ JsonVal::Obj(_)) => vec![obj],
                        _ => Vec::new(),
                    };
                    {
                        for v in rows {
                            let lat = key_or_constant(&eff_lat_key, v);
                            let lon = key_or_constant(&eff_lon_key, v);
                            let alt = if alt_key.is_empty() {
                                Some(0.0)
                            } else {
                                match jpath(v, alt_key) {
                                    Some(a) => Some(a * alt_scale),
                                    None => Some(0.0),
                                }
                            };
                            let position = match (lat, lon, alt) {
                                (Some(la), Some(lo), Some(al)) => {
                                    let mut lat_val = la;
                                    if let Some(sign_key) = lat_sign
                                        && let Some(vv) = jpath_val(v, sign_key)
                                        && let JsonVal::Str(s) = vv
                                        && (s.contains('S') || s.contains('s'))
                                    {
                                        lat_val = -la;
                                    }
                                    let mut lon_val = lo;
                                    if let Some(sign_key) = lon_sign
                                        && let Some(vv) = jpath_val(v, sign_key)
                                        && let JsonVal::Str(s) = vv
                                        && (s.contains('W') || s.contains('w'))
                                    {
                                        lon_val = -lo;
                                    }
                                    let speed = if vel_key.is_empty() {
                                        None
                                    } else {
                                        jpath(v, vel_key).map(|s| s * vel_scale)
                                    };
                                    let track = if trk_key.is_empty() {
                                        None
                                    } else {
                                        jpath(v, trk_key)
                                    };
                                    let vrate = if vr_key.is_empty() {
                                        None
                                    } else {
                                        jpath(v, vr_key).map(|s| s * vel_scale)
                                    };
                                    if let (Some(sp), Some(tr)) = (speed, track) {
                                        Position::SurfaceFlow {
                                            body_name: frame_body_name(&src.frame),
                                            lat: lat_val,
                                            lon: lon_val,
                                            alt: al,
                                            speed: sp,
                                            track: tr,
                                            vrate,
                                        }
                                    } else {
                                        Position::Surface {
                                            body_name: frame_body_name(&src.frame),
                                            lat: lat_val,
                                            lon: lon_val,
                                            alt: al,
                                        }
                                    }
                                }
                                (_, _, _) => match &src.frame {
                                    crate::archivar::Frame::Barycenter { body_name, scale } => {
                                        Position::Barycenter {
                                            body_name: body_name.clone(),
                                            scale: *scale,
                                        }
                                    }
                                    _ => continue,
                                },
                            };
                            let epoch = if eff_epoch_key.is_empty() {
                                now
                            } else if let Some(ev) = jpath_val(v, &eff_epoch_key) {
                                match ev {
                                    JsonVal::Str(s) => {
                                        if let Some(t) = parse_iso_tdb(s, lsk) {
                                            t
                                        } else {
                                            continue;
                                        }
                                    }
                                    JsonVal::Num(n) => match lsk.unix_to_tdb(*n * epoch_scale) {
                                        Some(t) => t,
                                        None => continue,
                                    },
                                    _ => continue,
                                }
                            } else {
                                continue;
                            };
                            let row_tau: Option<f64> = if tau_key.is_empty() {
                                None
                            } else {
                                match jpath(v, tau_key) {
                                    Some(t) if t > 0.0 => Some(t),
                                    Some(_) => continue,
                                    None => None,
                                }
                            };
                            for fc in fields {
                                if !val_key.is_empty() && fc.name != *val_key {
                                    continue;
                                }
                                let mut raw = jpath(v, &fc.key);
                                if !mag_type_key.is_empty()
                                    && fc.unit.eq_ignore_ascii_case("mw")
                                    && let Some(t) = jstr(v, mag_type_key)
                                    && !is_moment_magnitude(&t)
                                {
                                    continue;
                                }
                                let mut transformed = false;
                                if let Some((op, key_b)) = &fc.fold {
                                    raw = fold_value(raw, jpath(v, key_b), *op);
                                } else if let Some(ref mag_key) = src.flux_from_mag
                                    && fc.key == *mag_key
                                {
                                    raw = raw.map(|r| 10.0f64.powf(-0.4 * r));
                                    transformed = true;
                                }
                                let val = match raw {
                                    Some(vv) => vv,
                                    None => continue,
                                };
                                if !val.is_finite() {
                                    continue;
                                }
                                let mut eff_fc = (*fc).clone();
                                if transformed {
                                    eff_fc.unit.clear();
                                }
                                if let Some(t) = row_tau {
                                    eff_fc.tau = t;
                                }
                                channels.push((
                                    Channel {
                                        z: 0.0,
                                        freq: 0.0,
                                        bin_width: 0.0,
                                        epoch,
                                        station_code: None,
                                        position: position.clone(),
                                        name: fc.name.clone(),
                                        value: val,
                                    },
                                    eff_fc,
                                ));
                            }
                        }
                    }
                }
            }
            Extract::ProfileMap {
                arr_path,
                lat_key,
                lon_key,
                epoch_key,
                pressure_var,
                pressure_scale,
                fields,
            } => {
                if let Some(ref j) = parsed_json {
                    let rows: Vec<&JsonVal> = match jpath_val(j, arr_path) {
                        Some(JsonVal::Arr(arr)) => arr.iter().collect(),
                        Some(obj @ JsonVal::Obj(_)) => vec![obj],
                        _ => Vec::new(),
                    };
                    for v in rows {
                        let lat = jpath(v, lat_key);
                        let lon = jpath(v, lon_key);
                        if let (Some(la), Some(lo)) = (lat, lon) {
                            let epoch = if epoch_key.is_empty() {
                                now
                            } else if let Some(ev) = jpath_val(v, epoch_key) {
                                match ev {
                                    JsonVal::Str(s) => {
                                        if let Some(t) = parse_iso_tdb(s, lsk) {
                                            t
                                        } else {
                                            continue;
                                        }
                                    }
                                    JsonVal::Num(n) => match lsk.unix_to_tdb(*n) {
                                        Some(t) => t,
                                        None => continue,
                                    },
                                    _ => continue,
                                }
                            } else {
                                continue;
                            };
                            let data = match jpath_val(v, "data") {
                                Some(JsonVal::Arr(d)) => d,
                                _ => continue,
                            };
                            let (var_names, pressure_idx) = match jpath_val(v, "data_info") {
                                Some(JsonVal::Arr(info)) => {
                                    let names: Vec<String> = match info.first() {
                                        Some(JsonVal::Arr(n)) => n
                                            .iter()
                                            .filter_map(|e| match e {
                                                JsonVal::Str(s) => Some(s.clone()),
                                                _ => None,
                                            })
                                            .collect(),
                                        _ => Vec::new(),
                                    };
                                    let pidx = names.iter().position(|n| n == pressure_var);
                                    (names, pidx)
                                }
                                _ => (Vec::new(), None),
                            };
                            let pidx = match pressure_idx {
                                Some(i) => i,
                                None => continue,
                            };
                            let pressure = match data.get(pidx) {
                                Some(JsonVal::Arr(p)) => p,
                                _ => continue,
                            };
                            let n_levels = pressure.len();
                            for fc in fields {
                                let vidx = match var_names.iter().position(|n| *n == fc.key) {
                                    Some(i) => i,
                                    None => continue,
                                };
                                let values = match data.get(vidx) {
                                    Some(JsonVal::Arr(a)) => a,
                                    _ => continue,
                                };
                                for k in 0..n_levels {
                                    let p = match pressure.get(k) {
                                        Some(JsonVal::Num(x)) => *x,
                                        _ => continue,
                                    };
                                    let val = match values.get(k) {
                                        Some(JsonVal::Num(x)) => *x,
                                        _ => continue,
                                    };
                                    if !val.is_finite() {
                                        continue;
                                    }
                                    let position = Position::Surface {
                                        body_name: frame_body_name(&src.frame),
                                        lat: la,
                                        lon: lo,
                                        alt: -p * pressure_scale,
                                    };
                                    channels.push((
                                        Channel {
                                            z: 0.0,
                                            freq: fc.freq,
                                            bin_width: fc.bin_width,
                                            epoch,
                                            position,
                                            station_code: src.station_code.clone(),
                                            name: fc.name.clone(),
                                            value: val,
                                        },
                                        fc.clone(),
                                    ));
                                }
                            }
                        }
                    }
                }
            }
            Extract::EpnCore {
                arr_path,
                body_key,
                lon_min_key,
                lon_max_key,
                lat_min_key,
                lat_max_key,
                alt_min_key,
                alt_max_key,
                s_region_key,
                epoch_key,
                epoch_mjd,
                val_key,
                fields,
            } => {
                if let Some(ref j) = parsed_json {
                    let rows: Vec<&JsonVal> = match jpath_val(j, arr_path) {
                        Some(JsonVal::Arr(arr)) => arr.iter().collect(),
                        Some(obj @ JsonVal::Obj(_)) => vec![obj],
                        _ => Vec::new(),
                    };
                    for v in rows {
                        let body_name = if body_key.is_empty() {
                            frame_body_name(&src.frame)
                        } else {
                            match jstr(v, body_key) {
                                Some(s) if !s.trim().is_empty() => s.trim().to_ascii_lowercase(),
                                _ => frame_body_name(&src.frame),
                            }
                        };
                        let lon = region_midpoint(v, lon_min_key, lon_max_key, s_region_key, 0);
                        let lat = region_midpoint(v, lat_min_key, lat_max_key, s_region_key, 1);
                        let (Some(lon), Some(lat)) = (lon, lat) else {
                            continue;
                        };
                        if !(lat.is_finite() && (-90.0..=90.0).contains(&lat) && lon.is_finite()) {
                            continue;
                        }
                        let alt = if alt_min_key.is_empty() && alt_max_key.is_empty() {
                            0.0
                        } else {
                            match region_midpoint(v, alt_min_key, alt_max_key, "", 2) {
                                Some(a) if a.is_finite() => a,
                                _ => continue,
                            }
                        };
                        let epoch = if epoch_key.is_empty() {
                            now
                        } else if let Some(ev) = jpath_val(v, epoch_key) {
                            match ev {
                                JsonVal::Str(s) => match parse_iso_tdb(s, lsk) {
                                    Some(t) => t,
                                    None => continue,
                                },
                                JsonVal::Num(n) if *epoch_mjd => {
                                    match crate::maxi::mjd_to_tdb(*n, lsk) {
                                        Some(t) => t,
                                        None => continue,
                                    }
                                }
                                JsonVal::Num(n) => match lsk.unix_to_tdb(*n) {
                                    Some(t) => t,
                                    None => continue,
                                },
                                _ => continue,
                            }
                        } else {
                            continue;
                        };
                        for fc in fields {
                            if !val_key.is_empty() && fc.name != *val_key {
                                continue;
                            }
                            let val = match jpath(v, &fc.key) {
                                Some(vv) => vv,
                                None => continue,
                            };
                            if !val.is_finite() {
                                continue;
                            }
                            channels.push((
                                Channel {
                                    z: 0.0,
                                    freq: 0.0,
                                    bin_width: 0.0,
                                    epoch,
                                    station_code: None,
                                    position: Position::Surface {
                                        body_name: body_name.clone(),
                                        lat,
                                        lon,
                                        alt,
                                    },
                                    name: fc.name.clone(),
                                    value: val,
                                },
                                fc.clone(),
                            ));
                        }
                    }
                }
            }
            Extract::Flatten {
                arr_path,
                geom_path,
                epoch_key,
                fields,
            } => {
                if let Some(ref j) = parsed_json
                    && let Some(JsonVal::Arr(arr)) = jpath_val(j, arr_path)
                {
                    for v in arr.iter() {
                        let coords = if geom_path.is_empty() {
                            match jpath_val(v, "coordinates") {
                                Some(JsonVal::Arr(c)) => c,
                                _ => match v {
                                    JsonVal::Arr(c) => c,
                                    _ => continue,
                                },
                            }
                        } else {
                            let geom = match jpath_val(v, geom_path) {
                                Some(g) => g,
                                None => continue,
                            };
                            match jpath_val(geom, "coordinates") {
                                Some(JsonVal::Arr(c)) => c,
                                _ => match geom {
                                    JsonVal::Arr(c) => c,
                                    _ => continue,
                                },
                            }
                        };
                        let vertices = flatten_geojson_coords(coords);
                        if vertices.is_empty() {
                            continue;
                        }
                        let row_epoch = if !epoch_key.is_empty() {
                            match jpath(v, epoch_key) {
                                Some(ev) => ev,
                                None => continue,
                            }
                        } else {
                            continue;
                        };
                        for (lon, lat, z) in vertices {
                            let position = Position::Surface {
                                body_name: frame_body_name(&src.frame),
                                lat,
                                lon,
                                alt: match z {
                                    Some(a) => a,
                                    None => continue,
                                },
                            };
                            for fc in fields {
                                let mut raw = jpath(v, &fc.key);
                                let mut transformed = false;
                                if let Some((op, key_b)) = &fc.fold {
                                    raw = fold_value(raw, jpath(v, key_b), *op);
                                } else if let Some(ref mag_key) = src.flux_from_mag
                                    && fc.key == *mag_key
                                {
                                    raw = raw.map(|r| 10.0f64.powf(-0.4 * r));
                                    transformed = true;
                                }
                                let val = match raw {
                                    Some(vv) => vv,
                                    None => continue,
                                };
                                if !val.is_finite() {
                                    continue;
                                }
                                let mut eff_fc = (*fc).clone();
                                if transformed {
                                    eff_fc.unit.clear();
                                }
                                channels.push((
                                    Channel {
                                        z: 0.0,
                                        freq: 0.0,
                                        bin_width: 0.0,
                                        epoch: row_epoch,
                                        station_code: None,
                                        position: position.clone(),
                                        name: fc.name.clone(),
                                        value: val,
                                    },
                                    eff_fc,
                                ));
                            }
                        }
                    }
                }
            }
            Extract::CmrPolygon {
                arr_path,
                fields,
                epoch_key,
                alt_key,
                val_key,
            } => {
                if let Some(ref j) = parsed_json
                    && let Some(JsonVal::Arr(arr)) = jpath_val(j, arr_path)
                {
                    for v in arr.iter() {
                        let polys = match jpath_val(v, "polygons") {
                            Some(JsonVal::Arr(p)) => p,
                            _ => continue,
                        };
                        let mut vertices: Vec<(f64, f64)> = Vec::new();
                        for ring_list in polys {
                            if let JsonVal::Arr(rings) = ring_list {
                                for ring_str_val in rings {
                                    if let JsonVal::Str(s) = ring_str_val {
                                        let nums: Vec<f64> = s
                                            .split_whitespace()
                                            .filter_map(|n| n.parse().ok())
                                            .collect();
                                        for pair in nums.chunks(2) {
                                            if pair.len() == 2 {
                                                vertices.push((pair[1], pair[0]));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if vertices.is_empty() {
                            continue;
                        }
                        let epoch = if epoch_key.is_empty() {
                            continue;
                        } else if let Some(ev) = jpath_val(v, epoch_key) {
                            match ev {
                                JsonVal::Str(s) => {
                                    if let Some(t) = parse_iso_tdb(s, lsk) {
                                        t
                                    } else {
                                        continue;
                                    }
                                }
                                JsonVal::Num(n) => match lsk.unix_to_tdb(*n) {
                                    Some(t) => t,
                                    None => continue,
                                },
                                _ => continue,
                            }
                        } else {
                            continue;
                        };
                        let alt = match alt_key {
                            k if k.is_empty() => continue,
                            _ => match jpath(v, alt_key) {
                                Some(a) => a,
                                None => continue,
                            },
                        };
                        for fc in fields {
                            if !val_key.is_empty() && fc.name != *val_key {
                                continue;
                            }
                            let mut raw = jpath(v, &fc.key);
                            let mut transformed = false;
                            if let Some(ref mag_key) = src.flux_from_mag
                                && fc.key == *mag_key
                            {
                                raw = raw.map(|r| 10.0f64.powf(-0.4 * r));
                                transformed = true;
                            }
                            let val = match raw {
                                Some(vv) => vv,
                                None => continue,
                            };
                            if !val.is_finite() {
                                continue;
                            }
                            let mut eff_fc = (*fc).clone();
                            if transformed {
                                eff_fc.unit.clear();
                            }
                            for (lon, lat) in vertices.iter() {
                                channels.push((
                                    Channel {
                                        z: 0.0,
                                        freq: 0.0,
                                        bin_width: 0.0,
                                        epoch,
                                        station_code: None,
                                        position: Position::Surface {
                                            body_name: frame_body_name(&src.frame),
                                            lat: *lat,
                                            lon: *lon,
                                            alt,
                                        },
                                        name: fc.name.clone(),
                                        value: val,
                                    },
                                    eff_fc.clone(),
                                ));
                            }
                        }
                    }
                }
            }
            Extract::CelestialPolygon {
                arr_path,
                radius,
                fields,
                epoch_key,
                val_key,
            } => {
                if let Some(ref j) = parsed_json
                    && let Some(JsonVal::Arr(arr)) = jpath_val(j, arr_path)
                {
                    for v in arr.iter() {
                        let geom = match jpath_val(v, "geometry") {
                            Some(g) => g,
                            None => continue,
                        };
                        let coords = match jpath_val(geom, "coordinates") {
                            Some(JsonVal::Arr(c)) => c,
                            _ => continue,
                        };
                        let vertices = flatten_geojson_coords(coords);
                        if vertices.is_empty() || *radius <= 0.0 {
                            continue;
                        }
                        let row_epoch = if !epoch_key.is_empty() {
                            match jpath(v, epoch_key) {
                                Some(ev) => ev,
                                None => continue,
                            }
                        } else {
                            continue;
                        };
                        for fc in fields {
                            if !val_key.is_empty() && fc.name != *val_key {
                                continue;
                            }
                            let mut raw = jpath(v, &fc.key);
                            let mut transformed = false;
                            if let Some(ref mag_key) = src.flux_from_mag
                                && fc.key == *mag_key
                            {
                                raw = raw.map(|r| 10.0f64.powf(-0.4 * r));
                                transformed = true;
                            }
                            let val = match raw {
                                Some(vv) => vv,
                                None => continue,
                            };
                            if !val.is_finite() {
                                continue;
                            }
                            let mut eff_fc = (*fc).clone();
                            if transformed {
                                eff_fc.unit.clear();
                            }
                            for (ra_deg, dec_deg, _z) in &vertices {
                                let ra = ra_deg.to_radians();
                                let dec = dec_deg.to_radians();
                                let (sa, ca) = ra.sin_cos();
                                let (sd, cd) = dec.sin_cos();
                                let p = [cd * ca * radius, cd * sa * radius, sd * radius];
                                channels.push((
                                    Channel {
                                        z: 0.0,
                                        freq: 0.0,
                                        bin_width: 0.0,
                                        epoch: row_epoch,
                                        station_code: None,
                                        position: Position::StateVector {
                                            p,
                                            v: [0.0, 0.0, 0.0],
                                            track: false,
                                        },
                                        name: fc.name.clone(),
                                        value: val,
                                    },
                                    eff_fc.clone(),
                                ));
                            }
                        }
                    }
                }
            }
            Extract::Rows {
                last_line,
                lat_key,
                lon_key,
                fields,
                tau_key,
                epoch_cols,
                gates,
                bin_s,
                name_prefix,
            } => {
                let text: &str = csv_zip_text.as_deref().unwrap_or(body);
                let position = match &src.frame {
                    Frame::Surface { lat, lon, alt, .. } => Position::Surface {
                        body_name: frame_body_name(&src.frame),
                        lat: *lat,
                        lon: *lon,
                        alt: *alt,
                    },
                    Frame::Barycenter { body_name, scale } => Position::Barycenter {
                        body_name: body_name.clone(),
                        scale: *scale,
                    },
                    Frame::Manifest => Position::Source,
                };
                let csv_mode = src.format == "csv" || src.format == "csv_zip";
                let split_row = |line: &str| -> Vec<String> {
                    if csv_mode {
                        split_csv_line(line)
                    } else {
                        split_data_line(line)
                            .into_iter()
                            .map(|s| s.to_string())
                            .collect()
                    }
                };
                let resolve_col = |key: &str| -> Option<usize> {
                    if let Ok(idx) = key.parse::<usize>() {
                        return Some(idx);
                    }
                    let lines: Vec<&str> = text
                        .lines()
                        .filter(|l| {
                            let t = l.trim();
                            !t.is_empty()
                        })
                        .collect();
                    let exact = lines.iter().find_map(|line| {
                        let s = line
                            .strip_prefix('#')
                            .map(|x| x.trim_start())
                            .unwrap_or(line.trim());
                        split_row(s)
                            .iter()
                            .position(|c| c.eq_ignore_ascii_case(key))
                    });
                    if exact.is_some() {
                        return exact;
                    }
                    lines.iter().find_map(|line| {
                        let s = line
                            .strip_prefix('#')
                            .map(|x| x.trim_start())
                            .unwrap_or(line.trim());
                        split_row(s).iter().position(|c| c.starts_with(key))
                    })
                };
                let lat_col = if lat_key.is_empty() {
                    None
                } else {
                    resolve_col(lat_key)
                };
                let lon_col = if lon_key.is_empty() {
                    None
                } else {
                    resolve_col(lon_key)
                };
                if (!lat_key.is_empty() && lat_col.is_none())
                    || (!lon_key.is_empty() && lon_col.is_none())
                {
                    eprintln!("rows lat/lon col unresolved in {} — rows skipped", src.url);
                    return ExtractResult::Measurements(Vec::new());
                }
                if lat_col.is_some() != lon_col.is_some() {
                    eprintln!(
                        "rows lat/lon in {} — both keys are required, rows skipped",
                        src.url
                    );
                    return ExtractResult::Measurements(Vec::new());
                }
                if (lat_col.is_some() || lon_col.is_some()) && *bin_s > 0 {
                    eprintln!(
                        "rows lat/lon with bin in {} — per-row position and bins are exclusive, rows skipped",
                        src.url
                    );
                    return ExtractResult::Measurements(Vec::new());
                }
                let col_fcs: Vec<(usize, Option<usize>, &FieldConfig)> = fields
                    .iter()
                    .filter_map(|fc| {
                        let idx = resolve_col(&fc.key)?;
                        let idx_b = match &fc.fold {
                            Some((_, kb)) => Some(resolve_col(kb)?),
                            None => None,
                        };
                        Some((idx, idx_b, fc))
                    })
                    .collect();
                let tau_col = if tau_key.is_empty() {
                    None
                } else {
                    resolve_col(tau_key)
                };
                let epoch_iso: Option<usize> = if epoch_cols.len() == 1 {
                    let idx = resolve_col(&epoch_cols[0]);
                    if idx.is_none() {
                        eprintln!("rows epoch col unresolved in {} — rows skipped", src.url);
                        return ExtractResult::Measurements(Vec::new());
                    }
                    idx
                } else {
                    None
                };
                let epoch_idxs: Option<Vec<Option<usize>>> = if epoch_cols.len() < 2 {
                    None
                } else {
                    let resolved: Vec<Option<usize>> = epoch_cols
                        .iter()
                        .enumerate()
                        .map(|(k, c)| {
                            if k >= 3 && c == "0" {
                                None
                            } else {
                                resolve_col(c)
                            }
                        })
                        .collect();
                    if resolved[..resolved.len().min(3)]
                        .iter()
                        .any(|i| i.is_none())
                    {
                        eprintln!("rows epoch cols unresolved in {} — rows skipped", src.url);
                        return ExtractResult::Measurements(Vec::new());
                    }
                    Some(resolved)
                };
                let lines: Vec<&str> = if *last_line {
                    text.lines()
                        .rev()
                        .find(|l| {
                            let t = l.trim();
                            !t.is_empty() && !t.starts_with('#')
                        })
                        .into_iter()
                        .collect()
                } else {
                    text.lines()
                        .filter(|l| {
                            let t = l.trim();
                            !t.is_empty() && !t.starts_with('#')
                        })
                        .collect()
                };
                let position_for_row = |cols: &[String]| -> Option<Position> {
                    let (Some(li), Some(oi)) = (lat_col, lon_col) else {
                        return Some(position.clone());
                    };
                    let lat = cols
                        .get(li)
                        .and_then(|s| s.trim().trim_matches('"').parse::<f64>().ok())?;
                    let lon = cols
                        .get(oi)
                        .and_then(|s| s.trim().trim_matches('"').parse::<f64>().ok())?;
                    if !lat.is_finite() || !lon.is_finite() {
                        return None;
                    }
                    let Frame::Surface { alt, .. } = &src.frame else {
                        return None;
                    };
                    Some(Position::Surface {
                        body_name: frame_body_name(&src.frame),
                        lat,
                        lon,
                        alt: *alt,
                    })
                };
                let row_vals = |line: &str| -> (Option<f64>, Vec<(usize, f64)>, Option<Position>) {
                    let cols = split_row(line.trim());
                    let row_pos = position_for_row(&cols);
                    let epoch: Option<f64> = match &epoch_idxs {
                        Some(idxs) => {
                            let mut n = [0i64; 5];
                            let mut read = true;
                            for (k, i) in idxs.iter().enumerate() {
                                match i {
                                    Some(idx) => {
                                        match cols
                                            .get(*idx)
                                            .and_then(|s| s.trim().parse::<i64>().ok())
                                        {
                                            Some(v) => n[k] = v,
                                            None => {
                                                read = false;
                                                break;
                                            }
                                        }
                                    }
                                    None => n[k] = 0,
                                }
                            }
                            if !read {
                                None
                            } else {
                                match crate::lsk::days_from_civil(n[0], n[1], n[2]) {
                                    Some(days) => {
                                        let unix = days as f64 * 86400.0
                                            + n[3] as f64 * 3600.0
                                            + n[4] as f64 * 60.0;
                                        lsk.unix_to_tdb(unix)
                                    }
                                    None => None,
                                }
                            }
                        }
                        None => match epoch_iso {
                            Some(idx) => cols.get(idx).and_then(|s| {
                                let t = s.trim().trim_matches('"');
                                parse_iso_tdb(t, lsk).or_else(|| {
                                    t.parse::<f64>().ok().and_then(|u| lsk.unix_to_tdb(u))
                                })
                            }),
                            None => Some(now),
                        },
                    };
                    let mut vals = Vec::new();
                    for (fi, (idx, idx_b, fc)) in col_fcs.iter().enumerate() {
                        let raw = cols
                            .get(*idx)
                            .and_then(|s| s.trim().trim_matches('"').parse::<f64>().ok());
                        let val = match (&fc.fold, idx_b) {
                            (Some((op, _)), Some(bi)) => fold_value(
                                raw,
                                cols.get(*bi)
                                    .and_then(|s| s.trim().trim_matches('"').parse::<f64>().ok()),
                                *op,
                            ),
                            _ => raw,
                        };
                        let val = match val {
                            Some(v) => v,
                            None => continue,
                        };
                        if let Some((_, lo, hi)) = gates.iter().find(|(k, _, _)| *k == fc.key)
                            && !(val >= *lo && val < *hi)
                        {
                            continue;
                        }
                        if !val.is_finite() {
                            continue;
                        }
                        vals.push((fi, val));
                    }
                    (epoch, vals, row_pos)
                };
                let series_name = |fc: &FieldConfig| -> String {
                    if name_prefix.is_empty() {
                        fc.name.clone()
                    } else {
                        format!("{}_{}", name_prefix, fc.name)
                    }
                };
                if *bin_s > 0 {
                    let dt = *bin_s as f64;
                    let mut sums: Vec<std::collections::BTreeMap<i64, (f64, u32)>> = col_fcs
                        .iter()
                        .map(|_| std::collections::BTreeMap::new())
                        .collect();
                    for line in lines {
                        let (epoch, vals, _) = row_vals(line);
                        let Some(epoch) = epoch else { continue };
                        let bin_i = (epoch / dt).floor() as i64;
                        for (fi, v) in vals {
                            let e = sums[fi].entry(bin_i).or_insert((0.0, 0));
                            e.0 += v;
                            e.1 += 1;
                        }
                    }
                    for (fi, (_, _, fc)) in col_fcs.iter().enumerate() {
                        for (bin_i, (s, c)) in &sums[fi] {
                            channels.push((
                                Channel {
                                    z: 0.0,
                                    freq: 0.0,
                                    bin_width: 0.0,
                                    epoch: *bin_i as f64 * dt,
                                    station_code: None,
                                    position: position.clone(),
                                    name: series_name(fc),
                                    value: s / *c as f64,
                                },
                                (*fc).clone(),
                            ));
                        }
                    }
                } else {
                    for line in lines {
                        let (epoch, vals, row_pos) = row_vals(line);
                        let Some(epoch) = epoch else { continue };
                        let Some(row_pos) = row_pos else { continue };
                        let row_tau: Option<f64> =
                            match tau_col {
                                None => None,
                                Some(idx) => {
                                    match line.split_whitespace().nth(idx).and_then(|s| {
                                        s.trim().trim_matches('"').parse::<f64>().ok()
                                    }) {
                                        Some(t) if t > 0.0 => Some(t),
                                        Some(_) => continue,
                                        None => None,
                                    }
                                }
                            };
                        for (fi, val) in vals {
                            let (_, _, fc) = col_fcs[fi];
                            let mut eff_fc = (*fc).clone();
                            if let Some(t) = row_tau {
                                eff_fc.tau = t;
                            }
                            channels.push((
                                Channel {
                                    z: 0.0,
                                    freq: 0.0,
                                    bin_width: 0.0,
                                    epoch,
                                    station_code: None,
                                    position: row_pos.clone(),
                                    name: series_name(fc),
                                    value: val,
                                },
                                eff_fc,
                            ));
                        }
                    }
                }
            }
            Extract::KeplerMap {
                arr_path,
                a_key,
                e_key,
                i_key,
                om_key,
                w_key,
                ma_key,
                epoch_key,
                q_key,
                tp_key,
                fields,
            } => {
                if let Some(ref j) = parsed_json
                    && let Some(JsonVal::Arr(arr)) = jpath_val(j, arr_path)
                {
                    let jd_now = tdb_to_jd(now);
                    for v in arr.iter() {
                        let (Some(e_val), Some(i_val), Some(om_val), Some(w_val)) = (
                            jpath(v, e_key),
                            jpath(v, i_key),
                            jpath(v, om_key),
                            jpath(v, w_key),
                        ) else {
                            continue;
                        };
                        if !(0.0..1.0).contains(&e_val) {
                            continue;
                        }
                        let (Some(epoch_val),) = (jpath(v, epoch_key),) else {
                            continue;
                        };
                        let a_au = if !a_key.is_empty() {
                            match jpath(v, a_key) {
                                Some(a) if a > 0.0 => a,
                                _ => continue,
                            }
                        } else if !q_key.is_empty() {
                            match jpath(v, q_key) {
                                Some(q) if q > 0.0 => q / (1.0 - e_val),
                                _ => continue,
                            }
                        } else {
                            continue;
                        };
                        let ma_deg = if !ma_key.is_empty() {
                            match jpath(v, ma_key) {
                                Some(m) => m,
                                None => continue,
                            }
                        } else if !tp_key.is_empty() {
                            let Some(tp) = jpath(v, tp_key) else {
                                continue;
                            };
                            let n_deg_day = GAUSS_K / (a_au * a_au * a_au).sqrt()
                                * (180.0 / std::f64::consts::PI);
                            n_deg_day * (epoch_val - tp)
                        } else {
                            continue;
                        };
                        let (p, vel) = match crate::kepler::elements_to_icrs_state(
                            &crate::kepler::KeplerElements {
                                a_au,
                                e: e_val,
                                incl_deg: i_val,
                                node_deg: om_val,
                                peri_deg: w_val,
                                ma_deg,
                                epoch_jd: epoch_val,
                                t_jd: jd_now,
                            },
                        ) {
                            Some(st) => st,
                            None => continue,
                        };
                        for fc in fields {
                            let mut raw = jpath(v, &fc.key);
                            let mut transformed = false;
                            if let Some(ref mag_key) = src.flux_from_mag
                                && fc.key == *mag_key
                            {
                                raw = raw.map(|r| 10.0f64.powf(-0.4 * r));
                                transformed = true;
                            }
                            let val = match raw {
                                Some(vv) => vv,
                                None => continue,
                            };
                            if !val.is_finite() {
                                continue;
                            }
                            let mut eff_fc = (*fc).clone();
                            if transformed {
                                eff_fc.unit.clear();
                            }
                            channels.push((
                                Channel {
                                    z: 0.0,
                                    freq: 0.0,
                                    bin_width: 0.0,
                                    epoch: now,
                                    station_code: None,
                                    position: Position::StateVector {
                                        p,
                                        v: vel,
                                        track: false,
                                    },
                                    name: fc.name.clone(),
                                    value: val,
                                },
                                eff_fc,
                            ));
                        }
                    }
                }
            }
            Extract::CelestialMap {
                arr_path,
                ra_key,
                dec_key,
                dist_key,
                dist_scale,
                plx_key,
                z_key,
                pmra_key,
                pmdec_key,
                rv_key,
                rv_scale,
                epoch_key,
                epoch_mjd,
                fields,
                tau_key,
            } => {
                let default_epoch = if let Some(e) = src.catalog_epoch {
                    e
                } else {
                    now
                };
                if let Some(ref j) = parsed_json
                    && let Some(JsonVal::Arr(arr)) = jpath_val(j, arr_path)
                {
                    for v in arr.iter() {
                        let (Some(ra_deg), Some(dec_deg)) = (jpath(v, ra_key), jpath(v, dec_key))
                        else {
                            continue;
                        };
                        let dist_scaled = |v: &JsonVal| -> Option<f64> {
                            let scale = (*dist_scale)?;
                            if dist_key.is_empty() {
                                return None;
                            }
                            match jpath(v, dist_key) {
                                Some(dd) if dd.is_finite() && dd > 0.0 => Some(dd * scale),
                                _ => None,
                            }
                        };
                        let d = if !plx_key.is_empty() {
                            match jpath(v, plx_key) {
                                Some(plx) if plx.is_finite() && plx > 0.0 => {
                                    PARSEC_M * 1000.0 / plx
                                }
                                _ => {
                                    if !z_key.is_empty() {
                                        match jpath(v, z_key) {
                                            Some(z) if z.is_finite() && z > 0.0 => {
                                                z * C_LIGHT / HUBBLE_H0
                                            }
                                            _ => match dist_scaled(v) {
                                                Some(dd) => dd,
                                                None => continue,
                                            },
                                        }
                                    } else {
                                        match dist_scaled(v) {
                                            Some(dd) => dd,
                                            None => continue,
                                        }
                                    }
                                }
                            }
                        } else if !dist_key.is_empty() {
                            match dist_scaled(v) {
                                Some(dd) => dd,
                                None => {
                                    if !z_key.is_empty() {
                                        match jpath(v, z_key) {
                                            Some(z) if z.is_finite() && z > 0.0 => {
                                                z * C_LIGHT / HUBBLE_H0
                                            }
                                            _ => continue,
                                        }
                                    } else {
                                        continue;
                                    }
                                }
                            }
                        } else if !z_key.is_empty() {
                            match jpath(v, z_key) {
                                Some(z) if z.is_finite() && z > 0.0 => z * C_LIGHT / HUBBLE_H0,
                                _ => continue,
                            }
                        } else {
                            continue;
                        };
                        let zval = if z_key.is_empty() {
                            0.0
                        } else {
                            match jpath(v, z_key) {
                                Some(z) if z.is_finite() && z > 0.0 => z,
                                _ => continue,
                            }
                        };
                        let ra = ra_deg.to_radians();
                        let dec = dec_deg.to_radians();
                        let (sa, ca) = ra.sin_cos();
                        let (sd, cd) = dec.sin_cos();
                        let p_hat = [cd * ca, cd * sa, sd];
                        let p = [p_hat[0] * d, p_hat[1] * d, p_hat[2] * d];
                        let mu_a = if pmra_key.is_empty() {
                            None
                        } else {
                            jpath(v, pmra_key)
                                .filter(|x| x.is_finite())
                                .map(|v| v * MAS_YR_TO_RAD_S)
                        };
                        let mu_d = if pmdec_key.is_empty() {
                            None
                        } else {
                            jpath(v, pmdec_key)
                                .filter(|x| x.is_finite())
                                .map(|v| v * MAS_YR_TO_RAD_S)
                        };
                        let vr = if rv_key.is_empty() {
                            None
                        } else {
                            match rv_scale {
                                Some(scale) => jpath(v, rv_key)
                                    .filter(|x| x.is_finite())
                                    .map(|v| v * scale),
                                None => None,
                            }
                        };
                        let a_hat = [-sa, ca, 0.0];
                        let d_hat = [-sd * ca, -sd * sa, cd];
                        let vel = [
                            d * (mu_a.map_or(0.0, |m| m * a_hat[0])
                                + mu_d.map_or(0.0, |m| m * d_hat[0]))
                                + vr.map_or(0.0, |v| v * p_hat[0]),
                            d * (mu_a.map_or(0.0, |m| m * a_hat[1])
                                + mu_d.map_or(0.0, |m| m * d_hat[1]))
                                + vr.map_or(0.0, |v| v * p_hat[1]),
                            d * (mu_a.map_or(0.0, |m| m * a_hat[2])
                                + mu_d.map_or(0.0, |m| m * d_hat[2]))
                                + vr.map_or(0.0, |v| v * p_hat[2]),
                        ];
                        let sample_epoch = if !epoch_key.is_empty() {
                            match jpath(v, epoch_key) {
                                Some(v) if *epoch_mjd => match crate::maxi::mjd_to_tdb(v, lsk) {
                                    Some(t) => t,
                                    None => continue,
                                },
                                Some(v) => v,
                                None => continue,
                            }
                        } else {
                            default_epoch
                        };
                        let row_tau: Option<f64> = if tau_key.is_empty() {
                            None
                        } else {
                            match jpath(v, tau_key) {
                                Some(t) if t > 0.0 => Some(t),
                                Some(_) => continue,
                                None => None,
                            }
                        };
                        for fc in fields {
                            let mut raw: Option<f64> = jpath(v, &fc.key);
                            let mut transformed = false;
                            if let Some((op, key_b)) = &fc.fold {
                                raw = fold_value(raw, jpath(v, key_b), *op);
                            } else if let Some(ref mag_field) = src.abs_mag_from {
                                if fc.name == *mag_field {
                                    raw = raw.map(|v| {
                                        let dist_pc = d / PARSEC_M;
                                        let abs_m = v - 5.0 * (dist_pc / 10.0).log10();
                                        10.0f64.powf(-0.4 * abs_m)
                                    });
                                    transformed = true;
                                }
                            } else if let Some(ref mag_key) = src.flux_from_mag
                                && fc.key == *mag_key
                            {
                                raw = raw.map(|r| 10.0f64.powf(-0.4 * r));
                                transformed = true;
                            }
                            let val = match raw {
                                Some(vv) => vv,
                                None => continue,
                            };
                            if !val.is_finite() {
                                continue;
                            }
                            let mut eff_fc = (*fc).clone();
                            if transformed {
                                eff_fc.unit.clear();
                            }
                            if let Some(t) = row_tau {
                                eff_fc.tau = t;
                            }
                            channels.push((
                                Channel {
                                    z: zval,
                                    freq: 0.0,
                                    bin_width: 0.0,
                                    epoch: sample_epoch,
                                    station_code: None,
                                    position: Position::StateVector {
                                        p,
                                        v: vel,
                                        track: false,
                                    },
                                    name: fc.name.clone(),
                                    value: val,
                                },
                                eff_fc,
                            ));
                        }
                    }
                }
            }
            Extract::GeojsonEvents {
                mag_key,
                min_mag,
                outputs,
                tau,
                absorption,
                advection,
                mag_type_key,
            } => {
                if outputs.len() >= 2
                    && let Some(ref j) = parsed_json
                    && let JsonVal::Obj(root) = j
                    && let Some(JsonVal::Arr(features)) = root.get("features")
                {
                    for feat in features {
                        if let JsonVal::Obj(f) = feat {
                            let mut elo = 0.0;
                            let mut ela = 0.0;
                            let mut ed = 0.0;
                            let mut mag: Option<f64> = None;
                            let mut valid = false;
                            if let Some(JsonVal::Obj(geom)) = f.get("geometry")
                                && let Some(JsonVal::Arr(c)) = geom.get("coordinates")
                                && c.len() >= 3
                            {
                                if let JsonVal::Num(n) = c[0] {
                                    elo = n;
                                }
                                if let JsonVal::Num(n) = c[1] {
                                    ela = n;
                                }
                                if let JsonVal::Num(n) = c[2] {
                                    ed = n;
                                }
                                valid = true;
                            }
                            if valid && let Some(props) = f.get("properties") {
                                if let Some(m) = jnum(props, mag_key)
                                    && m.is_finite()
                                {
                                    mag = Some(m);
                                }
                                if !mag_type_key.is_empty()
                                    && let Some(t) = jstr(props, mag_type_key)
                                    && !is_moment_magnitude(&t)
                                {
                                    continue;
                                }
                            }
                            if let Some(mag) = mag
                                && mag >= *min_mag
                            {
                                channels.push((
                                    Channel {
                                        z: 0.0,
                                        freq: 0.0,
                                        bin_width: 0.0,
                                        epoch: now,
                                        station_code: None,
                                        position: Position::Surface {
                                            body_name: frame_body_name(&src.frame),
                                            lat: ela,
                                            lon: elo,
                                            alt: -ed * 1000.0,
                                        },
                                        name: outputs[0].clone(),
                                        value: mag,
                                    },
                                    FieldConfig {
                                        key: outputs[0].clone(),
                                        name: outputs[0].clone(),
                                        kernel: 0,
                                        force: 3,
                                        tau: *tau,
                                        absorption: *absorption,
                                        advection: *advection,
                                        unit: "Mw".to_string(),
                                        freq: 0.0,
                                        bin_width: 0.0,
                                        fold: None,
                                        aperture: Aperture::None,
                                    },
                                ));
                                channels.push((
                                    Channel {
                                        z: 0.0,
                                        freq: 0.0,
                                        bin_width: 0.0,
                                        epoch: now,
                                        station_code: None,
                                        position: Position::Surface {
                                            body_name: frame_body_name(&src.frame),
                                            lat: ela,
                                            lon: elo,
                                            alt: -ed * 1000.0,
                                        },
                                        name: outputs[1].clone(),
                                        value: ed * 1000.0,
                                    },
                                    FieldConfig {
                                        key: outputs[1].clone(),
                                        name: outputs[1].clone(),
                                        kernel: 0,
                                        force: 3,
                                        tau: *tau,
                                        absorption: *absorption,
                                        advection: *advection,
                                        unit: String::new(),
                                        freq: 0.0,
                                        bin_width: 0.0,
                                        fold: None,
                                        aperture: Aperture::None,
                                    },
                                ));
                            }
                        }
                    }
                }
            }
            Extract::QuakeMlEvents {
                outputs,
                tau,
                absorption,
                advection,
            } => {
                if outputs.len() >= 2 {
                    for ev in crate::archivar::quakeml::parse_quakeml(body) {
                        let Some(epoch) = lsk.unix_to_tdb(ev.time) else {
                            continue;
                        };
                        let position = Position::Surface {
                            body_name: frame_body_name(&src.frame),
                            lat: ev.lat,
                            lon: ev.lon,
                            alt: 0.0,
                        };
                        if let Some(m0) = ev.scalar_moment_nm {
                            channels.push((
                                Channel {
                                    z: ev.depth_km,
                                    freq: 0.0,
                                    bin_width: 0.0,
                                    epoch,
                                    station_code: None,
                                    position: position.clone(),
                                    name: outputs[0].clone(),
                                    value: m0,
                                },
                                FieldConfig {
                                    key: outputs[0].clone(),
                                    name: outputs[0].clone(),
                                    kernel: 1,
                                    force: 3,
                                    tau: *tau,
                                    absorption: *absorption,
                                    advection: *advection,
                                    unit: "N m".to_string(),
                                    freq: 0.0,
                                    bin_width: 0.0,
                                    fold: None,
                                    aperture: Aperture::None,
                                },
                            ));
                        }
                        let mww = match ev.mag_type.as_deref() {
                            Some(t) if is_moment_magnitude(t) => ev.magnitude,
                            _ => None,
                        };
                        if let Some(mww) = mww {
                            channels.push((
                                Channel {
                                    z: ev.depth_km,
                                    freq: 0.0,
                                    bin_width: 0.0,
                                    epoch,
                                    position,
                                    station_code: src.station_code.clone(),
                                    name: outputs[1].clone(),
                                    value: mww,
                                },
                                FieldConfig {
                                    key: outputs[1].clone(),
                                    name: outputs[1].clone(),
                                    kernel: 3,
                                    force: 4,
                                    tau: *tau,
                                    absorption: *absorption,
                                    advection: *advection,
                                    unit: "Mw".to_string(),
                                    freq: 0.0,
                                    bin_width: 0.0,
                                    fold: None,
                                    aperture: Aperture::None,
                                },
                            ));
                        }
                    }
                }
            }
            Extract::Hapi(pairs) => {
                if let Some(ref j) = parsed_json
                    && let JsonVal::Obj(root) = j
                    && let Some(JsonVal::Arr(data)) = root.get("data")
                {
                    let mut col: HashMap<String, usize> = HashMap::new();
                    let mut fill_of: HashMap<String, f64> = HashMap::new();
                    let mut has_params = false;
                    if let Some(JsonVal::Arr(params)) = root.get("parameters") {
                        for (i, p) in params.iter().enumerate() {
                            if let JsonVal::Obj(po) = p
                                && let Some(JsonVal::Str(nn)) = po.get("name")
                            {
                                col.insert(nn.clone(), i);
                                has_params = true;
                                if let Some(fv) = po.get("fill").and_then(scalar_of) {
                                    fill_of.insert(nn.clone(), fv);
                                }
                            }
                        }
                    }
                    for (k, v) in &src.hapi_fill {
                        fill_of.entry(k.clone()).or_insert(*v);
                    }
                    if !has_params {
                        if pairs.len() == 1 && !pairs[0].0.contains('.') {
                            if let Some(JsonVal::Arr(row)) = data.last()
                                && let Some(val) = row.last().and_then(scalar_of)
                                && fill_of.get(pairs[0].0.as_str()).is_none_or(|&f| val != f)
                            {
                                extracted.insert(pairs[0].1.clone(), val);
                            }
                            continue;
                        }
                        let mut next_col = 0usize;
                        for (param, _) in pairs.iter() {
                            let base = param.split('.').next().unwrap_or(param);
                            if !col.contains_key(base) {
                                next_col += 1;
                                col.insert(base.to_string(), next_col);
                            }
                        }
                    }
                    if let Some(last_row) = data.last()
                        && let JsonVal::Arr(row) = last_row
                    {
                        for (param, name) in pairs {
                            let (base, comp) = match param.rfind('.') {
                                Some(dot)
                                    if param[dot + 1..].chars().all(|c| c.is_ascii_digit()) =>
                                {
                                    (&param[..dot], param[dot + 1..].parse::<usize>().ok())
                                }
                                _ => (param.as_str(), None),
                            };
                            if let Some(&idx) = col.get(base) {
                                let v = match comp {
                                    Some(i) => row.get(idx).and_then(|cell| {
                                        if let JsonVal::Arr(a) = cell {
                                            a.get(i).and_then(scalar_of)
                                        } else {
                                            None
                                        }
                                    }),
                                    None => row.get(idx).and_then(scalar_of),
                                };
                                if let Some(val) = v
                                    && fill_of.get(base).is_none_or(|&f| val != f)
                                {
                                    extracted.insert(name.clone(), val);
                                }
                            }
                        }
                    }
                }
            }
            Extract::Alerce(_) => {}
            Extract::Volume { .. } => {}
        }
    }
    if !extracted.is_empty() {
        for (name, val) in &extracted {
            let fc = effective_extracts.iter().find_map(|ext| match ext {
                Extract::Field(fc)
                | Extract::First(fc, _)
                | Extract::Last(fc, _)
                | Extract::Count(fc)
                | Extract::LastRow(fc)
                | Extract::ObjLast(fc)
                | Extract::Path(fc)
                | Extract::Deep(fc)
                | Extract::Regex(fc) => {
                    if fc.name == *name && fc.tau > 0.0 {
                        Some(fc)
                    } else {
                        None
                    }
                }
                _ => None,
            });
            if let Some(fc) = fc {
                let mut raw = Some(*val);
                let mut transformed = false;
                if let Some(ref mag_key) = src.flux_from_mag
                    && fc.key == *mag_key
                {
                    raw = raw.map(|v| 10.0f64.powf(-0.4 * v));
                    transformed = true;
                }
                let val = match raw {
                    Some(v) => v,
                    None => continue,
                };
                if !val.is_finite() {
                    continue;
                }
                let mut eff_fc = fc.clone();
                if transformed {
                    eff_fc.unit.clear();
                }
                channels.push((
                    Channel {
                        z: 0.0,
                        freq: 0.0,
                        bin_width: 0.0,
                        epoch: now,
                        station_code: None,
                        position: Position::Source,
                        name: fc.name.clone(),
                        value: val,
                    },
                    eff_fc,
                ));
            }
        }
    }
    channels.retain(|(c, _)| c.value.is_finite());
    if let Some((from, until)) = src.window {
        channels.retain(|(c, _)| {
            lsk.tdb_to_unix(c.epoch)
                .map(|u| u >= from && u <= until)
                .unwrap_or(false)
        });
    }
    ExtractResult::Measurements(channels)
}

pub fn extract(src: &SourceConfig, body: &str, now: f64, lsk: &LeapSeconds) -> ExtractResult {
    let mut result = extract_raw(src, body, now, lsk);
    match &mut result {
        ExtractResult::Measurements(channels) | ExtractResult::WithEphemeris(channels, _) => {
            for (channel, _) in channels.iter_mut() {
                channel.station_code = src.station_code.clone();
            }
        }
    }
    result
}

pub fn series_epoch_of(el: &JsonVal, lsk: &LeapSeconds) -> Option<f64> {
    match el {
        JsonVal::Obj(map) => {
            for (k, v) in map {
                if !is_time_key(k) {
                    continue;
                }
                match v {
                    JsonVal::Str(s) => {
                        if let Some(t) = parse_iso_tdb(s, lsk) {
                            return Some(t);
                        }
                    }
                    JsonVal::Num(n) => {
                        if let Some(t) = lsk.unix_to_tdb(*n) {
                            return Some(t);
                        }
                    }
                    _ => {}
                }
            }
            None
        }
        JsonVal::Arr(row) => {
            let first = row.first()?;
            match first {
                JsonVal::Str(s) => parse_iso_tdb(s, lsk),
                JsonVal::Num(n) => lsk.unix_to_tdb(*n),
                _ => None,
            }
        }
        _ => None,
    }
}

pub fn is_time_key(k: &str) -> bool {
    let kl = k.to_lowercase();
    kl == "time"
        || kl == "time_tag"
        || kl == "timestamp"
        || kl == "epoch"
        || kl == "t"
        || kl == "date"
        || kl.contains("time")
        || kl.contains("date")
}

fn hapi_csv_body(src: &SourceConfig, body: &str) -> Option<JsonVal> {
    if parse_json(body).is_some() {
        return None;
    }
    let names = hapi_csv::csv_header_fields(body).or_else(|| {
        let mut names = vec!["time".to_string()];
        for ext in &src.extracts {
            if let Extract::Hapi(pairs) = ext {
                for (param, _) in pairs {
                    names.push(param.split('.').next().unwrap_or(param).to_string());
                }
            }
        }
        (names.len() > 1).then_some(names)
    })?;
    hapi_csv::csv_envelope(body, &names, None)
}

pub fn extract_series(src: &SourceConfig, body: &str, lsk: &LeapSeconds) -> Vec<(f64, f64)> {
    let parsed = match src.format.as_str() {
        "votable" => votable_to_json(body),
        "html" => html_to_json(body),
        "tap" => tap_body_to_json(&src.url, body),
        _ => parse_json(body),
    };
    let parsed = match parsed {
        Some(j) => Some(j),
        None if src.extracts.iter().any(|e| matches!(e, Extract::Hapi(_)))
            && matches!(src.format.as_str(), "csv" | "" | "json" | "universal") =>
        {
            hapi_csv_body(src, body)
        }
        other => other,
    };
    let Some(ref j) = parsed else {
        return Vec::new();
    };
    let mut out: Vec<(f64, f64)> = Vec::new();
    for ext in &src.extracts {
        match ext {
            Extract::First(fc, filter) => {
                let JsonVal::Arr(elements) = j else {
                    continue;
                };
                for el in elements {
                    if let Some((fk, fv)) = filter
                        && !row_matches(el, fk, fv)
                    {
                        continue;
                    }
                    let raw = match el {
                        JsonVal::Obj(map) => map.get(&fc.key).and_then(scalar_of),
                        JsonVal::Arr(row) => {
                            if let Ok(idx) = fc.key.parse::<usize>() {
                                row.get(idx).and_then(scalar_of)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    };
                    let (Some(raw), Some(epoch)) = (raw, series_epoch_of(el, lsk)) else {
                        continue;
                    };
                    let Some(val) = convert_to_si(raw, &fc.unit) else {
                        register_unconverted_unit(&fc.unit, &fc.name);
                        continue;
                    };
                    if val.is_finite() {
                        out.push((epoch, val));
                    }
                }
            }
            Extract::Last(fc, filter) => {
                let JsonVal::Arr(elements) = j else {
                    continue;
                };
                for el in elements {
                    if let Some((fk, fv)) = filter
                        && !row_matches(el, fk, fv)
                    {
                        continue;
                    }
                    let raw = match el {
                        JsonVal::Obj(map) => map.get(&fc.key).and_then(scalar_of),
                        JsonVal::Arr(row) => {
                            if let Ok(idx) = fc.key.parse::<usize>() {
                                row.get(idx).and_then(scalar_of)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    };
                    let (Some(raw), Some(epoch)) = (raw, series_epoch_of(el, lsk)) else {
                        continue;
                    };
                    let Some(val) = convert_to_si(raw, &fc.unit) else {
                        register_unconverted_unit(&fc.unit, &fc.name);
                        continue;
                    };
                    if val.is_finite() {
                        out.push((epoch, val));
                    }
                }
            }
            Extract::Path(fc) => {
                let JsonVal::Arr(elements) = j else {
                    continue;
                };
                for el in elements {
                    let raw = jpath(el, &fc.key);
                    let (Some(raw), Some(epoch)) = (raw, series_epoch_of(el, lsk)) else {
                        continue;
                    };
                    let Some(val) = convert_to_si(raw, &fc.unit) else {
                        register_unconverted_unit(&fc.unit, &fc.name);
                        continue;
                    };
                    if val.is_finite() {
                        out.push((epoch, val));
                    }
                }
            }
            Extract::Hapi(pairs) => {
                let JsonVal::Obj(root) = j else {
                    continue;
                };
                let Some(JsonVal::Arr(data)) = root.get("data") else {
                    continue;
                };
                let mut col: HashMap<String, usize> = HashMap::new();
                let mut fill_of: HashMap<String, f64> = HashMap::new();
                if let Some(JsonVal::Arr(params)) = root.get("parameters") {
                    for (i, p) in params.iter().enumerate() {
                        if let JsonVal::Obj(po) = p
                            && let Some(JsonVal::Str(nn)) = po.get("name")
                        {
                            col.insert(nn.clone(), i);
                            if let Some(fv) = po.get("fill").and_then(scalar_of) {
                                fill_of.insert(nn.clone(), fv);
                            }
                        }
                    }
                }
                for (k, v) in &src.hapi_fill {
                    fill_of.entry(k.clone()).or_insert(*v);
                }
                if col.is_empty() {
                    let mut next_col = 0usize;
                    for (param, _) in pairs.iter() {
                        let base = param.split('.').next().unwrap_or(param);
                        if !col.contains_key(base) {
                            next_col += 1;
                            col.insert(base.to_string(), next_col);
                        }
                    }
                }
                for row in data {
                    let JsonVal::Arr(cells) = row else {
                        continue;
                    };
                    let Some(epoch) = series_epoch_of(row, lsk) else {
                        continue;
                    };
                    for (param, name) in pairs {
                        let (base, comp) = match param.rfind('.') {
                            Some(dot) if param[dot + 1..].chars().all(|c| c.is_ascii_digit()) => {
                                (&param[..dot], param[dot + 1..].parse::<usize>().ok())
                            }
                            _ => (param.as_str(), None),
                        };
                        let Some(&idx) = col.get(base) else {
                            continue;
                        };
                        let v = match comp {
                            Some(i) => cells.get(idx).and_then(|cell| {
                                if let JsonVal::Arr(a) = cell {
                                    a.get(i).and_then(scalar_of)
                                } else {
                                    None
                                }
                            }),
                            None => cells.get(idx).and_then(scalar_of),
                        };
                        let Some(raw) = v else {
                            continue;
                        };
                        if fill_of.get(base).is_some_and(|&f| raw == f) {
                            continue;
                        }
                        let Some(fc) = src.extracts.iter().find_map(|e| match e {
                            Extract::Field(fc) if fc.name == *name => Some(fc),
                            _ => None,
                        }) else {
                            continue;
                        };
                        let Some(val) = convert_to_si(raw, &fc.unit) else {
                            register_unconverted_unit(&fc.unit, &fc.name);
                            continue;
                        };
                        if val.is_finite() {
                            out.push((epoch, val));
                        }
                    }
                }
            }
            _ => {}
        }
    }
    out
}

#[cfg(test)]
mod votable_tests {
    use super::*;

    #[test]
    fn votable_tabledata_reads_fields_and_rows() {
        let body = r#"<?xml version="1.0"?>
<VOTABLE version="1.3"><RESOURCE><TABLE>
<FIELD name="ra" datatype="double" unit="deg"/>
<FIELD name="dec" datatype="double" unit="deg"/>
<FIELD name="flux" datatype="float" unit="Jy"/>
<DATA><TABLEDATA>
<TR><TD>10.5</TD><TD>41.0</TD><TD>0.25</TD></TR>
<TR><TD>10.6</TD><TD>40.9</TD><TD/></TR>
</TABLEDATA></DATA>
</TABLE></RESOURCE></VOTABLE>"#;
        let json = votable_to_json(body).unwrap();
        let rows = match &json {
            JsonVal::Arr(a) => a,
            _ => panic!("not an array"),
        };
        assert_eq!(rows.len(), 2);
        assert_eq!(jnum(&rows[0], "ra"), Some(10.5));
        assert_eq!(jnum(&rows[0], "flux"), Some(0.25));
        assert_eq!(jnum(&rows[1], "dec"), Some(40.9));
        assert_eq!(jnum(&rows[1], "flux"), None);
    }

    #[test]
    fn tap_format_reads_the_url_parameter() {
        assert_eq!(
            tap_format("http://x/sync?REQUEST=doQuery&FORMAT=votable&QUERY=Q").as_deref(),
            Some("votable")
        );
        assert_eq!(
            tap_format("http://x/sync?FORMAT=CSV").as_deref(),
            Some("csv")
        );
        assert_eq!(
            tap_format("http://x/sync?FORMAT=votable/td").as_deref(),
            Some("votable/td")
        );
        assert_eq!(tap_format("http://x/sync?REQUEST=doQuery").as_deref(), None);
    }

    #[test]
    fn votable_without_tabledata_is_none() {
        let body = r#"<VOTABLE><RESOURCE><TABLE><FIELD name="ra"/><DATA><BINARY/></DATA></TABLE></RESOURCE></VOTABLE>"#;
        assert!(votable_to_json(body).is_none());
    }

    #[test]
    fn votable_escaped_string_cell_reads() {
        let body = r#"<VOTABLE><RESOURCE><TABLE><FIELD name="name"/><DATA><TABLEDATA><TR><TD>a&amp;b</TD></TR></TABLEDATA></DATA></TABLE></RESOURCE></VOTABLE>"#;
        let json = votable_to_json(body).unwrap();
        let rows = match &json {
            JsonVal::Arr(a) => a,
            _ => panic!("not an array"),
        };
        match &rows[0] {
            JsonVal::Obj(m) => match m.get("name") {
                Some(JsonVal::Str(s)) => assert_eq!(s, "a&b"),
                other => panic!("name is not the escaped string: {other:?}"),
            },
            _ => panic!("not an object"),
        }
    }

    #[test]
    fn votable_alma_obscore_body_reads() {
        let body = r#"<?xml version="1.0" encoding="UTF-8"?>
<VOTABLE xmlns="http://www.ivoa.net/xml/VOTable/v1.3" version="1.4">
  <RESOURCE type="results">
    <INFO name="QUERY_STATUS" value="OK" />
    <TABLE>
      <FIELD name="s_ra" datatype="double" ucd="pos.eq.ra" unit="deg" utype="obscore:Char.SpatialAxis.Coverage.Location.Coord.Position2D.Value2.C1" xtype="adql:DOUBLE">
        <DESCRIPTION>RA of central coordinates</DESCRIPTION>
      </FIELD>
      <FIELD name="s_dec" datatype="double" ucd="pos.eq.dec" unit="deg" utype="obscore:Char.SpatialAxis.Coverage.Location.Coord.Position2D.Value2.C2" xtype="adql:DOUBLE">
        <DESCRIPTION>DEC of central coordinates</DESCRIPTION>
      </FIELD>
      <FIELD name="target_name" datatype="char" arraysize="256*" ucd="meta.id;src" utype="obscore:Target.Name">
        <DESCRIPTION>name of intended target</DESCRIPTION>
      </FIELD>
      <FIELD name="proposal_id" datatype="char" arraysize="64*" ucd="meta.id;obs.proposal" utype="obscore:Provenance.Proposal.identifier">
        <DESCRIPTION>Identifier of proposal to which NO observation belongs.</DESCRIPTION>
      </FIELD>
      <DATA>
        <TABLEDATA>
          <TR>
            <TD>359.85821666667465</TD>
            <TD>0.7214125000000109</TD>
            <TD>RGALX360p01MPAID056552</TD>
            <TD>2022.1.01515.S</TD>
          </TR>
          <TR>
            <TD>1.5578874999698953</TD>
            <TD>-6.393148611111178</TD>
            <TD>J0006-0623</TD>
            <TD>2025.1.01024.S</TD>
          </TR>
        </TABLEDATA>
      </DATA>
    </TABLE>
  </RESOURCE>
</VOTABLE>"#;
        let json = votable_to_json(body).unwrap();
        let rows = match &json {
            JsonVal::Arr(a) => a,
            _ => panic!("not an array"),
        };
        assert_eq!(rows.len(), 2);
        assert_eq!(jnum(&rows[0], "s_ra"), Some(359.85821666667465));
        assert_eq!(jnum(&rows[1], "s_dec"), Some(-6.393148611111178));
        assert_eq!(
            jstr(&rows[0], "target_name").as_deref(),
            Some("RGALX360p01MPAID056552")
        );
        assert_eq!(
            jstr(&rows[1], "proposal_id").as_deref(),
            Some("2025.1.01024.S")
        );
        assert_eq!(jnum(&rows[0], "target_name"), None);
    }

    #[test]
    fn base64_decoder_reads_rfc4648_vectors() {
        assert_eq!(base64_decode("").unwrap(), Vec::<u8>::new());
        assert_eq!(base64_decode("TQ==").unwrap(), b"M".to_vec());
        assert_eq!(base64_decode("TWE=").unwrap(), b"Ma".to_vec());
        assert_eq!(base64_decode("TWFu").unwrap(), b"Man".to_vec());
        assert_eq!(base64_decode("ACo=").unwrap(), vec![0x00, 0x2A]);
        assert_eq!(base64_decode("Kg==").unwrap(), vec![0x2A]);
        assert_eq!(base64_decode("TWFu\n").unwrap(), b"Man".to_vec());
        assert!(base64_decode("TQ=").is_none());
        assert!(base64_decode("T=Q=").is_none());
        assert!(base64_decode("TWFu!").is_none());
        assert!(base64_decode("====").is_none());
    }

    fn binary_fields() -> Vec<VotField> {
        vec![
            VotField {
                name: "ra".to_string(),
                kind: VotKind::Float,
                elem_bytes: 8,
                shape: VotShape::Scalar,
            },
            VotField {
                name: "mag".to_string(),
                kind: VotKind::Float,
                elem_bytes: 4,
                shape: VotShape::Scalar,
            },
            VotField {
                name: "name".to_string(),
                kind: VotKind::Char,
                elem_bytes: 1,
                shape: VotShape::Fixed(8),
            },
            VotField {
                name: "counts".to_string(),
                kind: VotKind::Int,
                elem_bytes: 4,
                shape: VotShape::Fixed(2),
            },
            VotField {
                name: "band".to_string(),
                kind: VotKind::Char,
                elem_bytes: 1,
                shape: VotShape::Variable,
            },
            VotField {
                name: "ok".to_string(),
                kind: VotKind::Bool,
                elem_bytes: 1,
                shape: VotShape::Scalar,
            },
        ]
    }

    #[test]
    fn votable_binary2_rows_read_values_and_omit_null_fields() {
        let fields = binary_fields();
        let mut bytes: Vec<u8> = Vec::new();
        bytes.push(0x48);
        bytes.extend_from_slice(&10.5f64.to_be_bytes());
        bytes.extend_from_slice(&0u32.to_be_bytes());
        bytes.extend_from_slice(b"NGC 123\0");
        bytes.extend_from_slice(&1i32.to_be_bytes());
        bytes.extend_from_slice(&2i32.to_be_bytes());
        bytes.extend_from_slice(&0u32.to_be_bytes());
        bytes.push(b'T');
        bytes.push(0x00);
        bytes.extend_from_slice(&(-2.25f64).to_be_bytes());
        bytes.extend_from_slice(&3.5f32.to_be_bytes());
        bytes.extend_from_slice(b"ABCDEFGH");
        bytes.extend_from_slice(&(-7i32).to_be_bytes());
        bytes.extend_from_slice(&9i32.to_be_bytes());
        bytes.extend_from_slice(&3u32.to_be_bytes());
        bytes.extend_from_slice(b"XYZ");
        bytes.push(b'F');
        let json = votable_binary_rows(&bytes, &fields, true).unwrap();
        let rows = match &json {
            JsonVal::Arr(a) => a,
            _ => panic!("not an array"),
        };
        assert_eq!(rows.len(), 2);
        assert_eq!(jnum(&rows[0], "ra"), Some(10.5));
        assert_eq!(jnum(&rows[1], "ra"), Some(-2.25));
        assert_eq!(jnum(&rows[1], "mag"), Some(3.5));
        assert_eq!(jstr(&rows[0], "name").as_deref(), Some("NGC 123"));
        assert_eq!(jstr(&rows[1], "name").as_deref(), Some("ABCDEFGH"));
        assert_eq!(jstr(&rows[1], "band").as_deref(), Some("XYZ"));
        match &rows[0] {
            JsonVal::Obj(m) => match m.get("ok") {
                Some(JsonVal::Bool(b)) => assert!(b),
                other => panic!("ok is not true: {other:?}"),
            },
            _ => panic!("not an object"),
        }
        match &rows[1] {
            JsonVal::Obj(m) => match m.get("ok") {
                Some(JsonVal::Bool(b)) => assert!(!b),
                other => panic!("ok is not false: {other:?}"),
            },
            _ => panic!("not an object"),
        }
        match &rows[0] {
            JsonVal::Obj(m) => {
                assert!(m.get("mag").is_none());
                assert!(m.get("band").is_none());
            }
            _ => panic!("not an object"),
        }
        match &rows[1] {
            JsonVal::Obj(m) => match m.get("counts") {
                Some(JsonVal::Arr(v)) => {
                    assert_eq!(v.len(), 2);
                    assert_eq!(scalar_of(&v[0]), Some(-7.0));
                    assert_eq!(scalar_of(&v[1]), Some(9.0));
                }
                other => panic!("counts is not a 2-element array: {other:?}"),
            },
            _ => panic!("not an object"),
        }
    }

    #[test]
    fn votable_binary_rows_read_without_flag_bytes() {
        let fields = binary_fields();
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend_from_slice(&10.5f64.to_be_bytes());
        bytes.extend_from_slice(&3.5f32.to_be_bytes());
        bytes.extend_from_slice(b"NGC 123\0");
        bytes.extend_from_slice(&1i32.to_be_bytes());
        bytes.extend_from_slice(&2i32.to_be_bytes());
        bytes.extend_from_slice(&3u32.to_be_bytes());
        bytes.extend_from_slice(b"XYZ");
        bytes.push(b'F');
        let json = votable_binary_rows(&bytes, &fields, false).unwrap();
        let rows = match &json {
            JsonVal::Arr(a) => a,
            _ => panic!("not an array"),
        };
        assert_eq!(rows.len(), 1);
        assert_eq!(jnum(&rows[0], "ra"), Some(10.5));
        assert_eq!(jnum(&rows[0], "mag"), Some(3.5));
        assert_eq!(jstr(&rows[0], "band").as_deref(), Some("XYZ"));
    }

    #[test]
    fn votable_binary2_second_flag_byte_reads() {
        let mut fields = Vec::new();
        for i in 0..9 {
            fields.push(VotField {
                name: format!("c{i}"),
                kind: VotKind::UInt,
                elem_bytes: 1,
                shape: VotShape::Scalar,
            });
        }
        let mut bytes: Vec<u8> = Vec::new();
        bytes.push(0x00);
        bytes.push(0x80);
        for v in 1u8..=9 {
            bytes.push(v);
        }
        let json = votable_binary_rows(&bytes, &fields, true).unwrap();
        let rows = match &json {
            JsonVal::Arr(a) => a,
            _ => panic!("not an array"),
        };
        assert_eq!(rows.len(), 1);
        match &rows[0] {
            JsonVal::Obj(m) => {
                assert_eq!(m.len(), 8);
                assert_eq!(scalar_of(m.get("c7").unwrap()), Some(8.0));
                assert!(m.get("c8").is_none());
            }
            _ => panic!("not an object"),
        }
    }

    #[test]
    fn votable_binary_nan_scalar_is_absent_and_nan_array_element_stays_null() {
        let fields = vec![
            VotField {
                name: "a".to_string(),
                kind: VotKind::Float,
                elem_bytes: 8,
                shape: VotShape::Scalar,
            },
            VotField {
                name: "b".to_string(),
                kind: VotKind::Float,
                elem_bytes: 8,
                shape: VotShape::Fixed(2),
            },
        ];
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend_from_slice(&f64::NAN.to_be_bytes());
        bytes.extend_from_slice(&1.0f64.to_be_bytes());
        bytes.extend_from_slice(&f64::NAN.to_be_bytes());
        let json = votable_binary_rows(&bytes, &fields, false).unwrap();
        let rows = match &json {
            JsonVal::Arr(a) => a,
            _ => panic!("not an array"),
        };
        match &rows[0] {
            JsonVal::Obj(m) => {
                assert!(m.get("a").is_none());
                match m.get("b") {
                    Some(JsonVal::Arr(v)) => {
                        assert_eq!(v.len(), 2);
                        assert_eq!(scalar_of(&v[0]), Some(1.0));
                        match &v[1] {
                            JsonVal::Null => {}
                            other => panic!("nan element is not null: {other:?}"),
                        }
                    }
                    other => panic!("b is not an array: {other:?}"),
                }
            }
            _ => panic!("not an object"),
        }
    }

    #[test]
    fn votable_binary2_stream_base64_block_reads_through_votable_to_json() {
        let body = r#"<VOTABLE version="1.4"><RESOURCE><TABLE>
<FIELD name="f0" datatype="unsignedByte"/>
<FIELD name="f1" datatype="unsignedByte"/>
<DATA><BINARY2><STREAM encoding="base64">QCoA</STREAM></BINARY2></DATA>
</TABLE></RESOURCE></VOTABLE>"#;
        let json = votable_to_json(body).unwrap();
        let rows = match &json {
            JsonVal::Arr(a) => a,
            _ => panic!("not an array"),
        };
        assert_eq!(rows.len(), 1);
        match &rows[0] {
            JsonVal::Obj(m) => {
                assert_eq!(scalar_of(m.get("f0").unwrap()), Some(42.0));
                assert!(m.get("f1").is_none());
            }
            _ => panic!("not an object"),
        }
    }

    #[test]
    fn votable_binary_stream_base64_block_reads_through_votable_to_json() {
        let body = r#"<VOTABLE version="1.4"><RESOURCE><TABLE>
<FIELD name="f" datatype="unsignedByte"/>
<DATA><BINARY><STREAM encoding="base64">Kg==</STREAM></BINARY></DATA>
</TABLE></RESOURCE></VOTABLE>"#;
        let json = votable_to_json(body).unwrap();
        let rows = match &json {
            JsonVal::Arr(a) => a,
            _ => panic!("not an array"),
        };
        assert_eq!(rows.len(), 1);
        assert_eq!(jnum(&rows[0], "f"), Some(42.0));
    }

    #[test]
    fn votable_binary_with_href_stream_is_none() {
        let body = r#"<VOTABLE version="1.4"><RESOURCE><TABLE>
<FIELD name="f" datatype="double"/>
<DATA><BINARY2><STREAM encoding="base64" href="http://x/rows.b64"/></BINARY2></DATA>
</TABLE></RESOURCE></VOTABLE>"#;
        assert!(votable_to_json(body).is_none());
    }

    #[test]
    fn votable_binary_unknown_datatype_is_none() {
        let body = r#"<VOTABLE version="1.4"><RESOURCE><TABLE>
<FIELD name="f" datatype="wibble"/>
<DATA><BINARY><STREAM encoding="base64">Kg==</STREAM></BINARY></DATA>
</TABLE></RESOURCE></VOTABLE>"#;
        assert!(votable_to_json(body).is_none());
    }

    #[test]
    fn votable_binary_without_stream_is_none() {
        let body = r#"<VOTABLE version="1.4"><RESOURCE><TABLE>
<FIELD name="f" datatype="double"/>
<DATA><BINARY2/></DATA>
</TABLE></RESOURCE></VOTABLE>"#;
        assert!(votable_to_json(body).is_none());
    }
}

#[cfg(test)]
mod html_tests {
    use super::*;

    const AEC_RECENT_LIST: &str = r#"<div id="block-recentearthquakelist">
<table class="desktop-centered">
<thead>
<tr><th>Mag</th>
<th>Earthquake Information</th>
<th>Depth (miles)</th>
</tr></thead>
<tbody id="eqTable"><tr class="even"><td><a id="grey-links" href="/event/aka2026sztdqy">1.9</a></td><td><a id="grey-links" href="/event/aka2026sztdqy">September 24 at 11:38 PM AKDT<br>40 mi NW of Cape Yakataga</a></td><td><a id="grey-links" href="/event/aka2026sztdqy">0</a></td></tr><tr class="odd"><td><a id="grey-links" href="/event/aka2026szqcic">1.9</a></td><td><a id="grey-links" href="/event/aka2026szqcic">September 24 at 10:07 PM AKDT<br>10 mi NW of Central</a></td><td><a id="grey-links" href="/event/aka2026szqcic">1</a></td></tr></tbody>
</table>
</div>"#;

    #[test]
    fn html_recent_list_rows_read() {
        let json = html_to_json(AEC_RECENT_LIST).unwrap();
        let rows = match &json {
            JsonVal::Arr(a) => a,
            _ => panic!("not an array"),
        };
        assert_eq!(rows.len(), 2);
        assert_eq!(jnum(&rows[0], "mag"), Some(1.9));
        assert_eq!(jnum(&rows[0], "depth_miles"), Some(0.0));
        assert_eq!(jnum(&rows[1], "depth_miles"), Some(1.0));
        assert_eq!(
            jstr(&rows[1], "earthquake_information").as_deref(),
            Some("September 24 at 10:07 PM AKDT\n10 mi NW of Central")
        );
    }

    #[test]
    fn html_empty_tbody_is_none() {
        let body = r#"<table class="desktop-centered"><thead><tr><th>Mag</th><th>Earthquake Information</th><th>Depth (miles)</th></tr></thead><tbody id="eqTable"></tbody></table>"#;
        assert!(html_to_json(body).is_none());
    }

    #[test]
    fn html_without_table_is_none() {
        assert!(html_to_json("<html><body>no table</body></html>").is_none());
    }

    #[test]
    fn html_empty_cell_is_null_not_zero() {
        let body = r#"<table><thead><tr><th>Mag</th><th>Note</th></tr></thead><tbody><tr><td>2.4</td><td> </td></tr></tbody></table>"#;
        let json = html_to_json(body).unwrap();
        let rows = match &json {
            JsonVal::Arr(a) => a,
            _ => panic!("not an array"),
        };
        match &rows[0] {
            JsonVal::Obj(m) => match m.get("note") {
                Some(JsonVal::Null) => {}
                other => panic!("empty cell is not Null: {other:?}"),
            },
            _ => panic!("not an object"),
        }
        assert_eq!(jnum(&rows[0], "note"), None);
    }

    #[test]
    fn html_headerless_table_is_none() {
        let body = r#"<table><tbody><tr><td>1.2</td><td>2.3</td></tr></tbody></table>"#;
        assert!(html_to_json(body).is_none());
    }

    #[test]
    fn html_table_after_a_headerless_one_reads() {
        let body = r#"<table><tbody><tr><td>1.2</td><td>2.3</td></tr></tbody></table>
<table><thead><tr><th>Mag</th></tr></thead><tbody><tr><td>3.5</td></tr></tbody></table>"#;
        let json = html_to_json(body).unwrap();
        let rows = match &json {
            JsonVal::Arr(a) => a,
            _ => panic!("not an array"),
        };
        assert_eq!(rows.len(), 1);
        assert_eq!(jnum(&rows[0], "mag"), Some(3.5));
    }
}

#[cfg(test)]
mod fixed_width_series_tests {
    use super::*;

    fn pds3_column(name: &str) -> pds3_table::TableColumn {
        pds3_table::TableColumn {
            name: name.to_string(),
            unit: None,
            data_type: Some("ASCII_REAL".to_string()),
            missing_constant: None,
            sampling_name: String::new(),
            sampling_unit: String::new(),
            sampling_min: None,
            sampling_max: None,
            start_byte: 0,
            bytes: 0,
        }
    }

    fn pds4_column(name: &str) -> pds4::Pds4Column {
        pds4::Pds4Column {
            name: name.to_string(),
            unit: None,
            data_type: Some("ASCII_REAL".to_string()),
            missing_constant: None,
            sampling_name: String::new(),
            sampling_unit: String::new(),
            sampling_min: None,
            sampling_max: None,
            start_byte: None,
            bytes: None,
        }
    }

    fn pds4_binary_column(name: &str) -> pds4_binary::Pds4BinaryColumn {
        pds4_binary::Pds4BinaryColumn {
            name: name.to_string(),
            unit: None,
            data_type: Some("IEEE754LSBDouble".to_string()),
            missing_constant: None,
            sampling_name: String::new(),
            sampling_unit: String::new(),
            sampling_min: None,
            sampling_max: None,
            start_byte: None,
            bytes: None,
        }
    }

    #[test]
    fn pds3_fixed_width_series_reads_axis_and_columns() {
        let table = pds3_table::Pds3Table {
            columns: vec![
                pds3_column("TIME_OFFSET"),
                pds3_column("RADIOMETER1"),
                pds3_column("PHOTOMETER1"),
            ],
            rows: vec![
                pds3_table::TableRow {
                    values: vec![Some(10.0), Some(1.5), None],
                },
                pds3_table::TableRow {
                    values: vec![Some(20.0), Some(2.5), Some(3.5)],
                },
            ],
        };
        let bin = pds3_table::pack(&table);
        let series = series_parse_bin("pds3_fixed_width", &bin).expect("series parses");
        assert_eq!(series, vec![(10.0, 1.5, 1), (20.0, 2.5, 1), (20.0, 3.5, 2)]);
    }

    #[test]
    fn pds4_fixed_width_series_reads_axis_and_columns() {
        let table = pds4::Pds4Table {
            columns: vec![pds4_column("TIME"), pds4_column("RADIANCE")],
            rows: vec![
                pds4::Pds4Row {
                    values: vec![Some(1.0), Some(9.0)],
                },
                pds4::Pds4Row {
                    values: vec![Some(2.0), Some(8.0)],
                },
            ],
            delimited: false,
            delimiter: None,
        };
        let bin = pds4::pack(&table);
        let series = series_parse_bin("pds4_fixed_width", &bin).expect("series parses");
        assert_eq!(series, vec![(1.0, 9.0, 1), (2.0, 8.0, 1)]);
    }

    #[test]
    fn fixed_width_series_rejects_foreign_bytes() {
        assert!(series_parse_bin("pds3_fixed_width", b"XXXX").is_none());
        assert!(series_parse_bin("pds4_fixed_width", b"XXXX").is_none());
        assert!(series_parse_bin("pds4_binary", b"XXXX").is_none());
    }

    #[test]
    fn hapi_csv_series_parses_written_bin_and_rejects_foreign_bytes() {
        let bin = hapi_csv::write_bin(2, &[(1.0, 5.0, 1), (2.0, 6.0, 2)]);
        let series = series_parse_bin("hapi_csv", &bin).expect("series parses");
        assert_eq!(series, vec![(1.0, 5.0, 1), (2.0, 6.0, 2)]);
        assert!(series_parse_bin("hapi_csv", b"XXXX").is_none());
    }

    #[test]
    fn pds3_named_series_carries_column_names_verbatim() {
        let table = pds3_table::Pds3Table {
            columns: vec![
                pds3_column("TIME_OFFSET"),
                pds3_column("RADIOMETER1"),
                pds3_column("PHOTOMETER1"),
            ],
            rows: vec![pds3_table::TableRow {
                values: vec![Some(10.0), Some(1.5), None],
            }],
        };
        let bin = pds3_table::pack(&table);
        let named = series_named("pds3_fixed_width", &bin).expect("named series parses");
        assert_eq!(
            named.names,
            vec!["TIME_OFFSET", "RADIOMETER1", "PHOTOMETER1"]
        );
        assert_eq!(named.rows.len(), 1);
        let row = named.rows[0];
        assert_eq!(row.comp, 1);
        assert_eq!(
            named.names.get(row.comp as usize),
            Some(&"RADIOMETER1".to_string())
        );
    }

    #[test]
    fn pds4_named_series_excludes_axis_and_maps_comp_to_name() {
        let table = pds4::Pds4Table {
            columns: vec![pds4_column("TIME"), pds4_column("RADIANCE")],
            rows: vec![pds4::Pds4Row {
                values: vec![Some(2.0), Some(8.0)],
            }],
            delimited: false,
            delimiter: None,
        };
        let bin = pds4::pack(&table);
        let named = series_named("pds4_fixed_width", &bin).expect("named series parses");
        assert_eq!(named.names, vec!["TIME", "RADIANCE"]);
        assert_eq!(named.rows.len(), 1);
        let row = named.rows[0];
        assert_eq!(row.t, 2.0);
        assert_eq!(row.comp, 1);
        assert_eq!(
            named.names.get(row.comp as usize),
            Some(&"RADIANCE".to_string())
        );
    }

    #[test]
    fn pds4_binary_series_reads_axis_and_columns() {
        let table = pds4_binary::Pds4BinaryTable {
            columns: vec![pds4_binary_column("TIME"), pds4_binary_column("DOPPLER")],
            rows: vec![
                pds4_binary::Pds4BinaryRow {
                    values: vec![Some(1.0), Some(9.0)],
                },
                pds4_binary::Pds4BinaryRow {
                    values: vec![Some(2.0), Some(8.0)],
                },
            ],
        };
        let bin = pds4_binary::pack(&table);
        let series = series_parse_bin("pds4_binary", &bin).expect("series parses");
        assert_eq!(series, vec![(1.0, 9.0, 1), (2.0, 8.0, 1)]);
    }

    #[test]
    fn pds4_binary_named_series_excludes_axis_and_maps_comp_to_name() {
        let table = pds4_binary::Pds4BinaryTable {
            columns: vec![pds4_binary_column("TIME"), pds4_binary_column("DOPPLER")],
            rows: vec![pds4_binary::Pds4BinaryRow {
                values: vec![Some(2.0), Some(8.0)],
            }],
        };
        let bin = pds4_binary::pack(&table);
        let named = series_named("pds4_binary", &bin).expect("named series parses");
        assert_eq!(named.names, vec!["TIME", "DOPPLER"]);
        assert_eq!(named.rows.len(), 1);
        let row = named.rows[0];
        assert_eq!(row.t, 2.0);
        assert_eq!(row.comp, 1);
        assert_eq!(
            named.names.get(row.comp as usize),
            Some(&"DOPPLER".to_string())
        );
    }

    fn pad_card(kw: &str, value: &str) -> [u8; 80] {
        let mut card = [b' '; 80];
        let k = kw.as_bytes();
        card[..k.len().min(8)].copy_from_slice(&k[..k.len().min(8)]);
        card[8] = b'=';
        let v = value.as_bytes();
        card[10..10 + v.len().min(20)].copy_from_slice(&v[..v.len().min(20)]);
        card
    }

    fn fits_image(cards: &[(&str, &str)], raw: &[u8]) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        let mut header: Vec<u8> = Vec::new();
        header.extend_from_slice(&pad_card("SIMPLE", "T"));
        for (k, v) in cards {
            header.extend_from_slice(&pad_card(k, v));
        }
        header.extend_from_slice(&pad_card("END", ""));
        while !header.len().is_multiple_of(2880) {
            header.extend_from_slice(&[b' '; 80]);
        }
        buf.extend_from_slice(&header);
        buf.extend_from_slice(raw);
        while !buf.len().is_multiple_of(2880) {
            buf.push(0);
        }
        buf
    }

    #[test]
    fn pds4_fits_series_reads_band_mean() {
        let mut raw = Vec::new();
        for v in [1.0f32, 2.0, 3.0, 4.0] {
            raw.extend_from_slice(&v.to_be_bytes());
        }
        let buf = fits_image(
            &[
                ("BITPIX", "-32"),
                ("NAXIS", "2"),
                ("NAXIS1", "2"),
                ("NAXIS2", "2"),
                ("BUNIT", "'K'"),
            ],
            &raw,
        );
        let series = series_parse_bin("pds4_fits", &buf).expect("series parses");
        assert_eq!(series, vec![(0.0, 2.5, 0)]);
    }

    #[test]
    fn pds4_fits_named_series_carries_bunit_name() {
        let mut raw = Vec::new();
        for v in [1.0f32, 2.0, 3.0, 4.0] {
            raw.extend_from_slice(&v.to_be_bytes());
        }
        let buf = fits_image(
            &[
                ("BITPIX", "-32"),
                ("NAXIS", "2"),
                ("NAXIS1", "2"),
                ("NAXIS2", "2"),
                ("BUNIT", "'K'"),
            ],
            &raw,
        );
        let named = series_named("pds4_fits", &buf).expect("named series parses");
        assert_eq!(named.names, vec!["K"]);
        assert_eq!(named.rows.len(), 1);
        assert_eq!(named.rows[0].comp, 0);
        assert_eq!(named.rows[0].value, 2.5);
    }

    #[test]
    fn gras_2c_series_reads_records() {
        let bin = gras_2c::write_bin(&[(1.5e9, 2.5, 0), (1.5e9 + 1.0, -3.5, 1)]);
        let series = series_parse_bin("gras_2c", &bin).expect("series parses");
        assert_eq!(series, vec![(1.5e9, 2.5, 0), (1.5e9 + 1.0, -3.5, 1)]);
    }

    #[test]
    fn gras_2c_named_series_carries_gate_channels_without_a_band() {
        let bin = gras_2c::write_bin(&[(1.5e9, 2.5, 0), (1.5e9 + 1.0, -3.5, 3)]);
        let named = series_named("gras_2c", &bin).expect("named series parses");
        assert_eq!(
            named.names,
            vec![
                "gras_2c_gate_0000",
                "gras_2c_gate_0001",
                "gras_2c_gate_0002",
                "gras_2c_gate_0003",
            ]
        );
        assert_eq!(named.rows.len(), 2);
        assert_eq!(named.rows[1].comp, 3);
        assert_eq!(named.rows[0].value, 2.5);
        assert_eq!(named.rows[0].freq, spectral::SPECTRAL_NO_BAND);
        assert_eq!(named.rows[0].bin_width, spectral::SPECTRAL_NO_BAND);
    }

    #[test]
    fn gras_2c_declares_gate_channels_without_field_lines() {
        let bin = gras_2c::write_bin(&[(1.5e9, 2.5, 0), (1.5e9 + 1.0, -3.5, 3)]);
        let named = series_named("gras_2c", &bin).expect("named series parses");
        let fields = series_declared_fields("gras_2c", &named.names, 604800.0);
        assert_eq!(fields.len(), 4);
        assert_eq!(fields[0].name, "gras_2c_gate_0000");
        assert_eq!(fields[3].name, "gras_2c_gate_0003");
        assert_eq!(fields[0].force, crate::force::force_id_of("em").unwrap());
        assert_eq!(
            fields[0].kernel,
            crate::archivar::kernel_id_of("inverse-square").unwrap()
        );
        assert_eq!(fields[0].unit, gras_2c::GATE_UNIT);
        assert_eq!(fields[0].tau, 604800.0);
        assert_eq!(fields[0].freq, spectral::SPECTRAL_NO_BAND);
        assert_eq!(fields[0].bin_width, spectral::SPECTRAL_NO_BAND);
        let joined: Vec<&str> = named
            .rows
            .iter()
            .filter_map(|row| {
                let name = series_component_name("gras_2c", row.comp)
                    .or_else(|| named.names.get(row.comp as usize).map(String::as_str))?;
                fields
                    .iter()
                    .find(|fc| fc.name == name)
                    .map(|fc| fc.name.as_str())
            })
            .collect();
        assert_eq!(joined.len(), named.rows.len());
        assert_eq!(joined, vec!["gras_2c_gate_0000", "gras_2c_gate_0003"]);
    }

    #[test]
    fn acs_nir_named_series_carries_raw_bins_without_field_lines() {
        let raw = include_bytes!("pds4_fixtures/exomars_acs_nir_ec_first1.tab");
        let named = series_named("pds4_acs_nir", raw).expect("named series parses");
        assert_eq!(named.names.len(), acs_nir::ROW_DATA_COUNT);
        assert_eq!(named.names[0], "acs_nir_bin_0000");
        assert_eq!(named.names[639], "acs_nir_bin_0639");
        assert_eq!(named.names[1279], "acs_nir_bin_1279");
        assert_eq!(named.rows.len(), acs_nir::ROW_DATA_COUNT);
        assert_eq!(named.rows[0].comp, 0);
        assert_eq!(named.rows[1279].comp, 1279);
        assert_eq!(named.rows[0].freq, spectral::SPECTRAL_NO_BAND);
        assert_eq!(named.rows[0].bin_width, spectral::SPECTRAL_NO_BAND);
        let fields = series_declared_fields("pds4_acs_nir", &named.names, 604800.0);
        assert_eq!(fields.len(), acs_nir::ROW_DATA_COUNT);
        assert_eq!(fields[0].name, "acs_nir_bin_0000");
        assert_eq!(fields[0].unit, acs_nir::UNIT);
        assert_eq!(fields[0].tau, acs_nir::TAU_INTEGRATION_S);
        let joined: Vec<&str> = named
            .rows
            .iter()
            .filter_map(|row| {
                let name = series_component_name("pds4_acs_nir", row.comp)
                    .or_else(|| named.names.get(row.comp as usize).map(String::as_str))?;
                fields
                    .iter()
                    .find(|fc| fc.name == name)
                    .map(|fc| fc.name.as_str())
            })
            .collect();
        assert_eq!(joined.len(), named.rows.len());
    }

    #[test]
    fn named_series_is_none_for_foreign_formats() {
        assert!(series_named("drs_fits", b"XXXX").is_none());
        assert!(series_named("pds3_fixed_width", b"XXXX").is_none());
        assert!(series_named("pds4_binary", b"XXXX").is_none());
        assert!(series_named("pds4_fits", b"XXXX").is_none());
        assert!(series_named("gras_2c", b"XXXX").is_none());
        assert!(series_named("pds4_acs_nir", b"XXXX").is_none());
    }

    #[test]
    fn astrometry_series_reads_a_direction_series_without_a_scalar_slot() {
        let series = astrometry_series::AstroSeries {
            name: "uranu".to_string(),
            samples: vec![
                astrometry_series::AstroSample {
                    tdb: 2.4e9,
                    ra_deg: 288.7273,
                    dec_deg: -22.7528,
                    e_ra_mas: 57.0,
                    e_dec_mas: 49.0,
                },
                astrometry_series::AstroSample {
                    tdb: 2.4e9 + 86400.0,
                    ra_deg: 288.7291,
                    dec_deg: -22.7521,
                    e_ra_mas: 47.0,
                    e_dec_mas: 42.0,
                },
            ],
        };
        let bin = astrometry_series::write_bin(std::slice::from_ref(&series))
            .expect("finite measured samples write");
        assert_eq!(astrometry_series_counts(&bin), Some((1, 2)));
        assert_eq!(verify_records("astrometry_series", &bin), Some(2));
        assert!(series_named("astrometry_series", &bin).is_none());
        assert!(astrometry_series_counts(b"XXXX").is_none());
        assert!(verify_records("astrometry_series", b"XXXX").is_none());
    }
}

#[cfg(test)]
mod edf_arm_tests {
    use super::*;

    struct Sig {
        label: &'static str,
        unit: &'static str,
        p_min: f64,
        p_max: f64,
        d_min: f64,
        d_max: f64,
        digital: Vec<i16>,
    }

    fn put(v: &mut Vec<u8>, s: &str, len: usize) {
        let mut f = s.as_bytes().to_vec();
        f.resize(len, b' ');
        v.extend_from_slice(&f);
    }

    fn synth(sigs: &[Sig]) -> Vec<u8> {
        let ns = sigs.len();
        let mut h = Vec::new();
        put(&mut h, "0", 8);
        put(&mut h, "patient", 80);
        put(&mut h, "recording", 80);
        put(&mut h, "01.01.20", 8);
        put(&mut h, "00.00.00", 8);
        put(&mut h, &(256 + 256 * ns).to_string(), 8);
        put(&mut h, "EDF+C", 44);
        put(&mut h, "1", 8);
        put(&mut h, "1", 8);
        put(&mut h, &ns.to_string(), 4);
        for s in sigs {
            put(&mut h, s.label, 16);
        }
        for _ in sigs {
            put(&mut h, "", 80);
        }
        for s in sigs {
            put(&mut h, s.unit, 8);
        }
        for s in sigs {
            put(&mut h, &s.p_min.to_string(), 8);
        }
        for s in sigs {
            put(&mut h, &s.p_max.to_string(), 8);
        }
        for s in sigs {
            put(&mut h, &s.d_min.to_string(), 8);
        }
        for s in sigs {
            put(&mut h, &s.d_max.to_string(), 8);
        }
        for _ in sigs {
            put(&mut h, "", 80);
        }
        for s in sigs {
            put(&mut h, &s.digital.len().to_string(), 8);
        }
        for _ in sigs {
            put(&mut h, "", 32);
        }
        for s in sigs {
            for v in &s.digital {
                h.extend_from_slice(&v.to_le_bytes());
            }
        }
        h
    }

    #[test]
    fn edf_arm_emits_voltage_channels_at_the_anchor() {
        let bytes = synth(&[Sig {
            label: "EEG Fpz-Cz",
            unit: "uV",
            p_min: 0.0,
            p_max: 100.0,
            d_min: 0.0,
            d_max: 100.0,
            digital: vec![0, 50, 100],
        }]);
        let position = Position::Barycenter {
            body_name: "earth".to_string(),
            scale: 1.0,
        };
        let channels = edf_emit_channels(&bytes, &position, 100.0, 60.0, EDF_CHANNEL_CAP);
        assert_eq!(channels.len(), 3);
        let (ch, fc) = &channels[2];
        assert_eq!(fc.force, crate::force::force_id_of("electric").unwrap());
        assert_eq!(fc.unit, "V");
        assert_eq!(ch.station_code.as_deref(), Some("EEG Fpz-Cz"));
        assert_eq!(ch.z, 0.0);
        assert_eq!(ch.freq, 0.0);
        assert_eq!(ch.bin_width, 0.0);
        assert_eq!(ch.epoch, 100.0);
        assert!((ch.value - 1.0e-4).abs() < 1.0e-12);
        match &ch.position {
            Position::Barycenter { body_name, scale } => {
                assert_eq!(body_name, "earth");
                assert_eq!(*scale, 1.0);
            }
            _ => panic!("anchor is not the source frame"),
        }
    }

    #[test]
    fn edf_arm_skips_a_unit_with_no_honest_force() {
        let bytes = synth(&[
            Sig {
                label: "EEG",
                unit: "uV",
                p_min: 0.0,
                p_max: 100.0,
                d_min: 0.0,
                d_max: 100.0,
                digital: vec![0, 50, 100],
            },
            Sig {
                label: "SpO2",
                unit: "%",
                p_min: 0.0,
                p_max: 100.0,
                d_min: 0.0,
                d_max: 100.0,
                digital: vec![97, 98],
            },
        ]);
        let channels = edf_emit_channels(&bytes, &Position::Source, 1.0, 1.0, EDF_CHANNEL_CAP);
        assert_eq!(channels.len(), 3);
        assert!(
            channels
                .iter()
                .all(|(c, _)| c.station_code.as_deref() != Some("SpO2"))
        );
    }

    #[test]
    fn edf_arm_skips_samples_whose_physical_value_is_absent() {
        let bytes = synth(&[Sig {
            label: "overflow",
            unit: "V",
            p_min: -1.7e308,
            p_max: 1.7e308,
            d_min: 0.0,
            d_max: 100.0,
            digital: vec![100],
        }]);
        let channels = edf_emit_channels(&bytes, &Position::Source, 1.0, 1.0, EDF_CHANNEL_CAP);
        assert!(channels.is_empty());
    }

    #[test]
    fn edf_arm_caps_channels_per_signal() {
        let bytes = synth(&[Sig {
            label: "EEG",
            unit: "V",
            p_min: 0.0,
            p_max: 20.0,
            d_min: 0.0,
            d_max: 20.0,
            digital: (0..20).collect(),
        }]);
        let channels = edf_emit_channels(&bytes, &Position::Source, 1.0, 1.0, 8);
        assert!(channels.len() <= 8);
        assert_eq!(channels.len(), 7);
    }
}
