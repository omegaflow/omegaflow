use omegaflow::cdn::upload_asset;
use omegaflow::inflate::gunzip_stream;
use omegaflow::json::parse_json;
use omegaflow::mpcorb::{encode_record, is_distant_object, rec_from_object};

struct ObjectScanner {
    hold: Vec<u8>,
    start: usize,
    array_closed: bool,
}

impl ObjectScanner {
    fn new() -> ObjectScanner {
        ObjectScanner {
            hold: Vec::new(),
            start: 0,
            array_closed: false,
        }
    }

    fn feed(&mut self, chunk: &[u8]) {
        self.hold.extend_from_slice(chunk);
    }

    fn top_object(&mut self) -> Option<Vec<u8>> {
        if self.array_closed {
            return None;
        }
        loop {
            if self.start == self.hold.len() {
                self.hold.clear();
                self.start = 0;
                return None;
            }
            let d = &self.hold[self.start..];
            match d[0] {
                b' ' | b'\n' | b'\r' | b'\t' | b',' => {
                    self.start += 1;
                }
                b'[' => {
                    self.start += 1;
                }
                b']' => {
                    self.array_closed = true;
                    self.start += 1;
                    return None;
                }
                b'{' => match self.object_end(d) {
                    Some(end) => {
                        let obj = d[..end].to_vec();
                        self.start += end;
                        return Some(obj);
                    }
                    None => {
                        self.hold.drain(..self.start);
                        self.start = 0;
                        return None;
                    }
                },
                _ => {
                    self.hold.drain(..self.start);
                    self.start = 0;
                    return None;
                }
            }
        }
    }

    fn object_end(&self, d: &[u8]) -> Option<usize> {
        let mut depth = 1usize;
        let mut in_string = false;
        let mut i = 1usize;
        while i < d.len() {
            let c = d[i];
            if in_string {
                if c == b'\\' {
                    i += 2;
                    continue;
                }
                if c == b'"' {
                    in_string = false;
                }
            } else {
                match c {
                    b'"' => in_string = true,
                    b'{' => depth += 1,
                    b'}' => {
                        depth -= 1;
                        if depth == 0 {
                            return Some(i + 1);
                        }
                    }
                    _ => {}
                }
            }
            i += 1;
        }
        None
    }
}

struct Counts {
    objects: usize,
    distant: usize,
    emitted: usize,
    json_void: usize,
    rec_void: usize,
}

fn compile_catalog(input: &str, out_path: &str) -> Counts {
    let packed = match std::fs::read(input) {
        Ok(bytes) => bytes,
        Err(e) => panic!("read {}: {}", input, e),
    };
    let mut counts = Counts {
        objects: 0,
        distant: 0,
        emitted: 0,
        json_void: 0,
        rec_void: 0,
    };
    let mut buf = Vec::new();
    let mut scanner = ObjectScanner::new();
    let total = gunzip_stream(&packed[..], |chunk| {
        scanner.feed(chunk);
        while let Some(obj) = scanner.top_object() {
            counts.objects += 1;
            let text = String::from_utf8_lossy(&obj);
            let Some(json) = parse_json(&text) else {
                counts.json_void += 1;
                continue;
            };
            if !is_distant_object(&json) {
                continue;
            }
            counts.distant += 1;
            match rec_from_object(&json) {
                Some(rec) => {
                    encode_record(&rec, &mut buf);
                    counts.emitted += 1;
                }
                None => counts.rec_void += 1,
            }
        }
    });
    match total {
        Ok(total) => eprintln!("decompressed {} bytes", total),
        Err(e) => panic!("gunzip {}: {}", input, e),
    }
    match std::fs::write(out_path, &buf) {
        Ok(()) => {}
        Err(e) => panic!("write {}: {}", out_path, e),
    }
    eprintln!(
        "mpcorb: objects {}, distant {}, emitted {} records, {} B -> {}",
        counts.objects,
        counts.distant,
        counts.emitted,
        buf.len(),
        out_path
    );
    eprintln!(
        "mpcorb: skipped — json_void {}, rec_void {}",
        counts.json_void, counts.rec_void
    );
    counts
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 5 {
        eprintln!(
            "usage: mpcorb_compiler --input <mpcorb_extended.json.gz> --out <mpcorb_distant.bin> [--ci-mode]"
        );
        std::process::exit(1);
    }
    let mut input: Option<String> = None;
    let mut out: Option<String> = None;
    let mut ci_mode = false;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--input" => {
                input = args.get(i + 1).cloned();
                i += 1;
            }
            "--out" => {
                out = args.get(i + 1).cloned();
                i += 1;
            }
            "--ci-mode" => ci_mode = true,
            _ => {}
        }
        i += 1;
    }
    let input = match input {
        Some(p) => p,
        None => {
            eprintln!("--input absent");
            std::process::exit(1);
        }
    };
    let out_path = match out {
        Some(p) => p,
        None => {
            eprintln!("--out absent");
            std::process::exit(1);
        }
    };
    compile_catalog(&input, &out_path);
    if ci_mode && !upload_asset(&out_path) {
        eprintln!("upload: {} did not reach the CDN", out_path);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scanner_splits_measured_pretty_array() {
        let body = concat!(
            "[\n",
            "{\n",
            "\"Number\": \"(1)\",\n",
            "\"Name\": \"Ceres\",\n",
            "\"Principal_desig\": \"A801 AA\",\n",
            "\"Epoch\": 2461200.5,\n",
            "\"Orbit_type\": \"MBA\"\n",
            "},\n",
            "{\n",
            "\"Number\": \"(90377)\",\n",
            "\"Principal_desig\": \"2003 VB12\",\n",
            "\"Epoch\": 2461200.5,\n",
            "\"Orbit_type\": \"Distant Object\",\n",
            "\"Other_desigs\": [\n",
            "\"2003 VB12\",\n",
            "\"2012 VP113\"\n",
            "]\n",
            "}\n",
            "]\n"
        );
        let mut sc = ObjectScanner::new();
        let mut objs = Vec::new();
        let half = body.len() / 2;
        sc.feed(&body.as_bytes()[..half]);
        while let Some(o) = sc.top_object() {
            objs.push(o);
        }
        sc.feed(&body.as_bytes()[half..]);
        while let Some(o) = sc.top_object() {
            objs.push(o);
        }
        assert_eq!(objs.len(), 2);
        assert!(String::from_utf8_lossy(&objs[0]).contains("\"Number\": \"(1)\""));
        assert!(String::from_utf8_lossy(&objs[1]).contains("\"Principal_desig\": \"2003 VB12\""));
    }

    #[test]
    fn scanner_handles_escaped_quote_inside_string() {
        let body = "[\n{\"Principal_desig\":\"A801 \\\"x\",\"Epoch\":1.5}\n]\n";
        let mut sc = ObjectScanner::new();
        sc.feed(body.as_bytes());
        let obj = sc.top_object().unwrap();
        assert!(String::from_utf8_lossy(&obj).contains("\\\"x"));
        assert!(sc.top_object().is_none());
    }
}
