use std::fs;

use rustdoc_types_ts::FORMAT_VERSION;

fn main() {
    let content = format!("export const FORMAT_VERSION = {FORMAT_VERSION} as const;");
    fs::write("bindings/FORMAT_VERSION.ts", content).unwrap();
}
