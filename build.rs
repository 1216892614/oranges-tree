use std::io::{BufRead, BufReader};
use std::{env, path::Path, process::Command};
use std::{fs, fs::File};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let codegen_dest_path = Path::new(&out_dir).join("generated.rs");
    env::set_var("PRISMA_OUT_FILE", &codegen_dest_path);
    env::set_var("CARGO_TARGET_DIR", &out_dir);
    let _ = Command::new("cargo")
        .args(&["run", "-p", "prisma-cli", "generate"])
        .output()
        .expect("Failed run to codegen command");

    let mut lines = lines_from_file(&codegen_dest_path);

    for line in &mut lines {
        if line.contains("#!") {
            *line = line.replace("#!", "#");
        }
    }

    fs::write(&codegen_dest_path, lines.join("\n")).expect("Failed to update file");
    println!("cargo:rerun-if-changed=/prisma");
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
