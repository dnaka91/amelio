use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;

use ignore::{DirEntry, Walk};
use sha1::{Digest, Sha1};

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    let mut map = HashMap::new();

    // Iterate over all non-hidden files, generate their SHA-1 hash and combine them with their
    // relative file path.
    for entry in Walk::new("assets") {
        let entry = entry.unwrap();

        if !is_file(&entry) {
            continue;
        }

        let data = fs::read(entry.path()).unwrap();
        let hash = Sha1::digest(&data);
        let hash_str = hex::encode(hash);

        map.insert(
            entry
                .path()
                .strip_prefix("assets")
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned(),
            format!("\"{}\"", hash_str),
        );
    }

    let mut phf_map = phf_codegen::Map::new();
    for (k, v) in &map {
        phf_map.entry(k.as_str(), v);
    }

    writeln!(
        &mut file,
        "/// HTTP `ETag` values for all embedded assets.
        #[allow(clippy::unreadable_literal)] \
        static ETAGS: phf::Map<&'static str, &'static str> = {};",
        phf_map.build()
    )
    .unwrap();
}

fn is_file(entry: &DirEntry) -> bool {
    entry.file_type().map(|ft| ft.is_file()).unwrap_or_default()
}
