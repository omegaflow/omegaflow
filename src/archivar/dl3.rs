use crate::archivar::fits::{FitsHeader, FitsTable};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dl3Event {
    pub ra_deg: f64,
    pub dec_deg: f64,
    pub energy_tev: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SkyCell {
    pub lon_deg: f64,
    pub lat_deg: f64,
    pub energy_tev: f64,
    pub count: u32,
}

fn find_column(table: &FitsTable, name: &str) -> Option<usize> {
    table
        .columns
        .iter()
        .position(|c| c.name.eq_ignore_ascii_case(name))
}

pub fn parse_events(buf: &[u8]) -> Option<Vec<Dl3Event>> {
    let mut off = 0usize;
    while off < buf.len() {
        let (header, _) = FitsHeader::parse(buf, off)?;
        let is_events = header.value("HDUCLAS1") == Some("'EVENTS'")
            || header.value("EXTNAME") == Some("'EVENTS'");
        if header.value("XTENSION") == Some("'BINTABLE'") && is_events {
            let (table, _next) = FitsTable::parse(buf, off)?;
            return events_from_table(buf, &table);
        }
        let next = if header.value("XTENSION") == Some("'BINTABLE'") {
            FitsTable::parse(buf, off)?.1
        } else {
            let (_, hdr_end) = FitsHeader::parse(buf, off)?;
            hdr_end
        };
        if next <= off || next >= buf.len() {
            return None;
        }
        off = next;
    }
    None
}

fn events_from_table(buf: &[u8], table: &FitsTable) -> Option<Vec<Dl3Event>> {
    let ra_idx = find_column(table, "RA")?;
    let dec_idx = find_column(table, "DEC")?;
    let energy_idx = find_column(table, "ENERGY")?;
    let ra = &table.columns[ra_idx];
    let dec = &table.columns[dec_idx];
    let energy = &table.columns[energy_idx];
    let mut out = Vec::with_capacity(table.n_rows);
    for r in 0..table.n_rows {
        let ra_deg = table.cell_f64(buf, r, ra)?;
        let dec_deg = table.cell_f64(buf, r, dec)?;
        let energy_tev = table.cell_f64(buf, r, energy)?;
        if !ra_deg.is_finite() || !dec_deg.is_finite() || !energy_tev.is_finite() {
            continue;
        }
        if !(0.0..=360.0).contains(&ra_deg) || !(-90.0..=90.0).contains(&dec_deg) {
            continue;
        }
        out.push(Dl3Event {
            ra_deg,
            dec_deg,
            energy_tev,
        });
    }
    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}

pub fn reduce_grid(events: &[Dl3Event], n_lon: usize, n_lat: usize) -> Vec<SkyCell> {
    if n_lon == 0 || n_lat == 0 {
        return Vec::new();
    }
    let dlon = 360.0 / n_lon as f64;
    let dlat = 180.0 / n_lat as f64;
    let mut cells: Vec<Vec<SkyCell>> = vec![Vec::new(); n_lon * n_lat];
    for e in events {
        let ilon = ((e.ra_deg / dlon).floor() as usize).min(n_lon - 1);
        let ilat = (((e.dec_deg + 90.0) / dlat).floor() as usize).min(n_lat - 1);
        let idx = ilat * n_lon + ilon;
        let cell = &mut cells[idx];
        if let Some(c) = cell.last_mut() {
            c.energy_tev += e.energy_tev;
            c.count += 1;
        } else {
            cell.push(SkyCell {
                lon_deg: (ilon as f64 + 0.5) * dlon,
                lat_deg: (ilat as f64 + 0.5) * dlat - 90.0,
                energy_tev: e.energy_tev,
                count: 1,
            });
        }
    }
    cells.into_iter().flatten().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pad_card(kw: &str, value: &str) -> [u8; 80] {
        let mut card = [b' '; 80];
        let k = kw.as_bytes();
        card[..k.len().min(8)].copy_from_slice(&k[..k.len().min(8)]);
        card[8] = b'=';
        let v = value.as_bytes();
        card[10..10 + v.len().min(20)].copy_from_slice(&v[..v.len().min(20)]);
        card
    }

    fn synth_events(rows: &[[f32; 3]]) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut primary: Vec<u8> = Vec::new();
        primary.extend_from_slice(&pad_card("SIMPLE", "T"));
        primary.extend_from_slice(&pad_card("BITPIX", "8"));
        primary.extend_from_slice(&pad_card("NAXIS", "0"));
        primary.extend_from_slice(&pad_card("END", ""));
        while !primary.len().is_multiple_of(2880) {
            primary.extend_from_slice(&[b' '; 80]);
        }
        buf.extend_from_slice(&primary);

        let mut ext: Vec<u8> = Vec::new();
        ext.extend_from_slice(&pad_card("XTENSION", "'BINTABLE'"));
        ext.extend_from_slice(&pad_card("BITPIX", "8"));
        ext.extend_from_slice(&pad_card("NAXIS", "2"));
        ext.extend_from_slice(&pad_card("NAXIS1", "12"));
        ext.extend_from_slice(&pad_card("NAXIS2", &rows.len().to_string()));
        ext.extend_from_slice(&pad_card("PCOUNT", "0"));
        ext.extend_from_slice(&pad_card("GCOUNT", "1"));
        ext.extend_from_slice(&pad_card("TFIELDS", "3"));
        ext.extend_from_slice(&pad_card("EXTNAME", "'EVENTS'"));
        ext.extend_from_slice(&pad_card("HDUCLAS1", "'EVENTS'"));
        ext.extend_from_slice(&pad_card("TTYPE1", "'RA'"));
        ext.extend_from_slice(&pad_card("TFORM1", "E"));
        ext.extend_from_slice(&pad_card("TBCOL1", "1"));
        ext.extend_from_slice(&pad_card("TTYPE2", "'DEC'"));
        ext.extend_from_slice(&pad_card("TFORM2", "E"));
        ext.extend_from_slice(&pad_card("TBCOL2", "5"));
        ext.extend_from_slice(&pad_card("TTYPE3", "'ENERGY'"));
        ext.extend_from_slice(&pad_card("TFORM3", "E"));
        ext.extend_from_slice(&pad_card("TBCOL3", "9"));
        ext.extend_from_slice(&pad_card("END", ""));
        while !ext.len().is_multiple_of(2880) {
            ext.extend_from_slice(&[b' '; 80]);
        }
        buf.extend_from_slice(&ext);
        for row in rows {
            for v in row {
                buf.extend_from_slice(&v.to_be_bytes());
            }
        }
        while !buf.len().is_multiple_of(2880) {
            buf.push(0);
        }
        buf
    }

    #[test]
    fn events_read_from_bintable() {
        let buf = synth_events(&[
            [83.6331, 22.0145, 1.2],
            [83.6331, 22.0145, 0.8],
            [84.0, 22.0, 0.5],
        ]);
        let events = parse_events(&buf).unwrap();
        assert_eq!(events.len(), 3);
        assert!((events[0].ra_deg - 83.6331).abs() < 1e-3);
        assert!((events[0].energy_tev - 1.2).abs() < 1e-4);
    }

    #[test]
    fn events_absent_when_no_events_table() {
        let mut buf = Vec::new();
        let mut primary: Vec<u8> = Vec::new();
        primary.extend_from_slice(&pad_card("SIMPLE", "T"));
        primary.extend_from_slice(&pad_card("BITPIX", "8"));
        primary.extend_from_slice(&pad_card("NAXIS", "0"));
        primary.extend_from_slice(&pad_card("END", ""));
        while !primary.len().is_multiple_of(2880) {
            primary.extend_from_slice(&[b' '; 80]);
        }
        buf.extend_from_slice(&primary);
        assert!(parse_events(&buf).is_none());
    }

    #[test]
    fn grid_sums_energy_into_cells() {
        let events = vec![
            Dl3Event {
                ra_deg: 10.0,
                dec_deg: 0.0,
                energy_tev: 1.0,
            },
            Dl3Event {
                ra_deg: 10.0,
                dec_deg: 0.0,
                energy_tev: 2.0,
            },
            Dl3Event {
                ra_deg: 190.0,
                dec_deg: -45.0,
                energy_tev: 3.0,
            },
        ];
        let cells = reduce_grid(&events, 36, 18);
        assert_eq!(cells.len(), 2);
        let first = cells.iter().find(|c| c.count == 2).unwrap();
        assert!((first.energy_tev - 3.0).abs() < 1e-9);
        let second = cells.iter().find(|c| c.count == 1).unwrap();
        assert!((second.energy_tev - 3.0).abs() < 1e-9);
    }
}
