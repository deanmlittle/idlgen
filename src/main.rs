use std::fs::{create_dir_all, File};
use std::io::{BufReader, Write};
use anyhow::Result;
use clap::Parser;
use convert_case::{Case, Casing};
use generators::common::{make_cargo_toml, make_lib_rs};
use serde::Serialize;
use types::IDL;
use zip::write::FileOptions;
use zip::ZipWriter;
mod types;
mod generators;

#[derive(
    clap::ValueEnum, Clone, Default, Debug, Serialize,
)]
#[serde(rename_all = "kebab-case")]
pub enum Sdk {
    /// Full SDK
    #[default]
    Full,
    /// Introspection only
    I11n,
    /// CPI only
    CPI,
}

impl ToString for Sdk {
    fn to_string(&self) -> String {
        match self {
            Sdk::Full => "full".to_string(),
            Sdk::I11n => "i11n".to_string(),
            Sdk::CPI => "CPI".to_string(),
        }
    }
}

#[derive(
    clap::ValueEnum, Clone, Default, Debug, Serialize,
)]
#[serde(rename_all = "kebab-case")]
pub enum Package {
    /// A single file
    File,
    /// A production-ready crate
    #[default]
    Crate,

    Zip,
}

impl ToString for Package {
    fn to_string(&self) -> String {
        match self {
            Package::File => "file".to_string(),
            Package::Crate => "crate".to_string(),
            Package::Zip => "zip".to_string(),
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Filename of IDL file
    #[arg(short, long)]
    filename: String,
    /// SDK Type (Full, I11n or CPI)
    #[clap(short, long, default_value_t, value_enum)]
    sdk: Sdk,
    /// Type of Package to generate (Crate or File)
    #[clap(short, long, default_value_t, value_enum)]
    package: Package,
}

fn make(idl: &IDL, sdk: &Sdk, package: &Package) -> Result<()> {
    match package {
        Package::File => {
            let mut file = File::create(format!("{}.rs", idl.name.to_case(Case::Snake)))?;
            file.write_all(make_lib_rs(idl, sdk).as_bytes())?;
        },
        Package::Crate => {
            create_dir_all(format!("{}/src", idl.name.to_case(Case::Snake)))?;
            let mut toml_file: File = File::create(format!("{}/Cargo.toml", idl.name.to_case(Case::Snake)))?;
            toml_file.write_all(make_cargo_toml(idl, sdk).as_bytes())?;

            let mut lib_file: File = File::create(format!("{}/src/lib.rs", idl.name.to_case(Case::Snake)))?;
            lib_file.write_all(make_lib_rs(idl, sdk).as_bytes())?;
        },
        Package::Zip => {
            let file = File::create(format!("{}.zip", idl.name.to_case(Case::Snake)))?;

            let mut zip = ZipWriter::new(file);

            zip.start_file("Cargo.toml", FileOptions::default())?;
            zip.write_all(make_cargo_toml(idl, sdk).as_bytes())?;
            zip.add_directory("src", FileOptions::default())?;
            zip.start_file("src/lib.rs", FileOptions::default())?;
            zip.write_all(make_lib_rs(idl, sdk).as_bytes())?;
            zip.finish()?;
        },
    }
    println!("✅ Successfully generated {} {} for {} v{}", sdk.to_string(), package.to_string(), idl.name, idl.version);
    Ok(())
}

fn main() -> Result<()> { 
    let args = Args::parse();
    let file = File::open(&args.filename)?;
    let reader = BufReader::new(file);

    let idl: IDL = serde_json::from_reader(reader)?;

    make(&idl, &args.sdk, &args.package)?;
    Ok(())
}
