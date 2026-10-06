#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WitnessKind {
    S2Direction,
    PointEvent,
    Gestalt,
    Presence,
    Substance,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FieldIdentity {
    Oscillator,
    Witness(WitnessKind),
    Footprint,
    Pending,
}

pub fn magic_identity(magic: [u8; 4]) -> Option<FieldIdentity> {
    match &magic {
        b"AMN1" | b"PAO1" | b"S2E1" | b"ERBQ" => {
            Some(FieldIdentity::Witness(WitnessKind::PointEvent))
        }
        b"SKY1" | b"SKD1" | b"VLDE" => Some(FieldIdentity::Witness(WitnessKind::S2Direction)),
        b"GBCO" | b"GL30" | b"GL90" | b"SLB2" | b"OCS1" | b"ERI1" | b"GMR1" | b"G3D1" | b"SRTM" => {
            Some(FieldIdentity::Witness(WitnessKind::Gestalt))
        }
        b"ISCB" | b"EHB1" => Some(FieldIdentity::Witness(WitnessKind::Presence)),
        b"RIXS" | b"RIXC" | b"EELS" | b"SRD6" => {
            Some(FieldIdentity::Witness(WitnessKind::Substance))
        }
        b"FP01" => Some(FieldIdentity::Footprint),
        b"NRS1" | b"PTLM" => Some(FieldIdentity::Pending),
        b"BGR1" | b"ARG1" | b"FDS1" | b"GIC1" | b"IGT1" | b"SDN1" | b"CSM1" | b"CRX1" | b"MAX1"
        | b"NXR1" | b"USC1" | b"VSAT" | b"DRSF" | b"OSM1" | b"OSM2" => {
            Some(FieldIdentity::Oscillator)
        }
        _ => None,
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WitnessVerdict {
    Holds(WitnessKind),
    Radiator,
    BareCoordinate,
    Pending,
}

pub fn witness_gate(
    magic: Option<[u8; 4]>,
    declared_art: Option<WitnessKind>,
    has_scalar: bool,
) -> WitnessVerdict {
    let Some(m) = magic else {
        return WitnessVerdict::Pending;
    };
    match magic_identity(m) {
        Some(FieldIdentity::Oscillator) => WitnessVerdict::Radiator,
        Some(FieldIdentity::Footprint) | Some(FieldIdentity::Pending) => WitnessVerdict::Pending,
        Some(FieldIdentity::Witness(art)) => {
            if !has_scalar {
                return WitnessVerdict::BareCoordinate;
            }
            if let Some(declared) = declared_art
                && declared != art
            {
                return WitnessVerdict::Pending;
            }
            WitnessVerdict::Holds(art)
        }
        None => WitnessVerdict::Pending,
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SeriesVerdict {
    Hold,
    Reject,
    Pending,
}

pub fn series_gate(magic: Option<[u8; 4]>, samples: &[(f64, f64, f64)]) -> SeriesVerdict {
    let Some(m) = magic else {
        return SeriesVerdict::Pending;
    };
    match magic_identity(m) {
        None => SeriesVerdict::Pending,
        Some(FieldIdentity::Witness(_)) | Some(FieldIdentity::Footprint) => SeriesVerdict::Reject,
        Some(FieldIdentity::Oscillator) | Some(FieldIdentity::Pending) => {
            for &(freq, bin_width, val) in samples {
                if !freq.is_finite() || !bin_width.is_finite() || !val.is_finite() {
                    return SeriesVerdict::Reject;
                }
                if freq < 0.0 || bin_width < 0.0 {
                    return SeriesVerdict::Reject;
                }
            }
            SeriesVerdict::Hold
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalogs_hold_s2_direction() {
        for m in [*b"SKY1", *b"SKD1", *b"VLDE"] {
            assert_eq!(
                magic_identity(m),
                Some(FieldIdentity::Witness(WitnessKind::S2Direction))
            );
        }
    }

    #[test]
    fn transient_event_records_are_point_events() {
        for m in [*b"AMN1", *b"PAO1", *b"S2E1", *b"ERBQ"] {
            assert_eq!(
                magic_identity(m),
                Some(FieldIdentity::Witness(WitnessKind::PointEvent))
            );
        }
    }

    #[test]
    fn gebco_is_gestalt() {
        assert_eq!(
            magic_identity(*b"GBCO"),
            Some(FieldIdentity::Witness(WitnessKind::Gestalt))
        );
    }

    #[test]
    fn gl30_is_gestalt() {
        assert_eq!(
            magic_identity(*b"GL30"),
            Some(FieldIdentity::Witness(WitnessKind::Gestalt))
        );
    }

    #[test]
    fn gl90_is_gestalt() {
        assert_eq!(
            magic_identity(*b"GL90"),
            Some(FieldIdentity::Witness(WitnessKind::Gestalt))
        );
    }

    #[test]
    fn srtm_is_gestalt() {
        assert_eq!(
            magic_identity(*b"SRTM"),
            Some(FieldIdentity::Witness(WitnessKind::Gestalt))
        );
        assert_eq!(
            witness_gate(Some(*b"SRTM"), Some(WitnessKind::Gestalt), true),
            WitnessVerdict::Holds(WitnessKind::Gestalt)
        );
    }

    #[test]
    fn slab2_is_gestalt() {
        assert_eq!(
            magic_identity(*b"SLB2"),
            Some(FieldIdentity::Witness(WitnessKind::Gestalt))
        );
    }

    #[test]
    fn ocs1_is_gestalt() {
        assert_eq!(
            magic_identity(*b"OCS1"),
            Some(FieldIdentity::Witness(WitnessKind::Gestalt))
        );
    }

    #[test]
    fn g3d1_is_gestalt() {
        assert_eq!(
            magic_identity(*b"G3D1"),
            Some(FieldIdentity::Witness(WitnessKind::Gestalt))
        );
        assert_eq!(
            witness_gate(Some(*b"G3D1"), Some(WitnessKind::Gestalt), true),
            WitnessVerdict::Holds(WitnessKind::Gestalt)
        );
    }

    #[test]
    fn gmr1_is_gestalt() {
        assert_eq!(
            magic_identity(*b"GMR1"),
            Some(FieldIdentity::Witness(WitnessKind::Gestalt))
        );
        assert_eq!(
            witness_gate(Some(*b"GMR1"), Some(WitnessKind::Gestalt), true),
            WitnessVerdict::Holds(WitnessKind::Gestalt)
        );
    }

    #[test]
    fn eri1_is_gestalt() {
        assert_eq!(
            magic_identity(*b"ERI1"),
            Some(FieldIdentity::Witness(WitnessKind::Gestalt))
        );
        assert_eq!(
            witness_gate(Some(*b"ERI1"), Some(WitnessKind::Gestalt), true),
            WitnessVerdict::Holds(WitnessKind::Gestalt)
        );
    }

    #[test]
    fn iscb_is_presence() {
        assert_eq!(
            magic_identity(*b"ISCB"),
            Some(FieldIdentity::Witness(WitnessKind::Presence))
        );
        assert_eq!(
            witness_gate(Some(*b"ISCB"), Some(WitnessKind::Presence), true),
            WitnessVerdict::Holds(WitnessKind::Presence)
        );
    }

    #[test]
    fn ehb1_is_presence() {
        assert_eq!(
            magic_identity(*b"EHB1"),
            Some(FieldIdentity::Witness(WitnessKind::Presence))
        );
        assert_eq!(
            witness_gate(Some(*b"EHB1"), Some(WitnessKind::Presence), true),
            WitnessVerdict::Holds(WitnessKind::Presence)
        );
    }

    #[test]
    fn kuprat_lab_channels_are_substance() {
        for m in [*b"RIXS", *b"RIXC", *b"EELS", *b"SRD6"] {
            assert_eq!(
                magic_identity(m),
                Some(FieldIdentity::Witness(WitnessKind::Substance))
            );
        }
    }

    #[test]
    fn rixs_substance_gate_holds() {
        assert_eq!(
            witness_gate(Some(*b"RIXS"), Some(WitnessKind::Substance), true),
            WitnessVerdict::Holds(WitnessKind::Substance)
        );
    }

    #[test]
    fn rixs_substance_gate_refuses_a_contradicting_declared_art() {
        assert_eq!(
            witness_gate(Some(*b"RIXS"), Some(WitnessKind::S2Direction), true),
            WitnessVerdict::Pending
        );
    }

    #[test]
    fn nrs1_stays_pending() {
        assert_eq!(magic_identity(*b"NRS1"), Some(FieldIdentity::Pending));
    }

    #[test]
    fn ptlm_stays_pending() {
        assert_eq!(magic_identity(*b"PTLM"), Some(FieldIdentity::Pending));
        assert_eq!(
            witness_gate(Some(*b"PTLM"), Some(WitnessKind::Presence), true),
            WitnessVerdict::Pending
        );
    }

    #[test]
    fn fp01_is_a_footprint_sibling_not_a_witness() {
        assert_eq!(magic_identity(*b"FP01"), Some(FieldIdentity::Footprint));
        assert_eq!(
            witness_gate(Some(*b"FP01"), Some(WitnessKind::S2Direction), true),
            WitnessVerdict::Pending
        );
        assert_eq!(
            series_gate(Some(*b"FP01"), &[(10.0, 1.0, 3.0)]),
            SeriesVerdict::Reject
        );
    }

    #[test]
    fn oscillator_magics_are_oscillators() {
        for m in [
            *b"BGR1", *b"ARG1", *b"FDS1", *b"GIC1", *b"IGT1", *b"SDN1", *b"CSM1", *b"CRX1",
            *b"MAX1", *b"NXR1", *b"USC1", *b"VSAT",
        ] {
            assert_eq!(magic_identity(m), Some(FieldIdentity::Oscillator));
        }
    }

    #[test]
    fn unknown_magic_is_none() {
        assert_eq!(magic_identity(*b"XXXX"), None);
    }

    #[test]
    fn osm1_is_an_oscillator() {
        assert_eq!(magic_identity(*b"OSM1"), Some(FieldIdentity::Oscillator));
    }

    #[test]
    fn gate_holds_a_scalar_witness() {
        assert_eq!(
            witness_gate(Some(*b"AMN1"), Some(WitnessKind::PointEvent), true),
            WitnessVerdict::Holds(WitnessKind::PointEvent)
        );
    }

    #[test]
    fn gate_refuses_a_radiator() {
        assert_eq!(
            witness_gate(Some(*b"BGR1"), Some(WitnessKind::S2Direction), true),
            WitnessVerdict::Radiator
        );
    }

    #[test]
    fn gate_refuses_a_bare_coordinate() {
        assert_eq!(
            witness_gate(Some(*b"AMN1"), Some(WitnessKind::PointEvent), false),
            WitnessVerdict::BareCoordinate
        );
    }

    #[test]
    fn gate_holds_pending_for_the_spectral_rift() {
        assert_eq!(
            witness_gate(Some(*b"NRS1"), Some(WitnessKind::S2Direction), true),
            WitnessVerdict::Pending
        );
    }

    #[test]
    fn gate_refuses_a_declared_art_that_contradicts_the_record() {
        assert_eq!(
            witness_gate(Some(*b"GBCO"), Some(WitnessKind::S2Direction), true),
            WitnessVerdict::Pending
        );
    }

    #[test]
    fn series_gate_holds_a_well_formed_series() {
        let samples = [(10.0, 1.0, 3.0), (20.0, 1.0, 4.0), (0.0, 0.0, -1.0)];
        assert_eq!(series_gate(Some(*b"NRS1"), &samples), SeriesVerdict::Hold);
        assert_eq!(series_gate(Some(*b"FDS1"), &samples), SeriesVerdict::Hold);
    }

    #[test]
    fn series_gate_rejects_non_finite_samples() {
        assert_eq!(
            series_gate(Some(*b"NRS1"), &[(f64::NAN, 1.0, 3.0)]),
            SeriesVerdict::Reject
        );
        assert_eq!(
            series_gate(Some(*b"NRS1"), &[(10.0, f64::INFINITY, 3.0)]),
            SeriesVerdict::Reject
        );
        assert_eq!(
            series_gate(Some(*b"NRS1"), &[(10.0, 1.0, f64::NAN)]),
            SeriesVerdict::Reject
        );
    }

    #[test]
    fn series_gate_rejects_negative_axis() {
        assert_eq!(
            series_gate(Some(*b"NRS1"), &[(-1.0, 1.0, 3.0)]),
            SeriesVerdict::Reject
        );
        assert_eq!(
            series_gate(Some(*b"NRS1"), &[(10.0, -1.0, 3.0)]),
            SeriesVerdict::Reject
        );
    }

    #[test]
    fn series_gate_rejects_a_witness_record_as_foreign() {
        assert_eq!(
            series_gate(Some(*b"AMN1"), &[(10.0, 1.0, 3.0)]),
            SeriesVerdict::Reject
        );
    }

    #[test]
    fn series_gate_pends_on_an_unknown_magic() {
        assert_eq!(
            series_gate(Some(*b"XXXX"), &[(10.0, 1.0, 3.0)]),
            SeriesVerdict::Pending
        );
        assert_eq!(
            series_gate(None, &[(10.0, 1.0, 3.0)]),
            SeriesVerdict::Pending
        );
    }
}
