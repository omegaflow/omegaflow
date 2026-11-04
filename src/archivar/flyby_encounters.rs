use crate::archivar::pds3_table::unix_of_iso;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EarthFlyby {
    pub spacecraft: &'static str,
    pub epoch_utc: &'static str,
}

pub const EARTH_FLYBYS: [EarthFlyby; 7] = [
    EarthFlyby {
        spacecraft: "Galileo",
        epoch_utc: "1990-12-08T20:34:34Z",
    },
    EarthFlyby {
        spacecraft: "Galileo",
        epoch_utc: "1992-12-08T15:09:25Z",
    },
    EarthFlyby {
        spacecraft: "Cassini",
        epoch_utc: "1999-08-18T03:28:00Z",
    },
    EarthFlyby {
        spacecraft: "Rosetta",
        epoch_utc: "2005-03-04T22:09:00Z",
    },
    EarthFlyby {
        spacecraft: "MESSENGER",
        epoch_utc: "2005-08-02T19:13:08Z",
    },
    EarthFlyby {
        spacecraft: "Rosetta",
        epoch_utc: "2007-11-13T20:57:00Z",
    },
    EarthFlyby {
        spacecraft: "Rosetta",
        epoch_utc: "2009-11-13T07:45:00Z",
    },
];

pub fn earth_flybys_by_spacecraft(spacecraft: &str) -> impl Iterator<Item = &'static EarthFlyby> {
    EARTH_FLYBYS
        .iter()
        .filter(move |e| e.spacecraft.eq_ignore_ascii_case(spacecraft))
}

pub fn earth_flyby_unix(epoch_utc: &str) -> Option<f64> {
    unix_of_iso(epoch_utc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn earth_flyby_table_carries_seven_encounters() {
        assert_eq!(EARTH_FLYBYS.len(), 7);
        assert_eq!(earth_flybys_by_spacecraft("rosetta").count(), 3);
        assert_eq!(earth_flybys_by_spacecraft("Galileo").count(), 2);
        assert_eq!(earth_flybys_by_spacecraft("MESSENGER").count(), 1);
    }

    #[test]
    fn earth_flyby_epochs_resolve_to_unix_and_are_ordered() {
        let mut last = 0.0f64;
        for e in &EARTH_FLYBYS {
            let t = earth_flyby_unix(e.epoch_utc).expect("epoch parses");
            assert!(
                t > last,
                "{} not after the previous encounter",
                e.spacecraft
            );
            last = t;
        }
    }
}
