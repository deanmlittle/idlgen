use std::fs::File;
use std::io::{BufReader, Write};
use std::str::FromStr;
use anyhow::Result;
use clap::Parser;
use convert_case::{Case, Casing};
use generators::common::make_sdk;
use serde::Serialize;
use types::IDL;
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

#[derive(
    clap::ValueEnum, Clone, Default, Debug, Serialize,
)]
#[serde(rename_all = "kebab-case")]
pub enum Package {
    /// A single file
    #[default]
    File,
    /// A usable crate
    Crate,
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
    let mut file = File::create(format!("{}.rs", idl.name.to_case(Case::Snake)))?;
    file.write_all(make_sdk(idl, sdk).as_bytes())?;
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
