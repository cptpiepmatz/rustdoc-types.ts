use std::{
    fmt::Write,
    fs,
    io::Read,
    ops::Deref,
};

use flate2::read::GzDecoder;
use reqwest::{
    blocking::Client,
    header::USER_AGENT,
};
use serde::Deserialize;
use tar::Archive;

#[derive(Debug, Deserialize)]
struct ThisPackage {
    pub name: String,
    pub version: String,
    pub bugs: ThisPackageBugs,
}

#[derive(Debug, Deserialize)]
struct ThisPackageBugs {
    pub url: String,
}

#[derive(Debug, Deserialize)]
struct MainPackage {
    #[serde(rename = "rustdoc-types")]
    pub rustdoc_types: Option<MainPackageRustDocTypes>,
}

#[derive(Debug, Deserialize)]
struct MainPackageRustDocTypes {
    pub version: Option<String>,
}

const CRATE_NAME: &'static str = "rustdoc-types";
const MAIN_PACKAGE: &'static str =
    include_str!("D:\\Projects\\tmp\\test-rustdoc-types.ts/package.json"); // include_str!(concat!(env!("INIT_CWD"), "/package.json"));
const THIS_PACKAGE: &'static str = include_str!("D:\\Projects\\rustdoc-types.ts\\package.json"); // include_str!(env!("npm_package_json"));

fn main() {
    let main_package: MainPackage = serde_json::from_str(&MAIN_PACKAGE).unwrap();
    let this_package: ThisPackage = serde_json::from_str(&THIS_PACKAGE).unwrap();

    let user_agent = format!(
        "{}/{} (+{})",
        this_package.name, this_package.version, this_package.bugs.url
    );

    let mut download_path = String::new();
    write!(
        download_path,
        "https://crates.io/api/v1/crates/{CRATE_NAME}"
    )
    .unwrap();
    if let Some(version) = main_package.rustdoc_types.and_then(|config| config.version) {
        write!(download_path, "/{version}").unwrap();
    }
    write!(download_path, "/download").unwrap();

    let client = Client::new();
    let response = client
        .get(download_path)
        .header(USER_AGENT, user_agent)
        .send()
        .unwrap();
    let bytes = response.bytes().unwrap();

    fs::create_dir_all("src").unwrap();
    fs::write(format!("src/{CRATE_NAME}.tar.gz"), &bytes).unwrap();

    let decompressed = GzDecoder::new(bytes.deref());
    let mut archive = Archive::new(decompressed);

    let lib_rs = archive
        .entries()
        .unwrap()
        .find_map(|entry| {
            let mut entry = entry.unwrap();
            let path = entry.path().unwrap();
            if path.to_string_lossy().ends_with("lib.rs") {
                let mut s = String::new();
                entry.read_to_string(&mut s).unwrap();
                return Some(s);
            }
            None
        })
        .unwrap();

    let lib_rs = lib_rs
        .replace("#[derive(", "#[derive(ts_rs::TS, ")
        .replace("pub struct", "#[ts(export)] pub struct")
        .replace("pub enum", "#[ts(export)] pub enum")
        .replace("mod tests;", "mod tests {}");

    fs::write("src/lib.rs", lib_rs).unwrap();
}
