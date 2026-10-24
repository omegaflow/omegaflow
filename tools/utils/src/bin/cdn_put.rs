use omegaflow::archivar::cdn::{tag_is_admitted, upload_release};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (Some(tag), Some(path)) = (args.first(), args.get(1)) else {
        eprintln!("usage: cdn_put <tag> <file>");
        std::process::exit(2);
    };
    if !tag_is_admitted(tag) {
        eprintln!(
            "cdn_put {tag}: not an admitted licence in src/archivar/licences.tsv — refused before any upload"
        );
        std::process::exit(2);
    }
    if !upload_release(tag, path) {
        std::process::exit(1);
    }
}
