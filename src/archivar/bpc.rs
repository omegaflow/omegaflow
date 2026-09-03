use crate::bsp_reader::daf::DafFile;
use std::path::Path;

pub struct BpcSegment {
    body: i32,
    frame: i32,
    data_type: u32,
    init: f64,
    intlen: f64,
    rsize: usize,
    n_records: usize,
    n_coef: usize,
    start_addr: u32,
    file: DafFile,
}

pub struct BpcFile {
    segments: Vec<BpcSegment>,
}

fn cheby_val(c: &[f64], s: f64) -> f64 {
    let mut b0 = 0.0;
    let mut b1 = 0.0;
    for i in (0..c.len()).rev() {
        let b2 = b1;
        b1 = b0;
        b0 = 2.0 * s * b1 - b2 + c[i];
    }
    b0 - s * b1
}

impl BpcSegment {
    fn from_segment(
        file: &DafFile,
        start_addr: u32,
        end_addr: u32,
        body: i32,
        frame: i32,
        data_type: u32,
    ) -> Result<Self, String> {
        let trailer = file
            .read_doubles(end_addr - 3, end_addr)
            .map_err(|e| e.to_string())?;
        let init = trailer[0];
        let intlen = trailer[1];
        let rsize = trailer[2] as usize;
        let n_records = trailer[3] as usize;
        if rsize < 2 || (rsize - 2) % 3 != 0 {
            return Err("PCK type 2 RSIZE not 2 + 3N".to_string());
        }
        let n_coef = (rsize - 2) / 3;
        if n_coef == 0 || intlen <= 0.0 {
            return Err("PCK type 2 degree < 0 or INTLEN <= 0".to_string());
        }
        Ok(BpcSegment {
            body,
            frame,
            data_type,
            init,
            intlen,
            rsize,
            n_records,
            n_coef,
            start_addr,
            file: file.clone(),
        })
    }

    fn evaluate(&self, et: f64) -> Option<(f64, f64, f64)> {
        if self.data_type != 2 {
            return None;
        }
        let raw_idx = ((et - self.init) / self.intlen).floor() as isize;
        if raw_idx < 0 || raw_idx >= self.n_records as isize {
            return None;
        }
        let idx = raw_idx as usize;
        let rec_start = self.start_addr + (idx * self.rsize) as u32;
        let rec_end = rec_start + self.rsize as u32 - 1;
        let rec = self.file.doubles_native(rec_start, rec_end).ok()?;
        let mid = rec[0];
        let radius = rec[1];
        if radius == 0.0 {
            return None;
        }
        let s = (et - mid) / radius;
        let n = self.n_coef;
        Some((
            cheby_val(&rec[2..2 + n], s),
            cheby_val(&rec[2 + n..2 + 2 * n], s),
            cheby_val(&rec[2 + 2 * n..2 + 3 * n], s),
        ))
    }
}

impl BpcFile {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let daf = DafFile::open(path).map_err(|e| e.to_string())?;
        let mut segments = Vec::new();
        for summary in daf.summaries().map_err(|e| e.to_string())? {
            if summary.doubles.len() < 2 || summary.integers.len() < 5 {
                continue;
            }
            let body = summary.integers[0];
            let frame = summary.integers[1];
            let data_type = summary.integers[2] as u32;
            let start_addr = summary.integers[3] as u32;
            let end_addr = summary.integers[4] as u32;
            match data_type {
                2 => match BpcSegment::from_segment(
                    &daf, start_addr, end_addr, body, frame, data_type,
                ) {
                    Ok(s) => {
                        eprintln!(
                            "bpc: body {} frame {} type {} records {} window {:.0}-{:.0}",
                            body,
                            frame,
                            data_type,
                            s.n_records,
                            s.init / 86400.0 + 2451545.0,
                            (s.init + s.n_records as f64 * s.intlen) / 86400.0 + 2451545.0
                        );
                        segments.push(s)
                    }
                    Err(e) => eprintln!("bpc segment ({} {}): {}", body, frame, e),
                },
                3 | 20 => eprintln!(
                    "bpc segment ({} {}): type {} not consumed by flattener",
                    body, frame, data_type
                ),
                _ => {}
            }
        }
        Ok(BpcFile { segments })
    }

    pub fn orient(&self, body: i32, frame: i32, et: f64) -> Option<(f64, f64, f64)> {
        for seg in &self.segments {
            if seg.body == body && seg.frame == frame {
                if let Some(v) = seg.evaluate(et) {
                    return Some(v);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::cheby_val;

    #[test]
    fn cheby_val_linear_combination() {
        let v = cheby_val(&[3.0, 2.0], 0.5);
        assert!((v - 4.0).abs() < 1e-12);
    }

    #[test]
    fn cheby_val_constant() {
        let v = cheby_val(&[7.0], -0.3);
        assert!((v - 7.0).abs() < 1e-12);
    }
}
