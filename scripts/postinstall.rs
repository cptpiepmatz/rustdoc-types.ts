use std::{
    fs::{self, File},
    io::Write,
};

use rustdoc_types_ts::FORMAT_VERSION;

fn main() {
    let mut types = vec![];
    for entry in fs::read_dir("bindings").unwrap() {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();
        types.push(file_name.replace(".ts", ""));
    }

    let mut index_d_ts = File::create("dist/index.d.ts").unwrap();
    writeln!(index_d_ts, "declare module \"rustdoc-types.ts\";").unwrap();
    for t in types.iter() {
        writeln!(index_d_ts, "export type {{ {t} }} from \"./types/{t}.js\";").unwrap();
    }

    let esm_content = format!("export const FORMAT_VERSION = {FORMAT_VERSION};");
    fs::write("dist/index.js", esm_content).unwrap();

    let commonjs_content = format!("exports.FORMAT_VERSION = {FORMAT_VERSION};");
    fs::write("dist/index.cjs", commonjs_content).unwrap();
}
