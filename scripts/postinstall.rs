use std::{
    fs::{self, File},
    io::Write,
};

fn main() {
    let mut types = vec![];
    for entry in fs::read_dir("bindings").unwrap() {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();
        types.push(file_name.replace(".ts", ""));
    }

    let mut index_d_ts = File::create("types/index.d.ts").unwrap();
    for t in types.iter() {
        writeln!(index_d_ts, "export type {{ {t} }} from \"./{t}\";").unwrap();
    }
}
