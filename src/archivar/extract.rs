use super::*;

pub fn series_parse_bin(format: &str, bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    match format {
        "rpw_efield" => crate::rpw::parse_bin(bytes),
        "goes_xrs" => goes::parse_bin(bytes),
        "intermagnet_dbdt" => intermagnet::parse_bin(bytes),
        "omni2_serie" => omni2::parse_bin(bytes),
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
        "vex_odf" => odf::parse_series(bytes),
        "galileo_odf" => odf::parse_series(bytes),
        "dawn_odf" => odf::parse_series(bytes),
        "cassini_odf" => odf::parse_series(bytes),
        "pioneer10_odf" => odf::parse_series(bytes),
        "pathfinder_odf" => odf::parse_series(bytes),
        "cassini_tnf" | "maven_tnf" | "dart_tnf" => odf::tnf_parse_series(bytes),
        "voyager_odr" => voyager_odr::parse_series(bytes),
        "galileo_odr" => galileo_odr::parse_series(bytes),
        "cassini_rsr" => cassini_rsr::parse_series(bytes),
        "flac" => flac::parse_series(bytes),
        "bidsleep" => bidsleep::parse_bin(bytes),
        "bison_velocity" => crate::bison_velocity::parse_bin(bytes)
            .map(|rs| rs.into_iter().map(|(t, v)| (t, v, 0)).collect()),
        "las" => crate::las::las_series::parse_series(bytes),
        _ => None,
    }
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
        "celestrak_eop" => match comp {
            celestrak_eop::COMP_UT1_UTC => Some("eop_iers_ut1_utc_s"),
            celestrak_eop::COMP_PMX => Some("eop_iers_polar_motion_x_arcsec"),
            celestrak_eop::COMP_PMY => Some("eop_iers_polar_motion_y_arcsec"),
            _ => None,
        },
        "gk2a_ami" => gk2a_ami::component_name(comp),
        "goes_abi" => goes_abi::component_name(comp),
        "atdf" => atdf::component_name(comp),
        "ulysses_atdf" => atdf::uly_component_name(comp),
        "ulysses_atdf_x" => atdf::uly_component_name(comp),
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
            voyager_saturn::COMP_RANGE_PART2 => Some("voyager_saturn_range_part2"),
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
        "pathfinder_odf" => match comp {
            odf::COMP_OBSERVABLE => Some("pathfinder_odf_observable_hz"),
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
        "voyager_odr" => match comp {
            voyager_odr::COMP_SAMPLE => Some("voyager_odr_sample_count"),
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
        "fmi_gic" => match comp {
            crate::geo::COMP_GIC_A => Some("fmi_gic_a"),
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
        Some(JsonVal::Str(s)) => s == fv,
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
        "gaussian-inverse" => Some(2),
        "erfc" => Some(3),
        "exponential-decay" => Some(4),
        "patch-levy" => Some(5),
        "inverse-linear" => Some(6),
        _ => None,
    }
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
        | Extract::ProfileMap { fields, .. } => fields.clone(),
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
    let table = {
        let p = data_part.find("<TABLEDATA")?;
        &data_part[p..]
    };
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
    let metadata = match obj.get("metadata") {
        Some(JsonVal::Arr(a)) => a,
        _ => return None,
    };
    let data = match obj.get("data") {
        Some(JsonVal::Arr(a)) => a,
        _ => return None,
    };
    let mut names: Vec<String> = Vec::new();
    for m in metadata {
        if let JsonVal::Obj(mo) = m
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

pub fn extract(src: &SourceConfig, body: &str, now: f64, lsk: &LeapSeconds) -> ExtractResult {
    if src.format == "ephemeris_binary" {
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
    if src.format == "vlde" {
        let mut buf = Vec::new();
        if let Ok(mut f) = std::fs::File::open(body) {
            use std::io::Read;
            f.read_to_end(&mut buf).ok();
        }
        let Some(field) = crate::vlies::parse_asset(&buf) else {
            return ExtractResult::Measurements(vec![]);
        };
        let Some(Extract::Field(fc)) = src.extracts.first() else {
            return ExtractResult::Measurements(vec![]);
        };
        let mut channels: Vec<(Channel, FieldConfig)> = Vec::with_capacity(field.counts.len());
        for (pix, &count) in field.counts.iter().enumerate() {
            let Some(p) = crate::vlies::pixel_direction(field.nside, pix as i64) else {
                return ExtractResult::Measurements(vec![]);
            };
            channels.push((
                Channel {
                    z: 0.0,
                    freq: 0.0,
                    bin_width: 0.0,
                    epoch: now,
                    position: Position::StateVector {
                        p,
                        v: [0.0, 0.0, 0.0],
                        track: false,
                    },
                    name: fc.name.clone(),
                    value: count as f64,
                },
                fc.clone(),
            ));
        }
        return ExtractResult::Measurements(channels);
    }
    if src.format == "arpansa" || src.format == "uvxml" {
        let fc = FieldConfig {
            key: "uv_index".to_string(),
            name: "uv_index".to_string(),
            kernel: 0,
            force: 0,
            tau: src.ttl as f64 / 10.0,
            absorption: 0.0,
            advection: 0.0,
            unit: "UVI".to_string(),
            freq: 0.0,
            bin_width: 0.0,
            fold: None,
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
        let tau = src.ttl as f64 / 10.0;
        let fcfg = |name: &str, kernel: u8, force: u8, unit: &str| FieldConfig {
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
                let emitted: [(Option<f64>, FieldConfig); 6] = [
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
                    if let Some(v) = value {
                        channels.push((
                            Channel {
                                z: 0.0,
                                freq: 0.0,
                                bin_width: 0.0,
                                epoch: now,
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
        let fc = FieldConfig {
            key: "fugin_moment0".to_string(),
            name: "fugin_moment0".to_string(),
            kernel: 0,
            force: 0,
            tau: src.ttl as f64 / 10.0,
            absorption: 0.0,
            advection: 0.0,
            unit: "K m/s".to_string(),
            freq: 0.0,
            bin_width: 0.0,
            fold: None,
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
        csv_to_json(body)
    } else if src.format == "free text" {
        text_to_json(body)
    } else if src.format == "tap" {
        match tap_format(&src.url).as_deref() {
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
    } else if src.format == "json" || src.format.is_empty() || src.format == "universal" {
        let body = body
            .strip_prefix("OK")
            .and_then(|r| r.strip_prefix('\n').or_else(|| r.strip_prefix("\r\n")))
            .unwrap_or(body);
        parse_json(body)
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
                            if let Some(v) = jpath(v, epoch_key) {
                                v
                            } else {
                                continue;
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
                                    },
                                ));
                                channels.push((
                                    Channel {
                                        z: 0.0,
                                        freq: 0.0,
                                        bin_width: 0.0,
                                        epoch: now,
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

pub fn extract_series(src: &SourceConfig, body: &str, lsk: &LeapSeconds) -> Vec<(f64, f64)> {
    let parsed = parse_json(body);
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
}
