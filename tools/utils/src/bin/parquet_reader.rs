use omegaflow::parquet::{
    codec_name, converted_type_name, encoding_name, parquet_type_name, repetition_name, ColumnChunk,
    FileMetaData, ParquetNote,
};
use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(first) = args.first() else {
        eprintln!("usage: parquet_reader <file.parquet>");
        eprintln!("       parquet_reader --url <url>");
        return;
    };
    let bytes = if first == "--url" {
        let Some(url) = args.get(1) else {
            eprintln!("usage: parquet_reader --url <url>");
            return;
        };
        match fetch(url) {
            Some(b) => b,
            None => {
                println!("curl fetch without answer: {}", url);
                return;
            }
        }
    } else {
        match std::fs::read(first) {
            Ok(b) => b,
            Err(_) => {
                println!("file does not open: {}", first);
                return;
            }
        }
    };
    match FileMetaData::parse(&bytes) {
        Ok(m) => struktur(&m),
        Err(note) => println!("{}", note_text(&note)),
    }
}

fn fetch(url: &str) -> Option<Vec<u8>> {
    let out = Command::new("curl").arg("-sSf").arg(url).output().ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        None
    }
}

fn note_text(note: &ParquetNote) -> String {
    match note {
        ParquetNote::Magic { bytes } => format!(
            "magic is {:02X} {:02X} {:02X} {:02X} — not a Parquet file",
            bytes[0], bytes[1], bytes[2], bytes[3]
        ),
        ParquetNote::FooterLength { len, file } => {
            format!("footer length {} reaches past the file end at byte {}", len, file)
        }
        ParquetNote::Truncated { off } => format!("footer ends at byte {} — truncated footer", off),
        ParquetNote::Type { tag, off } => format!("compact type {} at byte {} unread", tag, off),
        ParquetNote::FieldId { off } => format!("field id at byte {} leaves the i16 range", off),
        ParquetNote::AbsentField { id } => format!("FileMetaData field {} absent", id),
    }
}

fn type_text(tag: Option<i32>) -> String {
    match tag {
        Some(t) => match parquet_type_name(t) {
            Some(n) => format!("{} ({})", n, t),
            None => format!("{}", t),
        },
        None => "absent".to_string(),
    }
}

fn rep_text(tag: Option<i32>) -> String {
    match tag {
        Some(t) => match repetition_name(t) {
            Some(n) => format!("{} ({})", n, t),
            None => format!("{}", t),
        },
        None => "absent".to_string(),
    }
}

fn converted_text(tag: Option<i32>) -> String {
    match tag {
        Some(t) => match converted_type_name(t) {
            Some(n) => format!("{} ({})", n, t),
            None => format!("{}", t),
        },
        None => "absent".to_string(),
    }
}

fn codec_text(tag: i32) -> String {
    match codec_name(tag) {
        Some(n) => format!("{} ({})", n, tag),
        None => format!("{}", tag),
    }
}

fn opt_i32(v: Option<i32>) -> String {
    match v {
        Some(n) => format!("{}", n),
        None => "absent".to_string(),
    }
}

fn column_text(c: &ColumnChunk) -> String {
    let path = match &c.file_path {
        Some(p) => format!("{}", p),
        None => "absent".to_string(),
    };
    match &c.meta_data {
        Some(m) => {
            let enc: Vec<String> = m
                .encodings
                .iter()
                .map(|e| match encoding_name(*e) {
                    Some(n) => format!("{} ({})", n, e),
                    None => format!("{}", e),
                })
                .collect();
            format!(
                "file_path={} file_offset={} type={} codec={} encodings=[{}] path=[{}] num_values={} uncompressed={} compressed={} data_page_offset={} dictionary_page_offset={}",
                path,
                c.file_offset,
                type_text(Some(m.type_tag)),
                codec_text(m.codec),
                enc.join(", "),
                m.path_in_schema.join("."),
                m.num_values,
                m.total_uncompressed_size,
                m.total_compressed_size,
                m.data_page_offset,
                match m.dictionary_page_offset {
                    Some(v) => format!("{}", v),
                    None => "absent".to_string(),
                }
            )
        }
        None => format!(
            "file_path={} file_offset={} meta_data=absent",
            path, c.file_offset
        ),
    }
}

fn struktur(m: &FileMetaData) {
    println!("version: {}", m.version);
    println!("num_rows: {}", m.num_rows);
    match &m.created_by {
        Some(s) => println!("created_by: {}", s),
        None => println!("created_by: absent"),
    }
    println!("schema: {} elements", m.schema.len());
    for (i, e) in m.schema.iter().enumerate() {
        println!(
            "  [{}] name={} type={} repetition_type={} num_children={} type_length={} converted_type={}",
            i,
            e.name,
            type_text(e.type_tag),
            rep_text(e.repetition_type),
            opt_i32(e.num_children),
            opt_i32(e.type_length),
            converted_text(e.converted_type)
        );
    }
    println!("row_groups: {}", m.row_groups.len());
    for (gi, g) in m.row_groups.iter().enumerate() {
        println!(
            "  [{}] total_byte_size={} num_rows={} columns={}",
            gi,
            g.total_byte_size,
            g.num_rows,
            g.columns.len()
        );
        for (ci, c) in g.columns.iter().enumerate() {
            println!("    [{}] {}", ci, column_text(c));
        }
    }
    match &m.key_value_metadata {
        Some(kv) => {
            println!("key_value_metadata: {}", kv.len());
            for (i, p) in kv.iter().enumerate() {
                match &p.value {
                    Some(v) => println!("  [{}] {} = {}", i, p.key, v),
                    None => println!("  [{}] {} = absent", i, p.key),
                }
            }
        }
        None => println!("key_value_metadata: absent"),
    }
}
