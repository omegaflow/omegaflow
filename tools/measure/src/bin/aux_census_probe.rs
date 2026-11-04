use omegaflow::archivar::aux;

fn main() {
    let arg = std::env::args().skip(1).find(|a| !a.starts_with('-'));
    let Some(path) = arg else {
        eprintln!("aux-census: --file absent — no interpolation file to read");
        return;
    };
    let Ok(text) = std::fs::read_to_string(&path) else {
        eprintln!("aux-census: read void ({path})");
        return;
    };
    let Some(file) = aux::parse_aux(&text) else {
        eprintln!("aux-census: parse void — {path} is not an interpolation file");
        return;
    };
    let total = aux::records(&file);
    println!(
        "aux-census {path}: file type {} · {total} record(s) in {} block(s) · {} column(s)",
        file.file_type,
        file.blocks.len(),
        file.columns
    );
    if let Some((first, last)) = aux::span(&file) {
        println!("aux-census: MJD2000 {first:.6} .. {last:.6}");
    }
    for i in 0..file.columns {
        if let Some((low, high)) = aux::column_range(&file, i) {
            println!("aux-census: column {} {low:.9e} .. {high:.9e}", i + 1);
        }
    }
}
